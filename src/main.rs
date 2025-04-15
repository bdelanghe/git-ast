use std::fs;
use tree_sitter::{Parser, Tree};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the parser
    let mut parser = Parser::new();

    // Set the language to Rust
    let language = tree_sitter_rust::language();
    parser.set_language(&language)?;

    // Read the sample file content
    let file_path = "poc_sample.rs";
    let source_code = fs::read_to_string(file_path)?;

    // Parse the source code
    let tree: Option<Tree> = parser.parse(&source_code, None);

    // Check if parsing was successful and print the AST S-expression
    if let Some(tree) = tree {
        let root_node = tree.root_node();
        println!("Successfully parsed {}", file_path);
        println!("AST S-expression:");
        println!("{}", root_node.to_sexp());
    } else {
        eprintln!("Error parsing {}", file_path);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_parse_poc_sample() {
        let mut parser = Parser::new();
        let language = tree_sitter_rust::language();
        parser.set_language(&language).expect("Error loading Rust grammar");

        let file_path = "poc_sample.rs";
        let source_code = fs::read_to_string(file_path).expect("Could not read poc_sample.rs");

        let tree = parser.parse(&source_code, None);
        assert!(tree.is_some(), "Parsing failed");

        let root_node = tree.unwrap().root_node();
        let actual_sexp = root_node.to_sexp();

        // NOTE: This expected output might be sensitive to formatting and exact tree-sitter version.
        let expected_sexp = "(source_file (function_item name: (identifier) parameters: (parameters (parameter pattern: (identifier) type: (primitive_type)) (parameter pattern: (identifier) type: (primitive_type))) return_type: (primitive_type) body: (block (line_comment) (binary_expression left: (identifier) right: (identifier)))) (function_item name: (identifier) parameters: (parameters) body: (block (let_declaration pattern: (identifier) value: (integer_literal)) (let_declaration pattern: (identifier) value: (integer_literal)) (let_declaration pattern: (identifier) value: (call_expression function: (identifier) arguments: (arguments (identifier) (identifier)))) (expression_statement (macro_invocation macro: (identifier) (token_tree (string_literal (string_content)) (identifier)))))))";

        // Compare without extra whitespace/newlines from the captured output
        let actual_sexp_cleaned = actual_sexp.split_whitespace().collect::<String>();
        let expected_sexp_cleaned = expected_sexp.split_whitespace().collect::<String>();

        assert_eq!(actual_sexp_cleaned, expected_sexp_cleaned);
    }
}
