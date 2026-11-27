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
* `install.sh`: Installs the skills and the settings.

## Install

```sh
git clone https://github.com/marcoieni/skills ~/.local/share/marcoieni/skills
~/.local/share/marcoieni/skills/install.sh
```

`install.sh` does the whole installation, so don't link the skills by hand:

* It symlinks every skill under `skills/` into `~/.codex/skills` and `~/.claude/skills`,
  so `git pull` updates them. Run it again after `git pull` to add new skills and remove deleted ones.
* It merges `settings/claude.json` into `~/.claude/settings.json`.
  Keys from `settings/claude.json` win; every other setting is kept.

## 📚 Docs

* [OpenAI skills metadata](https://learn.chatgpt.com/docs/build-skills#optional-metadata)
* [Claude Code settings](https://code.claude.com/docs/en/settings-reference)

## Prompt to install

```
Clone https://github.com/marcoieni/skills into ~/.local/share/marcoieni/skills if absent, otherwise pull it.
Then run its install.sh and show me its output.
```

## Inspired by

* [tstack](https://github.com/t3dotgg/tstack)
