# GitHub Wiki Integration

This guide explains how to sync Git AST documentation with GitHub Wiki for improved discoverability and accessibility.

## Setting Up GitHub Wiki Integration

GitHub Wiki provides a user-friendly way to access documentation directly from the repository page. Here's how to integrate our documentation with GitHub Wiki:

### Method 1: Wiki Repository as a Git Submodule

1. **Enable Wiki for your repository**
   - Go to repository Settings → Features → Wiki → Enable

2. **Clone the wiki repository locally**
   ```bash
   git clone https://github.com/yourusername/git-ast.wiki.git
   ```

3. **Set up a script to sync docs to wiki**
   Create a script (`sync-docs-to-wiki.sh`) in the project root:
   ```bash
   #!/bin/bash
   
   # Define paths
   DOCS_DIR="./docs"
   WIKI_DIR="../git-ast.wiki"
   
   # Ensure wiki repo is cloned
   if [ ! -d "$WIKI_DIR" ]; then
     echo "Wiki directory not found. Please clone the wiki repository first."
     exit 1
   fi
   
   # Copy documentation to wiki
   cp -r $DOCS_DIR/* $WIKI_DIR/
   
   # Special handling for Home page
   cp $DOCS_DIR/start-here.md $WIKI_DIR/Home.md
   
   # Commit and push changes to wiki
   cd $WIKI_DIR
   git add .
   git commit -m "Sync documentation from main repository"
   git push
   
   echo "Documentation synced to wiki successfully!"
   ```

4. **Run the script whenever documentation is updated**
   ```bash
   chmod +x sync-docs-to-wiki.sh
   ./sync-docs-to-wiki.sh
   ```

### Method 2: GitHub Actions Automation

You can automate the sync process with GitHub Actions:

1. **Create a workflow file** in `.github/workflows/sync-wiki.yml`:
   ```yaml
   name: Sync Docs to Wiki
   
   on:
     push:
       branches:
         - main
       paths:
         - 'docs/**'
   
   jobs:
     sync-wiki:
       runs-on: ubuntu-latest
       steps:
         - name: Checkout Repository
           uses: actions/checkout@v3
   
         - name: Checkout Wiki
           uses: actions/checkout@v3
           with:
             repository: ${{github.repository}}.wiki
             path: wiki
   
         - name: Sync Docs to Wiki
           run: |
             cp -r docs/* wiki/
             cp docs/start-here.md wiki/Home.md
   
         - name: Commit and Push Changes
           working-directory: wiki
           run: |
             git config user.name "GitHub Actions"
             git config user.email "actions@github.com"
             git add .
             git diff-index --quiet HEAD || git commit -m "Sync docs from main repository"
             git push
   ```

## Wiki Navigation Structure

For best navigation in GitHub Wiki, consider these adjustments:

1. **Update links in documentation** - When syncing to wiki, links should be relative to the wiki root
2. **Create a _Sidebar.md file** in the wiki to customize the sidebar navigation
3. **Use the Home.md file** (synced from start-here.md) as the wiki landing page

## Best Practices

- Keep documentation in the main repository as the source of truth
- Sync to wiki for improved accessibility
- Consider using relative links in markdown to work in both contexts
- Test wiki navigation after syncing to ensure links work properly 
