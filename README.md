# Git AST

## Project Status

**Current Phase:** Proof of Concept (POC)

The project is currently in the early stages, focusing on building a basic Proof of Concept for the FUSE-based virtual filesystem component using Rust and the `fuser` library. The initial goal is to mount a Git repository and expose a simplified view of its structure.

## Roadmap

This outlines the planned development phases for Git AST.

### Phase 1: FUSE Filesystem POC (Current)

-   [x] Basic FUSE mount setup using `fuser` and `libgit2`.
-   [ ] Implement basic `lookup`, `getattr`, `readdir` for root directory.
-   [ ] Read basic Git repository information (e.g., HEAD commit).
-   [ ] Represent top-level repository files/directories in the FUSE mount.

### Phase 2: Basic AST Representation & Read Operations

-   [ ] Integrate Tree-sitter for a specific language (e.g., Rust or C).
-   [ ] Parse files from the Git repository into ASTs upon access.
-   [ ] Define and implement a simple AST-to-filesystem mapping (similar to the `math.c` example).
    -   Files representing AST nodes (functions, structs, etc.).
    -   Files representing node properties (name, type, content).
-   [ ] Implement FUSE `read` operation to generate source code from AST fragments.
-   [ ] Handle basic navigation of the AST structure via the filesystem.

### Phase 3: Write Operations & Git Integration

-   [ ] Implement FUSE `write` operation.
-   [ ] Use Tree-sitter's incremental parsing (`edit()`) to update the AST based on file writes.
-   [ ] Define strategy for mapping AST changes back to Git objects (AST node blobs, tree objects).
-   [ ] Implement basic `git add` / `git commit` wrapper functionality:
    -   Serialize the current AST state into Git tree/blob objects.
    -   Create Git commits representing the AST snapshot.

### Phase 4: Semantic Operations

-   [ ] Implement AST-based `git diff`.
    -   Perform tree diffs between ASTs.
    -   Generate human-readable diffs (unified format) highlighting semantic changes.
-   [ ] Implement AST-based `git merge`.
    -   Perform three-way AST merges.
    -   Handle simple non-conflicting changes automatically.
    -   Develop a strategy for representing/resolving AST conflicts.

### Future Enhancements

-   [ ] Implement AST-based `git blame`.
-   [ ] Multi-language support via Tree-sitter.
-   [ ] Performance optimizations (caching, lazy loading).
-   [ ] Handling of comments, whitespace, and preprocessor directives.
-   [ ] Stable AST node identification across changes.
-   [ ] Enhanced CLI wrapper for more Git commands.
-   [ ] Potential editor integrations (e.g., VS Code extension).
-   [ ] Containerized development environment (Docker/Dagger).

AST-Based Git System: Architectural Overview and Feasibility Analysis

Motivation and Goals

Traditional Git treats source files as plain text, which means versioning and merging occur at the line level without understanding code structure ￼. This leads to spurious diffs (e.g. purely formatting changes) and unnecessary merge conflicts (e.g. two developers rearranging or refactoring code) because Git lacks semantic awareness ￼. An AST-backed Git system aims to make version control language-aware by tracking changes to the code's abstract syntax tree (AST) rather than raw text. Key goals include:
	•	Semantic diffs and merges: Only actual code logic changes show up in diffs, ignoring cosmetic differences like spacing or brace placement ￼. Merges become smarter – for example, one developer moving a function and another editing its body would no longer conflict, since the move and edit apply to the same AST node and can be merged automatically ￼.
	•	Consistent formatting and view flexibility: Because code is stored as an AST, formatting can be regenerated consistently. Developers could even use personalized formatting in their editor without affecting the stored history ￼ ￼. Styling debates vanish, as the canonical form is determined by code generation.
	•	Fine-grained blame and history: Version history can be tracked at the AST node level. Each function, class, or even statement could carry metadata about the commit that last modified it, enabling a more precise git blame that pinpoints who changed a specific syntax node, not just a line in a file.
	•	Integration with existing tools: Despite changing the underlying model, it's crucial that the system works with existing developer workflows. Editors like VS Code and other tools invoke standard Git commands and expect textual diffs and file content. The AST-based system must present a compatible interface (command-line and filesystem) so these tools continue to work normally, satisfying the goal of seamless adoption.

In summary, the goal is to replace the Git CLI's text-based versioning with an AST-aware layer that improves diff quality, merge automation, and consistency, while appearing as normal Git to developers. Next, we explore the architecture to achieve this.

System Architecture Overview

The proposed system introduces an intermediate AST representation between the developer's working code and Git's storage. The core components are:
	•	Tree-sitter Parser & AST – Tree-sitter is used to parse source files into concrete syntax trees (ASTs) and incrementally update them on edits ￼. It provides robust parsing for many languages and an API (including .edit()) to apply text edits to an existing tree efficiently.
	•	AST Virtual Filesystem (FUSE) – A FUSE-based filesystem presents the AST as a directory tree in the developer's workspace. Each source file is mounted as a directory containing subdirectories/files for its syntax nodes. This virtual filesystem lets developers and tools interact with AST nodes as if they were files. The FUSE layer translates normal file reads/writes into AST queries or updates.
	•	Git Storage (Blobs and Trees) – Instead of storing raw source files as blobs, the repository's .git database stores each AST node as a blob object and directories of nodes as tree objects. Commits then capture snapshots of the AST structure. Git's object model (blobs/trees) is thus used to persist the AST structure of the code rather than flat text files.
	•	CLI Wrapper – A drop-in replacement for the git command (or a wrapper script) intercepts common Git operations (add, commit, diff, merge, blame, etc.) and redirects them to operate on the AST layer. It ensures that from the user's perspective, they are still using Git commands, but under the hood these commands work with the AST representation.

Below is a high-level flow of how these components interact in the developer workflow:
	•	The developer opens a source file in VS Code (or another editor). The file content is provided by the AST FUSE filesystem, which on read generates the code text from the stored AST (more on code generation below). The developer sees normal code and makes edits.
	•	On save, the editor writes the updated file. The FUSE layer intercepts this write and uses Tree-sitter's incremental parsing to apply the edit to the AST instead of treating it as plain text ￼. The AST is now updated in memory.
	•	The developer runs git diff or checks the VS Code source control view. The Git CLI wrapper intercepts this and computes a diff of AST changes. It then formats the diff in a familiar unified diff format for display, ignoring cosmetic changes. For example, if only indentation changed, the AST diff would be empty (no semantic change), so git diff would show no changes, whereas line-based diff would show modifications. This is possible because the diff is based on AST node changes (e.g. "node X modified" or "moved"), which the tool can translate back into a textual diff of generated code that highlights only real changes ￼.
	•	The developer runs git add and git commit. The wrapper assembles the current AST from the FUSE (or uses the in-memory Tree-sitter AST) and serializes it into Git objects. Each node corresponds to a blob object (containing e.g. the source text of that syntax node or a token sequence), and each directory of nodes corresponds to a tree object. It then writes a commit object referring to the root AST tree. In effect, the commit is recording the AST structure at that point in time. (For compatibility, the commit message or an optional pipeline might embed a copy of the generated source text, but the primary content tracked is the AST.)
	•	When the developer pushes to a remote or another developer pulls, they exchange the AST-based commits. On checkout, the system reconstructs the working directory via the AST filesystem. Developers can also generate the full source code if needed (e.g. for builds) using the code generator.
	•	git merge is intercepted to perform merges on the AST. If two branches changed different parts of the AST, the merge can often be auto-resolved because they appear as changes to different nodes (different files in the AST tree). If the same AST node was modified in two branches, a conflict is reported at that node. The conflict can be presented by generating the conflicting source snippet or via a structured merge tool. In many cases, AST merges will reduce conflicts – for example, renaming a variable and editing its usage in parallel would not conflict, since the rename changes an identifier node and the usage edit changes a different node, whereas line-based merge might conflict if they were on the same line.

This architecture ensures the developer's experience (editing files and using Git commands) remains largely unchanged, but all operations are now AST-aware. We will now dive deeper into how source files are represented as an AST filesystem and how Git objects are created from it.

AST as a Virtual File Tree (Tree-sitter + FUSE)

Representing AST nodes as files: In the AST virtual filesystem, each source file appears as a directory, and its contents are broken down into subdirectories or files representing the AST nodes of that file. For example, consider a simple C file with two functions:

int add(int a, int b) {
    return a + b;
}
int main() {
    int result = add(2, 3);
    return result;
}

In the AST filesystem (using a possible scheme), this file math.c might be represented as follows:

math.c/                      (directory for the file's AST)
 ├─ fn_add/                  (function definition node for `add`)
 │   ├─ return_type.txt      ("int")
 │   ├─ name.txt             ("add")
 │   ├─ params/
 │   │    ├─ param1.txt      ("int a")
 │   │    └─ param2.txt      ("int b")
 │   └─ body/
 │        └─ 1_return/
 │             └─ op_add/
 │                  ├─ left.txt       ("a")
 │                  ├─ operator.txt   ("+")
 │                  └─ right.txt      ("b")
 └─ fn_main/                (function definition node for `main`)
      ├─ return_type.txt    ("int")
      ├─ name.txt           ("main")
      ├─ params/            (empty, no parameters)
      └─ body/
           ├─ 1_decl/
           │    ├─ type.txt         ("int")
           │    ├─ name.txt         ("result")
           │    └─ init_call/
           │         ├─ function.txt  ("add")
           │         ├─ arg1.txt      ("2")
           │         └─ arg2.txt      ("3")
           └─ 2_return/
                └─ value.txt       ("result")

Each directory and file here corresponds to a syntax element. For instance, fn_add is a function node with child nodes for its return type, name, parameter list, and body. Within the body, 1_return represents the first statement (a return statement), which has a child op_add for the binary addition expression (a + b), which in turn has children for the left operand, operator, and right operand. We prepend indices like 1_, 2_ to preserve the order of statements in the AST (this ensures that when listing the directory, the original order of code is retained).

FUSE implementation: Using FUSE (Filesystem in Userspace), we implement a virtual filesystem to present this structure. FUSE allows a user-space process to supply file/directory contents dynamically ￼. In our case, the user-space process is connected to the Tree-sitter parser:
	•	When an editor or tool reads a file like math.c, the FUSE layer on-the-fly assembles the file's text from the AST nodes (essentially the inverse of the above structure). It would concatenate return_type, name, params, etc. into a valid function definition, regenerate the braces, punctuation, and formatting as needed. This code generation step uses either a pretty-printer or a Tree-sitter-based code exporter. We assume deterministic codegen, so the same AST always produces the same formatted source (e.g. a canonical style).
	•	When a write occurs (saving changes), FUSE intercepts the new text and calls Tree-sitter to parse it (or use incremental edit). Tree-sitter's incremental parsing (ts_tree_edit and reparse) updates the AST efficiently in response to text edits ￼. Essentially, the FUSE layer plays the role of a live AST synchronizer: it keeps the AST in memory consistent with the latest file contents.

Deterministic round-tripping: A fundamental assumption is that parsing and pretty-printing are inverse operations, at least for the code structures we care about. Tree-sitter produces a concrete syntax tree including all tokens (it can capture comments and whitespace as syntax trivia or tokens). For any given AST, the code generator will output source code (in a standardized style). If that code is fed back into the parser, we assume it yields the same AST. This determinism is critical: it means the AST is a canonical form of the code. (If formatting or insignificant differences could produce different AST layouts, that would break this model. Tools like Prettier already demonstrate that code can be reformatted consistently, so we rely on similar logic for codegen.)

Comments and non-code tokens: One open challenge is handling comments, whitespace, and preprocessor directives. These do not affect AST structure but need to be preserved in a round-trip. We can treat comments as special AST nodes (e.g. attached to the nearest AST node or as children of a comment syntax node). Whitespace and formatting is generally regenerated, but significant whitespace (like Python indentation or alignment in micro-managed code) needs careful handling – likely by enforcing a standard format or including indentation as part of the structural representation. For the prototype, we assume languages where formatting is either irrelevant (C-style syntax) or we have tools to normalize it.

By mounting the AST as a filesystem, we achieve tool compatibility. To VS Code or any other program, the project still looks like a set of directories and files. A developer can open math.c and see actual C code. They can navigate the project structure – though interestingly, they might also peek into the math.c directory to see the AST breakdown if they choose (this could be useful for advanced users, but normally they wouldn't). The key point is that standard tools operate on this virtual filesystem backed by the AST, unaware of the magic beneath.

Mapping AST to Git Objects

With the AST represented as a hierarchical filesystem, storing it in Git becomes analogous to storing a normal directory tree in Git, just at a finer granularity. Git's object model has blobs (file content) and trees (directories listing blobs/trees). We leverage this directly:
	•	Each AST node file (leaf in the virtual FS) becomes a Git blob object. For example, name.txt containing "add" or operator.txt containing "+" would be stored as small blob objects. Even multi-line constructs (if we chose to put entire function bodies as one blob, for instance) would still be blobs.
	•	Each AST node directory becomes a Git tree object. For example, the directory fn_add/ is represented as a tree object containing entries for return_type.txt, name.txt, params (a subtree), and body (a subtree). The root directory of the project (or of each source file) is also a tree.
	•	A Git commit then points to the root tree of the project (which includes as subtrees each source file's AST tree, among other files).

If we commit the example above, the Git object database would contain something like:
	•	A tree object for math.c (listing fn_add and fn_main as subtrees).
	•	A tree for fn_add (listing blob name.txt, blob return_type.txt, tree params, tree body).
	•	Blobs for fn_add/name.txt ("add"), fn_add/return_type.txt ("int"), etc.
	•	Similar objects for fn_main and its children.
	•	Finally, a commit object pointing to the math.c tree (and any others).

For instance, the Git tree for fn_add might look like this when inspected (each line is an entry with mode, type, SHA, and name):

040000 tree bdd34c9...    body
100644 blob d28d40b...    name.txt
040000 tree a93b2d0...    params
100644 blob 4e0b2da...    return_type.txt

This shows that under fn_add/, two blobs (name.txt and return_type.txt) and two subtrees (body/ and params/) are tracked. Similarly, the body tree would contain an entry for 1_return subtree, and so on, down to individual token blobs like "+" in operator.txt.

It's important to note that Git has no issue storing deeply nested trees or many small blobs – it will delta-compress objects as needed. In fact, by breaking a large file into many small pieces, we might enable Git's delta compression to work more effectively in some cases (since unchanged pieces remain identical and compressible across versions). However, we are also vastly increasing the number of tracked files. This raises potential performance issues (discussed later) when we have, say, tens of thousands of AST nodes as separate files. Git commands like status may slow down because they check many files ￼. We will address performance trade-offs in a dedicated section.

Unique identifiers for nodes: One challenge is ensuring that a given AST node consistently maps to a path/name in the virtual filesystem, to maintain continuity across commits. In our example, we used ordinal prefixes (1_return, 2_return) to enforce statement order. If one commit adds a new statement in the middle, subsequent statements get renumbered, appearing as many renames/changes in Git. We might consider alternative naming schemes: e.g., a stable node ID that persists even if siblings are added or removed. This could be an internal GUID or a hash of the node's content. However, maintaining stable IDs is complex – it may require embedding IDs into the source as annotations or a separate mapping, which is an area of ongoing design. For now, we assume a simple scheme where structural edits can cause renaming of node files. Git will treat that as delete+add (or rename detection might pair them), which is acceptable albeit not ideal. The assumption is that minor edits (not involving massive reorderings) will result in localized changes in the AST file tree.

Staging and committing: When the developer runs git add on a source file (or on the whole project), the tool will traverse the AST directory and mark all corresponding blobs as staged (similar to how git add adds all file content to the index). In implementation, our wrapper might bypass the index and directly form tree objects from the AST, because constructing so many index entries could be slow. It could use Git's lower-level commands (like git mktree) to create tree objects from a directory listing of the AST in memory. Then git commit would use git commit-tree or similar plumbing to create the commit object.

The end result is that the Git history is a history of AST snapshots. Each commit knows the exact structured state of every file at that revision. Traditional Git commands can still operate on this repository – for example, git checkout with our system would need to populate the FUSE with the AST from that commit (or generate source files if checking out to a plain filesystem). Tools like git log remain unchanged (they show commit messages). git grep would by default search the serialized AST files; we could override it to search code text by generating on the fly, but that might be unnecessary if we keep a working tree of generated code for grepping.

Example Git objects (illustration): To ground this with a simple illustration, imagine a commit that adds the add and main functions above. In a traditional Git, that commit would have one tree with a blob for math.c. In our AST-Git, the commit's tree might look like:

040000 tree <sha1>    math.c

And math.c tree (as shown earlier) references fn_add and fn_main trees. If later we modify the add function's return expression (say return a + b; to return a - b;), only the blob for operator.txt under fn_add/1_return/op_add would change from "+" to "-". In the new commit, Git would reuse all other blobs/trees from the prior commit (since they are unchanged), and just have a new blob for the operator and new tree objects up the path (op_add tree changes to reference new operator blob, 1_return tree changes to reference new op_add, etc., up to fn_add). This is analogous to how a small edit in a large file normally makes Git store a new blob for the whole file; here it stores a new tiny blob and a few tree objects. The storage efficiency might actually improve in some cases because unchanged parts of a file remain as existing objects ￼ ￼. But the trade-off is the sheer number of objects and the overhead in traversing them.

Git Operations on the AST Repository

We now examine how key Git operations are redefined or implemented in this AST-based repository, and how we ensure compatibility with existing tools.

Diffing at the AST Level

A core benefit of this system is improved diffs. The git diff command is implemented by comparing ASTs of two commits (or a commit vs. the working AST). Instead of line-by-line longest common subsequence, we perform a tree diff:
	•	We match subtrees between the two ASTs (using node identity or content matching). Nodes present in one side but not the other are additions/deletions; nodes present in both but with differences in their subtree content are changes.
	•	This diff can be reported in a human-friendly way. One approach is to pretty-print a pseudo-diff of the source code that highlights only the changed code pieces. Another approach is a structured diff format (like showing the AST nodes that were added/removed). For compatibility with Git UIs, our tool can output a unified diff of the generated code, but since it knows which AST nodes changed, it can limit the diff to relevant lines. For example, renaming a variable foo to bar in one function would result in a diff showing only that line changed, even if indentation or other lines in the file moved around.
	•	Tools like Difftastic and diffsitter demonstrate the approach of diffing ASTs to produce cleaner diffs ￼. For instance, diffsitter "computes a diff on the AST of a file rather than on the text," and as a result, formatting changes produce no diff ￼. We leverage the same idea, built-in to the VCS.

From the user's perspective, git diff might show something like:

@@ func add(int a, int b) @@
-return a + b;
+return a - b;

It looks like a normal diff of math.c, but it was generated by examining the operator node's change. Because of our deterministic formatting, we can confidently generate such a snippet for context (we know the surrounding code didn't semantically change).

If a change is structural, say moving a function f() from one file to another, a textual diff would show a deletion in one file and addition in another. Our AST diff could potentially recognize it as a moved subtree (if we have some identification for it), but even if not, the result is similar – the function appears removed in one AST and added in another. The key difference is that we could present it with full function context rather than line-based context.

Tool compatibility: We ensure the output of git diff is in unified diff format so that tools like VS Code's diff viewer can display it normally. Under the hood, we may ignore certain changes (like purely reordering functions) or mark them differently, but to the external world, it's just a diff. For more AST-specific visualization, one could imagine an extension in the future (e.g. showing a tree diff), but that's optional.

Merging and Conflict Resolution

Three-way merges become more intelligent with AST awareness. Git's default merge is textual, which often produces conflicts if two changes overlap in a file. Our system will do a three-way merge of ASTs:
	•	It takes the base commit's AST and the two branch ASTs. It aligns corresponding nodes (using an algorithm akin to tree diff).
	•	If a node was edited in one branch and not the other, the merge keeps the edited version. If edited in both, we have a conflicting edit on that AST node.
	•	If new nodes were inserted at nearby positions (e.g. two new statements in the same function), the merge can include both, deciding an order (perhaps by base order or some deterministic rule). Because each statement is a separate node/file, two inserts don't textually conflict; they appear as two new files in the body directory. Git's tree merge will actually merge them cleanly if they have distinct names. For example, if two branches each add a new statement node with a unique ID or different ordinal, the merged tree will have both nodes. The order might need adjustment, but since code logic doesn't have an inherent order conflict (unless order matters), we could accept either order or sort by an attribute.
	•	Moves (like moving a function from one class to another in an OO language) are handled by noting the same subtree present in different locations. A pure text merge would often conflict or duplicate code in such cases, but an AST merge can recognize a move as deletion+insertion of the same subtree. It can then merge by not duplicating the subtree but placing the updated version in the new location.

A concrete scenario described in research: Alice changes the contents of a method m in her branch, while Bob moves method m to a different class/file in his branch. Textual Git sees one side deleting m and the other side editing it – a conflict the user must resolve manually. Semantically, these changes don't conflict: we want the method m in the new location with Alice's edits. An AST merge can do exactly that: it sees that Bob's branch has m under a different parent node, and Alice's branch has modifications in the m subtree. It will merge by applying Alice's changes to the m subtree in the new location ￼. This automatically resolves what Git would flag as a conflict, illustrating the power of structure-aware merging.

If an actual conflict occurs (both branches changed the same property of a node in different ways – e.g. both edited the same line of a function in different ways), the system can present a structured conflict. Possibly, it could fall back to an inline textual conflict in the generated code for that node. The developer would resolve it by editing the code (or potentially via a specialized AST conflict resolver UI). Once resolved (and parsed back into an AST), a new commit is created.

Under the hood, we can implement merges by checking out the base AST tree, applying changes from one branch's AST, then the other's, akin to how patch merge works. Git's merge driver can be configured to delegate to our tool for certain paths (here effectively all code files). This is similar to how SemanticMerge (a third-party tool) works as a custom merge driver for Git – it parses and merges code with knowledge of the language ￼. In fact, we could integrate at that level: registering a merge driver that calls Tree-sitter to do merges for supported languages. The difference in our system is that the storage itself is AST, so we've fully embraced that model, rather than just using it transiently in merging.

Blame and History Annotation

Git blame on a file usually shows, for each line, the commit that last changed it. In an AST repository, a more meaningful git blame can be implemented:
	•	We can map each AST leaf node (or small group of leaves) to the commit that introduced or last modified it. This mapping can be precomputed by traversing history (similar to how blame works by walking backwards through diffs).
	•	The output of blame could be formatted as the code with per-line commit annotations, just like normal. Since we can generate the file text with knowledge of node boundaries, we can still attribute each line to a commit. For example, the line return a - b; would be attributed to the commit that changed the operator node from + to -. If a whole statement or function was added, all lines of that block get the commit that added the node.
	•	We could also provide a mode to blame by AST element, e.g., "who last modified this function" (which might be simpler than line-by-line for large nodes). Because our history is at node granularity, answering that is straightforward: just find the last commit touching any part of the function's subtree.

To remain compatible with existing IDE integrations, we'll output blame in the standard format (one line per source line with commit SHA and author). The underlying logic ensures that if only formatting changes occurred to a line, the blame might attribute it to the earlier commit that changed the logic (since a pure reformat commit might not even register as changing the AST nodes). This is desirable: trivial reformat commits should ideally not overwrite the blame history of code lines. With AST tracking, we can possibly ignore or separate formatting-only changes from semantic changes in history.

Git CLI and Tool Compatibility

One design goal is not breaking existing Git-based integrations. This means our system likely needs to masquerade as Git to some extent:
	•	The user could alias git to our tool, so when VS Code runs git status or git show, it hits our implementation. Our tool can parse the arguments and either handle them or delegate to the real git for non-code files.
	•	Many Git commands (status, log, branch, etc.) don't need special handling beyond working with the different object structure, which Git itself can mostly handle. For example, git status will show modified files if the AST filesystem has unsaved changes (we could mark the entire file as modified if any AST node changed, or even integrate finer granularity, but Git's granularity is file-level in the index anyway). We might need to override status to avoid showing the thousands of internal AST "files" – likely we will configure .gitignore or similar so that the internal directories (like math.c/fn_add/...) are not treated as untracked files in the working copy. Actually, in a typical use, the working copy is the AST FS itself, which might confuse git status. More likely, we operate with an index and HEAD that reflect AST state, and present to the user a simplified view where each source file is considered one logical unit. The CLI wrapper can intercept status to print a simplified output (e.g. "math.c modified" rather than listing every changed node file).
	•	VS Code integration: VS Code's Source Control panel runs git diff and expects a unified diff to display. We provide that as described. It also runs git commit (which we handle) and git push (which we can mostly pass through to underlying git since pushing objects/refs works normally). Because the commits and trees we create are legitimate Git objects, hosting services like GitHub could technically host the repo. However, browsing it on GitHub would show the AST folders unless they have no way to interpret it. This is a downside unless the service also understands our structure. A workaround is to have a CI job that generates a normal source-code branch for browsing, or use a custom viewer. In any case, the local experience in editors would be preserved by our shim.
	•	Other tools like git grep: by default, if run in our AST repo, it would grep the AST pseudo-files. To be useful, we might override git grep to actually grep through generated source text. This could be done by on-the-fly codegen of each file and searching, or by maintaining a mirror cache of the latest generated source for each file for performance.
	•	Git hooks and scripts: If the project uses Git hooks (like pre-commit hooks) expecting certain files, those might need adaptation. For example, a linting hook expecting to read staged .c files – in our system, the staged content is AST data. We may need to generate a temp copy of the file's source and pass it to the linter. Similarly, any tooling that reads the repository directly would see the AST directories. This is a challenge unless the tools are aware. We could suggest that developers work in the AST-mounted view for editing, but run builds or other tools in a generated source checkout. There could be a command to export the entire repo as real source files for such purposes. This is somewhat analogous to how some systems treat code as an artifact – you might store a model and generate code for build/test.

In summary, by intercepting key Git commands and using a FUSE filesystem, we strive to let developers use their existing IDEs and workflows. The AST nature is mostly under the hood; the main visible differences would be: diffs are cleaner, merges might auto-resolve more often, and the .git directory is filled with unusual objects (which most users won't inspect directly).

Code Generation and Round-Tripping with Tree-sitter

One of the trickiest components is the code generator that turns ASTs back into source code. Tree-sitter itself is a parser, and while it doesn't provide a built-in unparser, we can leverage its grammar definitions to create one. In fact, Tree-sitter generates a file grammar.json and node-types.json for each language's grammar, which describe the AST node types and their fields. This can be used to systematically produce source code from an AST ￼. A blog post by the Symflower team notes that "TreeSitter… can be leveraged to generate code for printing and formatting ASTs" using the grammar definitions ￼. In practice, we would implement a pretty-printer for each language (or possibly derive one automatically from Tree-sitter grammar) to emit code.

Ensuring lossless generation: For a correct round-trip, the generator must insert all required syntax (keywords, punctuation, etc.) that might not be explicit nodes. For example, an if statement node in the AST might have children for the condition and the body, but the generated code needs the literal text if (...) { ... }. These literals are defined in the grammar, so the generator knows to put them. Comments attached to nodes should also be emitted in appropriate positions. If the AST did not store a comment (because it was considered whitespace), the comment might be lost unless we deliberately captured it as a node – so we likely treat comments as part of the AST to preserve them through commits.

We assume for this analysis that such a codegen is implemented for the target languages (perhaps by integrating an existing formatter or pretty-printer). The determinism means developers cannot influence the exact formatting in the repo; it will be standardized. This could actually be a positive, similar to running a formatter on commit. It separates formatting from editing – developers can format as they like locally, but when committing, the AST (and thus a normalized format) is what goes in. Conversely, when viewing diffs or blaming, formatting changes are ignored if they don't affect AST. This addresses the scenario mentioned in the Prettier issue discussion: "If we version controlled the AST, I could view the code however I want, and save it back to git as the AST… taking styling debates out of teams completely." ￼.

Tree-sitter's .edit() API: Tree-sitter allows applying text edits to an existing parse tree by adjusting node positions, then reparsing minimally. Our system would utilize this heavily for performance. Instead of reparsing a file from scratch on every keystroke or save, we maintain the parse tree and on each edit do ts_tree_edit (which adjusts the tree for the edit's delete/insert region) followed by ts_parser_parse with the old tree as a reference. This yields an updated AST very quickly. This is exactly how editors like Atom (with Tree-sitter) achieve realtime parsing ￼. Using .edit(), we can keep node identities stable across small edits (where possible), which might help preserve the mapping to file names (if a node's identity is stable, we don't need to create a new blob and can reuse the old one with minor changes, though in Git any content change = new blob object, but identity stability helps diffing algorithms).

Codegen for working files vs committed files: There are two ways to handle the working copy. One approach is what we described: the working copy the developer sees is generated code from the AST at all times (so the AST is the source of truth, and developer edits go into AST via FUSE, then instantly reflect as updated code in the same filesystem). Another approach could be to let the developer actually edit files normally on disk, parse them on commit, and discard the text after. But that approach risks divergence and requires parsing on each operation. The FUSE live AST approach has the advantage that the AST is always up to date and can be used for immediate diffing, etc.

However, generating the entire file text for every read could be slow for large files. We can optimize by caching the rendered text until the AST changes. FUSE can keep an in-memory copy of the last generated source for each file, invalidating it on edits.

AST and build integration: If compilers could consume ASTs, we'd feed them directly. But typically, compilers expect source code. For now, codegen is needed to produce actual source files for the compiler. In a development environment, since the AST FS is showing source, the compiler (like gcc or javac) can just read the files from the mounted AST filesystem, which will trigger generation transparently. This means developers can compile and run the code normally, and it will work (with a slight overhead that reading the file invokes the generator).

In CI, we might not want to run a FUSE. Instead, the CI pipeline can have a step "generate sources from AST". This could be a command that exports a directory of .c files (or whichever language) from the AST commit. After that, the usual build steps run (tests, etc.). We treat the generated source as a build artifact – a byproduct of the true source (AST). This is a reversal of the usual view (normally ASTs are ephemeral byproducts of source); here source text is the byproduct of AST. The implication is that repository code reviews might prefer to review AST diffs or standardized diffs, but in practice, humans will read code. So the codegen should produce readable code (which it will, especially if using a pretty-printer). Code review tools might need slight adjustment to hide noise if any. In the simplest case, code review on platforms would just see the same diffs our git diff produces, which we've tailored to be human-friendly.

Assumptions and Open Challenges

While the concept is powerful, implementing it requires several assumptions and leaves some open research questions:
	•	Deterministic AST serialization: We assume that for a given source program, there is a unique AST representation (ignoring irrelevant formatting). This generally holds if we include comments as nodes; parsing is deterministic. We also assume the AST can be serialized to a filesystem in a consistent way. Minor differences in how we choose to split nodes could change the granularity, but once chosen, it should be consistent. We must ensure Tree-sitter grammars produce the same AST given the same code (which is true) and that our file ordering or naming scheme is consistent (which is up to our implementation).
	•	Unique node mapping and IDs: As discussed, one tough assumption is that we can assign stable identities to AST nodes such that across versions we can recognize "the same" node that moved or changed. In text, diff algorithms do this implicitly by similarity of lines. For AST, we can use node type and perhaps some content to match. Tools like GumTree (AST diff) find move correspondences by hashing subtrees, etc. In our Git model, if we don't explicitly track an ID, a move appears as delete+add of files. We might accept that. Alternatively, we could embed an "ID" field in each node (as a sort of hidden property) that is persisted (perhaps as a special comment or metadata file). This opens questions: how to generate new IDs (UUIDs?), how to store them without affecting code semantics (comments could do). For feasibility, many systems skip stable IDs and rely on diff to detect renames. Our assumption is that even without explicit IDs, diff quality is improved; explicit IDs would be a future enhancement for even better rename tracking at AST level.
	•	Performance scaling: The approach potentially creates a huge number of tiny files. For example, a single large source file of 1000 lines might result in hundreds of AST files (each statement, expression, etc.). Git can handle a lot of files, but operations like git status or checkout will involve many syscalls. There are known performance issues when a repo has extremely many files or directories. One suggestion is to limit files per directory (we are kind of doing that naturally by AST hierarchy) ￼. The Linux filesystem will also have to manage all these entries. Caching and perhaps using packfiles for the working directory (like Git's sparse index or GVFS) might be needed. We assume that with modern filesystems and not overly insane granularity, this is manageable (e.g., 10k files might be okay, but 1 million is not). In practice, one might choose a coarser AST granularity for very large files (e.g., treat a whole function body as one blob rather than every expression). That's a tunable trade-off between diff resolution and performance.
	•	Tool support and adoption: This system is a radical departure from plain text versioning. We assume developers are using our custom CLI or GUI that hides the complexity. If someone tries to interact with the repo without the special tools (e.g., running a normal git command), they might get confused by the AST directories. For instance, running vanilla git status in the AST working copy might list every single node file. To prevent that, we could mark the entire AST tree as hidden or use an overlay. Perhaps the working copy on disk is empty or contains normal files and the AST is only in memory – however, then standard editors wouldn't see the code. So, likely everyone on the project must use the AST-git tooling. This is a cultural/rollout assumption rather than a technical one.
	•	Multi-language support: We assume Tree-sitter has grammars for all languages in the repository (Tree-sitter supports dozens of languages, so this is plausible for most). Each language will require a code generator. This is engineering work per language. If a language is not supported, the system could default to treating those files as binary or plain text (no AST benefits). The design should allow mixing (e.g., for images or data files, just store them as normal blobs).
	•	Binary files and others: Non-code files can bypass AST and be stored normally. Our Git wrapper would detect if a file type has an AST grammar. If not, it can fall back to standard Git behavior. This ensures we're not regressing on versioning of assets, configs, etc.
	•	Maturity of AST merge algorithms: While academic projects and tools like SemanticMerge show the promise of AST merging, it's still an area of active research to make it general for many languages and complex edits. We might encounter edge cases where a merge of ASTs yields an AST that doesn't parse to valid code (perhaps due to dependencies between parts of the code). Our merge should ideally ensure the result is syntactically valid – since we operate on AST, we won't produce an outright syntax error, but there could be logical issues (like two halves of code that needed coordination). Those are no different from text merges though.
	•	Deterministic build artifact assumption: If the source is treated as an artifact generated from AST, we assume that building from the generated source is equivalent to building from the original developer-authored code. This should hold because the AST is the source in a structural sense. However, consider cases like intentional formatting affecting program behavior (rare, but e.g. in Makefiles or Python significant indent, though our system would handle indent in Python by structure). We have to ensure the semantics are preserved 100%. Another corner case: tools that embed code snippets or do string matching in code (maybe tests that grep the source files for certain text) might break if formatting changes. This is a broader impact of normalizing code – usually considered acceptable.

Many of these assumptions highlight that this is feasible but complex. It's not a purely theoretical idea; parts of it exist in isolation (incremental parsing in editors, structural diff tools, semantic merge tools, etc.). Combining them into a seamless version control system is an engineering challenge.

Performance Trade-offs

Let's evaluate expected performance implications compared to traditional Git:
	•	Repository size: Storing AST means storing more objects (each small piece separately). However, many of these pieces are highly compressible and reusable. If a single line changes in a 1000-line function, traditional Git stores a new blob for all 1000 lines (maybe delta-compressed later). AST-Git will store a new blob for just that line's node (plus some tree object overhead). As a result, the packfile size might actually be competitive or even smaller. Git's delta compression will attempt to compress the old vs new blob too, but with AST we possibly reuse identical blobs outright for untouched nodes (no need for delta). The .git directory might have more loose objects between GCs. Regular git gc will pack them. There is a concern that object count grows large, which could strain memory when hashing or enumerating objects. But Git can handle repositories with hundreds of thousands of objects (for example, the Linux kernel has ~1M objects). If every AST node is an object, a big codebase might reach that scale. It's borderline but maybe acceptable with modern Git and some tuning (like the shared object database or pack filters).
	•	Working directory performance: As noted, git status iterating over thousands of files is slow because of many lstat calls ￼. We can mitigate this with Git's file system monitoring (FSMonitor) which caches what changed ￼. Also, since our tool knows exactly which files (nodes) changed (from the AST diff), it could programmatically update the index without scanning. In fact, the staging process might skip the index and directly commit, avoiding git status entirely. We could make git status just query our AST state (which tracks dirty flags).
	•	Merge performance: Merging large ASTs might be slower than a text merge, because it involves parsing (already done) and tree-diff algorithms. But these are still O(n) in the size of the AST, comparable to O(n) in lines for text. The overhead might come from analyzing structure. For typical merge scenarios, we expect it to be manageable. If needed, one could fall back to textual merge for extremely large files or unknown languages.
	•	Memory usage: Keeping ASTs in memory for all open files or for a whole project might use more memory than just reading text on demand. But Tree-sitter trees are fairly compact (they store nodes and tokens). It's a trade-off: we use memory to hold AST for quick diff/merge, whereas plain Git uses none for that until diff is run (when it loads files). On modern systems with plenty of RAM, having, say, a few hundred MB for AST data of a big repository is not unreasonable.
	•	Networking and remote: Pushing and pulling AST commits involves more objects, but Git's pack protocol will send deltas, so likely it's not significantly more data than sending the raw code. The number of round trips might increase if many refs, but not much difference otherwise. Clone might be slightly slower due to more objects to check out (especially if checking out means generating code for each file).
	•	Checkout speed: Checking out a commit means reconstructing all files. In text Git, that's writing each file from a blob (possibly after decompressing delta). In AST Git, it means creating directories and small files (or feeding them to FUSE). If we use FUSE with on-demand generation, an initial ls of the project may cause generation of all files anyway (editors might do that). This could be slower if done naïvely, but we could optimize by generating only on open. The first build will cause many reads (thus generation of all files). This is akin to a fresh clone where you then compile – some overhead to generate code. This step could be parallelized since each file generation is independent. It might end up comparable to compiler parse time, ironically. In CI, this overhead is just a new step.

In short, the system will be heavier in certain operations but possibly lighter in others. There is likely a hit in day-to-day status/checkout responsiveness if not optimized. Git developers have put a lot of work into scaling (e.g., Microsoft's GVFS for massive repos); similar techniques (like not expanding all AST nodes until needed) can be applied. We could, for instance, lazily create blob objects for unchanged nodes instead of writing them on each commit – reuse by reference to a previous tree object might suffice. (Though Git objects are content-addressed, we might not even need to write an object if it's identical to an existing one – we just refer to it by hash in the new tree.)

Developer Workflow Walk-through

To make it concrete, let's walk through the entire lifecycle of a change using this system:
	1.	Editing and Committing a Change: Suppose a developer wants to change the implementation of add(a, b) to multiply instead. They open math.c in VS Code. The AST-FUSE presents the file with content as generated from the latest AST (initially, "return a + b;"). The developer edits that line to "return a * b;" and saves. Under the hood, the FUSE captured the edit; Tree-sitter updated the AST (operator node value from "+" to ""). Now the AST in memory differs from HEAD. The developer runs git diff: our Git wrapper computes an AST diff between HEAD's AST (which had "+") and the working AST (which has ""). It finds that in fn_add -> return -> op_add -> operator.txt the content changed from "+" to "*". It produces a unified diff of math.c highlighting the changed line:

-    return a + b;
+    return a * b;

This is shown to the developer (maybe also indicating in a comment that it's an AST-based diff, but format-wise it's standard). The developer is satisfied and runs git commit -am "Use multiplication in add()". The -a flag triggers our wrapper to stage all changes; it sees operator.txt changed and thus marks math.c as needing a new tree. It constructs the new tree objects (most are reused, only operator.txt blob and ancestor trees for that path are new). It writes a commit object with the message. The commit now exists in the .git database. The working AST is clean (no unstaged changes).

	2.	Pushing and Collaboration: The developer pushes. On the remote (say GitHub or a bare repo), there's nothing inherently different about the commit – it's just that the tree objects represent an AST. Another developer pulls this commit. Their local AST-FUSE will now reflect the updated AST. If they open math.c, they'll see return a * b; generated from the AST. They didn't need to run a pretty-printer – it's automatic from the AST.
	3.	Merge Scenario: Now assume concurrently, another developer on an old commit had moved the add function to a different file (or perhaps changed its signature). When that developer merges with the main branch (which has the multiplication change), the merge process takes the AST where add moved (say from math.c to math_utils.c) and the AST where add's internals changed. The three-way merge identifies that in one branch, fn_add node moved to a different parent (different file AST), and in the other branch, the fn_add/operator child changed. It will merge by placing the updated fn_add in the new location. If our system can't automatically detect it, it might at worst show a conflict where fn_add node is edited in one and deleted in the other (since it moved). But an AST-aware merge tool would see it as a move and handle it. Let's assume it handles it: the resulting AST has fn_add in math_utils.c with the * change applied. The merge commit created will have that state.
The developer might inspect the merge in a diff tool – it should ideally show that add function was moved (perhaps as a deletion from one file and addition in another) and that within it, one line changed. Traditional Git would have likely given a conflict that needed manual resolution, but our scenario avoided that.
	4.	Continuous Integration (CI/CD): When CI checks out the repository at a certain commit, it gets the AST structure. The CI script then runs a step to generate code files. This could be as simple as mounting the AST via a command-line FUSE tool or running a git ast-export command that writes out the *.c files. Once that is done, the CI has a normal codebase to run builds and tests on. Because the generated code is in a consistent format, tools like linters or format checkers might not be needed (format is standardized). If CI wants to ensure no one bypassed the system, it could verify that regenerating AST from the code yields the same AST (to catch if someone manually edited the generated code without updating AST, though in our workflow that shouldn't happen because developers don't directly edit generated code).
If the source is considered an artifact, one could even imagine not exposing the generated source to developers at all, only to CI. But that's impractical for development – developers need to see and write code. So practically, developers see code, but the truth is AST.
	5.	Blame and History: Later, someone runs git blame math.c on that file. The system generates math.c (or uses cached text) and for each line, determines the commit. The line return a * b; is attributed to the multiplication commit (with that SHA and author) ￼, even if later formatting changes occurred (none in this case). The surrounding lines (function signature, etc.) might be attributed to earlier commits that introduced the function. If the function moved files, blame might get trickier (Git blame follows file history by content, but if file path changed, usually you need -C or similar options). In AST blame, since the fn_add subtree itself has an identity (or at least an object identity in commit history), we could track it across moves. We might implement a custom blame that can follow a node's history across file moves by looking at the AST in previous commits to find a matching subtree (this is advanced, but doable by searching commit history for a matching fn_add structure).
From the user's standpoint, blame output looks normal. But we could also offer an AST-blame view: e.g. show that the function was last moved in commit X by Bob, the return statement last changed in commit Y by Alice, etc. This is new information not readily available in line-based blame.

Overall, the developer workflow doesn't drastically change in terms of commands. Where they will notice a difference is in fewer merge conflicts and diffs that omit irrelevant changes. For example, reordering functions or files will not produce massive diffs – the AST diff might show no changes at all if nothing semantically changed (though technically the order change might not even be recorded unless we impose an order). This encourages refactoring, as the VCS is more accommodating to code moves and renames. It also means code reviews can focus on logical changes instead of noise.

CI/CD and Build System Implications

Treating source code as a compiled artifact (from AST) has some implications:
	•	Build integration: As mentioned, an extra step is needed to produce source files from the AST. Build systems could incorporate a rule, e.g., %.c: %.ast (if we had a textual .ast representation) or simply run a script to dump AST to files. This step should be deterministic and versioned (perhaps the code generator version is tied to the repo to avoid differences). The build cache should consider that if AST hasn't changed, generated code hasn't (which it will if deterministic).
	•	Testing and artifacts: If the build produces artifacts (binaries, etc.), that's unchanged. But what about the source itself being delivered? In most cases, teams don't ship source except in open source. If they do (say a source release), they would run the generator and package the output.
	•	Integration with code analysis tools: Static analysis or code indexing tools might need to operate on the generated code. Alternatively, such tools could operate directly on the AST (which might be more efficient or accurate). For example, a code navigation tool could skip parsing because the AST is already available. Our system could even expose an API to query the AST for IDE features, effectively acting as a language server. This is a nice side benefit – the VCS and language intelligence could blend.
	•	Gradual adoption: An organization might not convert all repositories at once. Maybe they introduce AST-Git for a new project or a subset. One must consider interop: probably no straightforward way to convert an existing repo's history into AST form without parsing every historical version (which is possible but effort-intensive). It might start from a point in time (the day you adopt it, you parse the current code, commit an AST commit that represents the same code, and from then on use AST commits). History before that remains text. This hybrid could be confusing; likely a clean break or a parallel history is used.
	•	Tooling for legacy workflows: If someone clones the repo with vanilla Git (not using our special tool), they get a bunch of folders and .txt files as shown. This is not really usable. We might mitigate this by storing a secondary plain-text representation on a special branch. For example, each commit on main (AST) triggers a CI job that commits the generated source to a main-text branch. Users who want a regular view could use that (read-only). This way platforms like GitHub can display code normally by looking at the text branch. The AST commit could even carry a pointer to a text commit (via an annotation or identical commit message) for cross-reference. This duplication is not ideal, but it's an integration strategy. We assume for the analysis that such solutions are possible if needed.

In essence, treating source as AST flips some conventional steps, but it doesn't break the fundamentals of the build/test cycle. It mostly inserts a generation step. Given that many modern systems already have transpilation or codegen steps (think of proto->code, or using linters/formatters in CI), this isn't alien. It does mean developers must trust that what runs is what they wrote logically – since the generator could theoretically introduce a bug if it misrenders code. That's why ensuring the generator is correct and perhaps minimal (ideally just reorders and prints tokens without altering meaning) is crucial.

Feasibility Analysis and Future Outlook

Building a Git-backed AST system is feasible but requires significant engineering effort and careful handling of edge cases. Let's summarize feasibility points:
	•	AST parsing and editing: Already possible with Tree-sitter for many languages. Incremental parsing on each edit provides a responsive experience ￼. So the core parsing technology is there.
	•	AST diff and merge: Proven in specialized tools. Our system would embed those capabilities natively. There might be cases where we fall back to manual conflict resolution, but overall it's a net win. SemanticMerge (by Plastic SCM) has shown that language-dependent merge can greatly ease conflict resolution ￼, though it is not trivial to support all languages (they maintain separate parsers for each language).
	•	Git plumbing usage: Git is flexible enough to store arbitrary trees. We'd be abusing it with many small files, but nothing fundamentally breaks. We might need to tweak configurations (e.g., raise limits on tree entry count, etc.). The fact that we can use Git's object database means we don't have to reinvent versioned storage. This is a big feasibility plus: we can incrementally implement our system as a layer on top of Git. In worst case, if performance suffers, we might consider alternatives (like a custom store or another VCS like Pijul which is not line-based internally ￼), but that complicates tool integration. Sticking with Git means we get network protocols, authentication, etc., for free.
	•	Developer acceptance: This is perhaps more a social issue. Developers are used to text diffs. But many would appreciate the improvements: one HN commenter said "I do kind of love the idea of Git using ASTs instead of source code. It makes a ton of sense… why can't we pull the code down, view it in whatever setup we want, then commit a normalized version?" ￼. There is an appetite for reducing friction like whitespace issues. As long as the tool is stable and the learning curve is small (maybe just some new commands or understanding that the repo is special), developers could adopt it, especially for large projects where merge conflicts are painful.
	•	Edge cases: We need to consider things like generated code in the repo – but those are usually handled via ignoring them or different processes. Our system shouldn't break binary file handling (we won't parse those). Another edge case: partially committed changes. In Git you can stage some lines of a file (git add -p). In AST world, staging at a sub-node level could be a feature (stage this function change but not that one). We could implement that by staging specific node files. However, typical UIs might not allow that easily. It might be an advanced use where our CLI offers to stage specific AST nodes. That's a potential benefit (more precise than hunk-by-hunk staging, because you ensure the AST stays valid).
	•	Learning from attempts: It's worth noting that while this idea has floated around for years, it hasn't seen widespread implementation in mainstream VCS. That indicates challenges (performance, complexity, lack of tooling support). A partial step that is happening is use of structure-aware diff and merge within the context of text Git – e.g., difftools that understand syntax ￼, or custom merge drivers. Our approach is more radical by changing storage. If performance concerns can be overcome, it could be a game changer. If not, one might implement many of these ideas (smart diff, semantic merge) without changing Git storage, just by integrating with Git's extension points (like diff/merge drivers ￼). The full AST-in-git approach however opens the door to consistently formatted code and truly treating code as data.

Unimplemented components and future work:
	•	A generic AST code generator for each language (perhaps aided by grammar files) is needed. This is a known gap (Symflower's blog indicated they plan to automate AST printing from Tree-sitter grammars) ￼. Initially, one might use existing formatters (e.g., use Prettier for JS, clang-format for C++) to generate code from AST, as a shortcut. As long as they produce a predictable style, that could work.
	•	FUSE stability and performance tuning would be critical for everyday use. There might be platform-specific issues (Windows doesn't have FUSE natively, but WinFsp or other mechanisms could be used; our system might be easier to prototype on Linux/macOS first).
	•	Conflict resolution UI: While auto-merging will handle more cases, when conflicts do occur, an AST-aware conflict viewer would be nice (e.g., show the two versions of a node's code side by side, rather than inline <<<<< >>>>> markers). That could be a custom tool or even integrated into editors via an extension that understands our conflict markers.
	•	Deterministic ordering of children: We touched on how to order e.g. statements or class members. Right now, order is given by the code. In AST storage, we might rely on lexical ordering of filenames or an explicit order file. This is something to finalize. Perhaps the simplest: keep an order.txt in each directory listing the node keys in sequence. Then the presence of two new items from merges can be resolved by merging those lists (which could conflict if both inserted at same position – though if the list is just sorted by some key, we avoid conflict but lose intentional ordering). This detail is open-ended in our design.
	•	Security and integrity: One should consider that generating code from AST is like a compiler step – bugs in it could introduce errors. Also, storing code differently might confuse security scanners or license scanners. Those tools would need to adapt (or run on generated output). It's an integration task.

Despite the complexities, the feasibility is bolstered by the fact that Git itself does not need to be modified. We use Git as a content-addressable storage and transport, which it does well. The intelligence is in the client side (parsing, FUSE, etc.). This means different implementations could exist: e.g., one could have a read-only web UI that shows AST diffs from a Git repo by parsing two versions of a file. Or a server-side merge bot that uses AST merges for pull requests. Our approach concentrates it in a unified system.

Conclusion

In conclusion, an AST-backed Git system is an ambitious yet achievable enhancement to software version control. By storing code in a structured form, we can eliminate superficial conflicts, make diffs more meaningful, and decouple the way code is stored from how developers view or edit it. The architecture requires a blend of compiler technology (parsing/printing) with VCS mechanics (object storage, merges), and careful bridging to existing workflows via a virtual filesystem and CLI wrappers.

We outlined how each major Git operation can be re-imagined in this paradigm: diffs become semantic and ignore formatting ￼, merges handle moves/renames gracefully ￼, and blame/history can operate at a logical element level. All this is done while preserving compatibility, so tools see just another Git repository (with perhaps some unusual contents, but accessible through our compatibility layer).

Key feasibility highlights include the reuse of Tree-sitter for multi-language support and incremental updates, and the reuse of Git's robust storage and distribution. The main challenges lie in managing performance with so many small files and ensuring the AST↔code round-trip is perfect and efficient. We also identified some assumptions (like deterministic codegen and stable node identity) that need to hold or be engineered around.

The performance trade-offs suggest some overhead, but with modern computing resources and optimizations (caching, lazy loading, etc.), these are not insurmountable. The benefit – a more intelligent VCS that understands code – could greatly reduce the time developers spend resolving trivial conflicts or combing through noisy diffs. A qualitative benefit is also that the repository can enforce a uniform code style automatically and even allow developers to use different viewing preferences locally ￼, since the true stored form is the AST (imagine two developers with opposing brace style preferences can both be happy, as their editors show their preferred style, but the repo stores one canonical style).

In practice, implementing this system would likely start with a prototype for one language (say Python or JavaScript, which Tree-sitter supports) and gradually generalize. The use of FUSE and hooking Git commands means it can be layered without modifying Git internals, which is a huge advantage. Over time, such a system could evolve from a power tool into a mainstream approach, especially as projects grow larger and the costs of mismerges and legibility issues rise.

This deep analysis shows that while non-trivial, replacing text-based Git with an AST-based layer is within reach. It aligns with the software engineering trend of raising abstraction levels – here we raise version control from lines to syntax trees. The immediate next steps would be to build a proof-of-concept, measure its behavior on real repositories, and refine the design (especially around node identity and performance). If successful, it could usher in a new era of version control where the VCS truly "understands" code structure, leading to more automation and less manual conflict resolution for developers.

## Setup

### Prerequisites

- **Rust Toolchain:** Ensure you have Rust installed. You can get it from [rustup.rs](https://rustup.rs/).
- **Homebrew (macOS):** If you are on macOS, you will need Homebrew to install system dependencies. You can install it from [brew.sh](https://brew.sh/).

### Dependencies

- **macFUSE (macOS):** The AST Virtual Filesystem relies on FUSE. On macOS, you need to install macFUSE:
  ```bash
  brew install macfuse
  ```

Follow the post-installation instructions provided by the `brew install` command, which may involve system settings changes.

*(Note: Containerized development environments using tools like Docker or Dagger are planned for the future to simplify dependency management across platforms.)*

### Running the POC

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/git-ast.git # Replace with actual URL if different
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
    Run the executable, providing the path to a Git repository and the mount point:
    ```bash
    # Replace /path/to/your/repo with an actual Git repository path
    ./target/debug/git-ast /path/to/your/repo /tmp/git_mount 
    ```
    The filesystem will mount and run in the foreground. You can explore `/tmp/git_mount` in another terminal. Press Ctrl+C in the terminal where `git-ast` is running to unmount and exit.

**Warning:** This is early-stage software. Use a test repository and mount point. Functionality is currently very limited.
