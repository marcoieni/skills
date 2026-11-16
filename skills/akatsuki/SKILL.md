---
name: akatsuki
description: Use when the user says "akatsuki".
---

1. If the user's request is about a GitHub PR and the T3 Code `link_pull_request` tool is available, link the PR to this thread.
2. Answer the user's request by starting subagents "model:claudeAgent/claude-opus-5-5 xhigh" and "model:codex/gpt-6-astra xhigh" to have multiple perspectives.
3. If there are disagreements, you can ask each subagent for clarification.
4. Collect both perspectives, and combine them into a single response. If there are unresolved disagreements, report them.
