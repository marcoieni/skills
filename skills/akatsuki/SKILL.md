---
name: akatsuki
description: Use when the user says "akatsuki".
---

1. Answer the user's request by starting subagents "model:claudeAgent/claude-opus-5-5 xhigh" and "model:codex/gpt-6-astra xhigh" to have multiple perspectives.
2. If there are disagreements, you can ask each subagent for clarification.
3. Collect both perspectives, and combine them into a single response. If there are unresolved disagreements, report them.
