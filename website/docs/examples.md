---
sidebar_position: 4
---

# Examples

## Tim Pope

Tim Pope ([@tpope](https://github.com/tpope)) is a prolific author of many popular Vim plugins. He outlined some best practices for "a well formed commit message" in [this blog post](https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html).

To follow his advice, you can use the following configuration:

```toml
# Process each non-empty line in the commit message as an individual commit.
split_lines = false

# Rule: capitalize the first letter of commit descriptions.
capitalize_description = true

# Rule: use the imperative mood in the description (e.g. "Fix bug" instead of "Fixed bug").
imperative = true

# Rule: limit the header to the specified length. A value of 0 disables this rule.
max_header_length = 50

# Rule: wrap the body at the specified length. A value of 0 disables this rule.
max_body_length = 72
```

## Linus Torvalds

Linus Torvalds ([@torvalds](https://github.com/torvalds)) is the principal author of the Linux kernel and the creator of Git. He wrote [some thoughts on how to write a good commit message](https://github.com/torvalds/subsurface-for-dirk/blob/master/README.md#contributing).

This configuration will ensure you follow his advice:

```toml
# Rule: use the imperative mood in the description (e.g. "Fix bug" instead of "Fixed bug").
imperative = true

# Rule: wrap the body at the specified length. A value of 0 disables this rule.
max_body_length = 74

# Rule: disallow leading/trailing whitespace and consecutive spaces.
whitespace = true
```
