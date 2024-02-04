---
sidebar_position: 2
---

# Configuration

git-**sumi** uses [TOML](https://github.com/toml-lang/toml) for the configuration file.

:::tip

Create a `sumi.toml` with the default values by running this command from the root of your repository:

```bash
git sumi --init config
```

:::

## Config file location

By default, git-**sumi** looks the configuration file `sumi.toml` or `.sumi.toml` in the following places:

1. The current directory
2. The current directory's parent directories
3. The user's home directory
4. A directory named `sumi` within the user's home directory

:::note

The user's home directory varies by platform:

| Platform | Value            | Example          |
|----------|------------------|------------------|
| Linux    | `$HOME`          | `/home/alice`    |
| macOS    | `$HOME`          | `/Users/Alice`   |
| Windows  | `{FOLDERID_Profile}` | `C:\Users\Alice` |

:::

In practice, this means git-**sumi** will respect the rules of the repository you are currently in, falling back to your user's configuration.

You can use a particular config file with the `--config | -c` option:

```bash
git sumi --config /path/to/sumi.toml
```

:::tip

Use `--config 'none'` to use the default configuration.

:::

## Configuration options

### Quiet

- **Description**: Suppress progress messages.

- **`sumi.toml` identifier**: `quiet`

- **Command line usage**: Long option: `--quiet`, Short option: `-q`

- **Environment variable**: `GIT_SUMI_QUIET`

- **Type of value**: Boolean (e.g., `false`)

- **Example**: Set `quiet = true` in `sumi.toml` to suppress progress messages, or use `git sumi --quiet`.

### Split Lines

- **Description**: Process each non-empty line of the commit message as an individual commit.

- **`sumi.toml` identifier**: `split_lines`

- **Command line usage**: Long option: `--split-lines`, Short option: `-s`

- **Environment variable**: `GIT_SUMI_SPLIT_LINES`

- **Type of value**: Boolean (e.g., `true`)

- **Example**: Set `split_lines = true` in `sumi.toml` or use `git sumi -s`.

### Display

- **Description**: Display the parsed commit message after linting.

- **`sumi.toml` identifier**: `display`

- **Command line usage**: Long option: `--display`, Short option: `-d`

- **Environment variable**: `GIT_SUMI_DISPLAY`

- **Type of value**: Boolean (e.g., `true`)

- **Example**: Set `display = true` in `sumi.toml` or use `git sumi --display`.

### Format

- **Description**: Specifies the output format for displaying the parsed commit message.

    Enabling this option automatically sets `display = true`.

- **`sumi.toml` identifier**: `format`

- **Command line usage**: Long option: `--format`, Short option: `-f`

- **Environment variable**: `GIT_SUMI_FORMAT`

- **Type of value**: String (options: "cli", "table", "json", "toml")

- **Default value**: "cli"

- **Example usage**: Set `format = "json"` in `sumi.toml` for JSON formatted output, or use `git sumi --format json`.

- **Example output**

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs>

<TabItem value="original" label="Original commit">

```txt
🐛 fix(auth)!: resolve token refresh issue

Fixes bug introduced in ce6df36 where the authentication token would
not refresh properly during a session, causing unexpected logouts.

Co-authored-by: John Doe <johndoe@example.com>
```

</TabItem>

<TabItem value="table" label="(Markdown) table">

```txt
| Key                  | Value                                                                |
|----------------------|----------------------------------------------------------------------|
| Gitmoji              | 🐛                                                                   |
| Commit type          | fix                                                                  |
| Scope                | auth                                                                 |
| Description          | resolve token refresh issue                                          |
| Body                 | Fixes bug introduced in ce6df36 where the authentication token would |
|                      | not refresh properly during a session, causing unexpected logouts.   |
| Footers              | Co-authored-by:John Doe <johndoe@example.com>                        |
| Is breaking          | true                                                                 |
| Breaking description | resolve token refresh issue                                          |
| References           | ce6df36                                                              |
```

</TabItem>

<TabItem value="cli" label="cli">

```txt
┌──────────────────────┬──────────────────────────────────────────────────────────────────────┐
│ Gitmoji              │ 🐛                                                                   │
├──────────────────────┼──────────────────────────────────────────────────────────────────────┤
│ Commit type          │ fix                                                                  │
├──────────────────────┼──────────────────────────────────────────────────────────────────────┤
│ Scope                │ auth                                                                 │
├──────────────────────┼──────────────────────────────────────────────────────────────────────┤
│ Description          │ resolve token refresh issue                                          │
├──────────────────────┼──────────────────────────────────────────────────────────────────────┤
│ Body                 │ Fixes bug introduced in ce6df36 where the authentication token would │
│                      │ not refresh properly during a session, causing unexpected logouts.   │
├──────────────────────┼──────────────────────────────────────────────────────────────────────┤
│ Footers              │ Co-authored-by:John Doe <johndoe@example.com>                        │
├──────────────────────┼──────────────────────────────────────────────────────────────────────┤
│ Is breaking          │ true                                                                 │
├──────────────────────┼──────────────────────────────────────────────────────────────────────┤
│ Breaking description │ resolve token refresh issue                                          │
├──────────────────────┼──────────────────────────────────────────────────────────────────────┤
│ References           │ ce6df36                                                              │
└──────────────────────┴──────────────────────────────────────────────────────────────────────┘
```

</TabItem>
<TabItem value="json" label="JSON">

```json
{
  "body": "Fixes bug introduced in ce6df36 where the authentication token would\nnot refresh properly during a session, causing unexpected logouts.",
  "breaking_description": "resolve token refresh issue",
  "commit_type": "fix",
  "description": "resolve token refresh issue",
  "footers": [
    "Co-authored-by:John Doe <johndoe@example.com>"
  ],
  "gitmoji": "🐛",
  "is_breaking": true,
  "references": [
    "ce6df36"
  ],
  "scope": "auth"
}
```

</TabItem>
<TabItem value="toml" label="TOML">

```toml
gitmoji = "🐛"
commit_type = "fix"
scope = "auth"
description = "resolve token refresh issue"
body = """
Fixes bug introduced in ce6df36 where the authentication token would
not refresh properly during a session, causing unexpected logouts."""
footers = ["Co-authored-by:John Doe <johndoe@example.com>"]
is_breaking = true
breaking_description = "resolve token refresh issue"
references = ["ce6df36"]
```

</TabItem>
</Tabs>

## Rules

See the [list of all available rules](/docs/rules).
