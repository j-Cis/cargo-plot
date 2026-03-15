
/// MIDDLEWARE: Rozwija klamry we wzorcach (Brace Expansion).
/// Np. "@tui{.rs,/,/**}" -> ["@tui.rs", "@tui/", "@tui/**"]
pub fn expand_braces(pattern: &str) -> Vec<String> {
    // Szukamy pierwszej otwierającej i zamykającej klamry
    if let (Some(start), Some(end)) = (pattern.find('{'), pattern.find('}')) {
        if start < end {
            let prefix = &pattern[..start];
            let suffix = &pattern[end + 1..];
            let options = &pattern[start + 1..end];

            let mut expanded = Vec::new();
            for opt in options.split(',') {
                let new_pattern = format!("{}{}{}", prefix, opt, suffix);
                // Rekurencja! Jeśli wzorzec miał więcej klamer, rozwijamy dalej
                expanded.extend(expand_braces(&new_pattern));
            }
            return expanded;
        }
    }
    // Jeśli nie ma (więcej) klamer, po prostu zwracamy gotowy string
    vec![pattern.to_string()]
}