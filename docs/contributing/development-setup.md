# Development Setup for Git AST

This guide will help you set up your development environment to contribute to Git AST.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Git**: Version 2.28.0 or later
- **Rust**: Version 1.67.0 or later (install via [rustup](https://rustup.rs/))
- **Cargo**: Comes with Rust
- **Tree-sitter CLI**: For testing parsers
- **mise** (formerly rtx): For environment management (optional but recommended)

## Setting Up the Development Environment

### Step 1: Clone the Repository

```bash
# If you haven't already forked the repository on GitHub:
git clone https://github.com/yourusername/git-ast.git
cd git-ast
```

### Step 2: Install Development Dependencies

We use `mise` to manage development dependencies:

```bash
# Install mise if you don't have it
curl https://mise.run | sh

# Install all tools defined in .mise.toml
mise install
```

If you prefer not to use `mise`, ensure you install:
- The specific Rust version listed in `.mise.toml`
- Any additional tools listed there

### Step 3: Install Rust Dependencies

```bash
# Install build dependencies
cargo build

# Install development tools
cargo install cargo-watch   # For auto-recompilation during development
cargo install cargo-expand  # For macro debugging
cargo install cargo-insta   # For snapshot testing
```

### Step 4: Set Up Tree-sitter

```bash
# Install the Tree-sitter CLI
npm install -g tree-sitter-cli

# Generate Tree-sitter parser
tree-sitter generate
```

### Step 5: Configure Testing Environment

To run the tests properly, you'll need:

```bash
# Create test config
cp config/test-config.example.json config/test-config.json
# Edit this file with your local settings
```

## Development Workflow

### Building the Project

```bash
# Standard build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific tests
cargo test clean_filter

# Run with verbose output
cargo test -- --nocapture
```

### Development Cycle

We recommend using `cargo watch` for rapid development:

```bash
# Automatically rebuild on changes
cargo watch -x build

# Run tests on changes
cargo watch -x "test --lib"
```

### Debugging

For debugging, you can:

1. Use `println!` or `eprintln!` for quick output
2. Enable logging with the `RUST_LOG` environment variable:
   ```bash
   RUST_LOG=debug cargo run -- clean --lang=rust sample.rs
   ```
3. Use a debugger like GDB or LLDB with VS Code

## Code Organization

Understanding the codebase structure:

- `src/`: Core source code
  - `parser/`: Integration with Tree-sitter and parsing logic
  - `filters/`: Git clean and smudge filter implementations
  - `serialization/`: AST/CST serialization formats
  - `formatters/`: Code formatting integration
- `tests/`: Integration tests
- `examples/`: Example code and usage demonstrations
- `scripts/`: Utility scripts for development
- `docs/`: Project documentation

## Setting Up Git Hooks

We have pre-commit hooks to ensure code quality:

```bash
# Install git hooks
cp scripts/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit
```

This will run `cargo fmt` and `cargo clippy` before each commit.

## Editor Integration

### VS Code

We recommend these extensions:
- rust-analyzer
- Better TOML
- CodeLLDB (for debugging)

Recommended `settings.json` additions:
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "editor.formatOnSave": true
}
```

### Other Editors

For other editors, ensure you have:
- Rust language support
- Auto-formatting with rustfmt
- Linting with clippy

## Creating a Test Configuration

For developing and testing Git AST with real repositories:

1. Create a test repository:
   ```bash
   mkdir ~/test-repos
   cd ~/test-repos
   git init test-repo
   cd test-repo
   ```

2. Configure Git AST in the test repository:
   ```bash
   # Create .gitattributes
   echo "*.rs filter=git-ast-rust" > .gitattributes
   
   # Configure Git to use your development version
   git config --local filter.git-ast-rust.clean "/path/to/your/git-ast/target/debug/git-ast clean --lang=rust"
   git config --local filter.git-ast-rust.smudge "/path/to/your/git-ast/target/debug/git-ast smudge --lang=rust"
   ```

## Troubleshooting

### Common Issues

1. **Build failures:**
   - Check that you have the correct Rust version
   - Run `cargo clean` followed by `cargo build`

2. **Test failures:**
   - Ensure your test configuration is correct
   - Check that all dependencies are installed

3. **Tree-sitter issues:**
   - Regenerate parsers with `tree-sitter generate`
   - Check for parser version mismatches

### Getting Help

If you're stuck, you can:
- Open an issue on GitHub
- Ask for help in our discussions forum
- Check the existing documentation

## Next Steps

- Review the [contribution guidelines](./guidelines.md)
- Look for beginner-friendly issues labeled "good first issue"
- Read the [architecture documentation](../architecture/design.md) to understand how the system works 
