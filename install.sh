#!/bin/sh
# Installs this repository for Codex and Claude Code:
# links every skill under skills/ and merges settings/claude.json into the Claude Code settings.
# Run it again after `git pull` to pick up new or removed skills.
set -eu

repo=$(cd "$(dirname "$0")" && pwd -P)
claude_home=${CLAUDE_CONFIG_DIR:-$HOME/.claude}
codex_home=${CODEX_HOME:-$HOME/.codex}

# Check every destination before changing anything, so a conflict leaves no partial install.
for dest in "$codex_home/skills" "$claude_home/skills"; do
    for skill in "$repo"/skills/*/SKILL.md; do
        link="$dest/$(basename "$(dirname "$skill")")"
        if [ -e "$link" ] && [ ! -L "$link" ]; then
            echo "error: $link exists and isn't a symlink; move it away and run again" >&2
            exit 1
        fi
    done
done

for dest in "$codex_home/skills" "$claude_home/skills"; do
    mkdir -p "$dest"
    # Remove links to skills that were deleted or renamed in this repository.
    for link in "$dest"/*; do
        if [ -L "$link" ] && [ ! -e "$link" ]; then
            case $(readlink "$link") in
                "$repo"/skills/*) rm "$link" && echo "Removed $link" ;;
            esac
        fi
    done
    count=0
    for skill in "$repo"/skills/*/SKILL.md; do
        dir=$(dirname "$skill")
        link="$dest/$(basename "$dir")"
        if [ -L "$link" ] && [ "$(readlink "$link")" != "$dir" ]; then
            echo "Replaced $link, which pointed to $(readlink "$link")"
        fi
        ln -sfn "$dir" "$link"
        count=$((count + 1))
    done
    echo "Linked $count skills into $dest"
done

# Merge rather than symlink: the settings file also holds machine-specific settings, and Claude Code writes to it.
# Keys from settings/claude.json win; every other setting is kept.
python3 - "$repo/settings/claude.json" "$claude_home/settings.json" <<'PY'
import json, os, sys

source = sys.argv[1]
# Write through a symlinked settings file, e.g. one kept in a dotfiles repository.
target = os.path.realpath(sys.argv[2])
with open(source) as f:
    wanted = json.load(f)
try:
    with open(target) as f:
        current = json.load(f)
except FileNotFoundError:
    current = {}
except json.JSONDecodeError as error:
    sys.exit(f"error: {target} isn't valid JSON ({error}); fix it and run again")

def merge(base, extra):
    for key, value in extra.items():
        if isinstance(value, dict) and isinstance(base.get(key), dict):
            merge(base[key], value)
        else:
            base[key] = value

before = json.dumps(current, sort_keys=True)
merge(current, wanted)
if json.dumps(current, sort_keys=True) == before:
    print(f"{target} already has the settings from {source}")
    sys.exit()
temporary = target + ".tmp"
with open(temporary, "w") as f:
    json.dump(current, f, indent=2)
    f.write("\n")
# Keep the file's permissions: it can hold credentials under "env".
if os.path.exists(target):
    os.chmod(temporary, os.stat(target).st_mode & 0o777)
os.replace(temporary, target)
print(f"Merged {source} into {target}")
PY
