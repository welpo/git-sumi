---
sidebar_position: 3
---

# Command line usage

```bash
git-sumi [OPTIONS] [--] [COMMIT_MESSAGE]
```

## Flags

```plaintext
-q, --quiet
        Suppress progress messages [env: GIT_SUMI_QUIET=]
-s, --split-lines
        Process each non-empty line as an individual commit [env: GIT_SUMI_SPLIT_LINES=]
-d, --display
        Display the parsed commit message [env: GIT_SUMI_DISPLAY=]
-c, --commit
        Commit the message after successful linting
    --force
        Force a commit, regardless of linting errors
-h, --help
        Print help
-V, --version
        Print version
```

## Options

```plaintext
    --init [<OPTION>]
        Initialize the default configuration ("config", default value) or commit-msg hook ("hook")
    --generate-shell-completion <SHELL>
        Generate shell completion script for the specified shell [possible values: bash, elvish, fish, powershell, zsh]
    --config <CONFIG>
        Path to a TOML configuration file [env: GIT_SUMI_CONFIG=]
-f, --format <FORMAT>
        Specify the output format for displaying the parsed commit message. Options: "table", "json", "toml". Default: "table" [env: GIT_SUMI_FORMAT=]
```

### Rules

Read the [rules documentation](/docs/rules) for more information.

```plaintext
-I, --imperative
        Use the imperative mood in the description [env: GIT_SUMI_IMPERATIVE=]
-G, --gitmoji
        Include one valid Gitmoji [env: GIT_SUMI_GITMOJI=]
-C, --conventional
        Follow Conventional Commits format [env: GIT_SUMI_CONVENTIONAL=]
-W, --whitespace
        Disallow leading/trailing whitespace and consecutive spaces [env: GIT_SUMI_WHITESPACE=]
-E, --description-case <CASE>
        Commit description must start with the selected case. Options: "lower", "upper", "any". Default: "any" [env: GIT_SUMI_DESCRIPTION_CASE=]
-D, --capitalize-description
        Capitalize the first letter of commit descriptions [env: GIT_SUMI_CAPITALIZE_DESCRIPTION=]
-P, --no-period
        Do not end commit header with a period [env: GIT_SUMI_NO_PERIOD=]
-H, --max-header-length <MAX_HEADER_LENGTH>
        Limit the header to the specified length [env: GIT_SUMI_MAX_HEADER_LENGTH=]
-B, --max-body-length <MAX_BODY_LENGTH>
        Wrap the body at the specified length [env: GIT_SUMI_MAX_BODY_LENGTH=]
-S, --scopes-allowed <SCOPES>
        Only allow the specified, comma-separated commit scopes [env: GIT_SUMI_SCOPES_ALLOWED=]
-T, --types-allowed <TYPES>
        Only allow the specified, comma-separated commit types [env: GIT_SUMI_TYPES_ALLOWED=]
-R, --header-pattern <PATTERN>
        Commit header must match the specified (regex) pattern [env: GIT_SUMI_HEADER_PATTERN=]
```

## Overriding configuration options

You can override any configuration option with a command line option or environment variable.

For example, if `split_lines` is set to `false` in your `sumi.toml` file, you can use `git sumi --split-lines` or `GIT_SUMI_SPLIT_LINES=true git sumi` to enable the option.
