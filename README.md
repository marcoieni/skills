# skills

AI skills I use.

Assumptions:

* You use GitHub for issue tracking.
* Agents have access to a GitHub account. E.g. I use [marcoienibot](https://github.com/marcoienibot).

## 📚 Docs

* [OpenAI skills metadata](https://learn.chatgpt.com/docs/build-skills#optional-metadata)

## Validate invocation settings

Every `SKILL.md` under `skills/` and `draft-skills/` must explicitly set
`disable-model-invocation` in its YAML frontmatter. Its sibling
`agents/openai.yaml` must explicitly set `policy.allow_implicit_invocation`.
Both values must be booleans and must be opposites:

| Invocation | `disable-model-invocation` | `policy.allow_implicit_invocation` |
| --- | --- | --- |
| Explicit only | `true` | `false` |
| Implicit allowed | `false` | `true` |

Run the Rust validator from the repository root:

```sh
cargo run --locked
```

The validator searches both skill directories recursively, reports invalid files,
and exits unsuccessfully if any settings are missing, malformed, or inconsistent,
or if no skills are found. Both skill directories must exist.
GitHub Actions runs this check, the tests, formatting, and Clippy on every push and
pull request.
