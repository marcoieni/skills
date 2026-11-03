---
name: fix
description: Fix code
disable-model-invocation: true
---

1. Use the lens skill to identify issues.
2. Start a subagent to fix findings you agree with. Be careful of scope creep: don't be scared to push back. Only fix findings of issues introduced in this branch. If it is a pre-existing issue, tell me. Push one commit per finding.
3. Go back to point 1 until no other findings are found.
4. Provide a summary of the changes made and any skipped findings. If necessary, include a section "⚠️ Pre-existing issues to address later" in your response.
