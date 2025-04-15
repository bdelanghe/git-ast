# Project Roadmap

The project is divided into phases, focusing on building foundational capabilities first using the clean/smudge filter architecture.

## Phase 1: Basic AST Parsing & Clean/Smudge POC (Current Focus - Effort: Medium)

**Goal:** Implement basic Git clean/smudge filters for one language (e.g., Rust) to store/retrieve ASTs, proving the core round-trip mechanism.

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

**Success Criteria / Testing:**
- **Round-Trip Fidelity:** On a curated test suite of diverse Rust files (covering common language features, comments, different formatting styles):
  - Verify `git add -> git checkout` results in formatted code that is semantically identical to the original (passes original tests/builds).
  - Verify 100% of comments are preserved and reasonably placed.
  - Verify `clean` filter output is deterministic (byte-for-byte identical for the same input).
- **Error Handling:** Committing a file with a syntax error reliably fails the `clean` filter with a clear error message.
- **Basic Workflow:** Standard `git add`, `commit`, `checkout`, `log`, `status` commands function correctly on a test repository using the filters.

**Go/No-Go Decision Point:**
- **GO:** Proceed to Phase 2 if round-trip fidelity criteria are met for >98% of the test suite files, comment preservation is reliable, basic workflow is functional, and error handling is predictable.
- **NO-GO / Re-evaluate:** If significant semantic drift occurs during round-trip, comment loss is frequent (>5% of test cases), or deterministic serialization proves problematic, **HALT**. Re-evaluate the serialization format, pretty-printing strategy, or CST representation fidelity before proceeding. Address identified failure modes.

## Phase 2: Robust Filtering & AST Storage (Effort: Medium)

**Goal:** Improve filter performance, error handling, and establish the canonical AST storage in Git.

- [ ] Optimize filter performance (e.g., using Git filter process protocol, caching parsers/printers).
- [ ] Implement robust error handling in filters (e.g., strategy for parse failures - fail commit vs. store error marker/fallback).
- [ ] Finalize the AST serialization format for space efficiency and stability (consider binary formats like CBOR if text-based proves too large).
- [ ] Develop utilities or tests to verify AST integrity and round-trip consistency (parse->store->retrieve->print->parse) at scale.
- [ ] Implement handling for non-code files (bypass filters based on file extension/type).

**Success Criteria / Testing:**
- **Performance:** Measure filter overhead on common operations (`add`, `commit`, `checkout`) on a moderately sized repository (e.g., 100-500 files, 10k-100k LOC). Overhead should ideally be < 10-20% compared to standard Git operations for incremental changes. Bulk operations should remain usable (e.g., full checkout within 2x standard time).
- **Robustness:** Filters handle parse errors according to the chosen strategy without crashing. Non-code files are correctly ignored. Round-trip consistency holds under stress testing (e.g., concurrent operations, large files).
- **Storage:** Monitor repository size growth compared to a text-only baseline. Ensure growth is acceptable (e.g., < 2-3x after Git compression for typical workloads).

**Go/No-Go Decision Point:**
- **GO:** Proceed to Phase 3 if performance overhead is acceptable, error handling is robust, non-code files are handled correctly, and storage growth is manageable.
- **NO-GO / Re-evaluate:** If filter performance makes common workflows feel sluggish (>50% slowdown), error handling leads to data loss or instability, or repository size explodes unacceptably, **HALT**. Revisit filter implementation (e.g., caching, parallelization), serialization format (e.g., switch to binary), or error handling strategy.

## Phase 3: AST-Aware Diff Driver (Effort: Large)

**Goal:** Provide developers with semantic diff views integrated into Git, ignoring formatting noise.

- [ ] Implement a `git diff` driver:
  - Takes two blob OIDs (representing serialized ASTs) as input.
  - Deserializes both ASTs.
  - Uses an AST diffing algorithm (e.g., leverage `difftastic` logic, `GumTree`, or a simpler structural comparison) to find differences.
  - Formats the diff output in a user-friendly way (e.g., unified diff format on the *generated code*, highlighting structural changes and ignoring formatting).
- [ ] Configure Git to use this driver for AST-managed files.

**Success Criteria / Testing:**
- **Diff Correctness:** Verify `git diff` output on a diverse set of changes:
  - Pure formatting changes produce an empty diff.
  - Simple structural changes (e.g., adding/removing statements) are accurately represented.
  - Moved code blocks are identified (even if initially shown as delete/add, the diff should be structurally sound).
- **Diff Readability:** Ensure the diff output is easily understandable by developers familiar with standard `git diff` (e.g., uses standard unified diff format markers).
- **Performance:** `git diff` performance should be comparable to text diffs for small changes and scale reasonably for larger changes (e.g., within 2-3x of text diff for moderate file changes).

**Go/No-Go Decision Point:**
- **GO:** Proceed to Phase 4 if the diff driver correctly ignores formatting, accurately represents structural changes for common cases, and performs acceptably.
- **NO-GO / Re-evaluate:** If diff output is frequently misleading, misses significant structural changes, fails to ignore formatting reliably, or is prohibitively slow, **HALT**. Re-evaluate the AST diffing algorithm, the diff presentation format, or performance optimizations. Consider simplifying the diff scope (e.g., only showing additions/deletions reliably, deferring perfect move detection).

## Phase 4: Basic Semantic Merge Driver (Effort: Large)

**Goal:** Implement an initial AST-based merge strategy to reduce conflicts caused by non-conflicting structural changes.

- [ ] Implement a `git merge` driver:
  - Takes three blob OIDs (base, local, remote serialized ASTs) as input.
  - Deserializes the three ASTs.
  - Performs a 3-way structural merge (e.g., using algorithms inspired by `SemanticMerge` or `GumTree`).
  - If merge succeeds without structural conflicts, serialize the merged AST and output it.
  - If conflicts occur, either:
    - Attempt to generate source code with conflict markers around the conflicting structures (goal: provide better context than text markers).
    - Or, fail the merge and provide information about the conflicting nodes (requiring manual resolution, possibly falling back to text merge). Define the fallback strategy clearly.
- [ ] Configure Git to use this driver for AST-managed files.

**Success Criteria / Testing:**
- **Conflict Reduction:** Demonstrate on a test suite of merge scenarios (e.g., function moved in one branch, edited in another; independent edits within the same function but different AST nodes) that the driver automatically resolves merges that would conflict textually. Quantify reduction (e.g., >30% reduction in conflicts for refactoring-heavy scenarios compared to text merge).
- **Correctness:** Verify that auto-merged results are semantically correct and pass tests.
- **Conflict Handling:** Ensure that when conflicts *are* reported, the output (markers or error report) is understandable and allows developers to resolve the conflict manually. The fallback mechanism should work reliably. Verify no data loss occurs compared to standard Git merge.

**Go/No-Go Decision Point:**
- **GO:** Proceed to Phase 5 if the merge driver demonstrably reduces common structural conflicts, produces correct auto-merges, and handles true conflicts safely and comprehensibly (even if via fallback).
- **NO-GO / Re-evaluate:** If the merge driver frequently produces incorrect merges, fails to reduce conflicts significantly, loses data, or handles conflicts poorly (e.g., confusing markers, crashes), **HALT**. Re-evaluate the merge algorithm, conflict representation, or significantly scope down the auto-merge capabilities (e.g., only merge non-overlapping AST changes automatically, fall back to text merge for everything else initially).

## Phase 5: Multi-Language Support & Refinements (Effort: Ongoing/Variable per Language)

**Goal:** Extend support to more languages and refine core functionality based on user feedback.

- [ ] Add support for additional high-priority languages (e.g., Python, JavaScript) by integrating their Tree-sitter grammars and appropriate pretty-printers/formatters. Document setup for each.
- [ ] Refine AST diff/merge algorithms based on experience (e.g., improve move detection, handling of specific language constructs identified in earlier phases).
- [ ] Investigate and address node identity tracking for more accurate history analysis (`git blame` aware of AST) - May become a dedicated phase if complex.
- [ ] Develop strategies for improving compatibility with external tools (e.g., CI systems, code review platforms - might involve generating text diffs on demand or specific integrations if diff driver isn't sufficient).

**Success Criteria / Testing:**
- **Language Support:** Successfully demonstrate Phases 1-4 capabilities for each newly added language on representative test projects.
- **Refinements:** Show measurable improvements in diff/merge quality or performance based on metrics gathered in previous phases or user feedback.
- **Compatibility:** Demonstrate working integration strategies for common CI workflows and provide guidance for code review platforms (e.g., using a local diff viewer).

**Go/No-Go Decision Point:** (Evaluated per language or major refinement)
- **GO:** Continue adding languages or refinements if the core system remains stable and benefits are realized for new languages/features.
- **NO-GO / Re-evaluate:** If adding a specific language proves exceptionally difficult (e.g., unreliable parsing/printing, fundamental mismatch with AST approach) or if a refinement introduces instability, pause that specific effort and potentially de-scope support for problematic languages/features.

## Phase 6: Advanced Features & Ecosystem Integration (Effort: Very Large / Exploratory)

**Goal:** Explore more advanced semantic operations and deeper integration based on project maturity and community needs.

- [ ] Implement more sophisticated semantic merge conflict resolution tools or UIs.
- [ ] Explore refactoring-aware history analysis (e.g., semantic `git blame`).
- [ ] Investigate potential integrations with IDEs or language servers for live feedback or AST manipulation.
- [ ] Revisit performance and scalability based on wider usage, potentially implementing advanced optimizations or storage strategies.
- [ ] Investigate advanced concepts like [Node Identity](../README.md#glossary) or alternative IRs (See [FUTURE_DIRECTIONS.md](./FUTURE_DIRECTIONS.md)).

**Success Criteria / Testing:**
- Driven by specific feature goals. Requires user validation and demonstrating clear value over existing solutions. Focus on stability and performance impact.

**Go/No-Go Decision Point:** (Evaluated per feature)
- Features proceed based on clear demand, technical feasibility, and positive impact on developer workflow without compromising core stability. Unproven or high-risk features may remain experimental or be deferred.
