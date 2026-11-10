---
name: shikamaru
description: Use when the user says "shikamaru".
---

1. Start a subagent to do what the user asked.
2. Use `akatsuki` to review the changes. Identify:
   - correctness, tests and security issues.
   - maintainability issues, like duplicate code or opportunities to write simpler or more idiomatic code. Check if this is the simplest possible solution.
3. Start a subagent to fix findings you agree with. Be careful of scope creep: don't be scared to push back. E.g. only fix findings of issues introduced by your changes.
4. Go back to point 2 until no other findings are found.
5. Provide a summary of the skipped findings. If necessary, include a section "⚠️ Pre-existing issues to address later" in your response.
