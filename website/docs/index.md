---
sidebar_position: 1
---

# Getting Started

git-**sumi** is the non-opinionated Rust-powered commit message linter.

Following clear guidelines for commit messages and consistently adhering to them makes using Git and collaborating with other developers much smoother.

Let's see how to get started with git-**sumi** in less than 5 minutes.

## Quick start

1. Install git-**sumi**:

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs>
<TabItem value="cargo" label="Cargo">
```bash
cargo install git-sumi
```

</TabItem>
<TabItem value="pip" label="pip">
```bash
pip install git-sumi
```

</TabItem>
</Tabs>

2. Initialize git-**sumi** from the root of your project:

```bash
git sumi --init
```

Edit the default configuration (`sumi.toml`) to your liking. Check out the [examples](/docs/examples) for some ideas.

To use git-**sumi**, you can either use it directly from the command line or set up a Git hook.

### Git hook

Set up a hook to lint your commit messages automatically:

```bash
git sumi --init hook
```

This will create a Bash [Git `commit-msg` hook](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks#_committing_workflow_hooks) in your `.git/hooks` directory. The hook will abort the commit if its message doesn't pass the rules you've set up.

### Command line

Use git-**sumi** with `-c | --commit` to validate the commit message before committing:

```bash
git sumi -c '🎨 refactor: format JS with Prettier'
```

If your message passes all rules, it will be committed. Otherwise, you'll see what went wrong.

## Further reading

- [Examples](/docs/examples)
- [Configuration](/docs/configuration)

## Contributing

Please do! We appreciate bug reports, improvements to documentation (however minor), feature requests, code contributions… To get started, take a look at the [contributing guidelines](https://github.com/welpo/git-sumi/blob/main/CONTRIBUTING.md).
