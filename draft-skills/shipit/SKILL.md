---
name: shipit
description: Implement an issue on a new branch, iterate on adversarial reviews, and open a draft pull request with CI follow-up.
disable-model-invocation: true
---

1. Create a new branch
2. Make your changes to fix the issue
3. commit
4. run a subagent to run an adversarial reviews with a pragmatic approach
5. fix all the findings if you find them reasonable, one for each commit.
6. go back to step 4 until there are no more findings.
7. open a draft PR. In the PR description explain the changes you did and why you did them. In the PR description write in a `<details><summary>` block where you explain the findings that were found and the commits in which they were addressed or why you didn't address certain findings
8. Wait for CI to finish and fix CI if needed.
