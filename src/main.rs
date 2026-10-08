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
    let is_delimiter = |line: &str| matches!(line, "---" | "---\n" | "---\r\n");
    let markdown = markdown.trim_start_matches('\u{feff}');
    let mut lines = markdown.split_inclusive('\n');
    let start = lines
        .next()
        .filter(|line| is_delimiter(line))
        .ok_or("expected YAML frontmatter starting with ---")?
        .len();
    let mut end = start;
    for line in lines {
        if is_delimiter(line) {
            return Ok(&markdown[start..end]);
        }
        // Claude Code ends the frontmatter at the first `---`, even mid-line.
        if line.contains("---") {
            return Err("frontmatter must not contain --- before the closing delimiter");
        }
        end += line.len();
    }
    Err("missing closing --- for YAML frontmatter")
}

/// Reads the boolean at the dotted `key` of the YAML that `yaml` extracts from `path`.
fn read_flag(
    path: &Path,
    key: &str,
    yaml: fn(&str) -> Result<&str, &'static str>,
) -> Result<bool, String> {
    let read = || -> Result<bool, Box<dyn Error>> {
        let value: Value = yaml_serde::from_str(yaml(&fs::read_to_string(path)?)?)?;
        key.split('.')
            .fold(&value, |value, key| &value[key])
            .as_bool()
            .ok_or_else(|| format!("{key} must be an explicit boolean").into())
    };
    read().map_err(|error| format!("{}: {error}", path.display()))
}

fn validate_skill(skill_path: &Path) -> Result<(), String> {
    let disabled = read_flag(skill_path, "disable-model-invocation", frontmatter)?;
    let agent_path = skill_path.with_file_name("agents/openai.yaml");
    let allowed = read_flag(&agent_path, "policy.allow_implicit_invocation", |yaml| {
        Ok(yaml)
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
