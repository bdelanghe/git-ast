# Git AST

Welcome to Git AST! This project explores a novel approach to version control by leveraging Abstract Syntax Trees (ASTs) instead of traditional line-based diffs. Our goal is to make Git language-aware, leading to more meaningful history, easier merges, and enhanced code consistency. We welcome contributions and feedback from the community!

## Project Status

**Current Phase:** Proof of Concept (POC)

This project explores replacing Git's traditional line-based versioning with an Abstract Syntax Tree (AST)-based approach. The goal is to make version control language-aware, enabling semantic diffs/merges and consistent code formatting.

**Current Focus:** Integrating Tree-sitter to parse source files from an existing Git repository into ASTs. This foundational step is necessary before exploring storage mechanisms or interaction models like FUSE.

**Initial POC Goal & Success Criteria:**

- Successfully parse a sample source file (e.g., Rust) from a target Git repository using Tree-sitter within the `git-ast` tool.
- Represent the parsed AST in memory using Tree-sitter's data structures.
- (Stretch Goal): Document a conceptual strategy for mapping this AST structure to Git blob/tree objects.

## Motivation

Traditional Git operates on text lines, leading to limitations:

- **No Semantic Understanding:** Formatting changes create noise in diffs; merges often conflict unnecessarily when code is moved or refactored.
- **Inconsistent Formatting:** History tracks textual changes, not structural intent, making consistent formatting across a team difficult.

An AST-based approach aims to:

- **Enable Semantic Operations:** Diffs ignore formatting noise; merges can automatically resolve structural changes (like moving a function edited by someone else).
- **Ensure Consistency:** Store the canonical AST; generate formatted code on demand, potentially allowing personalized views.
- **Provide Fine-Grained History:** Track changes at the AST node level (function, statement, etc.).

## Architecture Overview (MVC & FUSE Approach)

The proposed system uses a Model-View-Controller (MVC) pattern, integrated with the existing developer workflow via a FUSE (Filesystem in Userspace) layer for broad tool compatibility:

- **Model (AST & Git Objects):** The canonical source of truth is the Abstract Syntax Tree (AST) representing the code's structure. This AST is persisted within the standard Git object database (`.git/objects`):
  - Individual AST nodes (or their properties) map to Git **blobs**.
  - The hierarchical structure of the AST (parent-child relationships) maps to Git **trees**.
  - Git commits point to the root AST tree, effectively snapshotting the code's structure at that point in time.
    (Design Consideration: Storing potentially many small objects for AST nodes might impact the performance of standard Git operations like `status`. We anticipate exploring Git features like packfile optimizations, `info/alternates`, or potentially MIDX indices as mitigation strategies later).
- **View (Generated Code via FUSE):** A FUSE filesystem presents a conventional directory and file view of the source code. When a tool (like an editor or compiler) reads a file:
  - The FUSE layer intercepts the read operation.
  - It generates the source code text on-the-fly from the stored AST **Model**.
  - Using deterministic code generation (pretty-printing) ensures consistency and allows for potentially personalized formatting views in the future.
    (Design Consideration: Is FUSE essential long-term? Perhaps not, as editor plugins or direct IDE integrations could bypass it. However, FUSE provides maximum _initial_ compatibility with the existing ecosystem of developer tools without requiring modifications to those tools).
- **Controller (Staging & Parsing):** When a developer saves changes to a file in the FUSE view and runs `git add`:
  - The change is detected (via FUSE write interception or file watching).
  - Tree-sitter parses the change, ideally incrementally (`edit()`), updating the in-memory AST **Model**.
  - `git commit` then serializes this updated AST Model into Git blob/tree objects.

**(Replacement vs. Extension Strategy):** While the ultimate vision might involve a complete Git replacement storing _only_ ASTs, this initial POC and subsequent phases function as an _extension_ or _view layer_ over a standard Git repository. We leverage `libgit2` to interact with the underlying repository data.

This architecture aims to keep the developer's core experience (editing files, using standard Git commands) largely unchanged, while making the underlying operations AST-aware.

## Core Concepts & Challenges (Condensed)

- **AST as Filesystem:** Representing the AST structure as a virtual filesystem (via FUSE) where nodes appear as files/directories. Maintaining order might require naming conventions (e.g., `1_`, `2_`) or metadata files.
- **Code Generation (Pretty-Printing):** Reliably converting the AST back into human-readable, consistently formatted source code. This requires robust pretty-printers, ideally leveraging Tree-sitter grammars for accuracy. Achieving deterministic round-tripping (parse -> AST -> generate -> parse = identical AST) is a key goal.
- **Semantic Diff/Merge:** Comparing ASTs directly to ignore formatting noise and intelligently handle structural changes (like refactorings) that might conflict textually. This relies on tree-diffing algorithms.
- **Performance:** As noted in Architecture, storing many small AST node objects could affect Git performance. Mitigation strategies (CLI optimization, Git features, potentially adjusting AST granularity) will be important.
- **Node Identity:** Reliably tracking the "same" semantic code element (e.g., a function) across commits, even if it's moved or modified, is crucial for accurate history and blame features. Potential solutions include content-hashing or embedding unique identifiers in the AST.
- **Tool Compatibility:** Primarily addressed via FUSE and wrapping Git commands. Tools or hooks directly accessing `.git` might require adaptation or need to rely on the generated source views presented by FUSE.
- **Non-Code Files:** Files without a supported Tree-sitter parser (e.g., images, text documents) will be handled by falling back to standard Git line-based behavior.

## Examples

(Illustrative examples demonstrating semantic diffs, merges, and the benefits of AST-based code generation will be added here as the project progresses.)

## Related Projects

_(A comparison to other relevant projects, such as AST-based version control systems (e.g., Plastic SCM's semantic merge), semantic diff/merge tools (e.g., `difftastic`, `gumtree`), and code formatters will be added here. This section will clarify the unique scope and approach of Git AST.)_

## Roadmap

The project is divided into phases, focusing on building foundational capabilities first. Each task aims to produce a testable deliverable.

### Phase 1: Basic AST Parsing & Representation (Current Focus)

**Goal:** Parse source code from a Git repository into an in-memory AST and define its mapping to Git objects.

- [ ] Integrate [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for an initial language (e.g., Rust).
- [ ] Implement functionality to read a specified file blob from a Git repository (using [`git2`](https://docs.rs/git2/)) and parse it into a Tree-sitter AST.
  - _Test:_ Verify programmatically that a known Rust file is parsed into an accessible AST structure.
- [ ] Document the conceptual mapping strategy from AST nodes/structure to Git blobs/trees (as described in Architecture Overview).

### Phase 2: AST-based Git Object Storage

**Goal:** Implement the serialization of the in-memory AST into Git's object store.

- [ ] Define and document the specific serialization format for representing the AST structure using Git blobs and trees.
- [ ] Implement basic `commit` functionality: Take the current in-memory AST state and write the corresponding Git blob/tree objects, creating a valid Git commit pointing to the root AST tree.
  - _Test:_ Verify that a commit created via this method produces the expected Git objects.
- [ ] Develop a method (e.g., a test utility or comparison script) to compare the resulting Git tree hash of an AST-based commit against a traditional commit of the identical source code (after generation).
  - _Test:_ Confirm that the generated tree hash matches the traditional one for simple cases.

### Phase 3: Core AST API & Basic Code Generation

**Goal:** Create an internal library (API) for loading, navigating, and generating code from the AST.

- [ ] Design and implement an internal Rust library (e.g., `libgitast`) providing an API to:
  - Load an AST representation from a Git commit OID (using Phase 2 logic).
  - Provide methods to query and navigate the loaded AST structure.
  - (Optional) Implement basic AST modification primitives (needed for later phases like `write`).
- [ ] Implement basic code generation (pretty-printing) within the library: Given an AST (or subtree), generate the corresponding source code string.
  - _Test:_ Write unit/integration tests for the Core AST library functions (load AST, navigate, generate code) and verify output correctness.

### Phase 4: CLI Wrapper Integration

**Goal:** Build a command-line interface (`git-ast`) utilizing the Core AST API.

- [ ] Define the specification for initial CLI commands (e.g., `git-ast show <commit>:<file>`, `git-ast commit`, `git-ast diff-ast`).
- [ ] Implement the `git-ast` CLI executable, calling the Core AST API (from Phase 3) to perform operations.
  - `show` should use the API's code generation.
  - `commit` should use the API wrapping Phase 2 logic.
  - `diff-ast` might initially show basic structural differences or rely on Phase 6.
  - _Test:_ Create end-to-end tests for the core CLI commands, verifying they interact correctly with the underlying API and a test Git repository.

### Phase 5: FUSE Integration

**Goal:** Implement the FUSE frontend to present the AST as a standard filesystem view.

- [ ] Integrate the [`fuser`](https://docs.rs/fuser/) library into the project.
- [ ] Implement FUSE read-only handlers (`lookup`, `getattr`, `readdir`, `read`) by querying the Core AST API (which loads data from Git objects) and using the API's code generation for `read`.
  - _Test:_ Write tests verifying standard filesystem operations (`ls`, `cat`) on a mounted repository view work as expected via the FUSE interface.
- [ ] (Later) Implement FUSE `write` handler by capturing file changes, parsing them to update the AST (potentially using Tree-sitter's `edit()` for incrementality), and using the Core AST API's modification primitives.

### Phase 6: Semantic Operations

**Goal:** Leverage the AST structure for smarter, language-aware Git operations.

- [ ] Implement AST-based semantic diff functionality within the Core API (using tree diffing algorithms), exposing it via the CLI (`git-ast diff-ast`).
  - _Test:_ Verify the semantic diff ignores formatting-only changes and correctly identifies structural modifications between two commits.
- [ ] Implement AST-based semantic merge functionality within the Core API (potentially using 3-way AST merge algorithms), exposing it via the CLI (e.g., `git-ast merge-ast`).
  - _Test:_ Verify that simple structural conflicts that would textually conflict can be merged automatically.

## Installation

As the project is currently in the Proof of Concept phase, installation primarily involves setting up the development environment.

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd git-ast
    ```
2.  **Install dependencies:** This project uses `mise` for environment management. Run the following command to install necessary tools and dependencies specified in the `.mise.toml` file:
    ```bash
    mise install
    ```

## Usage

The goal of `git-ast` is to eventually serve as a drop-in replacement for standard `git` commands. Once the core functionality is implemented (see Roadmap), usage should mirror the familiar Git workflow (e.g., `git-ast add`, `git-ast commit`, `git-ast diff`). Detailed command references will be added as features become available.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

We welcome contributions! Please feel free to open an issue on GitHub to discuss bugs, feature requests, or design ideas. If you'd like to contribute code, please see the issue tracker for areas where help is needed. (Link to contribution guidelines can be added later).
