# Git AST

## Project Status

**Current Phase:** Proof of Concept (POC)

This project explores replacing Git's traditional line-based versioning with an Abstract Syntax Tree (AST)-based approach. The goal is to make version control language-aware, enabling semantic diffs/merges and consistent code formatting.

**Current Focus:** Building a basic FUSE-based virtual filesystem POC using Rust (`fuser`, `git2`). This initial phase validates mounting an existing Git repository and presenting its structure through a FUSE interface, laying the groundwork for future AST integration.

**POC Goal & Success Criteria:**
*   Successfully mount an existing Git repository via the `git-ast` tool.
*   List (`ls`) and retrieve basic attributes (`ls -l`) for top-level files/directories within the mount point, reflecting the actual repository state.

*(Self-Correction/Critique points regarding FUSE necessity and Replacement vs. Extension are noted below in the Architecture section).*

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

### Phase 1: FUSE Filesystem POC (Current)
**Goal:** Demonstrate a working FUSE mount reflecting the basic structure of an existing Git repository.
-   [x] Basic FUSE mount setup using [`fuser`](https://docs.rs/fuser/) and [`git2`](https://docs.rs/git2/) (Rust bindings for `libgit2`).
-   [ ] **Implement `lookup`:** Resolve top-level filenames. (Test: `ls /mount/point/some_file`).
-   [ ] **Implement `getattr`:** Provide basic file attributes. (Test: `ls -l /mount/point`).
-   [ ] **Implement `readdir`:** List root directory contents. (Test: `ls /mount/point`).
-   [ ] Read basic Git repository information (HEAD commit, root tree) via `libgit2`.

### Phase 2: Basic AST Representation & Read Operations
-   [ ] Integrate [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for one language (e.g., Rust).
-   [ ] Define AST-to-filesystem mapping (e.g., `file.rs/fn_foo/body/...`).
-   [ ] Implement FUSE `read` to generate source code from AST nodes.
-   [ ] Parse files from the Git repo into ASTs on demand.

### Phase 3: Write Operations & Git Integration
-   [ ] Implement FUSE `write` interception.
-   [ ] Use Tree-sitter `edit()` to incrementally update AST from writes.
-   [ ] Implement `git add` wrapper: Update AST based on changed files.
-   [ ] Implement `git commit` wrapper: Serialize AST to Git objects (blobs/trees).

### Phase 4: Semantic Operations
-   [ ] Implement AST-based `git diff` (using tree diffing).
-   [ ] Implement AST-based `git merge` (using 3-way AST merge).

### Future Enhancements
-   [ ] AST-based `git blame`.
-   [ ] Multi-language support.
-   [ ] Performance optimizations (caching, lazy loading, optimized status).
-   [ ] Handling comments, whitespace, preprocessor directives.
-   [ ] Stable AST node identification.
-   [ ] Enhanced CLI wrapper & editor integrations.
-   [ ] Containerized development environment.
-   [ ] **Advanced Git Object Store Integration:** Explore `info/alternates`, MIDX, `git replace` for deduplication/layering.
-   [ ] **Language-Agnostic AST Representation:** Investigate typed IR (e.g., Zod schemas) for cross-language features.

## Setup

### Prerequisites

*   **Rust Toolchain:** Install via [rustup.rs](https://rustup.rs/).
*   **[`libgit2`](https://libgit2.org/):** Required by the `git2` crate. Install via system package manager (e.g., `libgit2-dev` on Debian/Ubuntu, `libgit2` on macOS via Homebrew) or potentially let `cargo` build it (may require `cmake`, `pkg-config`, etc.).
*   **FUSE Implementation:**
    *   **macOS:** Install [macFUSE](https://osxfuse.github.io/) (`brew install macfuse`). Follow post-install instructions.
    *   **Linux:** Install FUSE development libraries (e.g., `libfuse-dev` on Debian/Ubuntu, `fuse-devel` on Fedora).

*(Note: Containerized development environments are planned to simplify setup).*

### Running the POC

1.  **Clone the repository:**
    ```bash
    git clone <your-repo-url>
    cd git-ast
    ```
2.  **Build the project:**
    ```bash
    cargo build
    ```
3.  **Run the FUSE filesystem:**
    Create a directory to use as a mount point:
    ```bash
    mkdir /tmp/git_mount
    ```
    Run the executable, providing the path to an *existing* Git repository and the mount point:
    ```bash
    # Replace /path/to/your/repo with an actual Git repository path
    ./target/debug/git-ast /path/to/your/repo /tmp/git_mount
    ```
    Explore `/tmp/git_mount` in another terminal. Press Ctrl+C to unmount.

**Warning:** This is early-stage POC software. Use a test repository. Functionality is currently limited to basic mounting and potentially listing root directory contents (pending Phase 1 completion).
