# Frequently Asked Questions (FAQ)

This document answers common questions about the Git AST project.

## General Questions

### What is Git AST?

Git AST is a language-aware extension for Git that operates on the structural representation (Abstract Syntax Trees or Concrete Syntax Trees) of code, rather than just lines of text. It enhances Git's version control capabilities to provide more meaningful diffs, smarter merges, and consistent formatting.

### How does Git AST differ from standard Git?

Standard Git tracks changes line-by-line as text, while Git AST understands the structure of your code. This means:
- Formatting changes don't appear in diffs
- Code movement is better tracked
- Merges can be smarter about structural changes
- Formatting is consistently applied

### Is Git AST a replacement for Git?

No, Git AST is an extension that works with Git. It uses Git's filter mechanism to integrate into the normal Git workflow, enhancing it with language awareness rather than replacing it.

### What languages does Git AST support?

Initially, Git AST focuses on a single language (likely Rust) to prove the concept. However, the architecture is designed to support multiple languages through Tree-sitter grammar plugins. The roadmap includes expanding language support over time.

## Technical Questions

### How does Git AST work?

Git AST uses Git's clean and smudge filters:
1. When committing, the clean filter parses source code into an AST/CST and serializes it for storage
2. When checking out, the smudge filter deserializes the AST/CST and generates formatted source code

This happens transparently as you use normal Git commands.

### What is Tree-sitter and why does Git AST use it?

Tree-sitter is a parsing library that generates concrete syntax trees from source code. Git AST uses it because:
- It's fast and incremental
- It handles syntax errors gracefully
- It preserves comments and whitespace information
- It supports many programming languages through grammar definitions

### What happens if my code doesn't parse?

By default, the clean filter will fail if your code doesn't parse, preventing the commit. However, Git AST provides an "AST fencing" mechanism using special comments that allows you to mark sections of code that shouldn't be parsed, enabling you to commit work-in-progress code.

### How does Git AST store the AST/CST?

The AST/CST is serialized into a deterministic format (such as JSON, S-expressions, or a binary format like CBOR) and stored as a Git blob. The exact serialization format is chosen to balance human readability, size efficiency, and information preservation.

### Does Git AST increase repository size?

Serialized AST/CST representations might be larger than the original source code in some cases. However:
- Git's compression should help mitigate this
- The benefits of structural understanding often outweigh the size increase
- Binary serialization formats can be used to reduce size if needed

## Usage Questions

### Do all team members need to install Git AST?

Yes, for the best experience, all team members should install Git AST. Otherwise, team members without it will see the serialized AST/CST format when checking out files, not the human-readable source code.

### Will Git AST change my coding style?

Yes, Git AST enforces a consistent code format through its smudge filter. When you check out files, they will be formatted according to the project's formatting rules, regardless of how they were originally written. This is a feature that ensures consistency across the team.

### Can I use Git AST with existing repositories?

Yes, Git AST can be added to existing repositories. However, it will cause a significant change in how files are stored in Git's object database when they're next modified. Consider:
- Starting with a small subset of files
- Ensuring all team members install Git AST
- Making the transition during a period of low development activity

### How do I view diffs with Git AST?

Standard Git diff commands (`git diff`, `git show`, etc.) will work as usual. The diffs will show changes between the formatted source code in your working directory, not the serialized AST/CST stored internally. This means you won't see purely formatting-related changes.

### Can I disable Git AST temporarily?

Yes, you can temporarily disable Git AST by setting the filter's `enabled` property to `false`:

```bash
git config --local filter.git-ast-rust.enabled false
```

Remember to re-enable it afterward:

```bash
git config --local filter.git-ast-rust.enabled true
```

## Performance Questions

### Will Git AST slow down my Git operations?

Git AST adds parsing and formatting steps to certain Git operations, which can affect performance:
- `git add`/`commit`: Additional time for parsing files
- `git checkout`: Additional time for formatting files

To mitigate this:
- Git AST uses Git's filter process protocol for better performance
- The implementation focuses on efficiency
- Tree-sitter is designed for speed
- You can selectively apply Git AST to specific file types

### How does Git AST handle large repositories?

For large repositories:
- Apply Git AST selectively to file types that benefit most
- Use the process filter protocol for better performance
- Consider using binary serialization formats for efficiency
- Test performance on a subset of files before applying broadly

## Contributing Questions

### How can I contribute to Git AST?

See our [Contributing Guidelines](../contributing/guidelines.md) for detailed information on how to contribute. Ways to help include:
- Code contributions
- Documentation improvements
- Testing and reporting issues
- Suggesting features
- Helping other users

### I found a bug in Git AST. How do I report it?

Please open an issue on our GitHub repository with:
- A clear description of the bug
- Steps to reproduce the issue
- Information about your environment (OS, Git version, etc.)
- If possible, a minimal example demonstrating the problem

## Future Development

### What's on the roadmap for Git AST?

See our full [Roadmap](../roadmap.md) for detailed information. Key future developments include:
- Support for additional programming languages
- True semantic diff and merge capabilities
- Improved integration with IDEs and Git hosting platforms
- Performance optimizations

### Will Git AST support semantic merging?

Yes, advanced semantic merging is a key goal on our roadmap. Initial versions focus on the core clean/smudge pipeline, but future development will aim to leverage the AST/CST representation for smarter merge operations that understand code structure.

### Is Git AST open source?

Yes, Git AST is open source under the MIT License. We welcome contributions from the community. 
