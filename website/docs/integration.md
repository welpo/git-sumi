---
sidebar_position: 6
---

# Integration

## Local setup

You can use git-**sumi** to lint each commit message, interrupting commits that don't meet the specified rules.

Before continuing, make sure you have installed git-**sumi** and have a configuration file (`sumi.toml`) at the root of your repository. The [Getting Started](/docs) guide will help you with this.

Run this command to set up a Git hook:

```bash
git sumi --init hook
```

This will create a Bash [Git `commit-msg` hook](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks#_committing_workflow_hooks) in your `.git/hooks` directory. The hook will abort the commit if its message doesn't pass the rules set up in `sumi.toml`.

## Linting pull request titles

If you're creating [squash-and-merge pull requests](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/configuring-commit-squashing-for-pull-requests), you can use git-**sumi** to lint the pull request title, which will be used as the merge commit message.

To do this, make sure you've set up `sumi.toml` at the root of the repository.

To achieve this, after setting up `sumi.toml`at the root of your repository, add the following action to your repository's `.github/workflows` directory:

```yaml title=".github/workflows/git-sumi.yaml"
name: Lint pull request title

on:
  pull_request:
    types:
      - opened
      - edited
      - synchronize
      - ready_for_review

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
