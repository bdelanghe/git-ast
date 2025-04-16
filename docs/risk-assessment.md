# Go/No-Go Risks and Key Assumptions

This document summarizes the critical risks and assumptions evaluated for the Git AST project. **For a more detailed strategic analysis of risks and mitigations, see [STRATEGY_MEMO.md#strategic-risks-and-mitigations](./STRATEGY_MEMO.md#strategic-risks-and-mitigations).**

## ✅ Greenlights (Validated Assumptions)

These assumptions have passed research, design alignment, or successful prototype de-risking.

| Assumption                                                     | Status | Notes                                                                                                                                     |
| :------------------------------------------------------------- | :----: | :---------------------------------------------------------------------------------------------------------------------------------------- |
| Tree-sitter can parse real-world code for most common languages |   🟢   | Proven via difftastic and diffsitter. Partial parse resilience exists. Your approach focuses on CSTs, which helps preserve fidelity.      |
| Clean/smudge filter architecture is Git-native and deployable today |   🟢   | Filters are supported, portable, and developer-transparent. Avoids FUSE pitfalls.                                                        |
| Canonical formatting is an acceptable constraint for many teams |   🟢   | Assumes Prettier/Black/gofmt culture. Early users should buy into "no custom formatting" mindset.                                        |
| AST/CST serialization overhead is manageable                  |   🟢   | Git's delta compression should keep storage bloat reasonable if whole-file blobs are used. Git LFS or binary formats (e.g. CBOR) are fallback options. |
| AST diff is immediately useful even before merge is supported   |   🟢   | Improves review experience and PR clarity. Already proven useful in Difftastic context. Can integrate as a Git diff driver.          |

## ⚠️ Yellow Flags (Track Closely)

These assumptions carry risks that need close monitoring and mitigation, as detailed in the strategy memo.

| Assumption                                                     | Risk   | Mitigation Summary (See Memo for Details)                                                                                        |
| :------------------------------------------------------------- | :----: | :------------------------------------------------------------------------------------------------------------------------------- |
| Semantic merge is reliable and improves over Git merge         |   🟡   | Extensive testing, clear fallback strategy, potentially phased rollout (diff first). Start simple, verify correctness rigorously. |
| Pretty-printing preserves comment placement and formatting enough |   🟡   | Use CSTs, choose robust formatters (`dprint`), extensive round-trip testing. Failure here breaks trust immediately.             |
| Git hosts (e.g., GitHub) are usable with AST blobs             |   🟡   | Requires custom diff driver configuration or acceptance of non-ideal web UI diffs initially. Plan for platform integration work. |
| Tooling (e.g., CI) remains compatible                          |   🟡   | Ensure filters run correctly in CI; verify tools operating on checkout vs. blobs.                                                  |
| Performance overhead is acceptable                             |   🟡   | Optimize filters (process protocol, caching), monitor metrics, possibly scope down for large files. Target <1.5x slowdown.       |
| Developer adoption / cultural fit                              |   🟡   | Clear communication, opt-in pilots, address blame/workflow concerns, provide escape hatches.                                      |

## ❌ Red Flags (No-Go If Fails)

Failure to meet these assumptions would likely make the project untenable, as detailed in the strategy memo.

| Assumption                                      | Go/No-Go Risk | Required Action / Implication if Fails                                                                                             |
| :---------------------------------------------- | :-----------: | :--------------------------------------------------------------------------------------------------------------------------------------- |
| AST round-trip is not lossy (no semantic drift) |       ❌       | Invalidates the approach. Requires halting/re-evaluating core parsing/printing fidelity. Cannot compromise correctness.                 |
| Developers can trust the merge logic            |       ❌       | If merge silently drops edits or introduces errors, trust collapses. Requires robust testing and safe fallback/conflict marking.         |
| Parsing doesn't block commit flow unreasonably  |       ❌       | If `git add`/`commit` fail frequently or unpredictably (even with fencing), devs will reject it. Requires robust error handling/fencing. |
| No major object bloat or Git performance regression |       ❌       | If Git slows unacceptably due to size/object count, adoption dies. Requires monitoring and potentially binary serialization/optimisation. |

## 📌 Summary

This project is still very viable, especially with the smart pivots:
*   Using clean/smudge filters instead of FUSE is a high-leverage tradeoff: you preserve ecosystem compatibility while minimizing performance complexity.
*   Starting with a read-only AST diff experience (rather than trying to win merge and blame from day one) gives you the ability to show early value.
*   Integrating tools like dprint, and examining GumTree, Jujutsu, and Datomic shows you're tracking best-in-class semantic, storage, and identity approaches.

The only dealbreaker class of risks is if AST round-trip changes code behavior or if the developer experience is unreliable (e.g., lost code, failed commits, unreadable diffs).
