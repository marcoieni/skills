---
name: lens
description: Review the current branch and pull request for correctness, testing, security, and maintainability issues.
disable-model-invocation: true
---

Spawn one or more subagents to review the current branch and the associated PR. Identify:
- correctness, tests and security issues.
- maintainability issues, like duplicate code or opportunities to write simpler or more idiomatic code. Check if this is the simplest possible solution.


If an issue is pre-existing (i.e. it wasn't introduced in this PR), tell me.
If the PR is in a GitHub stack, don't report issues that will be fixed in subsequent PRs.

Directories:
- `skills/`: Skills I use.
- `draft-skills/`: Draft skills I am working on.
  I might never use them. They might be there just as ideas for the future.
