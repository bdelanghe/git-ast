# Key Concepts in Git AST

This document explains the core concepts and terminology used throughout the Git AST project. Understanding these concepts is essential for developers and users of the system.

## Abstract Syntax Trees (ASTs) and Concrete Syntax Trees (CSTs)

### Abstract Syntax Tree (AST)
An **Abstract Syntax Tree** is a tree representation of the abstract syntactic structure of source code. ASTs remove syntactic details like whitespace, comments, and parentheses, focusing on the structural and semantic content of the code.

**Example:** For the expression `a + b * c`, an AST might look like:
```
   +
  / \
 a   *
    / \
   b   c
```

### Concrete Syntax Tree (CST)
A **Concrete Syntax Tree** (or parse tree) is a more detailed representation that preserves all syntactic elements of the source code, including whitespace, comments, and formatting details.

In Git AST, we primarily work with CSTs rather than ASTs because:
1. CSTs preserve comments and formatting details, which are critical for code readability
2. CSTs provide a more complete picture of the source code
3. CSTs make round-trip conversion (code → tree → code) more reliable

## Tree-sitter

**Tree-sitter** is the parsing technology at the heart of Git AST. It's an incremental parsing library that:

1. Generates concrete syntax trees from source code
2. Handles syntax errors gracefully (important for real-world code)
3. Can efficiently update a CST when only parts of the source change
4. Supports many programming languages through grammar definitions

Tree-sitter is well-suited for Git AST because it:
- Is fast enough for interactive use
- Handles comments and whitespace appropriately 
- Can generate CSTs that preserve all necessary information
- Has a growing ecosystem with support for many languages

## Git Clean and Smudge Filters

Git AST integrates with Git using its filter system:

### Clean Filter
The **clean filter** is applied when content is added to Git's object database (during `git add` or `git commit`). In Git AST, the clean filter:

1. Takes source code from the working directory
2. Parses it into a CST using Tree-sitter
3. Serializes the CST into a deterministic format
4. Passes this serialized representation to Git for storage

### Smudge Filter
The **smudge filter** is applied when content is retrieved from Git's object database (during `git checkout`). In Git AST, the smudge filter:

1. Takes the serialized CST from Git
2. Deserializes it into a memory representation
3. Generates formatted source code using a code formatter
4. Writes this source code to the working directory

These filters allow Git AST to transparently transform between source code (for human editing) and structured representations (for version control operations).

## AST Fencing

**AST Fencing** is a mechanism in Git AST that allows developers to mark sections of code that should not be parsed by the clean filter. This is useful for:

1. Work-in-progress code that doesn't yet parse correctly
2. Generated code that shouldn't be reformatted
3. Code sections that Tree-sitter has trouble parsing

Fencing uses special comments:
```
// git-ast:fence:start-wip
// Code here won't be parsed structurally
function notYetCompleted(x {
    // Missing closing parenthesis above
// git-ast:fence:end-wip
```

When the clean filter encounters these markers, it treats the enclosed content as an opaque text block, preserving it exactly without trying to parse its structure.

## Semantic Diff and Merge

**Semantic diffing** and **semantic merging** refer to comparing and combining code based on its structural meaning rather than its textual representation:

### Semantic Diff
Instead of showing line-by-line text differences, semantic diff shows structural changes:
- Adding, removing, or modifying functions, classes, or methods
- Changes to control flow or expressions
- Code movements (e.g., moving a function from one file to another)
- Ignoring purely stylistic changes (whitespace, formatting)

### Semantic Merge
Semantic merge uses structural understanding to resolve conflicts more intelligently:
- Automatically resolving non-conflicting structural changes
- Identifying true semantic conflicts (vs. apparent textual conflicts)
- Handling code movement and reorganization without unnecessary conflicts

## Node Identity

**Node identity** refers to the challenge of tracking the "same" code element (function, class, etc.) across different versions of a codebase. This is critical for:

1. Accurate semantic diff (knowing what changed vs. what moved)
2. Intelligent merging (correctly combining changes to the same logical element)
3. History tracking (e.g., "git blame" for AST nodes)

Approaches to node identity include:
- Content-based hashing (identify nodes by their content)
- Position-based heuristics (identify nodes by their structural location)
- Explicit IDs or annotations

Node identity is a complex problem that becomes more important as Git AST evolves to support advanced semantic operations.

## Deterministic Pretty Printing

**Pretty printing** (or code generation) is the process of converting an AST/CST back into human-readable source code. In Git AST, this happens in the smudge filter.

**Deterministic** pretty printing means that the same AST/CST always produces the same source code output, regardless of who runs the process or when. This property is essential for:

1. Consistent formatting across a team
2. Avoiding artificial changes in diffs
3. Reliable round-trip conversion (code → AST → code)

Git AST uses formatters like `dprint` that provide consistent, configurable code generation with support for multiple languages. 
