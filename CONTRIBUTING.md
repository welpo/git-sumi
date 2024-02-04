# Contributing to git-sumi

Halló!

Thanks for contributing to [git-sumi](https://github.com/welpo/git-sumi). Before implementing new features and changes, please [submit an issue](https://github.com/welpo/git-sumi/issues/new/choose) so that we can discuss it.

We welcome contributions in many forms, including:

- Bug reports
- Bug fixes
- Improvements to the codebase
- Documentation improvements
- Feature requests
- UX suggestions

If you're not sure how to contribute or need help with something, please don't hesitate to reach out via the [issue tracker](https://github.com/welpo/git-sumi/issues) or [email](mailto:osc@osc.garden?subject=[GitHub]%git-sumi).

## Getting started

This is a pure Rust project, so you'll only need to [install Rust](https://www.rust-lang.org/tools/install) to use `cargo`:

- `cargo fmt` to format your code.
- `cargo check` to check for errors.
- `cargo build` to compile.
- `cargo clippy` to catch common mistakes and improve code.
- `cargo test` to run tests.

To check **code coverage**, you can use [`tarpaulin`](https://github.com/xd009642/tarpaulin). First, install it with `cargo install cargo-tarpaulin`. Then:

- `cargo tarpaulin` to run tests and display a coverage report.
- `cargo tarpaulin --out html --out lcov` to generate a coverage report in HTML and LCOV formats. The HTML report will be available on the root of the project (`tarpaulin-report.html`). The LCOV (`lcov.info`) report can be used with the VSCode extension [`Coverage Gutters`](https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters) to display coverage in the editor.

## Coding guidelines

- Aim for 100% test coverage. 80% is the minimum.
- Keep the code clean and maintainable. Here are some guidelines:

<details>
  <summary>Click to expand guidelines</summary>

1. **Test coverage**: Ensure comprehensive code coverage and keep tests readable. 80% coverage is the minimum; 100% is nice to have.

2. **Short, focused functions**: Keep functions brief and adhere to a single responsibility. Minimise arguments and make function signatures intuitive.

3. **Descriptive naming**: Use unambiguous names to clarify function and variable purpose.

4. **Consistent level**: Maintain one level of abstraction or focus within functions.

5. **DRY**: Don't Repeat Yourself; abstract repeated code into functions.

6. **Error handling**: Use logging and provide clear, actionable error messages.

7. **Minimal comments**: Keep code self-explanatory. Explain the why, not the how.

8. **Early returns**: Avoid deep nesting.

</details>

## Pull Requests

Working on your first Pull Request? You can learn how from this free video series:

[**How to Contribute to an Open Source Project on GitHub**](https://egghead.io/courses/how-to-contribute-to-an-open-source-project-on-github)

Please make sure the following is done when submitting a pull request:

1. **Keep your PR small**. Small pull requests (~300 lines of diff) are much easier to review and more likely to get merged. Make sure the PR does only one thing, otherwise please split it.
2. **Use descriptive titles**. It is recommended to follow this [commit message style](#conventional-commit-messages).
3. **Test your changes**. Make sure to include tests that cover your changes. We prefer integration tests. You can find them in the `tests/lint` directory.
4. **Fill the PR template**. The template will guide you through the process of submitting a PR.

Our integration systems run automated tests to guard against mistakes. To speed things up, make sure you have done the following before submitting your PR:

- Run `cargo fmt` to format your code.
- Run `cargo clippy` to catch common mistakes and improve code.
- Make sure all new and existing tests pass with `cargo test`.
- If necessary, update the documentation (in `website/docs/`).

### Conventional Commit Messages with Gitmoji

See how a minor change to your commit message style can make you a better programmer.

Format: `<gitmoji> <type>(<scope>): <description>`

`<gitmoji>` is an emoji from the [gitmoji](https://gitmoji.dev/) list. It makes it easier to visually scan the commit history and quickly grasp the purpose of changes.

`<scope>` is optional. If your change affects a specific part of the codebase, consider adding the scope. Scopes should be brief but recognizable, e.g. `config`, `gitmoji`, or `cli`.

The various types of commits:

- `feat`: a new API or behavior **for the end user**.
- `fix`: a bug fix **for the end user**.
- `docs`: a change to the website or other Markdown documents.
- `refactor`: a change to code that doesn't change behavior, e.g. splitting files, renaming internal variables, improving code style…
- `test`: adding missing tests, refactoring tests; no production code change.
- `chore`: upgrading dependencies, releasing new versions… Chores that are **regularly done** for maintenance purposes.
- `misc`: anything else that doesn't change production code, yet is not `test` or `chore`. e.g. updating GitHub actions workflow.

The commits within your PR don't need to follow this convention (we'll [squash them](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/configuring-commit-squashing-for-pull-requests)). However, the PR title should be in this format. If you're not sure about the title, don't worry, we'll help you fix it. Your code is more important than conventions!

Example:

```
✨ feat(rules): add custom header pattern
^  ^--^^-----^  ^-----------------------^
|  |   |        |
|  |   |        +-> Description in imperative and lowercase.
|  |   |
|  |   +-> The scope of the change.
|  |
|  +-------> Type: see above for the list we use.
|
+----------> A valid gitmoji.
```

## Code of conduct

We expect all contributors to follow our [Code of Conduct](./CODE_OF_CONDUCT.md). Please be respectful and professional when interacting with other contributors.

Thank you for your contributions!
