use std::fs;
use std::io;

/// Find lines in `path` that contain `word` (case-insensitive).
/// Returns a Vec of matching lines (as owned Strings).
pub fn find_lines<P: AsRef<std::path::Path>>(path: P, word: &str) -> io::Result<Vec<String>> {
    let text = fs::read_to_string(path)?;
    let needle = word.to_lowercase();
    let matches = text
        .lines()
        .filter(|l| l.to_lowercase().contains(&needle))
        .map(|l| l.to_string())
        .collect();
    Ok(matches)
}
