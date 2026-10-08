use std::{error::Error, fs, io, path::Path, path::PathBuf, process::ExitCode};
use yaml_serde::Value;

fn main() -> ExitCode {
    let mut skills = Vec::new();
    let mut failed = false;
    for directory in ["skills", "draft-skills"].map(Path::new) {
        // Git doesn't track empty directories, so a missing one just has no skills.
        if let Ok(false) = directory.try_exists() {
            continue;
        }
        if let Err(error) = collect_skills(directory, &mut skills) {
            eprintln!("{}: {error}", directory.display());
            failed = true;
        }
    }
    skills.sort();
    if skills.is_empty() {
        eprintln!("no SKILL.md files found in skills/ or draft-skills/");
        failed = true;
    }
    for skill in &skills {
        if let Err(error) = validate_skill(skill) {
            eprintln!("{error}");
            failed = true;
        }
    }
    if failed {
        return ExitCode::FAILURE;
    }
    println!("Validated invocation settings for {} skills.", skills.len());
    ExitCode::SUCCESS
}

fn collect_skills(directory: &Path, skills: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            collect_skills(&entry.path(), skills)?;
        } else if entry.file_name() == "SKILL.md" {
            skills.push(entry.path());
        }
    }
    Ok(())
}

fn frontmatter(markdown: &str) -> Result<&str, &'static str> {
    // Editors hide a byte order mark, so the missing `---` error below would be confusing.
    if markdown.starts_with('\u{feff}') {
        return Err("file must not start with a byte order mark");
    }
    let is_delimiter = |line: &str| line.trim_end() == "---";
    let mut lines = markdown.split_inclusive('\n');
    let start = lines
        .next()
        .filter(|line| is_delimiter(line))
        .ok_or("expected YAML frontmatter starting with ---")?
        .len();
    let mut end = start;
    for line in lines {
        if is_delimiter(line) {
            // Starting at the opening line's `\n` makes YAML error line numbers match the file.
            return Ok(&markdown[start - 1..end]);
        }
        // Claude Code ends the frontmatter at the first `---`, even mid-line.
        if line.contains("---") {
            return Err("frontmatter must not contain --- before the closing delimiter");
        }
        end += line.len();
    }
    Err("missing closing --- for YAML frontmatter")
}

/// Reads `path` and parses its text, prefixing any error with the path.
fn read(
    path: &Path,
    parse: impl FnOnce(&str) -> Result<bool, Box<dyn Error>>,
) -> Result<bool, String> {
    fs::read_to_string(path)
        .map_err(Box::from)
        .and_then(|text| parse(&text))
        .map_err(|error| format!("{}: {error}", path.display()))
}

/// Returns the boolean at the dotted `key` of `yaml`.
fn flag(yaml: &str, key: &str) -> Result<bool, Box<dyn Error>> {
    let value: Value = yaml_serde::from_str(yaml)?;
    key.split('.')
        .fold(&value, |node, part| &node[part])
        .as_bool()
        .ok_or_else(|| format!("{key} must be an explicit boolean").into())
}

fn validate_skill(skill_path: &Path) -> Result<(), String> {
    let disabled = read(skill_path, |text| {
        flag(frontmatter(text)?, "disable-model-invocation")
    })?;
    let agent_path = skill_path.with_file_name("agents/openai.yaml");
    let allowed = read(&agent_path, |text| {
        flag(text, "policy.allow_implicit_invocation")
    })?;
    if disabled == allowed {
        return Err(format!(
            "{}: disable-model-invocation is {disabled}, but {}: policy.allow_implicit_invocation is {allowed}; they must be opposite booleans",
            skill_path.display(),
            agent_path.display(),
        ));
    }
    Ok(())
}
