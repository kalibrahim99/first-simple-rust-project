use first_simple_rust_project::find_lines;

#[test]
fn finds_rust_lines_in_index() {
    // The test uses the workspace root path; index.html is at the repository root.
    let lines = find_lines("index.html", "rust").expect("failed to read index.html");
    assert!(!lines.is_empty(), "expected at least one line containing 'rust'");
    for line in lines {
        assert!(line.to_lowercase().contains("rust"), "line did not contain 'rust': {}", line);
    }
}
