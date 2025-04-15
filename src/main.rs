use tree_sitter::{Parser, Tree};
use std::error::Error; // Add Error trait import
// Remove Command/Output, add git2 types
// use std::process::{Command, Output};
use git2::{Repository, Oid}; // Add Repository, Oid

// Remove unused imports for fs and Tree
// use std::fs;
// use git2::Repository;

// Use the language function directly from the crate
// extern "C" { fn tree_sitter_rust() -> Language; }

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

fn main() -> Result<(), Box<dyn Error>> {
    // println!("--- Git Repository PoC ---");
    // // Open the current directory as a Git repository
    // let repo = Repository::open(".")?;
    // println!("Opened Git repository at: {:?}", repo.path());

    // // Example: Get the HEAD commit
    // let head = repo.head()?;
    // let head_commit = head.peel_to_commit()?;
    // println!("HEAD commit: {} - {}", head_commit.id(), head_commit.summary().unwrap_or("[no summary]"));

    // println!("\n--- Tree-sitter PoC ---");

    let source_code = r#"
fn main() {
    println!("Hello, AST!");
}
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
"#;
    println!("Parsing source code...");
    let tree = parse_rust_code(source_code)?; // Call the new function
    let root_node = tree.root_node();
    println!("Parsed AST:\n{}", root_node.to_sexp());

    // Example of using get_git_tree_entries
    println!("\nListing Git tree entries for HEAD:");
    match get_git_tree_entries(".", "HEAD") {
        Ok(entries) => {
            println!("Success! Found {} entries:", entries.len());
            for (name, oid) in entries {
                println!("- {} ({})", name, oid);
            }
        }
        Err(e) => {
            eprintln!("Error getting git tree entries: {}", e);
        }
    }

    // --- Query Logic (Commented Out - needs fixing) ---
    /*
    println!("\nFinding function names:");
    let query_str = "(function_item name: (identifier) @function_name)";
    // Pass language by reference - Need language object here if uncommented
    // let language = tree_sitter_rust::language(); // Re-fetch if needed
    // let query = Query::new(&language, query_str)?;
    let mut cursor = QueryCursor::new();

    // Get the iterator from captures()
    let captures_iterator = cursor.captures(&query, root_node, source_bytes); // Needs query, root_node, source_bytes

    println!("Attempting to iterate over captures...");
    // Simple loop to test iteration without accessing captures yet
    let mut count = 0;
    for (_mat, _capture_index) in captures_iterator { // Compile error here
        // Just print a marker for each item iterated
        // println!("Iterated one item: match={:?}, index={}", _mat, _capture_index);
        count += 1;
    }
    println!("Finished iterating. Found {} capture items.", count);

    // Temporarily comment out the detailed capture processing
    /*
    for (mat, capture_index) in captures_iterator {
        let capture = mat.captures[capture_index];

        if query.capture_names()[capture.index as usize] == "function_name" {
            let captured_node = capture.node;
            let function_name = captured_node.utf8_text(source_bytes)?;
            println!("- Found function name: '{}' at {:?}", function_name, captured_node.range());
        }
    }
    */
    */

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; // Keep this to access parse_rust_code
    // No longer need Parser here directly
    // use tree_sitter::Parser;
    // Remove unused import
    // use std::fs;

    #[test]
    fn test_parse_poc_sample() {
        // Use the same simple source code as in main for testing
        let source_code = r#"
fn main() {
    println!("Hello, AST!");
}
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
"#;
        // Call the new function and assert it succeeds
        let parse_result = parse_rust_code(source_code);
        assert!(parse_result.is_ok(), "Parsing failed: {:?}", parse_result.err());

        // Optional: Further checks on the returned tree if needed
        if let Ok(tree) = parse_result {
             let root_node = tree.root_node();
             assert_eq!(root_node.kind(), "source_file", "Root node kind mismatch");
        }
    }

    #[test]
    fn test_get_git_tree_entries() { // Renamed test
        // Assuming the tests are run within a git repository
        let result = get_git_tree_entries(".", "HEAD");
        assert!(result.is_ok(), "get_git_tree_entries failed: {:?}", result.err());

        // Check if the list of entries is not empty
        if let Ok(entries) = result {
            assert!(!entries.is_empty(), "git tree entries list should not be empty for HEAD");
            // Optionally, check for a specific known file/entry
            // assert!(entries.iter().any(|(name, _)| name == "src/main.rs"), "Did not find src/main.rs in HEAD");
        }
    }

    // We will add the git test function here later
}
