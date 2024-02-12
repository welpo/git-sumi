---
sidebar_position: 4
---

# Examples

## Tim Pope

Tim Pope ([@tpope](https://github.com/tpope)) is a prolific author of many popular Vim plugins. He outlined some best practices for "a well formed commit message" in [this blog post](https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html).

To follow his advice, you can use the following configuration:

```toml
# Processes each non-empty line as an individual commit.
split_lines = false

# Rule: Description must start with the specified case.
# Options: 'any', 'lower', 'upper'.
description_case = "upper"

# Rule: Use the imperative mood in the description.
# Example: 'Fix bug' instead of 'Fixed bug'.
imperative = true

# Rule: Header length limit.
# A value of 0 disables the rule.
max_header_length = 50

# Rule: Body line length limit.
# A value of 0 disables the rule.
max_body_length = 72
```

## Linus Torvalds

Linus Torvalds ([@torvalds](https://github.com/torvalds)) is the principal author of the Linux kernel and the creator of Git. He wrote [some thoughts on how to write a good commit message](https://github.com/torvalds/subsurface-for-dirk/blob/master/README.md#contributing).

This configuration will ensure you follow his advice:

```toml
# Rule: Use the imperative mood in the description.
# Example: 'Fix bug' instead of 'Fixed bug'.
imperative = true

# Rule: Body line length limit.
# A value of 0 disables the rule.
max_body_length = 74

# Rule: No leading, trailing, or consecutive spaces.
whitespace = true
```
