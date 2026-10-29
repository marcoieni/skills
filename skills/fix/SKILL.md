---
name: fix
description: Fix code
disable-model-invocation: true
---

1. Use the lens skill to identify issues.
2. Spawn one or more subagents to fix findings you agree with. Be careful of scope creep and don't be scared to push back.
3. Only fix findings of issues introduced in this branch. If it is a pre-existing issue, tell me. Push one commit per finding.
4. Go back to point 1 until no other findings are found.
5. Provide a summary of the changes made and any skipped findings.
