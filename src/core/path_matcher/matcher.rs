use super::sort::SortStrategy;
use super::stats::MatchStats;
use regex::Regex;
use std::collections::HashSet;

/// [POL]: Struktura odpowiedzialna za dopasowanie pojedynczego wzorca z uwzględnieniem zależności strukturalnych.
/// [ENG]: Structure responsible for matching a single pattern considering structural dependencies.
pub struct PathMatcher {
    regex: Regex,
    targets_file: bool,
    requires_sibling: bool, // [POL]: Flaga @ (para plik-folder)                 | [ENG]: Flag @ (file-directory pair)
    requires_orphan: bool, // [POL]: Flaga $ (jednostronna relacja)             | [ENG]: Flag $ (one-way relation)
    is_deep: bool, // [POL]: Flaga + (rekurencyjne zacienianie)         | [ENG]: Flag + (recursive shadowing)
    base_name: String, // [POL]: Nazwa bazowa modułu do weryfikacji relacji | [ENG]: Base name of the module for relation verification
    pub is_negated: bool, // [POL]: Flaga negacji (!).                         | [ENG]: Negation flag (!).
}

impl PathMatcher {
    pub fn new(pattern: &str, case_sensitive: bool) -> Result<Self, regex::Error> {
        // [POL]: Kompiluje wzorzec tekstowy do wyrażenia regularnego, ekstrahując flagi sterujące.
        // [ENG]: Compiles a text pattern into a regular expression, extracting control flags.

        // [POL]: Detekcja negacji. Jeśli obecny '!', oznaczamy i obcinamy go do dalszej analizy.
        // [ENG]: Negation detection. If '!' is present, mark it and trim it for further analysis.
        let is_negated = pattern.starts_with('!');
        let actual_pattern = if is_negated { &pattern[1..] } else { pattern };

        let is_deep = actual_pattern.ends_with('+');
        let requires_sibling = actual_pattern.contains('@');
        let requires_orphan = actual_pattern.contains('$');
        let clean_pattern_str = actual_pattern
            .replace('@', "")
            .replace('$', "")
            .replace('+', "");

        let base_name = clean_pattern_str
            .trim_end_matches('/')
            .trim_end_matches("**")
            .split('/')
            .last()
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

        // [POL]: KLASYFIKACJA CELU DOPASOWANIA. Zmienna określa, czy wzorzec odnosi się wyłącznie do plików.
        // Brak ukośnika '/' lub sekwencji '**' na końcu oznacza restrykcję do obiektów niebędących katalogami.
        // [ENG]: MATCH TARGET CLASSIFICATION. This variable determines if the pattern is restricted to files only.
        // The absence of a trailing slash '/' or the '**' sequence implies a restriction to non-directory objects.
        // // let targets_file = !pattern.ends_with('/') && !pattern.ends_with("**");
        // [POL]: ANALIZA CIĄGU ZNORMALIZOWANEGO. Weryfikacja odbywa się na zmiennej 'p' (wzorzec bazowy),
        // a nie na surowym 'pattern'. Gwarantuje to, że flagi sterujące (np. '@', '$', '+') nie zostaną
        // błędnie zinterpretowane jako część ścieżki, co zafałszowałoby wykrycie intencji wzorca.
        // [ENG]: NORMALISED STRING ANALYSIS. Verification is performed on variable 'p' (base pattern)
        // instead of the raw 'pattern'. This ensures that control flags (e.g. '@', '$', '+') are not
        // misinterpreted as path components, which would compromise the detection of the intended target type.
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
                // '/' => re.push('/'),
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
                        options.split(',').map(|s| regex::escape(s)).collect();
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
            for path in &matched {
                on_match(path.as_ref());
            }
        }

        if show_exclude {
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
