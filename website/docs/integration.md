---
sidebar_position: 6
---

# Integration

## Using pre-commit

To integrate git-**sumi** with [pre-commit](https://pre-commit.com/#intro), add the following hook to your pre-commit config:

```yaml
repos:
-   repo: https://github.com/welpo/git-sumi
    rev: v0.0.9  # check latest version: https://github.com/welpo/git-sumi/tags
    hooks:
    -   id: git-sumi
```

## Local linting with Git hooks

:::tip
To set up both Git hooks described below, run `git sumi --init hooks`.
:::

### commit-msg hook

You can use git-**sumi** to lint each commit message, interrupting commits that don't meet the specified rules.

Before continuing, make sure you have installed git-**sumi** and have a configuration file (`sumi.toml`) at the root of your repository. The [Getting Started](/docs) guide will help you with this.

Run this command to set up a commit-msg Git hook:

```bash
git sumi --init commit-msg
```

This will create a Bash [Git `commit-msg` hook](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks#_committing_workflow_hooks) in your `.git/hooks` directory. The hook will abort the commit if its message doesn't pass the rules set up in `sumi.toml`.

### prepare-commit-msg hook

To see the enabled rules when writing a commit message, you can use the `prepare-commit-msg` hook:

```bash
git sumi --init prepare-commit-msg
```

The Bash hook (in `.git/hooks/prepare-commit-msg`) will prepend the enabled rules to the commit message template. This way, when you run `git commit`, you'll see something like:

```plaintext
# git-sumi rules enabled:
# Follow Conventional Commits format.
# Use the imperative mood in the description.
# Header length limit: 50
# List of allowed commit types: feat, fix, docs, refactor, test, chore, misc

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
```

## Linting pull request titles

If you're creating [squash-and-merge pull requests](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/configuring-commit-squashing-for-pull-requests), you can use git-**sumi** to lint the pull request title, which will be used as the merge commit message.

To do this, make sure you've set up `sumi.toml` at the root of the repository.

To achieve this, after setting up `sumi.toml`at the root of your repository, add the following action to your repository's `.github/workflows` directory:

```yaml title=".github/workflows/git-sumi.yml"
name: Lint pull request title

on:
  pull_request:
    types:
      - opened
      - edited
      - synchronize
      - ready_for_review

concurrency:
  group: ${{ github.workflow }}-${{ github.event.pull_request.number || github.ref }}
  cancel-in-progress: true

permissions:
  pull-requests: read

jobs:
  main:
    name: Run git-sumi
    runs-on: ubuntu-latest
    steps:
      - uses: welpo/git-sumi-action@main
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

This workflow uses the [git-**sumi** action](https://github.com/welpo/git-sumi-action) to lint the pull request title. It will fail if the title doesn't meet the rules set up in `sumi.toml`.

The workflow is triggered by pull request changes, including opening, editing, syncing, or marking as ready.
