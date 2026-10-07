---
name: fix
description: Review the current branch and iteratively fix newly introduced issues, with one commit per finding and a GitHub summary.
disable-model-invocation: true
---

1. Use the lens skill to identify issues.
2. Spawn one or more subagents to fix findings you agree with. One commit per finding. Only fix findings of issues introduced in this branch. If it is a pre-existing issue, tell me (don't include it in GitHub comments). Push the commits and post a github comment explaining:
   - what you did in every commit and why.
   - the findings you decided to skip, explaining why.
3. go back to point 1 until no other findings are found
