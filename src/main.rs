use serde::Deserialize;
use std::{env, fs, path::Path, path::PathBuf, process::ExitCode};

#[derive(Deserialize)]
struct SkillMetadata {
    #[serde(rename = "disable-model-invocation")]
    disable_model_invocation: bool,
}

#[derive(Deserialize)]
struct AgentMetadata {
    policy: InvocationPolicy,
}

#[derive(Deserialize)]
struct InvocationPolicy {
    allow_implicit_invocation: bool,
}

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let root = args
        .next()
        .map_or_else(|| PathBuf::from("."), PathBuf::from);
    if args.next().is_some() {
        eprintln!("Usage: validate-skill-invocation [repository-root]");
        return ExitCode::FAILURE;
    }

    match validate_repository(&root) {
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
        if let Err(error) = collect_skills(&root.join(directory), &mut skills) {
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
    let skill: SkillMetadata = serde_yaml_ng::from_str(&yaml)
        .map_err(|error| format!("{}: {error}", skill_path.display()))?;

    let agent_path = skill_path.parent().unwrap().join("agents/openai.yaml");
    let agent: AgentMetadata = serde_yaml_ng::from_str(&read_file(&agent_path)?)
        .map_err(|error| format!("{}: {error}", agent_path.display()))?;

    if skill.disable_model_invocation == agent.policy.allow_implicit_invocation {
        return Err(format!(
            "{}: disable-model-invocation is {}, but {}: policy.allow_implicit_invocation is {}; they must be opposite booleans",
            skill_path.display(),
            skill.disable_model_invocation,
            agent_path.display(),
            agent.policy.allow_implicit_invocation,
        ));
    }
    Ok(())
}
