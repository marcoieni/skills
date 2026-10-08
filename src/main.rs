use std::{fs, path::Path, path::PathBuf, process::ExitCode};
use yaml_serde::Value;

fn main() -> ExitCode {
    match validate_repository(Path::new(".")) {
        Ok(count) => {
            println!("Validated invocation settings for {count} skills.");
            ExitCode::SUCCESS
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{error}");
            }
            ExitCode::FAILURE
        }
    }
}

fn validate_repository(root: &Path) -> Result<usize, Vec<String>> {
    let mut skills = Vec::new();
    let mut errors = Vec::new();
    for directory in ["skills", "draft-skills"] {
        let directory = root.join(directory);
        // Git doesn't track empty directories, so a missing one just has no skills.
        if let Ok(false) = directory.try_exists() {
            continue;
        }
        if let Err(error) = collect_skills(&directory, &mut skills) {
            errors.push(error);
        }
    }
    skills.sort();
    if skills.is_empty() {
        errors.push(format!("{}: no SKILL.md files found", root.display()));
    }
    for skill in &skills {
        if let Err(error) = validate_skill(skill) {
            errors.push(error);
        }
    }
    if errors.is_empty() {
        Ok(skills.len())
    } else {
        Err(errors)
    }
}

fn collect_skills(directory: &Path, skills: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        fs::read_dir(directory).map_err(|error| format!("{}: {error}", directory.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("{}: {error}", directory.display()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("{}: {error}", path.display()))?;
        if file_type.is_dir() {
            collect_skills(&path, skills)?;
        } else if entry.file_name() == "SKILL.md" {
            skills.push(path);
        }
    }
    Ok(())
}

fn read_file(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))
}

fn frontmatter(markdown: &str) -> Result<String, &str> {
    let mut lines = markdown.trim_start_matches('\u{feff}').lines();
    if lines.next() != Some("---") {
        return Err("expected YAML frontmatter starting with ---");
    }
    let mut yaml = Vec::new();
    for line in lines {
        if line == "---" {
            return Ok(yaml.join("\n"));
        }
        yaml.push(line);
    }
    Err("missing closing --- for YAML frontmatter")
}

fn validate_skill(skill_path: &Path) -> Result<(), String> {
    let markdown = read_file(skill_path)?;
    let yaml =
        frontmatter(&markdown).map_err(|error| format!("{}: {error}", skill_path.display()))?;
    let skill: Value = yaml_serde::from_str(&yaml)
        .map_err(|error| format!("{}: {error}", skill_path.display()))?;
    let disabled = skill["disable-model-invocation"].as_bool().ok_or_else(|| {
        format!(
            "{}: disable-model-invocation must be an explicit boolean",
            skill_path.display()
        )
    })?;

    let agent_path = skill_path.parent().unwrap().join("agents/openai.yaml");
    let agent: Value = yaml_serde::from_str(&read_file(&agent_path)?)
        .map_err(|error| format!("{}: {error}", agent_path.display()))?;
    let allowed = agent["policy"]["allow_implicit_invocation"]
        .as_bool()
        .ok_or_else(|| {
            format!(
                "{}: policy.allow_implicit_invocation must be an explicit boolean",
                agent_path.display()
            )
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
