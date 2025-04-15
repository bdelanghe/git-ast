# Git AST: A Language-Aware Git Extension

Welcome to Git AST! This project provides **language-aware extensions for Git**, leveraging Abstract Syntax Trees ([ASTs](#glossary)) – or more accurately, Concrete Syntax Trees ([CSTs](#glossary)) as produced by tools like [Tree-sitter](#glossary) – instead of traditional line-based diffs. Our goal is to enhance Git with semantic understanding, leading to more meaningful history, easier merges, and enhanced code consistency. We welcome contributions and feedback from the community!

## Value Proposition

Why use Git AST?

- **Cleaner Diffs:** Focus on meaningful code changes, ignore formatting noise.
- **Smarter Merges:** Reduce conflicts caused by code movement or non-competing structural edits.
- **Consistent Formatting:** Enforce a canonical code style automatically across your repository.

## Table of Contents

- [Project Status](#project-status)
- [Motivation](#motivation)
- [Architecture Overview (Clean/Smudge Filter Approach)](#architecture-overview-cleansmudge-filter-approach)
- [Core Concepts & Challenges (Condensed)](#core-concepts--challenges-condensed)
- [Examples](#examples)
- [Related Projects](#related-projects)
- [Potential Future Direction: MLIR Integration](#potential-future-direction-mlir-integration)
- [Roadmap](#roadmap)
- [Glossary](#glossary)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)
- [Contributing](#contributing)

## Project Status

**Current Phase:** Proof of Concept (POC) refinement

This project explores extending Git's traditional line-based versioning with a structure-aware approach using Concrete Syntax Trees ([CSTs](#glossary)) derived from Abstract Syntax Trees ([ASTs](#glossary)). The goal is to make version control language-aware, enabling [semantic diffs/merges](#glossary) and consistent code formatting by treating the structured representation as the canonical source.

**Current Focus:** Refining the architecture to use Git's [clean/smudge filters](#glossary) for seamless integration, leveraging [Tree-sitter](#glossary) to parse source files from an existing Git repository into CSTs/ASTs. This foundational step is necessary before implementing robust storage mechanisms or semantic operations.

**Initial POC Goal & Success Criteria:**

- Successfully parse a sample source file (e.g., Rust) from a target Git repository using [Tree-sitter](#glossary) within the `git-ast` tool.
- Represent the parsed structure (CST/AST including comments) in memory using Tree-sitter's data structures.
- Implement a basic Git [clean filter](#glossary) that converts source code to a serialized AST/CST representation on staging/commit.
- Implement a basic Git [smudge filter](#glossary) that converts the stored AST/CST back into formatted source code on checkout.
- Document the chosen serialization format and the strategy for mapping this structure to Git blob objects.

## Motivation

Traditional Git operates on text lines, leading to limitations:

- **No Semantic Understanding:** Formatting changes create noise in diffs; merges often conflict unnecessarily when code is moved or refactored.
- **Inconsistent Formatting:** History tracks textual changes, not structural intent, making consistent formatting across a team difficult.

An [AST/CST](#glossary)-based approach aims to:

- **Enable Semantic Operations:** Diffs ignore formatting noise; merges can automatically resolve structural changes (like moving a function edited by someone else).
- **Ensure Consistency:** Store the canonical AST/CST; generate formatted code on demand using deterministic [pretty-printing](#glossary), potentially enforcing a single style.
- **Provide Fine-Grained History:** Track changes at the AST node level (function, statement, etc.), though this requires robust [node identity](#glossary) tracking.

## Architecture Overview (Clean/Smudge Filter Approach)

The proposed system integrates with the existing developer workflow using Git's built-in [clean and smudge filters](#glossary), rather than a FUSE filesystem, for broad tool compatibility with minimal disruption:

- **Model (AST/CST & Git Objects):** The canonical source of truth is the structured representation ([AST/CST](#glossary), including comments) derived from the code. This structure is persisted within the standard Git object database (`.git/objects`):
  - The serialized AST/CST for an entire file is stored as a single Git **blob**. This avoids the performance pitfalls of storing potentially thousands of small objects per file.
  - Git commits point to the standard Git tree objects, which in turn reference these AST/CST blobs for tracked files.
    (Design Consideration: Storing serialized AST/CST might result in larger blob sizes compared to source text, potentially impacting repository size and clone/fetch times. Compression and Git's delta mechanisms should mitigate this, but it requires monitoring. We explicitly avoid mapping AST nodes to Git tree entries to prevent object store explosion).
- **View (Working Directory Code):** Developers interact with standard source code files in their working directory. These files are generated on-the-fly by the **[smudge filter](#glossary)** when checking out branches or files:
  - The smudge filter reads the canonical AST/CST blob from Git's object store.
  - It generates the source code text using a deterministic code generator ([pretty-printer](#glossary)), such as Prettier, Black, or a custom one, ensuring consistent formatting.
    (Design Consideration: This implies that the system enforces a specific code style. Custom formatting is typically lost, which is a trade-off for consistency and cleaner diffs. Preserving comments is crucial and must be handled by the parsing/printing process).
- **Controller (Staging & Parsing via Clean Filter):** When a developer modifies a file and runs `git add` or `git commit`:
  - The **[clean filter](#glossary)** intercepts the file content being staged.
  - It uses [Tree-sitter](#glossary) to parse the source code into an [AST/CST](#glossary).
  - This AST/CST is serialized into the canonical format (e.g., JSON, binary S-expression) which Git then stores as a blob object.
    (Design Consideration: Parsing happens on staging/commit. If the code fails to parse, the filter must decide whether to fail the operation (forcing valid syntax) or store a representation of the error/fallback to text. The former enforces quality but disrupts workflow; the latter adds complexity. Handling partial parses gracefully is a challenge).

**(Extension Strategy):** This approach functions as a **Git extension** layered onto a standard Git repository via filters configured in `.gitattributes` and `git config`. Developers use standard Git commands (`git add`, `git commit`, `git checkout`, `git diff`). The filters handle the AST/CST conversion transparently. We leverage `libgit2` or standard Git commands for underlying repository interactions.

**For a detailed explanation of how this clean/smudge filter pipeline works, including configuration, scripts, performance considerations, and platform integration challenges, see [Git Clean/Smudge Filters for ASTs](./docs/technical-architecture/clean-smudge-filters.md).**

This architecture aims to keep the developer's core experience largely unchanged while making the underlying versioning structure-aware.

_(For a deeper dive into the design rationale and evaluation, see [docs/project_evaluation_and_design_feedback.md](./docs/project_evaluation_and_design_feedback.md))._

## Core Concepts & Challenges (Condensed)

- **AST/CST Representation:** Defining a robust and serializable format for the [Tree-sitter](#glossary) [CST](#glossary) that includes comments and potentially enough information for deterministic [pretty-printing](#glossary). Tree-sitter produces CSTs, which are closer to the source text and include formatting/comments, making them suitable.
- **Code Generation (Pretty-Printing):** Reliably converting the stored [AST/CST](#glossary) back into human-readable, consistently formatted source code via the [smudge filter](#glossary). Using established formatters (Prettier, Black, rustfmt) where possible is preferred. Achieving deterministic round-tripping (parse -> AST -> generate -> parse = identical AST) is key, though enforcing a canonical format simplifies this by design. Preserving comments accurately is non-negotiable.
- **Clean/Smudge Filter Implementation:** Building efficient and robust filter processes. These need to handle different languages, manage parser dependencies, deal with parse errors gracefully, and perform quickly enough not to significantly slow down common Git operations (commit, checkout, diff). Using Git's filter process protocol can help performance by reusing parser instances.
- **[Semantic Diff/Merge](#glossary):** (Future Goal) Comparing stored [ASTs](#glossary) directly to ignore formatting noise and intelligently handle structural changes. This requires tree-diffing algorithms (like [GumTree](#glossary)) and careful design for presenting diffs and resolving merge conflicts structurally. Initial phases might rely on diffing the generated text or the serialized AST text, deferring true semantic diff.
- **Performance:** Parsing on commit ([clean filter](#glossary)) and pretty-printing on checkout ([smudge filter](#glossary)) adds overhead. While [Tree-sitter](#glossary) is fast, this could be noticeable for large files or bulk operations. Filter performance optimization is critical. Repository size might also increase if serialized ASTs are larger than source text.
- **[Node Identity](#glossary):** (Crucial for Semantic Ops) Reliably tracking the "same" semantic code element across commits, even if moved or refactored. This is essential for accurate semantic diffs/merges and history analysis (blame). Potential solutions (content-hashing, UIDs, heuristics) are complex and deferred beyond the initial POC.
- **Tool Compatibility:** The [clean/smudge](#glossary) approach ensures tools operating on the working directory (editors, linters, compilers) see standard source code. However, tools directly inspecting Git history or diffs (like `git log -p`, GitHub/GitLab diff views) will see the serialized AST blobs unless diff drivers are configured to use the smudge filter for display. Achieving seamless integration with platforms like GitHub requires custom diff drivers or platform support.
- **Handling Non-Code & Unparseable Files:** Files without a supported [Tree-sitter](#glossary) parser (images, binaries) or files with syntax errors need strategies. Non-code files can bypass filters. Unparseable code might fail the clean filter or be stored as text/error representation.
- **Determinism & Fidelity:** Ensuring the entire process (parse -> serialize -> deserialize -> print) is deterministic. Accepting the trade-off of losing custom formatting in favor of a canonical style is a key design decision driven by this requirement.

## Examples

(Illustrative examples demonstrating the workflow, [clean/smudge](#glossary) in action, and potential future [semantic diffs/merges](#glossary) will be added here.)

See the `examples/` directory for concrete illustrations of the data transformations involved.

## Related Projects

Several projects explore semantic version control, AST diffing, or alternative VCS approaches. Understanding them provides context for Git AST:

- **Plastic SCM SemanticMerge:** A commercial tool demonstrating successful [AST](#glossary)-based merging for specific languages (C#, Java, etc.). It significantly reduces conflicts from refactoring but requires dedicated parsers and is proprietary. Shows the _value_ of semantic merge.
- **Difftastic / Diffsitter:** Open-source structural diff tools using [Tree-sitter](#glossary). They highlight semantic changes and ignore formatting noise for many languages, improving code review. However, they are read-only (no merge) and can have performance issues. Show the _feasibility_ of Tree-sitter based diffing.
- **[GumTree](#glossary):** An academic algorithm for computing fine-grained [AST](#glossary) edit scripts (add, delete, update, move). Provides a theoretical foundation for [semantic diff/merge](#glossary) but requires integration into practical tools.
- **Pijul:** A patch-based VCS with a different theoretical model aiming for better merge properties than Git, though not inherently [AST](#glossary)-aware. Highlights that alternatives to line-based diffs exist.
- **FUSE/Virtual FS Ideas:** Concepts explored for presenting [ASTs](#glossary) as filesystems. While offering potential transparency, they introduce significant complexity and performance concerns, which Git AST avoids by using [clean/smudge filters](#glossary).
- **Compiler APIs (e.g., Roslyn):** Some languages offer high-fidelity [ASTs](#glossary) via their compiler APIs, which could be an alternative to [Tree-sitter](#glossary) for specific languages if higher fidelity is needed.

Git AST aims to integrate [AST](#glossary)-awareness directly into the Git workflow using standard mechanisms (filters) and leverage the broad language support of [Tree-sitter](#glossary), learning from the strengths and limitations of these related efforts.

## Potential Future Direction: MLIR Integration

An intriguing possibility for future development is the integration of the [MLIR (Multi-Level Intermediate Representation)](https://mlir.llvm.org/) framework. MLIR is a powerful compiler infrastructure designed for representing code at multiple levels of abstraction, defining custom operations (dialects), and performing complex transformations.

**How MLIR Could Apply to `git-ast`:**

Instead of directly mapping [Tree-sitter](#glossary) [ASTs](#glossary) to Git objects, MLIR could serve as the core intermediate representation:

- **Structured Representation:** Source code could be lowered into custom MLIR dialects representing language semantics (e.g., `rust.gitast`) and potentially even Git concepts.
- **Semantic Operations:** MLIR's infrastructure is built for analysis and transformation. Semantic diffing, merging, and refactoring operations could potentially be implemented as MLIR passes operating directly on the IR.
- **Node Identity:** MLIR's structured nature might offer more robust ways to track semantic [node identity](#glossary) across commits.
- **Code Generation:** MLIR includes [pretty-printing](#glossary) capabilities that could drive the code generation for the FUSE view.

Essentially, `git-ast` could become a system that "compiles" source code changes into MLIR transformations, which are then serialized and stored in Git (perhaps using MLIR's [bytecode format](https://mlir.llvm.org/docs/BytecodeFormat/)).

**Considerations:**

- **Novelty:** Applying MLIR in this manner is outside its typical compiler optimization and hardware targeting use cases.
- **Complexity:** Integrating MLIR would add significant complexity and a major dependency.

**Status:** This is currently an exploratory idea and **not** part of the core roadmap defined above. It represents a potential long-term evolution or alternative architecture worth investigating once the foundational pieces of `git-ast` are in place.

## Roadmap

The project is divided into phases, focusing on building foundational capabilities first using the [clean/smudge filter](#glossary) architecture. The current focus is Phase 1.

For detailed phase descriptions and tasks, please see the [**Full Roadmap Document**](./docs/ROADMAP.md).

## Glossary

- **AST (Abstract Syntax Tree):** A tree representation of the abstract syntactic structure of source code. Each node denotes a construct occurring in the source code. It typically omits details like comments, whitespace, and parentheses.
- **CST (Concrete Syntax Tree):** Also known as a parse tree, a CST represents the source code exactly, including all tokens like punctuation, whitespace, and comments. [Tree-sitter](#glossary) produces CSTs, which are beneficial for this project as they retain fidelity needed for accurate code generation.
- **Clean/Smudge Filters:** Git mechanisms that automatically transform file content when it's staged/committed (`clean`) or checked out (`smudge`). `git-ast` uses these to convert between source text (in the working directory) and serialized [AST/CST](#glossary) (stored in Git).
- **GumTree:** A well-known algorithm for computing differences between two [ASTs](#glossary), producing an edit script (add, delete, move, update operations) that represents the structural changes.
- **Node Identity:** The challenge of reliably identifying the "same" logical code element (like a specific function or variable) across different versions of an [AST](#glossary), even if it has been moved, renamed, or internally modified. Crucial for accurate [semantic diff/merge](#glossary).
- **Pretty-Printing:** The process of converting an [AST/CST](#glossary) back into formatted, human-readable source code text. Deterministic pretty-printing ensures the same AST always produces the same output text.
- **Semantic Diff/Merge:** Diffing or merging operations that operate on the code's structure ([AST/CST](#glossary)) rather than its textual representation. This allows ignoring formatting changes and intelligently handling structural modifications like code movement.
- **Tree-sitter:** A parser generator tool and incremental parsing library. It can build a [CST](#glossary) for a source file and efficiently update it as the source file is edited. It supports a wide range of programming languages via community-maintained grammars.

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

The goal of the `git-ast` tooling is to eventually integrate seamlessly with standard `git` commands via the [clean/smudge filters](#glossary) and custom diff/merge drivers. Usage should mirror the familiar Git workflow (e.g., `git add`, `git commit`, `git diff`). Configuration details and any necessary wrapper commands will be documented as features become available.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

We welcome contributions! Please feel free to open an issue on GitHub to discuss bugs, feature requests, or design ideas. If you'd like to contribute code, please see the issue tracker for areas where help is needed. (Link to contribution guidelines can be added later).
