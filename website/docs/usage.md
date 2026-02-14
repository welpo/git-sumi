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
    --from <REV>
        Start of the revision range, exclusive (use with --to)
    --to <REV>
        End of the revision range, inclusive (use with --from)
```

#### Linting a commit range

Use `--from` and `--to` to lint all commits in a Git revision range:

```bash
# Lint the last 5 commits
git-sumi --from HEAD~5 --to HEAD

# Lint commits between a tag and HEAD
git-sumi --from v1.0.0 --to HEAD

# Lint all commits on a feature branch (since it diverged from main)
git-sumi --from main --to HEAD
```

Each commit is linted individually. Failures show the short SHA and specific errors, with a summary at the end:

```plaintext
[9d04cad] ️❗ Failed to parse as a conventional commit: 'Missing type in the commit summary, expected `type: description`'
❌ Error: [9d04cad] Found 1 linting error
[f4af66c] ✅ All 1 check passed.
❌ Error: 1 out of 2 commits failed linting. See the errors above
```

`--from` and `--to` cannot be combined with a positional commit message or `--file`.

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
