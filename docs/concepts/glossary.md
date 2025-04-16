# Git AST Glossary

This glossary defines the key terms used throughout the Git AST project documentation.

## A

### AST (Abstract Syntax Tree)
A tree representation of the abstract syntactic structure of source code. Each node denotes a construct occurring in the source code. It typically omits details like comments, whitespace, and parentheses, focusing on the semantic structure.

## C

### Clean Filter
A Git mechanism that automatically transforms file content when it's staged or committed. In Git AST, the clean filter parses source code into an AST/CST and serializes it for storage.

### CST (Concrete Syntax Tree)
Also known as a parse tree, a CST represents the source code exactly, including all tokens like punctuation, whitespace, and comments. Tree-sitter produces CSTs, which are beneficial for this project as they retain the fidelity needed for accurate code generation.

## D

### Deterministic Pretty Printing
The process of generating source code from an AST/CST in a way that is completely consistent - the same input always produces the same output, regardless of environment or context.

## G

### Git Filter
Git's mechanism for transforming file content as it moves between the working directory and Git's object database. Git AST uses both clean filters (for staging/committing) and smudge filters (for checkout).

### GumTree
A well-known algorithm for computing differences between two ASTs, producing an edit script (add, delete, move, update operations) that represents the structural changes.

## N

### Node Identity
The challenge of reliably identifying the "same" logical code element (like a specific function or variable) across different versions of an AST, even if it has been moved, renamed, or internally modified. Crucial for accurate semantic diff/merge.

## P

### Pretty-Printing
The process of converting an AST/CST back into formatted, human-readable source code text. Deterministic pretty-printing ensures the same AST always produces the same output text.

## S

### Semantic Diff
A comparison between two versions of code that operates on the code's structure (AST/CST) rather than its textual representation. This allows ignoring formatting changes and focusing on structural modifications.

### Semantic Merge
A merge operation that works at the AST level to intelligently combine changes from different branches, handling structural modifications better than text-based merges.

### Smudge Filter
A Git mechanism that automatically transforms file content when it's checked out from the repository. In Git AST, the smudge filter deserializes stored AST/CST data and generates formatted source code.

## T

### Tree-sitter
A parser generator tool and incremental parsing library. It can build a CST for a source file and efficiently update it as the source file is edited. It supports a wide range of programming languages via community-maintained grammars.

## Additional Resources

For more detailed explanations of these concepts and how they apply to Git AST, see:
- [Key Concepts](./key-concepts.md) - In-depth explanations of the core concepts
- [Architecture Design](../architecture/design.md) - How these concepts are implemented in the system architecture 
