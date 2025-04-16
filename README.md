# Git AST: A Language-Aware Git Extension

Git AST provides **language-aware extensions for Git**, leveraging Abstract Syntax Trees (ASTs) instead of traditional line-based diffs. This enhances Git with semantic understanding, leading to more meaningful history, easier merges, and enhanced code consistency.

## Value Proposition

Why use Git AST?

- **Cleaner Diffs:** Focus on meaningful code changes, ignore formatting noise
- **Smarter Merges:** Reduce conflicts caused by code movement or non-competing structural edits
- **Consistent Formatting:** Enforce a canonical code style automatically across your repository

## How It Works

Git AST leverages Git's clean and smudge filters to operate on the structure of your code:

1. **When You Commit:** Source code is parsed into a syntax tree (AST/CST) and stored in Git
2. **When You Check Out:** The stored tree is converted back into consistently formatted source code

This structural approach lets you focus on semantic changes rather than textual differences.

## Getting Started

- [Installation Guide](./docs/getting-started/installation.md) - Set up Git AST in your environment
- [Usage Guide](./docs/getting-started/usage.md) - Learn how to use Git AST in your workflow
- [Documentation](./docs/start-here.md) - Comprehensive documentation

## Documentation

### Core Documentation
- [Project Overview](./docs/overview.md) - Goal, mechanism, and key concepts
- [Roadmap](./docs/roadmap.md) - Project development timeline

### Technical Documentation
- [Architecture Design](./docs/architecture/design.md) - Technical architecture and data flow
- [Clean/Smudge Filters](./docs/architecture/clean-smudge-filters.md) - Details on the Git filter implementation

### Concepts and Reference
- [Key Concepts](./docs/concepts/key-concepts.md) - Detailed explanation of core concepts
- [Glossary](./docs/concepts/glossary.md) - Definition of terms
- [FAQ](./docs/concepts/faq.md) - Frequently asked questions

### Contributing
- [Contribution Guidelines](./docs/contributing/guidelines.md) - How to contribute
- [Development Setup](./docs/contributing/development-setup.md) - Setting up your development environment

For a full documentation overview, see [Documentation Index](./docs/README.md).

## Project Status

**Current Phase:** Proof of Concept (POC) refinement

This project is under active development. The current focus is on refining the architecture to use Git's clean/smudge filters for seamless integration, leveraging Tree-sitter to parse source files into structured representations.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

We welcome contributions! Please see our [contribution guidelines](./docs/contributing/guidelines.md) for how to get involved.
