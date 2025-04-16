# Git AST Architecture Design

This document outlines the architectural design of Git AST, focusing on the core mechanisms, data flow, and integration points.

## Architecture Overview

Git AST integrates with the existing Git workflow using Git's built-in clean and smudge filters. This approach provides several advantages:

1. **Minimal Disruption:** Developers continue to use standard Git commands
2. **Broad Tool Compatibility:** IDEs, editors, and other tools see normal source files
3. **Leverages Git Infrastructure:** Uses Git's blob storage and filter mechanisms

### High-Level Architecture Diagram

```
┌───────────────────┐         ┌────────────────────┐         ┌──────────────────┐
│  Working          │         │  Git AST Pipeline  │         │  Git Repository  │
│  Directory        │         │                    │         │                  │
│  (Source Files)   │         │                    │         │  (AST/CST Blobs) │
└─────────┬─────────┘         └─────────┬──────────┘         └────────┬─────────┘
          │                             │                             │
          │                             │                             │
          │                             │                             │
          │                             │                             │
┌─────────▼─────────┐         ┌─────────▼──────────┐         ┌────────▼─────────┐
│                   │         │                    │         │                  │
│  git add/commit   │──────►  │  clean filter     │──────►  │  Git storage     │
│                   │         │  (AST Parser)     │         │  (.git/objects)  │
│                   │         │                   │         │                  │
└───────────────────┘         └────────────────────┘         └──────────────────┘
                                                                      │
                                                                      │
                                                                      │
                                                                      │
┌───────────────────┐         ┌────────────────────┐         ┌────────▼─────────┐
│                   │         │                    │         │                  │
│  Working          │◄──────  │  smudge filter    │◄──────  │  Git checkout    │
│  Directory        │         │  (Pretty Printer) │         │                  │
│                   │         │                   │         │                  │
└───────────────────┘         └────────────────────┘         └──────────────────┘
```

## Core Components

### 1. Clean Filter (Working Directory → Git Storage)

The clean filter transforms source code files into serialized AST/CST representations:

1. **Parsing:** Tree-sitter parses source code into a concrete syntax tree (CST)
2. **Serialization:** The CST is converted to a deterministic format (e.g., JSON)
3. **Storage:** Git stores this serialized representation as a blob

**Key Design Considerations:**
- **Error Handling:** How to handle parse errors (fail commit, provide warnings)
- **AST Fencing:** Special comments to mark WIP sections that shouldn't be parsed
- **Performance:** Optimizing parsing for large files and bulk operations

### 2. Smudge Filter (Git Storage → Working Directory)

The smudge filter transforms serialized AST/CST representations back into source code:

1. **Deserialization:** Convert the stored blob into an in-memory AST/CST
2. **Code Generation:** Use a formatter (dprint) to generate consistently formatted source code
3. **Output:** Write the formatted code to the working directory

**Key Design Considerations:**
- **Deterministic Formatting:** Ensuring consistent output regardless of who runs the filter
- **Comment Preservation:** Maintaining all comments, including documentation and pragmas
- **Round-trip Fidelity:** Ensuring no information is lost in the clean → smudge cycle

### 3. Serialization Format

The serialization format is critical for reliable operation:

- **Requirements:** Deterministic, compact, preserves all necessary information
- **Options:** JSON (human-readable but verbose), CBOR (compact binary format), S-expressions
- **Node Identity:** If implementing semantic operations, need a way to track node identity across versions

## Integration Points

### Git Configuration

Git AST is configured through standard Git mechanisms:

1. **`.gitattributes`:** Specifies which files use the clean/smudge filters
2. **`git config`:** Configures the filter commands and their behavior

Example `.gitattributes`:
```
*.rs filter=git-ast-rust
*.js filter=git-ast-javascript
```

Example filter configuration:
```sh
git config --local filter.git-ast-rust.clean "git-ast clean --lang=rust"
git config --local filter.git-ast-rust.smudge "git-ast smudge --lang=rust"
```

### Tool Ecosystem Integration

Git AST aims to integrate with the broader development ecosystem:

- **IDEs/Editors:** Working directory files are normal source code
- **Git Hosting Platforms:** May require custom diff drivers for proper visualization
- **CI/CD Systems:** Need Git AST installed to properly check out and process files

## Performance Considerations

Performance is critical for developer experience:

1. **Filter Process Protocol:** Use Git's long-running filter process protocol for better performance
2. **Tree-sitter Efficiency:** Leverage Tree-sitter's incremental parsing for speed
3. **Caching:** Cache parsed ASTs when possible to avoid redundant parsing
4. **Selective Application:** Apply filters only to supported languages and files under a threshold size

## Error Handling Strategy

Robust error handling ensures a good developer experience:

1. **Parse Errors:** By default, fail the clean operation if code doesn't parse
2. **AST Fencing:** Allow marking sections as "do not parse" for WIP code
3. **Fallback Mode:** Option to store unparseable files as-is with warnings
4. **Verbose Logging:** Detailed error messages for debugging parse issues

## Security Considerations

1. **Untrusted Repositories:** Consider how filters behave when cloning untrusted repositories
2. **Performance Attacks:** Guard against maliciously crafted files that might cause performance issues
3. **Command Injection:** Ensure proper escaping in filter commands

## Future Architectural Considerations

The design allows for future enhancements:

1. **Semantic Diff/Merge:** Comparing and merging at the AST/CST level
2. **Language Server Integration:** Providing semantic information to editors
3. **Extended Metadata:** Storing additional metadata alongside the AST/CST

## References

- [Git Filter Documentation](https://git-scm.com/docs/gitattributes#_filter)
- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [dprint Documentation](https://dprint.dev/) 
