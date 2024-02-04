---
sidebar_position: 3
---

# Rules

As git-**sumi** is non-opinionated, no rules are enabled by default.

Choose which rules to enable based on your project's needs and the conventions you want to enforce. You can enable or disable rules using the `sumi.toml` [configuration file](/docs/configuration), [command line options or environment variables](/docs/usage#rules).

:::tip

In your commit messages, don't describe the code, describe the intent and the approach.

:::

## Conventional Commits

- **Description**: Enforces adherence to the Conventional Commits specification, facilitating automated changelog generation and semantic versioning.

- **Why it matters**: Following the Conventional Commits format standardizes commit messages, making them more readable and enabling automated tools to process versioning and [changelog generation](https://git-cliff.org/).

- **`sumi.toml` identifier**: `conventional`

- **Command line usage**: Long option: `--conventional`, Short option: `-C`

- **Environment variable**: `GIT_SUMI_CONVENTIONAL`

- **Type of value**: Boolean (e.g., `true`)

- **Example**: Set `conventional = true` in `sumi.toml`, or use `git sumi --conventional`.

## Imperative mood

- **Description**: Enforces the use of the imperative mood in commit descriptions (e.g. "Add feature" instead of "Added feature").

:::info

This is a best-effort heuristic designed to have a very low false positive rate (i.e., if it flags a commit message, it's almost certainly not in the imperative mood).

If you encounter a false positive (or can come up with a false negative), please [open an issue](https://github.com/welpo/git-sumi/issues/new/choose).

:::

- **Why it matters**: The imperative mood matches Git's own built-in messages (e.g. `git merge` or `git revert`), promoting consistency and clarity.

- **`sumi.toml` identifier**: `imperative`

- **Command line usage**: `--imperative` or `-I`

- **Environment variable**: `GIT_SUMI_IMPERATIVE`

- **Type of value**: Boolean (e.g., `true`)

- **Example**: Set `imperative = true` in `sumi.toml`, or use `git sumi -I`.

## Whitespace

- **Description**: Disallows leading/trailing whitespace and consecutive spaces within commit messages.

- **Why it matters**: Excessive or improper whitespace can make commit messages look untidy and can cause issues with tools that process commit messages, affecting readability and automation.

- **`sumi.toml` identifier**: `whitespace`

- **Command line usage**: Long option: `--whitespace`, Short option: `-W`

- **Environment variable**: `GIT_SUMI_WHITESPACE`

- **Type of value**: Boolean (e.g., `true`)

- **Example**: Set `whitespace = true` in `sumi.toml`, or use `git sumi --whitespace`.

## Description case

- **Description**: Ensures that commit descriptions follow a specified case. The available options are `upper`, `lower`, and `any` (disables the rule).

- **Why it matters**: Consistency in casing for commit descriptions helps maintain a uniform style across the project's commit history, improving readability.

- **`sumi.toml` identifier**: `description_case`

- **Command line usage**: Long option: `--description-case`, Short option: `-E`

- **Environment variable**: `GIT_SUMI_DESCRIPTION_CASE`

- **Type of value**: String (e.g., `upper`) from the list `upper` `lower`, and `any`.

- **Example**: Set `description_case = "lower"` in `sumi.toml`, use `git sumi --description-case lower` or `GIT_SUMI_DESCRIPTION_CASE=lower git sumi`.

## No period

- **Description**: Prohibits ending commit headers with a period.

- **Why it matters**: Removing the period from the end of commit headers follows a common stylistic convention, making headers succinct and focused, like an email subject line.

- **`sumi.toml` identifier**: `no_period`

- **Command line usage**: Long option: `--no-period`, Short option: `-P`

- **Environment variable**: `GIT_SUMI_NO_PERIOD`

- **Type of value**: Boolean (e.g., `true`)

- **Example**: Set `no_period = true` in `sumi.toml`, or use `git sumi --no-period`.

## Max header length

- **Description**: Restricts the commit header to a specified maximum length.

- **Why it matters**: Enforcing a maximum header length ensures commit messages are concise and fit well in various Git interfaces, like logs or the GitHub UI.

- **`sumi.toml` identifier**: `max_header_length`

- **Command line usage**: Long option: `--max-header-length`, Short option: `-H`

- **Environment variable**: `GIT_SUMI_MAX_HEADER_LENGTH`

- **Type of value**: Integer (e.g., `50`)

- **Example**: Set `max_header_length = 50` in `sumi.toml`, or use `git sumi -H 50`.

:::tip

Not sure what a good header length limit should be for your project? Run the following Bash command from your repository to analyze the lengths of the headers in your last 100 commit messages:

```bash
git log -n 100 --pretty=%s | awk '{ print length }' | sort -n | awk 'BEGIN {min = 999999; max = 0; count=0; sum=0; sumsq=0} {if ($1<min) min=$1; if ($1>max) max=$1; sum+=$1; sumsq+=$1*$1; count++} END {print "Mean: " sum/count; print "Min: " min; print "Max: " max; print "80th Percentile: " int(count*0.8 - 1); print "Standard Deviation: " sqrt(sumsq/count - (sum/count)^2)}'
```

This will give you the mean, min, max, 80th percentile, and standard deviation of your last 100 commits, which can guide you in setting up your linting rules.

:::

## Max body length

- **Description**: Restricts the length of each line in the commit body to a specified maximum length.

- **Why it matters**: Keeping the body text within a readable length makes commit messages easier to read, especially in command-line interfaces or tools with limited line width.

- **`sumi.toml` identifier**: `max_body_length`

- **Command line usage**: Long option: `--max-body-length`, Short option: `-B`

- **Environment variable**: `GIT_SUMI_MAX_BODY_LENGTH`

- **Type of value**: Integer (e.g., `72`)

- **Example**: Set `max_body_length = 72` in `sumi.toml`, or use `git sumi --max-body-length 72`.

## Gitmoji

- **Description**: Requires the inclusion of a [valid Gitmoji](https://gitmoji.dev/) in commit messages.

- **Why it matters**: Gitmoji makes it easier to visually scan through commit logs and quickly grasp the purpose of changes.

- **`sumi.toml` identifier**: `gitmoji`

- **Command line usage**: Long option: `--gitmoji`, Short option: `-G`

- **Environment variable**: `GIT_SUMI_GITMOJI`

- **Type of value**: Boolean (e.g., `true`)

- **Example**: Set `gitmoji = true` in `sumi.toml`, or use `git sumi -G`.

:::tip

Want to enforce a specific Gitmoji position? Combine this rule with [`header_pattern`](#header-pattern). For example, to ensure it's always the first element in the header:

```toml
gitmoji = true
header_pattern = '^(:\w+:|[\p{Emoji_Presentation}\p{Extended_Pictographic}\u{200D}])'
```

:::

## Scopes allowed

- **Description**: Specifies a list of allowed commit scopes. Automatically enables the `conventional` rule.

- **Why it matters**: Defining allowed scopes helps in maintaining consistency and clarity in the scope designation, making it easier to categorize and filter commits by scope.

- **`sumi.toml` identifier**: `scopes_allowed`

- **Command line usage**: Long option: `--scopes-allowed`, Short option: `-S`

- **Environment variable**: `GIT_SUMI_SCOPES_ALLOWED`

- **Type of value**: List of strings (e.g., `["docs", "cli"]`)

- **Example**: Set `scopes_allowed = ["docs", "cli"]` in `sumi.toml`, or use `git sumi -S docs,cli`.

## Types allowed

- **Description**: Limits commit types to a predefined list, ensuring uniformity in the types of changes being committed. Automatically enables the `conventional` rule.

- **Why it matters**: Consistent use of commit types aids in understanding the nature of changes at a glance and can be crucial for automated tools and scripts that rely on commit types.

- **`sumi.toml` identifier**: `types_allowed`

- **Command line usage**: Long option: `--types-allowed`, Short option: `-T`

- **Environment variable**: `GIT_SUMI_TYPES_ALLOWED`

- **Type of value**: List of strings (e.g., `["feat", "fix"]`)

- **Example**: Set `types_allowed = ["feat", "fix"]` in `sumi.toml`, or use `git sumi -T feat,fix`.

Adding to the existing documentation, let's document the new feature related to specifying a regex pattern for commit message headers, with an emphasis on the importance of correctly escaping special characters in regex patterns.

## Header pattern

- **Description**: Enforces a specified regex pattern that commit message headers must match. This rule allows for flexible and customizable validation of commit headers to adhere to specific formatting guidelines.

- **Why it matters**: Certain projects may require including a JIRA issue key, a specific prefix, or a custom format in commit headers.

- **`sumi.toml` identifier**: `header_pattern`

- **Command line usage**: Long option: `--header-pattern`, Short option: `-R`

- **Environment variable**: `GIT_SUMI_HEADER_PATTERN`

- **Type of value**: String (e.g., `'^JIRA-\d+:'`)

- **Example**: Set `header_pattern = '^JIRA-\d+:'` in `sumi.toml`, or use `git sumi --header-pattern '^JIRA-\d+:'`.

:::warning
Make sure to properly escape special characters in regex patterns. This won't work:

```toml
header_pattern = "^JIRA-\d+:"
```

You can either use double backslashes or use a literal string:

```toml
# Double backslashes with double quotes.
header_pattern = "^JIRA-\\d+:"

# Literal string (single quotes).
header_pattern = '^JIRA-\d+:'
```
:::

:::tip
It's a good practice to test your regular expressions using a [regex tester](https://regex101.com/) or a similar tool before enforcing them with git-**sumi**.
:::
