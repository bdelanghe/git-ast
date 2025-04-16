# Contributing to Git AST

Thank you for your interest in contributing to Git AST! This document outlines the process for contributing to the project and helps you get started.

## Ways to Contribute

There are many ways to contribute to Git AST:

1. **Code contributions:** Implementing new features or fixing bugs
2. **Documentation:** Improving or adding documentation
3. **Testing:** Testing the software and reporting issues
4. **Ideas and feedback:** Suggesting new features or improvements
5. **Community support:** Helping other users in discussions

## Development Setup

See [Development Setup](./development-setup.md) for detailed instructions on setting up your development environment.

## Contribution Workflow

### 1. Find or Create an Issue

- Browse existing [issues](https://github.com/yourusername/git-ast/issues) to find something to work on
- If you have a new idea, create a new issue to discuss it before starting work
- For small fixes, you can skip this step

### 2. Fork and Clone the Repository

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/your-username/git-ast.git
cd git-ast
git remote add upstream https://github.com/original-owner/git-ast.git
```

### 3. Create a Branch

Create a branch for your work:

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

Use a descriptive branch name that reflects the work you're doing.

### 4. Make Your Changes

- Follow the coding standards (see below)
- Write tests for your changes
- Update documentation as needed

### 5. Commit Your Changes

Write clear, descriptive commit messages:

```bash
git commit -m "feat: Add support for Python language"
# or
git commit -m "fix: Resolve parsing error for template literals"
```

We follow [Conventional Commits](https://www.conventionalcommits.org/) for commit messages.

### 6. Stay Updated with Upstream

Regularly sync your fork with the main repository:

```bash
git fetch upstream
git rebase upstream/main
```

### 7. Push Your Changes

```bash
git push origin feature/your-feature-name
```

### 8. Create a Pull Request

- Go to the GitHub repository page
- Click "Pull Request"
- Select your branch and fill out the PR template
- Link related issues

## Coding Standards

### General Guidelines

- Write clean, readable code
- Comment complex logic, but focus on making the code self-explanatory
- Follow the existing code style and patterns

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` to format code
- Run `cargo clippy` to check for common mistakes
- Ensure all tests pass with `cargo test`

### Documentation

- Document public APIs with rustdoc comments
- Keep documentation updated with code changes
- Use examples where appropriate

## Testing

- Write unit tests for new functionality
- Add integration tests for end-to-end workflows
- Ensure all existing tests pass before submitting a PR

## Review Process

Once you submit a PR:

1. Automated checks will run (CI/CD)
2. Maintainers will review your code
3. You may need to make changes based on feedback
4. Once approved, a maintainer will merge your PR

## Recognition

All contributors are valued members of the Git AST community:

- All contributors are listed in the project's CONTRIBUTORS.md file
- Significant contributions may lead to maintainer status

## Communication

- For questions about contributing, open a discussion on GitHub
- For bug reports, open an issue
- For feature discussions, open an issue with the "enhancement" label

## Code of Conduct

We follow a Code of Conduct to ensure a welcoming community. Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before participating.

## License

By contributing to Git AST, you agree that your contributions will be licensed under the project's MIT License.

Thank you for contributing to Git AST! 
