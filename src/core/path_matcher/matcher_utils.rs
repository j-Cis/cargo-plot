/// [POL]: Wykonuje rozwinięcie klamer we wzorcu (np. {a,b} -> [a, b]). Obsługuje rekurencję.
/// [ENG]: Performs brace expansion in the pattern (e.g. {a,b} -> [a, b]). Supports recursion.
pub fn expand_braces(pattern: &str) -> Vec<String> {
    if let (Some(start), Some(end)) = (pattern.find('{'), pattern.find('}')) {
        if start < end {
            let prefix = &pattern[..start];
            let suffix = &pattern[end + 1..];
            let options = &pattern[start + 1..end];

            let mut expanded = Vec::new();
            for opt in options.split(',') {
                let new_pattern = format!("{}{}{}", prefix, opt, suffix);
                expanded.extend(expand_braces(&new_pattern));
            }
            return expanded;
        }
    }
    vec![pattern.to_string()]
}
