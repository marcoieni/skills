---
name: lens
description: Use when the user says "lens".
---

Spawn subagents to review the current branch and the associated PR. Use both "Opus 5.5 Extra high" and "GPT-6-Astra Extra high" to have multiple perspectives.
Identify:
- correctness, tests and security issues.
- maintainability issues, like duplicate code or opportunities to write simpler or more idiomatic code. Check if this is the simplest possible solution.

If an issue is pre-existing (i.e. it wasn't introduced in this PR), tell me.
If the PR is in a GitHub stack, don't report issues that will be fixed in subsequent PRs.
Deduplicate findings to avoid reporting the same issue multiple times.
