---
name: sakura
description: Use when the user says "sakura".
---

1. Use the `lens` skill to identify issues.
2. Start a subagent to fix findings you agree with. One commit per finding. Be careful of scope creep: don't be scared to push back. Only fix findings of issues introduced in this PR.
3. Go back to point 1 until no other findings are found.
4. Provide a summary of the changes made. In additions to the sections of the `lens` skill, if necessary, include also the following sections:
   - "🐸 Skipped findings": findings that you chose not to address (eg because of scope creep)
5. Push the commits to the PR.
