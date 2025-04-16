# Using Git AST

This guide explains how to use Git AST in your daily development workflow.

## Basic Workflow

The beauty of Git AST is that it integrates seamlessly with your existing Git workflow. Once configured, you can use standard Git commands and Git AST works behind the scenes.

### Standard Git Commands

```bash
# Edit files as usual in your editor
vim myfile.rs

# Stage files with git add
git add myfile.rs

# Git AST processes the file through the clean filter
# (parsing it into an AST/CST before storing)

# Commit as usual
git commit -m "Updated myfile.rs"

# When checking out, Git AST's smudge filter
# converts the stored AST back to formatted source code
git checkout another-branch
```

### What Happens Behind the Scenes

1. When you run `git add`, the clean filter:
   - Parses your source code into an AST/CST
   - Serializes the AST/CST
   - Stores this representation in Git

2. When you run `git checkout`, the smudge filter:
   - Retrieves the serialized AST/CST
   - Generates formatted source code
   - Places this in your working directory

## Working with AST Fencing

Sometimes you need to commit code that doesn't parse correctly (e.g., work-in-progress). Git AST provides AST fencing for these situations:

```rust
// Your normal code here...

// git-ast:fence:start-wip
// This code doesn't parse yet, but you can still commit it
fn incomplete_function(param {
    // Missing closing parenthesis above
    let x = 10
    // No semicolon
// git-ast:fence:end-wip

// More normal code...
```

Fenced sections are stored as raw text, bypassing the AST parsing step.

## Viewing Diffs

Git AST affects how files are stored, but standard Git diff commands still work:

```bash
# Basic diff
git diff

# Diff between branches
git diff main feature-branch

# Diff against commit
git diff HEAD~3
```

The diffs you see are between the formatted code in your working directory, not the serialized AST/CST stored internally.

### Benefits of Git AST Diffs

With Git AST, you'll notice:
- Formatting changes don't appear in diffs
- Only meaningful code changes show up
- Code movement is better handled

## Common Scenarios

### Adding a New File Type

To start using Git AST with a new file type:

1. Update `.gitattributes`:
   ```
   *.py filter=git-ast-python
   ```

2. Configure Git:
   ```bash
   git config --local filter.git-ast-python.clean "git-ast clean --lang=python"
   git config --local filter.git-ast-python.smudge "git-ast smudge --lang=python"
   git config --local filter.git-ast-python.required true
   ```

### Working with Team Members Without Git AST

Team members without Git AST installed will:
- See the serialized AST/CST format when checking out files
- Need to install Git AST for a proper workflow

Best practice is for all team members to install Git AST when working on a repository configured to use it.

### Temporarily Disabling Git AST

If you need to temporarily disable Git AST:

```bash
# Disable for a specific operation
GIT_CONFIG_COUNT=1 \
GIT_CONFIG_KEY_0=filter.git-ast-rust.enabled \
GIT_CONFIG_VALUE_0=false \
git checkout main

# Or globally disable
git config --local filter.git-ast-rust.enabled false
```

Remember to re-enable it afterward:
```bash
git config --local filter.git-ast-rust.enabled true
```

## Troubleshooting

### Common Issues

1. **Unexpected Formatting Changes:**
   - Git AST enforces a canonical format on checkout
   - Local formatting preferences will be overridden

2. **Parse Errors on Commit:**
   - Git AST requires valid syntax for parsing
   - Use AST fencing for work-in-progress code

3. **Performance Concerns:**
   - For large repositories, enable process filters
   - Consider selectively applying Git AST to specific file types

### Getting Help

If you encounter issues:
- Check the [FAQ](../concepts/faq.md)
- Report bugs via GitHub Issues
- Consult the [architecture documentation](../architecture/design.md)

## Best Practices

1. **Consistent Configuration:** Ensure all team members use the same Git AST configuration
2. **Repository Setup:** Include Git AST setup instructions in your repository's README
3. **Commit Small Changes:** While Git AST handles large changes well, smaller commits are still better practice
4. **Formatting Config:** Agree on formatter settings to ensure consistent results across the team

## Next Steps

- Learn about [contributing](../contributing/guidelines.md) to Git AST
- Explore more [concepts](../concepts/key-concepts.md) behind Git AST
- Read about future [roadmap](../roadmap.md) plans 
