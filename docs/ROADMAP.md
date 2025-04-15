# Project Roadmap

The project is divided into phases, focusing on building foundational capabilities first using the clean/smudge filter architecture.

## Phase 1: Basic AST Parsing & Clean/Smudge POC (Current Focus)

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

## Phase 2: Robust Filtering & AST Storage

**Goal:** Improve filter performance, error handling, and establish the canonical AST storage in Git.

- [ ] Optimize filter performance (e.g., using Git filter process protocol, caching parsers/printers).
- [ ] Implement robust error handling in filters (e.g., strategy for parse failures - fail commit vs. store error marker/fallback).
- [ ] Finalize the AST serialization format for space efficiency and stability.
- [ ] Develop utilities or tests to verify AST integrity and round-trip consistency (parse->store->retrieve->print->parse).
- [ ] Implement handling for non-code files (bypass filters).
  - _Test:_ Measure filter performance on larger files/commits. Test behavior with syntax errors. Verify repository interactions (`clone`, `fetch`, `push` with filtered content).

## Phase 3: AST-Aware Diff Driver

**Goal:** Provide developers with semantic diff views integrated into Git.

- [ ] Implement a `git diff` driver:
  - Takes two blob OIDs (representing serialized ASTs) as input.
  - Deserializes both ASTs.
  - Uses an AST diffing algorithm (e.g., leverage `difftastic` logic, `GumTree`, or a simpler structural comparison) to find differences.
  - Formats the diff output in a user-friendly way (e.g., unified diff format on the *generated code*, highlighting structural changes and ignoring formatting).
- [ ] Configure Git to use this driver for AST-managed files.
  - _Test:_ Verify `git diff` ignores formatting-only changes and clearly shows structural changes (additions, deletions, modifications). Test diff presentation for moved code blocks (may initially show as delete/add).

## Phase 4: Basic Semantic Merge Driver

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

## Phase 5: Multi-Language Support & Refinements

**Goal:** Extend support to more languages and refine core functionality.

- [ ] Add support for additional languages by integrating their Tree-sitter grammars and potentially language-specific pretty-printers/formatters.
- [ ] Refine AST diff/merge algorithms based on experience (e.g., improve move detection, handling of specific language constructs).
- [ ] Investigate and address node identity tracking for more accurate history analysis (`git blame` aware of AST).
- [ ] Develop strategies for improving compatibility with external tools (e.g., CI systems, code review platforms - might involve generating text diffs on demand or specific integrations).

## Phase 6: Advanced Features & Ecosystem Integration

**Goal:** Explore more advanced semantic operations and deeper integration.

- [ ] Implement more sophisticated semantic merge conflict resolution tools or UIs.
- [ ] Explore refactoring-aware history analysis.
- [ ] Investigate potential integrations with IDEs or language servers for live feedback or AST manipulation.
- [ ] Revisit performance and scalability based on wider usage. 
