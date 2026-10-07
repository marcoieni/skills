---
name: github-writer
description: Apply GitHub publishing conventions when pushing commits or opening or updating pull requests, including draft PRs and CI follow-up.
disable-model-invocation: false
---

* When pushing a commit:
  * wait until CI is green. If it fails, fix it and push again.
  * if the PR already existed and was opened by your GitHub account, update the description if needed
* When opening a PR, open it in draft
* When you are explaining how you validated your work, don't mention verification steps verified by CI.
