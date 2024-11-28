<p align="center">
    <a href="https://sumi.rs">
        <img src="https://raw.githubusercontent.com/welpo/git-sumi/main/website/static/img/logo.png" width="300" alt="git-sumi logo: a lantern held on a bamboo stick over the sea">
    </a>
    <br><br>
    <a href="https://github.com/welpo/git-sumi">
        <img src="https://img.shields.io/badge/clean_commits-git--sumi-0?style=flat-square&labelColor=202b2d&color=b05275" alt="Clean commits"></a>
    <a href="CONTRIBUTING.md#pull-requests">
        <img src="https://img.shields.io/badge/PRs-welcome-0?style=flat-square&labelColor=202b2d&color=b05275" alt="PRs welcome"></a>
    <a href="https://github.com/welpo/git-sumi/releases">
        <img src="https://img.shields.io/github/v/release/welpo/git-sumi?style=flat-square&labelColor=202b2d&color=b05275" alt="Latest release"></a>
    <a href="https://crates.io/crates/git-sumi">
        <img src="https://img.shields.io/crates/v/git-sumi?style=flat-square&labelColor=202b2d&color=b05275" alt="Crates.io"></a>
    <a href="https://codecov.io/gh/welpo/git-sumi">
        <img src="https://img.shields.io/codecov/c/gh/welpo/git-sumi?style=flat-square&labelColor=202b2d&color=b05275" alt="Codecov"></a>
    <br>
    <a href="https://github.com/welpo/git-sumi/actions/workflows/ci.yml">
        <img src="https://img.shields.io/github/actions/workflow/status/welpo/git-sumi/ci.yml?style=flat-square&labelColor=202b2d" alt="CI"></a>
    <a href="https://github.com/welpo/git-sumi/actions/workflows/release.yml">
        <img src="https://img.shields.io/github/actions/workflow/status/welpo/git-sumi/release.yml?style=flat-square&labelColor=202b2d&label=deploy" alt="Deployment"></a>
    <a href="https://sumi.rs/docs">
        <img src="https://img.shields.io/website?url=https%3A%2F%2Fsumi.rs&style=flat-square&label=docs&labelColor=202b2d" alt="Documentation"></a>
    <a href="#-license">
        <img src="https://img.shields.io/badge/license-MIT%20or%20Apache%202.0-0?style=flat-square&labelColor=202b2d&color=b05275" alt="MIT or Apache 2.0 License"></a>
    <a href="http://isitmaintained.com/project/welpo/git-sumi">
        <img src="http://isitmaintained.com/badge/resolution/welpo/git-sumi.svg" alt="Average time to resolve an issue"></a>
    <a href="http://isitmaintained.com/project/welpo/git-sumi">
        <img src="http://isitmaintained.com/badge/open/welpo/git-sumi.svg" alt="Percentage of issues still open"></a>
</p>

<h4 align="center">
  <a href="https://sumi.rs/docs">Documentation</a> |
  <a href="https://sumi.rs">Website</a>
</h4>

<h1 align="center">git-sumi</h1>

<h3 align="center">The non-opinionated Rust-based commit message linter</h3>

> sumi (墨, /<span title="/s/: 's' in 'sigh'">s</span><span title="/ɯ/: like 'u' in 'flute', but unrounded">ɯ</span><span title="/m/: 'm' in 'my'">m</span><span title="/i/: 'i' in 'fleece'">i</span>/): ink, especially the type used in [traditional ink wash painting](https://en.wikipedia.org/wiki/Ink_wash_painting).

## 🎥 Demo

See how git-**sumi** can help you write better commit messages:

https://github.com/welpo/git-sumi/assets/6399341/cf1b4f00-3f79-454f-a533-5b36812dd464

## ✨ Main features

- **Customizable rules**: Configure rules to enforce [Conventional Commits](https://www.conventionalcommits.org/), length limits, [Gitmoji](https://gitmoji.dev/) usage, and [more](https://sumi.rs/docs/rules).

- **Clear error reporting**: Provides detailed error reporting, making fixing commit messages straightforward and educational.

- **Seamless integration**: As a single binary, git-sumi easily integrates into your existing workflow with minimal setup. You can even use the [GitHub Action](https://github.com/welpo/git-sumi-action) to lint your commits (or PR titles) without installing anything.

## 🚀 Quick start

Install git-sumi:

```bash
cargo install git-sumi
# or use pip
pip install git-sumi  # or pipx install git-sumi
```

Create a base `sumi.toml` configuration file in your repository:

```bash
git sumi --init
```

Edit `sumi.toml` with your project's rules:

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

Set up automatic commit message validation:

```bash
git sumi --init commit-msg
```

Before each commit, git-sumi will lint your commit message. If it doesn't meet your project's rules, you'll see an error message and the commit will be aborted.

## 📝 Documentation

Learn how to use git-**sumi** from the [documentation](https://sumi.rs/docs).

- [Quick setup](https://sumi.rs/docs/#quickstart)
- [Configuration](https://sumi.rs/docs/configuration)
- [Rules](https://sumi.rs/docs/rules)
- [Examples](https://sumi.rs/docs/examples)
- [Integration](https://sumi.rs/docs/integration)
- [Frequently Asked Questions](https://sumi.rs/docs/faq)

## 👥 Contributing

Please do! Contributions are always welcome. We appreciate improvements to the documentation, development of new rules, code cleanup, resolving issues, requesting or developing new functionality…

Take a look at our [Contributing Guidelines](/CONTRIBUTING.md) for more information on how to get started.

## 📄 License

This project is licensed under the terms of both the [MIT license](/LICENSE-MIT) and the [Apache License (Version 2.0)](/LICENSE-APACHE), at your option.
