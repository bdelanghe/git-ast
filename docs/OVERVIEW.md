# Git AST Project Overview

## 1. Goal: Semantic Version Control

The primary goal of `git-ast` is to enhance Git's version control capabilities by operating on the structural representation (Abstract Syntax Trees or Concrete Syntax Trees - AST/CST) of code, rather than just lines of text. This aims to provide:

*   **More Meaningful Diffs:** Ignoring purely stylistic changes (formatting, whitespace) and highlighting structural code modifications.
*   **Smarter Merges:** Reducing conflicts caused by code movement or trivial refactorings.
*   **Consistent Formatting:** Enforcing a canonical code format across the repository automatically.

## 2. Core Mechanism: Git Filters

`git-ast` leverages Git's built-in `clean` and `smudge` filters:

*   **`clean` Filter (on `git add`/`git commit`):**
    1.  Takes the source code file from the working directory.
    2.  Parses it into an AST/CST using a parser (like Tree-sitter).
    3.  Serializes this AST/CST into a defined format (e.g., JSON, S-expressions).
    4.  This serialized AST/CST is what Git stores in its object database (blob).
*   **`smudge` Filter (on `git checkout`):**
    1.  Takes the serialized AST/CST blob from Git's object database.
    2.  Deserializes it back into an in-memory AST/CST representation.
    3.  Generates formatted source code from the AST/CST using a pretty-printer/formatter (like `rustfmt` or potentially `dprint`).
    4.  This generated source code is written to the working directory.

Developers interact only with the source code in their working directory; the AST/CST storage is handled transparently by Git via these filters.

## 3. Key Technologies & Concepts

*   **Tree-sitter:** The core parsing library. It generates CSTs from source code, handling syntax errors gracefully and preserving comments, which is crucial for round-tripping code accurately. It supports multiple languages via grammars.
*   **Code Formatters (e.g., `dprint`):** Used in the `smudge` process to generate human-readable, consistently formatted code from the AST/CST. **[`dprint`](https://dprint.dev/) is the primary choice** due to its speed, pluggable nature supporting multiple languages via WASM plugins, and Rust implementation, aligning well with the potential use of Rust for the core tooling.
*   **AST Serialization:** A deterministic format (like JSON, S-expression, or potentially a binary format like CBOR for efficiency) is needed to store the AST/CST in Git blobs reliably. Consistency is key to avoid phantom diffs.
*   **AST Fencing (Proposed):** A mechanism using special comments (e.g., `// git-ast:fence:start-wip` and `// git-ast:fence:end-wip`) to mark sections of syntactically incomplete or intentionally non-parseable code. This allows developers to commit work-in-progress without causing the `clean` filter to fail parsing. The filter would treat fenced blocks as opaque strings or special nodes within the AST structure, preserving them but not parsing their content. This addresses the practical need to commit WIP code while maintaining the integrity of the AST for parseable sections.
*   **Jujutsu (`jj`) (Potential Integration):**
    *   `git-ast` could integrate *with* `jj`, leveraging its potentially more advanced features or plugin system.
    *   `git-ast` could reuse components *from* `jj`'s Apache 2.0 licensed Rust codebase (e.g., its commit graph logic, revision sets, merge algorithms) to accelerate development.

## 4. Scope & MVP Definition (Based on `docs/scope.md`)

To ensure feasibility, the project scope is initially limited. The Minimum Viable Product (MVP) focuses on proving the core pipeline:

1.  **Single Language Focus:** Start with one language (e.g., Rust) to establish the parsing, serialization, and formatting pipeline.
2.  **End-to-End Filters:** Implement working `clean` and `smudge` filters for the chosen language, ensuring lossless round-tripping (preserving code logic and comments).
3.  **Deterministic AST Serialization:** Choose and implement a simple, stable serialization format (e.g., JSON).
4.  **Git Integration Setup:** Provide clear instructions or scripts for users to configure the filters (`.gitattributes`, `git config`).
5.  **Basic Diff/Merge:** Rely initially on Git's standard text diff/merge on the *formatted* code in the working directory. The goal is to show improvement over standard Git (fewer formatting diffs) and ensure it doesn't break existing merge workflows. Semantic diff/merge tools (like Difftastic integration) are deferred.
6.  **Performance Baseline:** Test on a moderately sized project to ensure performance is acceptable for common Git operations.
7.  **Documentation:** Provide a README explaining the workflow, setup, limitations, and known issues.

## 5. How Concepts Apply

*   **Tree-sitter:** Foundational parser for the `clean` filter.
*   **dprint:** Preferred formatter for the `smudge` filter.
*   **AST Fencing:** Addresses the practical need to commit incomplete code within the AST-based system by treating fenced blocks as opaque data.
*   **Jujutsu:** Offers potential integration points or reusable code components under its license.
*   **Git Plugins:** Refers to extending Git via filters, hooks, and custom commands, not a formal plugin architecture within Git itself. 
