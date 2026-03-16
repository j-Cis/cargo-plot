use super::matcher::PathMatcher;
use super::sort::SortStrategy;
use super::stats::MatchStats;
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
        //for pat in patterns {
        // [POL]: Przetwarzanie wstępne wzorca (Brace Expansion).
        // [ENG]: Pattern preprocessing (Brace Expansion).
        // let expanded_patterns = expand_braces(pat.as_ref());
        for pat in patterns {
            matchers.push(PathMatcher::new(pat.as_ref(), case_sensitive)?);
        }
        //}
        Ok(Self { matchers })
    }

    /// [POL]: Weryfikuje, czy ścieżka pasuje do dowolnego ze skonfigurowanych wzorców (logika OR).
    /// [ENG]: Verifies if the path matches any of the configured patterns (OR logic).
    /// [POL]: Weryfikuje, czy ścieżka spełnia warunki narzucone przez zbiór wzorców (w tym negacje).
    /// [ENG]: Verifies if the path meets the conditions imposed by the pattern set (including negations).
    pub fn is_match(&self, path: &str, env: &HashSet<&str>) -> bool {
        if self.matchers.is_empty() {
            return false;
        }

        let mut has_positive = false;
        let mut matched_positive = false;

        for matcher in &self.matchers {
            if matcher.is_negated {
                // [POL]: Twarde WETO. Dopasowanie negatywne bezwzględnie odrzuca ścieżkę.
                // [ENG]: Hard VETO. A negative match unconditionally rejects the path.
                if matcher.is_match(path, env) {
                    return false;
                }
            } else {
                has_positive = true;
                if !matched_positive && matcher.is_match(path, env) {
                    matched_positive = true;
                }
            }
        }

        // [POL]: Ostateczna decyzja na podstawie zebranych danych.
        // [ENG]: Final decision based on collected data.
        if has_positive {
            matched_positive // Zwykłe dopasowanie pozytywne (OR)
        } else {
            true // Jeśli użytkownik podał TYLKO wzorce z '!' (np. "!tests/"), domyślnie akceptujemy resztę.
        }
    }

    /// [POL]: Ewaluuje zbiór ścieżek, sortuje je i wykonuje odpowiednie domknięcia.
    /// [ENG]: Evaluates a set of paths, sorts them, and executes respective closures.
    // #[allow(clippy::too_many_arguments)]
    pub fn evaluate<I, S, OnMatch, OnMismatch>(
        &self,
        paths: I,
        env: &HashSet<&str>,
        strategy: SortStrategy,
        show_include: bool,
        show_exclude: bool,
        mut on_match: OnMatch,
        mut on_mismatch: OnMismatch,
    ) -> MatchStats
    where
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

        strategy.apply(&mut matched);
        strategy.apply(&mut mismatched);

        let stats = MatchStats {
            matched: matched.len(),
            rejected: mismatched.len(),
            total: matched.len() + mismatched.len(),
            included: matched.iter().map(|s| s.as_ref().to_string()).collect(),
            excluded: mismatched.iter().map(|s| s.as_ref().to_string()).collect(),
        };

        if show_include {
            for path in matched {
                on_match(path.as_ref());
            }
        }

        if show_exclude {
            for path in mismatched {
                on_mismatch(path.as_ref());
            }
        }

        stats
    }
}
