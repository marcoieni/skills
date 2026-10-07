---
name: github-writer
description: Write data in GitHub
disable-model-invocation: false
---

* When pushing a commit:
  * If this is the last commit you will push, wait until CI is green. If it fails, and your changes introduced the failure, fix it and push again.
  * if the PR already existed and was opened by your GitHub account, update the description if needed
* When opening a PR, open it in draft
* When you are explaining how you validated your work, don't mention verification steps verified by CI.
