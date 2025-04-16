# Git AST Architecture Documentation

This directory contains detailed documentation about Git AST's architectural design and implementation details.

## Contents

- [Design Overview](./design.md) - High-level architectural design explaining the core mechanisms, data flow, and integration points
- [Clean/Smudge Filters](./clean-smudge-filters.md) - Detailed explanation of how Git AST uses Git's clean and smudge filters to implement AST-aware version control

## Architecture Overview

Git AST integrates with the existing Git workflow using Git's built-in clean and smudge filters. This approach provides several advantages:

1. **Minimal Disruption:** Developers continue to use standard Git commands
2. **Broad Tool Compatibility:** IDEs, editors, and other tools see normal source files
3. **Leverages Git Infrastructure:** Uses Git's blob storage and filter mechanisms

The basic flow is:

- **When Committing:** Source code is parsed into a syntax tree and stored in Git
- **When Checking Out:** The stored tree is converted back into formatted source code

For a complete understanding of the architecture, start with the [Design Overview](./design.md). 
