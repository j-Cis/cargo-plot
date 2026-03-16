/// [POL]: Kontekst wzorców - przechowuje oryginalne wzorce użytkownika oraz ich rozwiniętą formę.
/// [ENG]: Pattern context - stores original user patterns and their tok form.
#[derive(Debug, Clone)]
pub struct PatternContext {
    pub raw: Vec<String>,
    pub tok: Vec<String>,
}

impl PatternContext {
    /// [POL]: Tworzy nowy kontekst, automatycznie rozwijając klamry w podanych wzorcach.
    /// [ENG]: Creates a new context, automatically expanding braces in the provided patterns.
    pub fn new<I, S>(patterns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut raw = Vec::new();
        let mut tok = Vec::new();

        for pat in patterns {
            let pat_str = pat.as_ref();
            raw.push(pat_str.to_string());
            tok.extend(Self::expand_braces(pat_str));
        }

        Self { raw, tok }
    }

    /// [POL]: Prywatna metoda: rozwija klamry we wzorcu (np. {a,b} -> [a, b]). Obsługuje rekurencję.
    /// [ENG]: Private method: expands braces in a pattern (e.g. {a,b} -> [a, b]). Supports recursion.
    fn expand_braces(pattern: &str) -> Vec<String> {
        if let (Some(start), Some(end)) = (pattern.find('{'), pattern.find('}')) {
            if start < end {
                let prefix = &pattern[..start];
                let suffix = &pattern[end + 1..];
                let options = &pattern[start + 1..end];

                let mut tok = Vec::new();
                for opt in options.split(',') {
                    let new_pattern = format!("{}{}{}", prefix, opt, suffix);
                    tok.extend(Self::expand_braces(&new_pattern));
                }
                return tok;
            }
        }
        vec![pattern.to_string()]
    }
}