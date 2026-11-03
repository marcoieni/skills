---
name: akatsuki
description: Use when the user says "akatsuki".
---

1. Start subagents "claude-opus-5-5 xhigh" and "gpt-6-astra xhigh" to answer the user's request.
2. If there are disagreements, you can ask each subagent for clarification.
3. Collect both perspectives, and combine them into a single response. If there are disagreements, report them.
