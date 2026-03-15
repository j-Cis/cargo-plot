use super::matcher::PathMatcher;
use super::matcher_utils::expand_braces;
use std::collections::HashSet;

/// [POL]: Kontener przechowujący kolekcję silników dopasowujących ścieżki.
/// [ENG]: A container holding a collection of path matching engines.
pub struct PathMatchers {
    matchers: Vec<PathMatcher>,
}

impl PathMatchers {
    /// [POL]: Tworzy nową instancję, kompilując listę wzorców po uprzednim rozwinięciu klamer.
    /// [ENG]: Creates a new instance by compiling a list of patterns after performing brace expansion.
    pub fn new<I, S>(patterns: I, case_sensitive: bool) -> Result<Self, regex::Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut matchers = Vec::new();
        for pat in patterns {
            // [POL]: Przetwarzanie wstępne wzorca (Brace Expansion).
            // [ENG]: Pattern preprocessing (Brace Expansion).
            let expanded_patterns = expand_braces(pat.as_ref());

            for expanded_pat in expanded_patterns {
                matchers.push(PathMatcher::new(&expanded_pat, case_sensitive)?);
            }
        }
        Ok(Self { matchers })
    }

    /// [POL]: Weryfikuje, czy ścieżka pasuje do dowolnego ze skonfigurowanych wzorców (logika OR).
    /// [ENG]: Verifies if the path matches any of the configured patterns (OR logic).
    pub fn is_match(&self, path: &str, env: &HashSet<&str>) -> bool {
        for matcher in &self.matchers {
            if matcher.is_match(path, env) {
                return true;
            }
        }
        false
    }

    /// [POL]: Ewaluuje zbiór ścieżek, wykonując odpowiednie domknięcia dla dopasowanych i niedopasowanych elementów.
    /// [ENG]: Evaluates a set of paths, executing respective closures for matched and mismatched elements.
    pub fn evaluate<I, S, OnMatch, OnMismatch>(
        &self,
        paths: I,
        env: &HashSet<&str>,
        mut on_match: OnMatch,
        mut on_mismatch: OnMismatch,
    ) where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        OnMatch: FnMut(&str),
        OnMismatch: FnMut(&str),
    {
        for path in paths {
            let path_ref = path.as_ref();
            if self.is_match(path_ref, env) {
                on_match(path_ref);
            } else {
                on_mismatch(path_ref);
            }
        }
    }
}
