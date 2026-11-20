---
name: write-code
description: Write code
disable-model-invocation: false
---

When writing code:

- Don't write tautological tests.
- In tests, if not trivial, add comments explaining the why of steps and assertions.

## Test cases

Write each test case so it can be read on its own.

For example:

- Bad:

  ```
  for n in [1, 2, 3, 4] {
      let expected = if n % 2 == 0 { "even" } else { "odd" };
      assert_eq!(parity(n), expected);
  }
  ```

- Good:

  ```
  assert_eq!(parity(1), "odd");
  assert_eq!(parity(2), "even");
  assert_eq!(parity(3), "odd");
  assert_eq!(parity(4), "even");
  ```
