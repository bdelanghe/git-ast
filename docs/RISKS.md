# Go/No-Go Risks and Key Assumptions

This document summarizes the critical risks and assumptions evaluated for the Git AST project.

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

These assumptions carry risks that need close monitoring and mitigation.

| Assumption                                                     | Risk   | Mitigation                                                                                                                                |
| :------------------------------------------------------------- | :----: | :---------------------------------------------------------------------------------------------------------------------------------------- |
| Semantic merge is reliable and improves over Git merge         |   🟡   | If merges silently break logic, trust erodes. SemanticMerge proves the concept but required per-language tuning.                         |
| Pretty-printing preserves comment placement and formatting enough |   🟡   | Poor formatting or dropped comments are immediate no-gos for dev trust.                                                                    |
| Git hosts (e.g., GitHub) are usable with AST blobs             |   🟡   | GitHub PR views will not show readable diffs by default. This degrades review UX unless mitigated.                                           |
| Tooling (e.g., CI) remains compatible                          |   🟡   | Lint, test, or format steps may operate on smudged code, but if they use repo blobs directly, behavior could differ.                    |

## ❌ Red Flags (No-Go If Fails)

Failure to meet these assumptions would likely make the project untenable.

| Assumption                                      | Go/No-Go Risk | Required Action                                                                                           |
| :---------------------------------------------- | :-----------: | :-------------------------------------------------------------------------------------------------------- |
| AST round-trip is not lossy (no semantic drift) |       ❌       | Any loss of code meaning (e.g., changing behavior or dropping required syntax) invalidates the approach. |
| Developers can trust the merge logic            |       ❌       | If merge drops edits, rewrites intent, or fails without clear UI, trust collapses.                           |
| Parsing doesn't block commit flow               |       ❌       | If `git add` or `git commit` fail regularly due to parse errors, devs will reject it.                     |
| No object bloat or Git performance regression   |       ❌       | If Git slows due to too many objects or large AST blobs, adoption dies.                                     |

## 📌 Summary

This project is still very viable, especially with the smart pivots:
*   Using clean/smudge filters instead of FUSE is a high-leverage tradeoff: you preserve ecosystem compatibility while minimizing performance complexity.
*   Starting with a read-only AST diff experience (rather than trying to win merge and blame from day one) gives you the ability to show early value.
*   Integrating tools like dprint, and examining GumTree, Jujutsu, and Datomic shows you're tracking best-in-class semantic, storage, and identity approaches.

The only dealbreaker class of risks is if AST round-trip changes code behavior or if the developer experience is unreliable (e.g., lost code, failed commits, unreadable diffs).
