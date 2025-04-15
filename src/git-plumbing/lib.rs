"""//! # git-ast: Language-Aware Git Extensions
//!
//! This crate provides the core logic for `git-ast`, a tool designed to
//! extend Git with language-aware capabilities by storing Abstract Syntax Trees
//! (ASTs) or Concrete Syntax Trees (CSTs) directly in the Git object database,
//! rather than plain source code.
//!
//! ## Core Concepts
//!
//! The primary mechanism involves using Git's filter and driver system:
//!
//! 1.  **Clean/Smudge Filters:** Convert source code to a serialized AST/CST
//!     representation when staging (`git add`) and back to source code when
//!     checking out (`git checkout`). See the [`filters`] module documentation
//!     and the detailed guide in `docs/technical-architecture/clean-smudge-filters.md`.
//! 2.  **Custom Diff Driver:** Provides semantic diffs by comparing ASTs/CSTs
//!     instead of text lines, making `git diff` and `git log -p` output more
//!     meaningful. See the [`drivers`] module documentation.
//! 3.  **Custom Merge Driver:** Enables more intelligent 3-way merges by operating
//!     on the code's structure, potentially auto-resolving conflicts caused by
//!     refactoring or code movement. See the [`drivers`] module documentation.
//!
//! ## How it Hooks into Git
//!
//! `git-ast` integrates with Git via configuration in `.gitattributes` and
//! `.git/config` (or global Git config). Users interact with standard Git commands.
//!
//! -   **`.gitattributes`:** Defines *which* files use `git-ast`'s features.
//!     ```gitattributes
//!     *.rs filter=ast diff=ast merge=ast
//!     *.py filter=ast diff=ast merge=ast
//!     ```
//! -   **`.git/config`:** Defines *how* `git-ast` is invoked.
//!     ```ini
//!     [filter "ast"]
//!         # Use git-ast for clean/smudge
//!         process = git-ast filter-process
//!         required = true
//!
//!     [diff "ast"]
//!         # Use git-ast for diffing
//!         command = git-ast diff-driver
//!         # Optional: Cache text conversion results (if using textconv approach)
//!         # cachetextconv = true
//!
//!     [merge "ast"]
//!         # Use git-ast for merging
//!         name = AST-based merge driver
//!         driver = git-ast merge-driver %O %A %B %L %P
//!         recursive = binary # Often fallback to binary for internal merges
//!     ```
//!
//! The `git-ast` executable needs to handle the `filter-process`, `diff-driver`,
//! and `merge-driver` subcommands accordingly.
//!
//! ## Platform Integration (GitHub, GitLab, etc.) - IMPORTANT LIMITATION
//!
//! **Remote platforms WILL NOT run your local `git-ast` filters or drivers.**
//! They interact directly with the raw objects stored in the repository. Since
//! `git-ast` stores *serialized AST/CST data* as Git blobs (via the clean filter),
//! this means:
//!
//! -   **Web UI:** Files on GitHub/GitLab will display the raw serialized data, not source code.
//! -   **Diffs:** Diffs in Pull Requests will compare serialized data, likely unreadable.
//! -   **Merges:** Merging via the web UI might bypass the custom merge driver.
//!
//! ### Workaround: Mirrored Repository
//!
//! The recommended approach for collaboration using platforms is to maintain two repositories:
//!
//! 1.  **Primary Repo (`*-ast`):** Stores ASTs, uses `git-ast` filters/drivers. Development happens here locally.
//! 2.  **Mirror Repo (`*-source`):** Stores standard source code. Used for PRs, code browsing on platforms.
//!
//! A CI/CD pipeline on the primary repo automatically checks out code (triggering the `smudge`
//! filter to generate source) and pushes the resulting source code to the mirror repo.
//! See the Mermaid diagram in `docs/technical-architecture/clean-smudge-filters.md`.
//!
//! ## Modules
//!
//! -   [`config`]: Handles parsing and applying configuration from Git.
//! -   [`drivers`]: Implements the custom diff and merge driver logic.
//! -   [`git_plumbing`]: (Placeholder) Logic for git-plumbing operations.
//! -   [`parsing`]: (Placeholder) Logic for parsing source code into AST/CSTs (e.g., using Tree-sitter).
//! -   [`serialization`]: (Placeholder) Logic for serializing/deserializing AST/CSTs.
//! -   [`pretty_printing`]: (Placeholder) Logic for generating source code from AST/CSTs.
//! -   [`commands`]: (Placeholder) CLI command handling (`filter-process`, `diff-driver`, `merge-driver`).

// Define module structure
pub mod config;
pub mod drivers;
pub mod git_plumbing;
// pub mod filters; // Removed as it's inside git_plumbing
// pub mod parsing;
// pub mod serialization;
// pub mod pretty_printing;
// pub mod commands;

/// Placeholder for shared error type
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Config(String),
    Parsing(String),
    Serialization(String),
    Generation(String),
    Driver(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

// Example of a function potentially called by a command handler
// pub fn run_filter_process() -> Result<(), Error> {
//     // Implementation using filters::run_long_running_filter...
//     Ok(())
// }
"" 
