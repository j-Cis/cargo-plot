use super::sort::SortStrategy;
use super::stats::{MatchStats,ResultSet, ShowMode};
use regex::Regex;
use std::collections::HashSet;

// ==============================================================================
// ⚡ POJEDYNCZY WZORZEC (PathMatcher)
// ==============================================================================

/// [POL]: Struktura odpowiedzialna za dopasowanie pojedynczego wzorca z uwzględnieniem zależności strukturalnych.
/// [ENG]: Structure responsible for matching a single pattern considering structural dependencies.
pub struct PathMatcher {
    regex: Regex,
    targets_file: bool,
    // [POL]: Flaga @ (para plik-folder) 
    // [ENG]: Flag @ (file-directory pair)
    requires_sibling: bool, 
    // [POL]: Flaga $ (jednostronna relacja)
    // [ENG]: Flag $ (one-way relation)
    requires_orphan: bool,  
    // [POL]: Flaga + (rekurencyjne zacienianie) 
    // [ENG]: Flag + (recursive shadowing)
    is_deep: bool,     
    // [POL]: Nazwa bazowa modułu do weryfikacji relacji     
    // [ENG]: Base name of the module for relation verification
    base_name: String,      
    // [POL]: Flaga negacji (!).
    // [ENG]: Negation flag (!).
    pub is_negated: bool,   
}

impl PathMatcher {
    pub fn new(pattern: &str, case_sensitive: bool) -> Result<Self, regex::Error> {
        let is_negated = pattern.starts_with('!');
        let actual_pattern = if is_negated { &pattern[1..] } else { pattern };

        let is_deep = actual_pattern.ends_with('+');
        let requires_sibling = actual_pattern.contains('@');
        let requires_orphan = actual_pattern.contains('$');
        let clean_pattern_str = actual_pattern.replace(['@', '$', '+'], "");

        let base_name = clean_pattern_str
            .trim_end_matches('/')
            .trim_end_matches("**")
            .split('/')
            .next_back()
            .unwrap_or("")
            .split('.')
            .next()
            .unwrap_or("")
            .to_string();

        let mut re = String::new();

        if !case_sensitive {
            re.push_str("(?i)");
        }

        let mut is_anchored = false;
        let mut p = clean_pattern_str.as_str();

        let targets_file = !p.ends_with('/') && !p.ends_with("**");

        if p.starts_with("./") {
            is_anchored = true;
            p = &p[2..];
        } else if p.starts_with("**/") {
            is_anchored = true;
        }

        if is_anchored {
            re.push('^');
        } else {
            re.push_str("(?:^|/)");
        }

        let chars: Vec<char> = p.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '\\' => {
                    if i + 1 < chars.len() {
                        i += 1;
                        re.push_str(&regex::escape(&chars[i].to_string()));
                    }
                }
                '.' => re.push_str("\\."),
                '/' => {
                    if is_deep && i == chars.len() - 1 {
                        // [POL]: Pominięcie końcowego ukośnika dla flagi '+'.
                        // [ENG]: Omission of trailing slash for the '+' flag.
                    } else {
                        re.push('/');
                    }
                }
                '*' => {
                    if i + 1 < chars.len() && chars[i + 1] == '*' {
                        if i + 2 < chars.len() && chars[i + 2] == '/' {
                            re.push_str("(?:[^/]+/)*");
                            i += 2;
                        } else {
                            re.push_str(".+");
                            i += 1;
                        }
                    } else {
                        re.push_str("[^/]*");
                    }
                }
                '?' => re.push_str("[^/]"),
                '{' => {
                    let mut options = String::new();
                    i += 1;
                    while i < chars.len() && chars[i] != '}' {
                        options.push(chars[i]);
                        i += 1;
                    }
                    let escaped: Vec<String> =
                        options.split(',').map(regex::escape).collect();
                    re.push_str(&format!("(?:{})", escaped.join("|")));
                }
                '[' => {
                    re.push('[');
                    if i + 1 < chars.len() && chars[i + 1] == '!' {
                        re.push('^');
                        i += 1;
                    }
                }
                ']' | '-' | '^' => re.push(chars[i]),
                c => re.push_str(&regex::escape(&c.to_string())),
            }
            i += 1;
        }

        if is_deep {
            re.push_str("(?:/.*)?$");
        } else {
            re.push('$');
        }

        Ok(Self {
            regex: Regex::new(&re)?,
            targets_file,
            requires_sibling,
            requires_orphan,
            is_deep,
            base_name,
            is_negated,
        })
    }

    /// [POL]: Sprawdza dopasowanie ścieżki, uwzględniając relacje rodzeństwa w strukturze plików.
    /// [ENG]: Validates path matching, considering sibling relations within the file structure.
    pub fn is_match(&self, path: &str, env: &HashSet<&str>) -> bool {
        if self.targets_file && path.ends_with('/') {
            return false;
        }

        let clean_path = path.strip_prefix("./").unwrap_or(path);

        if !self.regex.is_match(clean_path) {
            return false;
        }

        // [POL]: Relacja rodzeństwa (@) lub sieroty ($) dla plików.
        // [ENG]: Sibling relation (@) or orphan relation ($) for files.
        if (self.requires_sibling || self.requires_orphan) && !path.ends_with('/') {
            if self.is_deep && self.requires_sibling {
                if !self.check_authorized_root(path, env) {
                    return false;
                }
                return true;
            }
            let mut components: Vec<&str> = path.split('/').collect();
            if let Some(file_name) = components.pop() {
                let parent_dir = components.join("/");
                let core_name = file_name.split('.').next().unwrap_or("");
                let expected_folder = if parent_dir.is_empty() {
                    format!("{}/", core_name)
                } else {
                    format!("{}/{}/", parent_dir, core_name)
                };

                if !env.contains(expected_folder.as_str()) {
                    return false;
                }
            }
        }

        // [POL]: Dodatkowa weryfikacja rodzeństwa (@) dla katalogów.
        // [ENG]: Additional sibling verification (@) for directories.
        if self.requires_sibling && path.ends_with('/') {
            if self.is_deep {
                if !self.check_authorized_root(path, env) {
                    return false;
                }
            } else {
                let dir_no_slash = path.trim_end_matches('/');
                let has_file_sibling = env.iter().any(|&p| {
                    p.starts_with(dir_no_slash)
                        && p[dir_no_slash.len()..].starts_with('.')
                        && !p.ends_with('/')
                });

                if !has_file_sibling {
                    return false;
                }
            }
        }

        true
    }

    /// [POL]: Ewaluuje kolekcję ścieżek, sortuje wyniki i wywołuje odpowiednie akcje.
    /// [ENG]: Evaluates a path collection, sorts the results, and triggers respective actions.
    // #[allow(clippy::too_many_arguments)]
    pub fn evaluate<I, S, OnMatch, OnMismatch>(
        &self,
        paths: I,
        env: &HashSet<&str>,
        strategy: SortStrategy,
        show_mode: ShowMode,
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
            m_size_matched: matched.len(),
            x_size_mismatched: mismatched.len(),
            total: matched.len() + mismatched.len(),
            m_matched: ResultSet {
                paths: matched.iter().map(|s| s.as_ref().to_string()).collect(),
                tree: None,
                list: None,
                grid: None, 
            },
            x_mismatched: ResultSet {
                paths: mismatched.iter().map(|s| s.as_ref().to_string()).collect(),
                tree: None,
                list: None,
                grid: None,
            },
        };

        if show_mode == ShowMode::Include || show_mode == ShowMode::Context {
            for path in &matched {
                on_match(path.as_ref());
            }
        }

        if show_mode == ShowMode::Exclude || show_mode == ShowMode::Context {
            for path in &mismatched {
                on_mismatch(path.as_ref());
            }
        }

        stats
    }

    /// [POL]: Weryfikuje autoryzację korzenia modułu w relacji plik-folder dla trybu 'deep'.
    /// [ENG]: Verifies module root authorisation in the file-directory relation for 'deep' mode.
    fn check_authorized_root(&self, path: &str, env: &HashSet<&str>) -> bool {
        let clean = path.strip_prefix("./").unwrap_or(path);
        let components: Vec<&str> = clean.split('/').collect();

        for i in 0..components.len() {
            let comp_core = components[i].split('.').next().unwrap_or("");

            if comp_core == self.base_name {
                let base_dir = if i == 0 {
                    self.base_name.clone()
                } else {
                    format!("{}/{}", components[0..i].join("/"), self.base_name)
                };

                let full_base_dir = if path.starts_with("./") {
                    format!("./{}", base_dir)
                } else {
                    base_dir
                };
                let dir_path = format!("{}/", full_base_dir);

                let has_dir = env.contains(dir_path.as_str());
                let has_file = env.iter().any(|&p| {
                    p.starts_with(&full_base_dir)
                        && p[full_base_dir.len()..].starts_with('.')
                        && !p.ends_with('/')
                });

                if has_dir && has_file {
                    return true;
                }
            }
        }
        false
    }
}


// ==============================================================================
// ⚡ KONTENER WIELU WZORCÓW (PathMatchers)
// ==============================================================================

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
            matchers.push(PathMatcher::new(pat.as_ref(), case_sensitive)?);
        }
        Ok(Self { matchers })
    }

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
            matched_positive
        } else {
            true
        }
    }

    /// [POL]: Ewaluuje zbiór ścieżek, sortuje je i wykonuje odpowiednie domknięcia.
    /// [ENG]: Evaluates a set of paths, sorts them, and executes respective closures.
    pub fn evaluate<I, S, OnMatch, OnMismatch>(
        &self,
        paths: I,
        env: &HashSet<&str>,
        strategy: SortStrategy,
        show_mode: ShowMode,
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
            m_size_matched: matched.len(),
            x_size_mismatched: mismatched.len(),
            total: matched.len() + mismatched.len(),
            m_matched: ResultSet {
                paths: matched.iter().map(|s| s.as_ref().to_string()).collect(),
                tree: None,
                list: None,
                grid: None,  
            },
            x_mismatched: ResultSet {
                paths: mismatched.iter().map(|s| s.as_ref().to_string()).collect(),
                tree: None,
                list: None,
                grid: None,  
            },
        };

        if show_mode == ShowMode::Include || show_mode == ShowMode::Context {
            for path in matched {
                on_match(path.as_ref());
            }
        }

        if show_mode == ShowMode::Exclude || show_mode == ShowMode::Context {
            for path in mismatched {
                on_mismatch(path.as_ref());
            }
        }

        stats
    }
}