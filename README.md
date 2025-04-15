# Git AST

Welcome to Git AST! This project explores a novel approach to version control by leveraging Abstract Syntax Trees (ASTs) – or more accurately, Concrete Syntax Trees (CSTs) as produced by tools like Tree-sitter – instead of traditional line-based diffs. Our goal is to make Git language-aware, leading to more meaningful history, easier merges, and enhanced code consistency. We welcome contributions and feedback from the community!

## Project Status

**Current Phase:** Proof of Concept (POC) refinement

This project explores replacing Git's traditional line-based versioning with a structure-aware approach using Concrete Syntax Trees (CSTs) derived from Abstract Syntax Trees (ASTs). The goal is to make version control language-aware, enabling semantic diffs/merges and consistent code formatting by treating the structured representation as the canonical source.

**Current Focus:** Refining the architecture to use Git's clean/smudge filters for seamless integration, leveraging Tree-sitter to parse source files from an existing Git repository into CSTs/ASTs. This foundational step is necessary before implementing robust storage mechanisms or semantic operations.

**Initial POC Goal & Success Criteria:**

- Successfully parse a sample source file (e.g., Rust) from a target Git repository using Tree-sitter within the `git-ast` tool.
- Represent the parsed structure (CST/AST including comments) in memory using Tree-sitter's data structures.
- Implement a basic Git clean filter that converts source code to a serialized AST/CST representation on staging/commit.
- Implement a basic Git smudge filter that converts the stored AST/CST back into formatted source code on checkout.
- Document the chosen serialization format and the strategy for mapping this structure to Git blob objects.

## Motivation

Traditional Git operates on text lines, leading to limitations:

- **No Semantic Understanding:** Formatting changes create noise in diffs; merges often conflict unnecessarily when code is moved or refactored.
- **Inconsistent Formatting:** History tracks textual changes, not structural intent, making consistent formatting across a team difficult.

An AST/CST-based approach aims to:

- **Enable Semantic Operations:** Diffs ignore formatting noise; merges can automatically resolve structural changes (like moving a function edited by someone else).
- **Ensure Consistency:** Store the canonical AST/CST; generate formatted code on demand using deterministic pretty-printing, potentially enforcing a single style.
- **Provide Fine-Grained History:** Track changes at the AST node level (function, statement, etc.), though this requires robust node identity tracking.

## Architecture Overview (Clean/Smudge Filter Approach)

The proposed system integrates with the existing developer workflow using Git's built-in clean and smudge filters, rather than a FUSE filesystem, for broad tool compatibility with minimal disruption:

- **Model (AST/CST & Git Objects):** The canonical source of truth is the structured representation (AST/CST, including comments) derived from the code. This structure is persisted within the standard Git object database (`.git/objects`):
  - The serialized AST/CST for an entire file is stored as a single Git **blob**. This avoids the performance pitfalls of storing potentially thousands of small objects per file.
  - Git commits point to the standard Git tree objects, which in turn reference these AST/CST blobs for tracked files.
    (Design Consideration: Storing serialized AST/CST might result in larger blob sizes compared to source text, potentially impacting repository size and clone/fetch times. Compression and Git's delta mechanisms should mitigate this, but it requires monitoring. We explicitly avoid mapping AST nodes to Git tree entries to prevent object store explosion).
- **View (Working Directory Code):** Developers interact with standard source code files in their working directory. These files are generated on-the-fly by the **smudge** filter when checking out branches or files:
  - The smudge filter reads the canonical AST/CST blob from Git's object store.
  - It generates the source code text using a deterministic code generator (pretty-printer), such as Prettier, Black, or a custom one, ensuring consistent formatting.
    (Design Consideration: This implies that the system enforces a specific code style. Custom formatting is typically lost, which is a trade-off for consistency and cleaner diffs. Preserving comments is crucial and must be handled by the parsing/printing process).
- **Controller (Staging & Parsing via Clean Filter):** When a developer modifies a file and runs `git add` or `git commit`:
  - The **clean** filter intercepts the file content being staged.
  - It uses Tree-sitter to parse the source code into an AST/CST.
  - This AST/CST is serialized into the canonical format (e.g., JSON, binary S-expression) which Git then stores as a blob object.
    (Design Consideration: Parsing happens on staging/commit. If the code fails to parse, the filter must decide whether to fail the operation (forcing valid syntax) or store a representation of the error/fallback to text. The former enforces quality but disrupts workflow; the latter adds complexity. Handling partial parses gracefully is a challenge).

**(Extension Strategy):** This approach functions as an _extension_ layered onto a standard Git repository via filters configured in `.gitattributes` and `git config`. Developers use standard Git commands (`git add`, `git commit`, `git checkout`, `git diff`). The filters handle the AST/CST conversion transparently. We leverage `libgit2` or standard Git commands for underlying repository interactions.

This architecture aims to keep the developer's core experience largely unchanged while making the underlying versioning structure-aware.

## Core Concepts & Challenges (Condensed)

- **AST/CST Representation:** Defining a robust and serializable format for the Tree-sitter CST that includes comments and potentially enough information for deterministic pretty-printing. Tree-sitter produces CSTs, which are closer to the source text and include formatting/comments, making them suitable.
- **Code Generation (Pretty-Printing):** Reliably converting the stored AST/CST back into human-readable, consistently formatted source code via the smudge filter. Using established formatters (Prettier, Black, rustfmt) where possible is preferred. Achieving deterministic round-tripping (parse -> AST -> generate -> parse = identical AST) is key, though enforcing a canonical format simplifies this by design. Preserving comments accurately is non-negotiable.
- **Clean/Smudge Filter Implementation:** Building efficient and robust filter processes. These need to handle different languages, manage parser dependencies, deal with parse errors gracefully, and perform quickly enough not to significantly slow down common Git operations (commit, checkout, diff). Using Git's filter process protocol can help performance by reusing parser instances.
- **Semantic Diff/Merge:** (Future Goal) Comparing stored ASTs directly to ignore formatting noise and intelligently handle structural changes. This requires tree-diffing algorithms (like GumTree) and careful design for presenting diffs and resolving merge conflicts structurally. Initial phases might rely on diffing the generated text or the serialized AST text, deferring true semantic diff.
- **Performance:** Parsing on commit (clean filter) and pretty-printing on checkout (smudge filter) adds overhead. While Tree-sitter is fast, this could be noticeable for large files or bulk operations. Filter performance optimization is critical. Repository size might also increase if serialized ASTs are larger than source text.
- **Node Identity:** (Crucial for Semantic Ops) Reliably tracking the "same" semantic code element across commits, even if moved or refactored. This is essential for accurate semantic diffs/merges and history analysis (blame). Potential solutions (content-hashing, UIDs, heuristics) are complex and deferred beyond the initial POC.
- **Tool Compatibility:** The clean/smudge approach ensures tools operating on the working directory (editors, linters, compilers) see standard source code. However, tools directly inspecting Git history or diffs (like `git log -p`, GitHub/GitLab diff views) will see the serialized AST blobs unless diff drivers are configured to use the smudge filter for display. Achieving seamless integration with platforms like GitHub requires custom diff drivers or platform support.
- **Handling Non-Code & Unparseable Files:** Files without a supported Tree-sitter parser (images, binaries) or files with syntax errors need strategies. Non-code files can bypass filters. Unparseable code might fail the clean filter or be stored as text/error representation.
- **Determinism & Fidelity:** Ensuring the entire process (parse -> serialize -> deserialize -> print) is deterministic. Accepting the trade-off of losing custom formatting in favor of a canonical style is a key design decision driven by this requirement.

## Examples

(Illustrative examples demonstrating the workflow, clean/smudge in action, and potential future semantic diffs/merges will be added here.)

## Related Projects

Several projects explore semantic version control, AST diffing, or alternative VCS approaches. Understanding them provides context for Git AST:

- **Plastic SCM SemanticMerge:** A commercial tool demonstrating successful AST-based merging for specific languages (C#, Java, etc.). It significantly reduces conflicts from refactoring but requires dedicated parsers and is proprietary. Shows the *value* of semantic merge.
- **Difftastic / Diffsitter:** Open-source structural diff tools using Tree-sitter. They highlight semantic changes and ignore formatting noise for many languages, improving code review. However, they are read-only (no merge) and can have performance issues. Show the *feasibility* of Tree-sitter based diffing.
- **GumTree:** An academic algorithm for computing fine-grained AST edit scripts (add, delete, update, move). Provides a theoretical foundation for semantic diff/merge but requires integration into practical tools.
- **Pijul:** A patch-based VCS with a different theoretical model aiming for better merge properties than Git, though not inherently AST-aware. Highlights that alternatives to line-based diffs exist.
- **FUSE/Virtual FS Ideas:** Concepts explored for presenting ASTs as filesystems. While offering potential transparency, they introduce significant complexity and performance concerns, which Git AST avoids by using clean/smudge filters.
- **Compiler APIs (e.g., Roslyn):** Some languages offer high-fidelity ASTs via their compiler APIs, which could be an alternative to Tree-sitter for specific languages if higher fidelity is needed.

Git AST aims to integrate AST-awareness directly into the Git workflow using standard mechanisms (filters) and leverage the broad language support of Tree-sitter, learning from the strengths and limitations of these related efforts.

## Potential Future Direction: MLIR Integration

An intriguing possibility for future development is the integration of the [MLIR (Multi-Level Intermediate Representation)](https://mlir.llvm.org/) framework. MLIR is a powerful compiler infrastructure designed for representing code at multiple levels of abstraction, defining custom operations (dialects), and performing complex transformations.

**How MLIR Could Apply to `git-ast`:**

Instead of directly mapping Tree-sitter ASTs to Git objects, MLIR could serve as the core intermediate representation:

- **Structured Representation:** Source code could be lowered into custom MLIR dialects representing language semantics (e.g., `rust.gitast`) and potentially even Git concepts.
- **Semantic Operations:** MLIR's infrastructure is built for analysis and transformation. Semantic diffing, merging, and refactoring operations could potentially be implemented as MLIR passes operating directly on the IR.
- **Node Identity:** MLIR's structured nature might offer more robust ways to track semantic node identity across commits.
- **Code Generation:** MLIR includes pretty-printing capabilities that could drive the code generation for the FUSE view.

Essentially, `git-ast` could become a system that "compiles" source code changes into MLIR transformations, which are then serialized and stored in Git (perhaps using MLIR's [bytecode format](https://mlir.llvm.org/docs/BytecodeFormat/)).

**Considerations:**

- **Novelty:** Applying MLIR in this manner is outside its typical compiler optimization and hardware targeting use cases.
- **Complexity:** Integrating MLIR would add significant complexity and a major dependency.

**Status:** This is currently an exploratory idea and **not** part of the core roadmap defined above. It represents a potential long-term evolution or alternative architecture worth investigating once the foundational pieces of `git-ast` are in place.

## Roadmap

The project is divided into phases, focusing on building foundational capabilities first using the clean/smudge filter architecture.

### Phase 1: Basic AST Parsing & Clean/Smudge POC (Current Focus)

**Goal:** Implement basic Git clean/smudge filters for one language to store/retrieve ASTs.

- [ ] Integrate [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for an initial language (e.g., Rust or Python).
- [ ] Implement a basic `clean` filter script/program:
  - Takes source code text as input.
  - Parses it into a Tree-sitter CST/AST (preserving comments).
  - Serializes the AST into a defined format (e.g., S-expression or CBOR).
  - Outputs the serialized data.
- [ ] Implement a basic `smudge` filter script/program:
  - Takes serialized AST data as input.
  - Deserializes it into an in-memory AST.
  - Generates formatted source code using a deterministic pretty-printer (e.g., integrate `rustfmt` or `black`).
  - Outputs the source code text.
- [ ] Configure `.gitattributes` and `git config` to use these filters for the chosen language.
- [ ] Document the chosen serialization format and initial pretty-printing strategy.
  - _Test:_ Verify that `git add`/`commit` stores serialized AST blobs and `git checkout` restores formatted source code for simple files. Check that comments are preserved. Test basic `git diff` (may show text diff of AST initially).

### Phase 2: Robust Filtering & AST Storage

**Goal:** Improve filter performance, error handling, and establish the canonical AST storage in Git.

- [ ] Optimize filter performance (e.g., using Git filter process protocol, caching parsers/printers).
- [ ] Implement robust error handling in filters (e.g., strategy for parse failures - fail commit vs. store error marker/fallback).
- [ ] Finalize the AST serialization format for space efficiency and stability.
- [ ] Develop utilities or tests to verify AST integrity and round-trip consistency (parse->store->retrieve->print->parse).
- [ ] Implement handling for non-code files (bypass filters).
  - _Test:_ Measure filter performance on larger files/commits. Test behavior with syntax errors. Verify repository interactions (`clone`, `fetch`, `push` with filtered content).

### Phase 3: AST-Aware Diff Driver

**Goal:** Provide developers with semantic diff views integrated into Git.

- [ ] Implement a `git diff` driver:
  - Takes two blob OIDs (representing serialized ASTs) as input.
  - Deserializes both ASTs.
  - Uses an AST diffing algorithm (e.g., leverage `difftastic` logic, `GumTree`, or a simpler structural comparison) to find differences.
  - Formats the diff output in a user-friendly way (e.g., unified diff format on the *generated code*, highlighting structural changes and ignoring formatting).
- [ ] Configure Git to use this driver for AST-managed files.
  - _Test:_ Verify `git diff` ignores formatting-only changes and clearly shows structural changes (additions, deletions, modifications). Test diff presentation for moved code blocks (may initially show as delete/add).

### Phase 4: Basic Semantic Merge Driver

**Goal:** Implement an initial AST-based merge strategy to reduce conflicts.

- [ ] Implement a `git merge` driver:
  - Takes three blob OIDs (base, local, remote serialized ASTs) as input.
  - Deserializes the three ASTs.
  - Performs a 3-way structural merge (e.g., using algorithms inspired by `SemanticMerge` or `GumTree`).
  - If merge succeeds without structural conflicts, serialize the merged AST and output it.
  - If conflicts occur, either:
    - Attempt to generate source code with conflict markers around the conflicting structures.
    - Or, fail the merge and provide information about the conflicting nodes (requiring manual resolution).
- [ ] Configure Git to use this driver for AST-managed files.
  - _Test:_ Verify that simple structural changes that would textually conflict (e.g., moving a function edited elsewhere) can be merged automatically. Test the conflict reporting mechanism.

### Phase 5: Multi-Language Support & Refinements

**Goal:** Extend support to more languages and refine core functionality.

- [ ] Add support for additional languages by integrating their Tree-sitter grammars and potentially language-specific pretty-printers/formatters.
- [ ] Refine AST diff/merge algorithms based on experience (e.g., improve move detection, handling of specific language constructs).
- [ ] Investigate and address node identity tracking for more accurate history analysis (`git blame` aware of AST).
- [ ] Develop strategies for improving compatibility with external tools (e.g., CI systems, code review platforms - might involve generating text diffs on demand or specific integrations).

### Phase 6: Advanced Features & Ecosystem Integration

**Goal:** Explore more advanced semantic operations and deeper integration.

- [ ] Implement more sophisticated semantic merge conflict resolution tools or UIs.
- [ ] Explore refactoring-aware history analysis.
- [ ] Investigate potential integrations with IDEs or language servers for live feedback or AST manipulation.
- [ ] Revisit performance and scalability based on wider usage.

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
