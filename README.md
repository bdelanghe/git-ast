# Git AST

## Project Status

**Current Phase:** Proof of Concept (POC)

This project explores replacing Git's traditional line-based versioning with an Abstract Syntax Tree (AST)-based approach. The goal is to make version control language-aware, enabling semantic diffs/merges and consistent code formatting.

**Current Focus:** Integrating Tree-sitter to parse source files from an existing Git repository into ASTs. This foundational step is necessary before exploring storage mechanisms or interaction models like FUSE.

**Initial POC Goal & Success Criteria:**
*   Successfully parse a sample source file (e.g., Rust) from a target Git repository using Tree-sitter within the `git-ast` tool.
*   Represent the parsed AST in memory (e.g., using Tree-sitter's data structures).
*   (Stretch Goal): Define a basic strategy for mapping this AST structure to Git blob/tree objects conceptually.

## Motivation

Traditional Git operates on text lines, leading to limitations:
*   **No Semantic Understanding:** Formatting changes create noise in diffs; merges often conflict unnecessarily when code is moved or refactored.
*   **Inconsistent Formatting:** History tracks textual changes, not structural intent, making consistent formatting across a team difficult.

An AST-based approach aims to:
*   **Enable Semantic Operations:** Diffs ignore formatting noise; merges can automatically resolve structural changes (like moving a function edited by someone else).
*   **Ensure Consistency:** Store the canonical AST; generate formatted code on demand, potentially allowing personalized views.
*   **Provide Fine-Grained History:** Track changes at the AST node level (function, statement, etc.).

## Architecture Overview (MVC & FUSE Approach)

The proposed system uses a Model-View-Controller (MVC) pattern, mediated by a FUSE filesystem for tool compatibility:

*   **Model (AST & Git Objects):** The canonical source of truth is the AST. This AST is persisted in the standard Git object database (`.git/objects`), where:
    *   Each AST node (or node property) maps to a Git **blob**.
    *   AST structure (parent-child relationships) maps to Git **trees**.
    *   Commits point to the root AST tree, snapshotting the code's structure.
    *(Critique: This implies many small objects, posing potential performance challenges for Git operations like `status`. Advanced features like `info/alternates` or MIDX might mitigate this for large-scale deduplication later).*
*   **View (Generated Code via FUSE):** A FUSE filesystem presents a conventional view of the source code. When a tool (editor, compiler) reads a file:
    *   The FUSE layer intercepts the read.
    *   It generates the source code text on-the-fly from the stored AST **Model**.
    *   Deterministic code generation (pretty-printing) ensures consistency.
    *(Critique: Is FUSE essential? Maybe not long-term (editor plugins?), but it provides maximum initial compatibility with existing tools).*
*   **Controller (Staging & Parsing):** When the user saves changes to the generated code **View** and runs `git add`:
    *   The change is detected (via FUSE write interception or file watching).
    *   Tree-sitter parses the change, ideally incrementally (`edit()`), updating the in-memory AST **Model**.
    *   `git commit` then serializes this updated AST Model into Git blob/tree objects.

**(Replacement vs. Extension):** While the ultimate vision might be a full Git replacement storing *only* ASTs, this POC and initial phases function as an *extension* or *view layer* on top of an existing Git repository, reading data via `libgit2`.

This architecture ensures the developer's experience (editing files and using Git commands) remains largely unchanged, but all operations are now AST-aware. We will now dive deeper into how source files are represented as an AST filesystem and how Git objects are created from it.

## Core Concepts & Challenges (Condensed)

*   **AST as Filesystem:** Representing AST nodes (functions, statements, expressions) as files/directories allows navigation and interaction using standard tools. Order might be maintained using prefixes (`1_`, `2_`) or dedicated metadata files.
*   **Code Generation:** Converting the AST back to human-readable, consistently formatted source code is crucial. Requires a pretty-printer per language, leveraging Tree-sitter grammars. Deterministic round-tripping (parse -> AST -> generate -> parse = same AST) is a key assumption.
*   **Semantic Diff/Merge:** Comparing ASTs directly allows ignoring formatting noise and intelligently merging structural changes (like refactorings) that conflict textually. Uses tree-diff algorithms.
*   **Performance:** Storing many small AST node objects in Git might impact performance (`git status`, checkout). Mitigation strategies include optimizing the CLI wrapper, using Git features like FSMonitor, packfile optimizations, and potentially coarser AST granularity.
*   **Node Identity:** Reliably identifying the "same" semantic node across commits (e.g., after moving a function) is challenging but important for accurate history/blame. Solutions might involve content hashing or embedded IDs.
*   **Tool Compatibility:** Achieved via FUSE and wrapping Git commands. Hooks or tools directly accessing `.git` might need adaptation or rely on generated source views.
*   **Non-Code Files:** Handled by falling back to standard Git behavior.

## Roadmap

### Phase 1: Basic AST Parsing & Representation (Current Focus)
**Goal:** Parse source code into ASTs and define a conceptual mapping to Git objects.
-   [ ] Integrate [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for one language (e.g., Rust).
-   [ ] Implement logic to read a file from a specified Git repository (using [`git2`](https://docs.rs/git2/)) and parse it using Tree-sitter.
-   [ ] Represent the parsed AST in memory.
-   [ ] Define a conceptual mapping from AST nodes/structure to Git blobs/trees (as described in Architecture Overview).
-   [ ] **Test:** Parse a known Rust file and verify the basic AST structure can be accessed programmatically.

### Phase 2: AST-based Git Object Storage
**Goal:** Implement the serialization of ASTs into Git's object store.
-   [ ] Define strategy for serializing the in-memory AST (from Phase 1) into Git blobs and trees based on the conceptual mapping.
-   [ ] Implement basic `commit` functionality: Take the current AST state and write corresponding Git objects, creating a commit.
-   [ ] **Compare:** Implement a way to compare the tree hash of a commit created via the AST method vs. a traditional commit of the same source code.

### Phase 3: Core AST API & Basic Code Generation
**Goal:** Create an internal API for AST manipulation and code viewing.
-   [ ] Design and implement an internal Rust API (e.g., a library module) to:
    -   Load an AST from a Git commit (using Phase 2 logic).
    -   Provide methods to query/navigate the AST.
    -   (Optional) Implement basic AST modification primitives (e.g., update node value - needed for later phases).
-   [ ] Implement basic code generation (pretty-printing) within the API: Given an AST (or subtree), generate the corresponding source code string.
-   [ ] **Test:** Directly test the API functions: load AST, generate code, verify output.

### Phase 4: CLI Wrapper Integration
**Goal:** Build a command-line interface leveraging the Core AST API.
-   [ ] Design basic CLI commands (e.g., `git-ast show <file>`, `git-ast commit`, `git-ast diff`).
-   [ ] Implement the CLI commands by calling the Core AST API (from Phase 3).
-   [ ] For `show`: Use the API's code generation.
-   [ ] For `commit`: Use Phase 2 logic (potentially exposed via API).
-   [ ] For `diff`: Initially, could show basic info or rely on Phase 6.
-   [ ] **Test:** Run CLI commands and verify they interact correctly with the underlying API and Git repository.

### Phase 5: FUSE Integration
**Goal:** Implement the FUSE frontend using the Core AST API.
-   [ ] Set up basic FUSE mount using [`fuser`](https://docs.rs/fuser/).
-   [ ] Implement `lookup`, `getattr`, `readdir` by querying the Core AST API (which loads data from Git).
-   [ ] Implement FUSE `read` using the API's code generation function.
-   [ ] (Later) Implement FUSE `write` by capturing changes and eventually calling AST modification methods in the API.
-   [ ] **Test:** Mount a repository, use standard filesystem commands (`ls`, `cat`) and verify they work correctly via the FUSE interface.

### Phase 6: Semantic Operations
**Goal:** Leverage the AST structure for smarter Git operations.
-   [ ] Implement AST-based `git diff` within the Core API (using tree diffing), expose via CLI/FUSE.
-   [ ] Implement AST-based `git merge` within the Core API (using 3-way AST merge), expose via CLI/FUSE.

### Future Enhancements
-   [ ] AST-based `
