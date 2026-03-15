use super::matcher::PathMatcher;
use super::matcher_utils::{expand_braces, SortStrategy, sort_paths};
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

    /// [POL]: Ewaluuje zbiór ścieżek, sortuje je i wykonuje odpowiednie domknięcia.
    /// [ENG]: Evaluates a set of paths, sorts them, and executes respective closures.
    pub fn evaluate<I, S, OnMatch, OnMismatch>(
        &self,
        paths: I,
        env: &HashSet<&str>,
        strategy: SortStrategy,
        mut on_match: OnMatch,
        mut on_mismatch: OnMismatch,
    ) where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        OnMatch: FnMut(&str),
        OnMismatch: FnMut(&str),
    {
        let mut matched = Vec::new();
        let mut mismatched = Vec::new();

        for path in paths {
            if self.is_match(path.as_ref(), env) {
                matched.push(path);
            } else {
                mismatched.push(path);
            }
        }

        sort_paths(&mut matched, strategy);
        sort_paths(&mut mismatched, strategy);

        for path in matched {
            on_match(path.as_ref());
        }
        for path in mismatched {
            on_mismatch(path.as_ref());
        }
    }
}
