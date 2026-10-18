# skills

AI skills I use.

Assumptions:

* You use GitHub for issue tracking.
* Agents have access to a GitHub account. E.g. I use [marcoienibot](https://github.com/marcoienibot).

Directories:

* `skills/`: Skills I use.
* `draft-skills/`: Draft skills I am working on (Don't install).
  I might never use them. They might be there just as ideas for the future.
* `settings/claude.json`: Claude Code settings I use.
* `scripts/install-ubuntu-server.sh`: Installs the system packages I need on Ubuntu Server.

## 📚 Docs

* [OpenAI skills metadata](https://learn.chatgpt.com/docs/build-skills#optional-metadata)
* [Claude Code settings](https://code.claude.com/docs/en/settings-reference)

## Prompt to install

```
Clone https://github.com/marcoieni/skills if absent and install the skills under `skills` for both codex and claude, globally.
Use symlinks so that when I run git pull on the repository, the skills are automatically updated.
Merge `settings/claude.json` into `~/.claude/settings.json`, keeping the settings already there.
If this machine is an Ubuntu Server, run `scripts/install-ubuntu-server.sh`.
```
