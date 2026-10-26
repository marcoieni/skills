---
name: github-writer
description: How to push commits and open Prs in GitHub
disable-model-invocation: false
---

* When pushing a commit:
  * If this is the last commit you will push:
    1. Wait until CI is green.
    2. If CI fails and your changes introduced the failure, fix it and push again.
  * if the PR already existed and was opened by your GitHub account, update the description if needed.
* When opening a PR, open it in draft.
* When you are explaining how you validated your work, don't mention verification steps verified by CI.

## Waiting CI on T3 code: `watch_pull_request`

If available, use the T3Code mcp tool watch_pull_request to wait for CI.

Handle the existing checks and comments first. Then call
`watch_pull_request` and end your turn. T3 Code wakes you with a message when
a check fails, the required checks pass, someone comments or reviews, or the
branch conflicts.

- **Only the thread that owns the PR watches it.** If you are a delegated
  child task (a reviewer, any `delegate_task` child), never call
  `watch_pull_request`. The parent owns the PR. Do not watch PRs you are not
  working on.
- **A wake is news, not a merge decision.** Check the merge gate yourself.
- **Unwatch when you hand off.** A watched thread stays in Working and out of
  the user's inbox. When you stop (ready, blocked, or asking the user a question),
  call `unwatch_pull_request` first. A merge or close ends the watch on its
  own.
- **"Stopped watching, could not read it"** usually means GitHub's rate limit
  is used up. Run `gh api rate_limit --jq .resources.graphql` before you
  watch again. If `remaining` is near 0, wait for `reset` with one background
  command, then watch again. Never watch again in a tight loop.
