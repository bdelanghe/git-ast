// Comment out unused tree-sitter types for now
// use tree_sitter::{Parser, Tree};
use std::error::Error;
use std::env; // Add env for command-line arguments
use std::path::Path; // Add Path for mount point
use libc; // Add use statement for libc

// Keep git2 imports as they might be used by git_fuse
// Remove unused Repository, Oid
// use git2::{Repository, Oid};

// Declare the new module
mod git_fuse;

// Import the FUSE filesystem struct
use git_fuse::GitFS;

// Comment out unused functions for now
/*
/// Parses the given Rust source code string using tree-sitter.
fn parse_rust_code(source_code: &str) -> Result<Tree, Box<dyn Error>> {
    let mut parser = Parser::new();
    let language = tree_sitter_rust::language();
    parser.set_language(&language)?; // Use ? for error propagation

    let source_bytes = source_code.as_bytes();
    let tree = parser.parse(source_bytes, None)
                     .ok_or("Parser timed out or failed")?; // Handle Option -> Result

    Ok(tree)
}

/// Lists entries (name and OID) in a git tree for a given treeish identifier.
fn get_git_tree_entries(repo_path: &str, treeish: &str) -> Result<Vec<(String, Oid)>, Box<dyn Error>> {
    let repo = Repository::open(repo_path)?;
    let obj = repo.revparse_single(treeish)?; // Find the object for the treeish

    // Get the tree; if the object is a commit, get its tree, otherwise assume it's a tree
    let tree = obj.peel_to_tree().map_err(|e|
        format!("Could not peel object {} ({}) to tree: {}", treeish, obj.id(), e)
    )?;

    let mut entries = Vec::new();
    for entry in tree.iter() {
        let name = entry.name().unwrap_or("[invalid utf8 name]").to_string();
        let oid = entry.id();
        // let kind = entry.kind(); // Could also get ObjectType::Blob, ::Tree, etc.
        entries.push((name, oid));
    }

    Ok(entries)
}
*/

fn main() -> Result<(), Box<dyn Error>> {
    // Argument parsing
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <repo_path> <mount_point>", args[0]);
        // Use libc exit code for invalid argument
        std::process::exit(libc::EINVAL);
    }
    let repo_path = &args[1];
    let mount_point = Path::new(&args[2]);

    println!(
        "Mounting Git repository '{}' at '{}'",
        repo_path,
        mount_point.display()
    );

    // Pass the repo_path to the constructor
    let filesystem = GitFS::new(repo_path.to_string());

    // Mount the filesystem
    // Note: Requires fuse development libraries installed (libfuse-dev, etc.)
    //       and potentially user permissions (e.g., being in the 'fuse' group).
    match fuser::mount2(filesystem, mount_point, &[]) {
        Ok(()) => println!("Filesystem mounted successfully. Press Ctrl+C to unmount."),
        Err(e) => eprintln!("Failed to mount filesystem: {}", e),
    }

    // Keep the main thread alive until unmounted (usually by Ctrl+C)
    // fuser::mount2 blocks, so we don't strictly need anything here,
    // but it's good practice if we were spawning the mount in a thread.

    // --- Old Logic (Commented Out) ---
    /*
    // ... tree-sitter parsing logic ...
    let tree = parse_rust_code(source_code)?;
    let root_node = tree.root_node();
    println!("Parsed AST:\n{}", root_node.to_sexp());

    // ... get_git_tree_entries logic ...
    match get_git_tree_entries(".", "HEAD") {
        Ok(entries) => { ... }
        Err(e) => { ... }
    }

    // ... commented out query logic ...
    */

    Ok(())
}

// Keep tests module, but maybe disable/comment out old tests if they interfere
#[cfg(test)]
mod tests {
    // use super::*; // Might not be needed if no functions from main are tested

    // Comment out old tests for now as they are unrelated to FUSE
    /*
    #[test]
    fn test_parse_poc_sample() {
        // ...
    }

    #[test]
    fn test_get_git_tree_entries() {
        // ...
    }
    */
}
