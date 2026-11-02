---
name: naruto
description: Use when the user says "naruto".
---

1. Spawn a subagent to do what the user asked.
2. Spawn subagents to review the changes. Use both "Opus 5.5 Extra high" and "GPT-6-Astra Extra high" to have multiple perspectives.
   Identify:
   - correctness, tests and security issues.
   - maintainability issues, like duplicate code or opportunities to write simpler or more idiomatic code. Check if this is the simplest possible solution.
3. Spawn a subagent to fix findings you agree with. Be careful of scope creep: don't be scared to push back. E.g. only fix findings of issues introduced by your changes.
4. Go back to point 2 until no other findings are found.
5. Open a GitHub PR. If you think it would be helpful for the reviewer, open a PR stack instead (https://docs.github.com/en/enterprise-cloud@latest/pull-requests/reference/stacked-prs-cli-commands).
6. Provide a summary of the skipped findings. If necessary, include a section "⚠️ Pre-existing issues to address later" in your response.
