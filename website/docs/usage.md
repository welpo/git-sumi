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
        Suppresses progress messages [env: GIT_SUMI_QUIET=]
-s, --split-lines
        Processes each non-empty line as an individual commit [env: GIT_SUMI_SPLIT_LINES=]
-d, --display
        Displays parsed commit message [env: GIT_SUMI_DISPLAY=]
-c, --commit
        Commit the message after successful linting
--force
        Force the commit even if linting fails
-h, --help
        Print help
-V, --version
        Print version
```

## Options

```plaintext
    --init [<OPTION>]
        Initialize the default configuration ('config'), commit-msg hook ('commit-msg'), prepare-commit-msg hook ('prepare-commit-msg') or both hooks ('hooks')
    --generate-shell-completion <SHELL>
        Generate shell completion script for the specified shell [possible values: bash, elvish, fish, powershell, zsh]
    --config <CONFIG>
        Path to a TOML configuration file [env: GIT_SUMI_CONFIG=]
-f, --format <FORMAT>
        Sets display format: cli, json, table, toml [env: GIT_SUMI_FORMAT=]
    --file <FILE>
        Read commit message from file
```

### Rules

Read the [rules documentation](/docs/rules) for more information.

```plaintext
-C, --conventional
        Follow Conventional Commits format [env: GIT_SUMI_CONVENTIONAL=]
-I, --imperative
        Use the imperative mood in the description ('fix', not 'fixed') [env: GIT_SUMI_IMPERATIVE=]
-G, --gitmoji
        Include one valid Gitmoji [env: GIT_SUMI_GITMOJI=]
-W, --whitespace
        No leading, trailing, or consecutive spaces [env: GIT_SUMI_WHITESPACE=]
-E, --description-case <CASE>
        Description must start with the specified case: any, lower, upper [env: GIT_SUMI_DESCRIPTION_CASE=]
-P, --no-period
        Do not end commit header with a period [env: GIT_SUMI_NO_PERIOD=]
-H, --max-header-length <MAX_HEADER_LENGTH>
        Header length limit [env: GIT_SUMI_MAX_HEADER_LENGTH=]
-B, --max-body-length <MAX_BODY_LENGTH>
        Body line length limit [env: GIT_SUMI_MAX_BODY_LENGTH=]
-S, --scopes-allowed <SCOPES>
        List of allowed commit scopes [env: GIT_SUMI_SCOPES_ALLOWED=]
-T, --types-allowed <TYPES>
        List of allowed commit types [env: GIT_SUMI_TYPES_ALLOWED=]
-R, --header-pattern <PATTERN>
        Header must match regex pattern [env: GIT_SUMI_HEADER_PATTERN=]
```

## Overriding configuration options

You can override any configuration option with a command line option or environment variable.

For example, if `split_lines` is set to `false` in your `sumi.toml` file, you can use `git sumi --split-lines` or `GIT_SUMI_SPLIT_LINES=true git sumi` to enable the option.
