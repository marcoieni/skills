use std::{fs, path::Path, process::Command, process::Output};

fn write_skill(root: &Path, directory: &str, markdown: &str, agent: Option<&str>) {
    let directory = root.join(directory);
    fs::create_dir_all(directory.join("agents")).unwrap();
    fs::write(directory.join("SKILL.md"), markdown).unwrap();
    if let Some(agent) = agent {
        fs::write(directory.join("agents/openai.yaml"), agent).unwrap();
    }
}

fn validate(root: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_validate-skill-invocation"))
        .current_dir(root)
        .output()
        .unwrap()
}

fn assert_failure(output: Output, expected: &[&str]) {
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    for expected in expected {
        assert!(
            stderr.contains(expected),
            "expected {expected:?} in {stderr}"
        );
    }
}

#[test]
fn accepts_both_invocation_modes() {
    // There is deliberately no draft-skills/ directory: git can't track it once its last
    // skill is gone, so a missing skill directory must count as having no skills.
    let repository = tempfile::tempdir().unwrap();
    write_skill(
        repository.path(),
        "skills/automatic",
        "---\nname: automatic\ndisable-model-invocation: false # explicit\n---\n# Body\n",
        Some("interface: {display_name: Automatic}\npolicy: {allow_implicit_invocation: true}\n"),
    );
    write_skill(
        repository.path(),
        "skills/category/manual",
        "---\r\nname: manual\r\ndisable-model-invocation: true\r\n---\r\n",
        Some("policy:\n  allow_implicit_invocation: false\n"),
    );
    let output = validate(repository.path());
    assert!(output.status.success(), "{:?}", output);
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Validated invocation settings for 2 skills.\n"
    );
}

#[test]
fn reports_conflicts_in_all_skills() {
    let repository = tempfile::tempdir().unwrap();
    for (directory, value) in [("skills/manual", true), ("draft-skills/automatic", false)] {
        write_skill(
            repository.path(),
            directory,
            &format!("---\ndisable-model-invocation: {value}\n---\n"),
            Some(&format!("policy:\n  allow_implicit_invocation: {value}\n")),
        );
    }
    assert_failure(
        validate(repository.path()),
        &[
            "skills/manual/SKILL.md",
            "skills/manual/agents/openai.yaml",
            "draft-skills/automatic/SKILL.md",
            "draft-skills/automatic/agents/openai.yaml",
            "they must be opposite booleans",
        ],
    );
}

#[test]
fn requires_a_boolean_in_skill_frontmatter() {
    let cases = [
        ("# No frontmatter\n", "expected YAML frontmatter"),
        (
            "---\ndisable-model-invocation: true\n",
            "missing closing ---",
        ),
        ("---\nname: missing\n---\n", "disable-model-invocation"),
        // Neither Markdown body text nor a description supplies the metadata flag.
        (
            "---\nname: missing\n---\ndisable-model-invocation: true\n",
            "disable-model-invocation",
        ),
        (
            "---\ndescription: |\n  disable-model-invocation: true\n---\n",
            "disable-model-invocation",
        ),
        ("---\ndisable-model-invocation: 'true'\n---\n", "boolean"),
        ("---\ndisable-model-invocation: null\n---\n", "boolean"),
        ("---\ndisable-model-invocation: 1\n---\n", "boolean"),
        ("---\ndisable-model-invocation: [\n---\n", "invalid type"),
        (
            "---\ndisable-model-invocation: true\ndisable-model-invocation: false\n---\n",
            "duplicate field",
        ),
    ];
    for (markdown, error) in cases {
        let repository = tempfile::tempdir().unwrap();
        write_skill(
            repository.path(),
            "skills/invalid",
            markdown,
            Some("policy:\n  allow_implicit_invocation: false\n"),
        );
        assert_failure(
            validate(repository.path()),
            &["skills/invalid/SKILL.md", error],
        );
    }
}

#[test]
fn requires_agent_metadata_with_an_explicit_policy_boolean() {
    let cases = [
        (None, "openai.yaml"),
        (Some("interface: {}\n"), "policy"),
        (Some("policy: {}\n"), "allow_implicit_invocation"),
        // A flag at the top level must not stand in for the policy setting.
        (
            Some("allow_implicit_invocation: false\npolicy: {}\n"),
            "allow_implicit_invocation",
        ),
        (
            Some("policy:\n  allow_implicit_invocation: 'false'\n"),
            "boolean",
        ),
        (
            Some("policy:\n  allow_implicit_invocation: null\n"),
            "boolean",
        ),
        (Some("policy:\n  allow_implicit_invocation: 0\n"), "boolean"),
        (Some("policy: [\n"), "invalid type"),
        (
            Some(
                "policy:\n  allow_implicit_invocation: false\n  allow_implicit_invocation: true\n",
            ),
            "duplicate field",
        ),
    ];
    for (agent, error) in cases {
        let repository = tempfile::tempdir().unwrap();
        write_skill(
            repository.path(),
            "draft-skills/invalid",
            "---\ndisable-model-invocation: true\n---\n",
            agent,
        );
        assert_failure(
            validate(repository.path()),
            &["draft-skills/invalid/agents/openai.yaml", error],
        );
    }
}

#[test]
fn fails_if_no_skills_are_found() {
    let repository = tempfile::tempdir().unwrap();
    assert_failure(validate(repository.path()), &["no SKILL.md files found"]);
}
