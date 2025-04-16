# Installing Git AST

This guide walks you through the process of installing Git AST and setting up your environment.

## Prerequisites

Before installing Git AST, ensure you have the following prerequisites:

- **Git:** Git AST is an extension to Git, so you need Git installed (version 2.28.0 or later recommended)
- **Rust:** The core tools are written in Rust (version 1.67.0 or later)
- **Tree-sitter:** Required for parsing source code files
- **Language-specific formatters:** For languages you plan to use with Git AST

## Installation Methods

### Option 1: Installing from Package Manager (Coming Soon)

We're working on providing package manager installation for common platforms. This will be available in the future.

### Option 2: Building from Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/git-ast.git
   cd git-ast
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Add to your PATH:**
   ```bash
   # For Unix/Linux/macOS:
   export PATH="$PATH:$(pwd)/target/release"
   
   # Add to your shell profile for persistence
   echo 'export PATH="$PATH:$HOME/path/to/git-ast/target/release"' >> ~/.bashrc  # or ~/.zshrc
   ```

## Repository Configuration

After installing Git AST, you need to configure each repository where you want to use it:

### Basic Setup

1. **Create `.gitattributes` file** in your repository root:
   ```
   # Configure Git AST for specific file types
   *.rs filter=git-ast-rust
   *.js filter=git-ast-javascript
   # Add more file types as needed
   ```

2. **Configure Git filters:**
   ```bash
   # For Rust files
   git config --local filter.git-ast-rust.clean "git-ast clean --lang=rust"
   git config --local filter.git-ast-rust.smudge "git-ast smudge --lang=rust"
   git config --local filter.git-ast-rust.required true
   
   # For JavaScript files
   git config --local filter.git-ast-javascript.clean "git-ast clean --lang=javascript"
   git config --local filter.git-ast-javascript.smudge "git-ast smudge --lang=javascript"
   git config --local filter.git-ast-javascript.required true
   ```

### Performance Optimization (Recommended)

For better performance, use Git's filter process protocol:

```bash
git config --local filter.git-ast-rust.process "git-ast filter-process --lang=rust"
git config --local filter.git-ast-javascript.process "git-ast filter-process --lang=javascript"
```

## Verifying Installation

To verify your installation:

1. **Check that Git AST is in your path:**
   ```bash
   git-ast --version
   ```

2. **Test on a sample file:**
   ```bash
   # Create a test file
   echo 'fn main() { println!("Hello, world!"); }' > test.rs
   
   # Add and commit
   git add test.rs
   git commit -m "Test Git AST"
   
   # This should process the file through Git AST
   ```

## Troubleshooting

If you encounter issues during installation:

### Common Problems

1. **Command not found:**
   - Ensure Git AST is in your PATH
   - Try using the full path to the executable

2. **Filter not working:**
   - Check your `.gitattributes` file
   - Verify Git config settings with `git config --local --list | grep filter`

3. **Parse errors:**
   - Ensure source files are syntactically valid
   - Check for unsupported language features

### Getting Help

- **GitHub Issues:** Report bugs or installation problems on our GitHub repository
- **Documentation:** Refer to the [Architecture](../architecture/design.md) documentation for more details on how Git AST works

## Next Steps

After installation:

1. Learn about [using Git AST](./usage.md) in your daily workflow
2. Explore [key concepts](../concepts/key-concepts.md) to better understand how Git AST works
3. Consider [contributing](../contributing/guidelines.md) to the project 
