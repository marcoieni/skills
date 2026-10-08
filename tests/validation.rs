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

// Comparing all of stderr also proves that no other error, such as a conflict, was reported.
fn assert_failure(output: Output, expected_stderr: &str) {
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(String::from_utf8(output.stderr).unwrap(), expected_stderr);
}

#[test]
fn accepts_both_invocation_modes() {
    // There is deliberately no draft-skills/ directory: git can't track it once its last
    // skill is gone, so a missing skill directory must count as having no skills.
    let repository = tempfile::tempdir().unwrap();
    // Claude Code and Codex both accept trailing whitespace after the `---` delimiters. The
    // trailing comment and flow-style mappings are valid YAML that a line-based check would miss.
    write_skill(
        repository.path(),
        "skills/automatic",
        "--- \nname: automatic\ndisable-model-invocation: false # explicit\n---\t\n# Body\n",
        Some("interface: {display_name: Automatic}\npolicy: {allow_implicit_invocation: true}\n"),
    );
    // Windows editors may save CRLF line endings, and skills can be grouped in subdirectories.
    write_skill(
        repository.path(),
        "skills/category/manual",
        "---\r\nname: manual\r\ndisable-model-invocation: true\r\n---\r\n",
        Some("policy:\n  allow_implicit_invocation: false\n"),
    );
    let output = validate(repository.path());
    assert!(output.status.success(), "{:?}", output);
    // The count proves the recursive search also found the nested skill.
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Validated invocation settings for 2 skills.\n"
    );
}

#[test]
fn reports_conflicts_in_all_skills() {
    // Both skills conflict, so the output proves the validator reports every error instead of
    // stopping at the first. Skills are checked in path order, so draft-skills/ comes first.
    let repository = tempfile::tempdir().unwrap();
    let mut expected = String::new();
    for (directory, value) in [("draft-skills/automatic", false), ("skills/manual", true)] {
        write_skill(
            repository.path(),
            directory,
            &format!("---\ndisable-model-invocation: {value}\n---\n"),
            Some(&format!("policy:\n  allow_implicit_invocation: {value}\n")),
        );
        expected += &format!(
            "{directory}/SKILL.md: disable-model-invocation is {value}, but {directory}/agents/openai.yaml: policy.allow_implicit_invocation is {value}; they must be opposite booleans\n"
        );
    }
    assert_failure(validate(repository.path()), &expected);
}

#[test]
fn requires_a_boolean_in_skill_frontmatter() {
    const MISSING_FLAG: &str = "disable-model-invocation must be an explicit boolean";
    let cases = [
        (
            "# No frontmatter\n",
            "expected YAML frontmatter starting with ---",
        ),
        // Codex finds the frontmatter with `line.trim() == "---"`, which keeps a byte order
        // mark, so it likely wouldn't load this skill's metadata.
        (
            "\u{feff}---\ndisable-model-invocation: false\n---\n",
            "expected YAML frontmatter starting with ---",
        ),
        (
            "---\ndisable-model-invocation: false\n",
            "missing closing --- for YAML frontmatter",
        ),
        // Claude Code ends the frontmatter at the first `---` anywhere, even inside a comment,
        // so it would never see the flag below it.
        (
            "---\nname: x\n# ---- invocation ----\ndisable-model-invocation: false\n---\n",
            "frontmatter must not contain --- before the closing delimiter",
        ),
        ("---\nname: missing\n---\n", MISSING_FLAG),
        // Neither Markdown body text nor a description supplies the metadata flag.
        (
            "---\nname: missing\n---\ndisable-model-invocation: false\n",
            MISSING_FLAG,
        ),
        (
            "---\ndescription: |\n  disable-model-invocation: false\n---\n",
            MISSING_FLAG,
        ),
        (
            "---\ndisable-model-invocation: 'false'\n---\n",
            MISSING_FLAG,
        ),
        // YAML parsers disagree on which duplicate key wins, so a duplicate is ambiguous. The
        // error locates the mapping by its line in the file, not in the frontmatter.
        (
            "---\ndisable-model-invocation: true\ndisable-model-invocation: false\n---\n",
            "duplicate entry with key \"disable-model-invocation\" at line 2 column 1",
        ),
    ];
    for (markdown, error) in cases {
        let repository = tempfile::tempdir().unwrap();
        // Claude Code defaults a missing flag to `false`, and each value above that must be
        // ignored is `false`-like, so pairing with `true` makes a too-lenient validator pass.
        write_skill(
            repository.path(),
            "skills/invalid",
            markdown,
            Some("policy:\n  allow_implicit_invocation: true\n"),
        );
        assert_failure(
            validate(repository.path()),
            &format!("skills/invalid/SKILL.md: {error}\n"),
        );
    }
}

#[test]
fn requires_agent_metadata_with_an_explicit_policy_boolean() {
    const MISSING_POLICY: &str = "policy.allow_implicit_invocation must be an explicit boolean";
    let cases = [
        (None, "No such file or directory (os error 2)"),
        (Some("interface: {}\n"), MISSING_POLICY),
        // A flag at the top level must not stand in for the policy setting.
        (
            Some("allow_implicit_invocation: true\npolicy: {}\n"),
            MISSING_POLICY,
        ),
        (
            Some("policy:\n  allow_implicit_invocation: 'true'\n"),
            MISSING_POLICY,
        ),
    ];
    for (agent, error) in cases {
        let repository = tempfile::tempdir().unwrap();
        // Codex defaults a missing `allow_implicit_invocation` to `true`, and each value above
        // that must be ignored is `true`-like, so pairing with `false` makes a too-lenient
        // validator pass.
        write_skill(
            repository.path(),
            "draft-skills/invalid",
            "---\ndisable-model-invocation: false\n---\n",
            agent,
        );
        assert_failure(
            validate(repository.path()),
            &format!("draft-skills/invalid/agents/openai.yaml: {error}\n"),
        );
    }
}

#[test]
fn fails_if_no_skills_are_found() {
    // Otherwise running from the wrong directory would silently pass.
    let repository = tempfile::tempdir().unwrap();
    assert_failure(
        validate(repository.path()),
        "no SKILL.md files found in skills/ or draft-skills/\n",
    );
}
