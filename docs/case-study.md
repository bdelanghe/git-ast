# Git AST Case Study: Lessons from Difftastic

## 1. Introduction & Synthesis

Thanks for sharing the full blog post from Wilfred Hughes about building Difftastic. It's a phenomenally rich source of both insight and hard-won lessons that are directly applicable to git-ast and your broader mission of structure-first version control.

Let me synthesize what this post reveals through the lens of your project — treating code as structured data, stored in Git, and later enabling querying, transformation, and compilation. I'll highlight what Difftastic teaches us technically and strategically, and how we can use that knowledge to de-risk and sharpen your roadmap.

⸻

## 2. Key Takeaways for git-ast

✅ **Why Difftastic Matters to Your Project**
	1.	**Proof of Possibility:** Difftastic shows that structural diffs can be done on real-world code at scale across multiple languages. That alone is validating. It was built with Tree-sitter, supports 40+ grammars, and works well enough for daily use.
	2.	**Parse Tree → S-expression Conversion:** Difftastic's trick of converting Tree-sitter CSTs into uniform s-expressions is a practical approach to normalize parse trees across languages. This reinforces that `git-ast` can do the same to support unified storage and diffs.
	3.	**Diffing as Matching:** The idea that diffing is "figuring out what hasn't changed" (vs what did) is profound. This mindset shift drives the use of graph search algorithms to optimize for readable, minimal diffs — a key concern for `git-ast`'s semantic diffs and merges.
	4.	**AST Diffs ≠ Minimal Textual Diffs:** Just because something is minimal structurally doesn't mean it's intuitively helpful to humans. Difftastic tweaked cost heuristics and even did post-processing for aesthetic reasons. `git-ast` needs similar UX-level tuning.

⸻

## 3. Lessons & Cautions from Difftastic

1.  **Performance Will Be a Bottleneck**

    > "The biggest performance bottleneck is vertex construction… the graph is O(L × R), where L and R are number of nodes in the left/right trees."

    *   🧠 **For git-ast:** Your system will do this all the time as part of `git diff` or `merge`. You must optimize this early. Consider:
        *   Lazy graph construction (as Wilfred does)
        *   Aggressive horizon trimming (ignore unchanged sections at the edges)
        *   Possibly switching to approximate match algorithms for big trees.

2.  **Nesting & Delimiters Are Hard**

    > "Graph vertices are really (L_pos, R_pos, parents_to_exit_together)… exponentially increases the graph size."

    *   🧠 **For git-ast:** Tree nesting is a worst-case scenario in diffs or merges. You will need to restrict depth, memoize subtree hashes, or use simplifications for large diffs. Consider:
        *   Hashing top-level nodes and only comparing diffs on changed hashes.
        *   Splitting files into semantic units (functions, classes) and diffing those separately.

3.  **Minimal Edit Scripts ≠ Human-Legible Diffs**

    > "Users would rather see the semantic intention (e.g., a new argument added) rather than minimal syntactic change."

    *   🧠 **For git-ast:** This is critical. Your diffs are not for machines — they're for reviewers. Consider heuristics like:
        *   Matching functions or expressions by structure and location, not just identity.
        *   "Prefer whole-node changes over partial" when unclear.
        *   Provide "diff styles": minimal vs readable, like `--diff-style readable`.

4.  **Fallbacks Are Required**

    > "If the graph is too big, Difftastic falls back to a line-oriented diff."

    *   🧠 **For git-ast:** You'll need similar fallbacks:
        *   For language grammar gaps (unsupported Tree-sitter grammars).
        *   For malformed syntax (use Tree-sitter's error-tolerant nodes).
        *   For performance cliffs (auto-revert to text mode for massive files).
        *   Maybe even a `git config` setting to disable AST mode for select files or repos.

⸻

## 4. Strategic Suggestions for git-ast

1.  **Adopt the S-expression Model**

    Normalize all Tree-sitter parse trees into a simplified, structural, S-expression-style representation.

    *   Flattened, but parent-aware.
    *   Normalizes across grammars.
    *   Enables generic diff/merge algorithms.

    This can become your internal interchange format.

2.  **Build a Heuristic Diff Strategy, Not a Perfect One**

    Difftastic uses Dijkstra's algorithm with adjusted edge costs, not "optimal" diffs.

    You should design your diff/merge engine around heuristics and cost models:
    *   Penalize unmatched delimiters, reordered parameters, etc.
    *   Tweak cost models to match real-world readability (optimize UX, not theory).
    *   Train on real diffs from your codebase to tune parameters.

3.  **Reframe Merges as Incremental Reconciliation**

    Instead of "merge trees A and B", think:
    1.  Split both trees into regions (e.g., functions).
    2.  Match nodes by name, position, or structure.
    3.  Compare matched nodes individually (smaller graphs).
    4.  Fallback to 3-way merge for unmatched/ambiguous nodes.

    This keeps the merge task scalable and human-decodable.

4.  **Dogfood Diff Quality Early**

    Make a `git-ast diff` command the first user-facing milestone. It will:
    *   Prove the AST pipeline works.
    *   Validate Tree-sitter grammars.
    *   Build buy-in from developers (e.g., "wow, clean diffs!").

    Dogfooding diffs ≫ merging early. This is Wilfred's own learning: diff quality is what people feel first.

5.  **Expect Structural Edge Cases**

    Wilfred mentions tricky patterns like:

    ```
    (foo (bar))    →   (foo (novel) (bar))
    ```

    You'll see this in JS, Python, etc., where order matters subtly. Have a test corpus of structural patterns:
    *   Param reordering
    *   Function inlining
    *   Nested conditionals
    *   Decorator changes
    *   Large string mutations

    Each of these will need different cost tweaks or even custom handling.

⸻

## 5. Final Thoughts

You're building something ambitious and long overdue. Wilfred Hughes' Difftastic is the clearest example of someone wrestling the same dragons — and succeeding, with real, daily usability.

The biggest lesson from Difftastic is this:

> *"Structural correctness is not the same as developer usefulness."*
> → You must constantly tune for real-world readability, not theoretical minimality.

Take this wisdom into `git-ast`. Build aggressively, but always in the loop with users. Prioritize diffs over merges. Tune over optimize. Fall back gracefully. And lean hard on Tree-sitter — it's your superpower.

If you'd like, I can help you:
*   Extract a sample structural corpus from your codebase to benchmark against.
*   Design a prototype of your own s-expression tree serializer.
*   Or even propose a `git-ast diff` CLI MVP based on Difftastic's model.

Let me know how you'd like to dive in next.

---
*This document synthesizes insights from Wilfred Hughes' blog post "[Difftastic, the Fantastic Diff](https://blog.wilfred.me.uk/difftastic/)" (Sept 6, 2022) as applied to the `git-ast` project.* 
