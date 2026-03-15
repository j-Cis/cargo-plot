use regex::Regex;
use std::collections::HashSet;
use super::path_matcher_utils::expand_braces;

pub struct PathMatcher {
    regex: Regex,
    targets_file: bool, 
    requires_sibling: bool, // @ : Para (Plik <=> Folder)
    requires_orphan: bool,  // $ : Jednostronne (Plik => Folder)
}

impl PathMatcher {
    pub fn new(pattern: &str, case_sensitive: bool) -> Result<Self, regex::Error> {
        // Detekcja flag i usuwanie ich ze wzorca (replace nie psuje reszty stringa)
        let requires_sibling = pattern.contains('@');
        let requires_orphan = pattern.contains('$');
        let clean_pattern_str = pattern.replace('@', "").replace('$', "");

        let mut re = String::new();

        // 🔥: Wstrzykujemy flagę niewrażliwości na wielkość liter
        if !case_sensitive {
            re.push_str("(?i)"); 
        }
        
        let mut is_anchored = false;

        // Używamy wyczyszczonego wzorca (bez '@'), żeby nie psuć parsera z tabeli!
        let mut p = clean_pattern_str.as_str();

        // BARIERA LOGICZNA: Jeśli wzorzec nie kończy się na ukośnik ani na '**',
        // to według Twojej tabeli celuje WYŁĄCZNIE w pliki.
        // let targets_file = !pattern.ends_with('/') && !pattern.ends_with("**");
        // BARDZO WAŻNE: targets_file sprawdzamy na 'p' (czystym wzorcu)
        let targets_file = !p.ends_with('/') && !p.ends_with("**");

        // 1. ZASADY KOTWICZENIA
        if p.starts_with("./") {
            is_anchored = true;
            p = &p[2..]; // Ucinamy ./
        } else if p.starts_with("**/") {
            is_anchored = true;
        }

        if is_anchored {
            re.push('^');
        } else {
            re.push_str("(?:^|/)");
        }

        // 2. PARSOWANIE ZNAKÓW
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
                '/' => re.push('/'),
                '*' => {
                    if i + 1 < chars.len() && chars[i + 1] == '*' {
                        if i + 2 < chars.len() && chars[i + 2] == '/' {
                            // Wzorzec: **/
                            re.push_str("(?:[^/]+/)*");
                            i += 2;
                        } else {
                            // Wzorzec: **
                            // POPRAWKA: Zamiana z .* na .+ (wymaga minimum 1 znaku zawartości!)
                            re.push_str(".+");
                            i += 1;
                        }
                    } else {
                        // Zwykła gwiazdka *
                        re.push_str("[^/]*");
                    }
                }
                '?' => re.push_str("[^/]"),
                '{' => {
                    // UWAGA: Ten blok '{' działa tylko jeśli klamry NIE ZOSTAŁY ROZWINIĘTE
                    // przez middleware (bo np. nie miały przecinka). Inaczej parser nigdy tu nie wejdzie.
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

        re.push('$');

        Ok(Self {
            regex: Regex::new(&re)?,
            targets_file,
            requires_sibling,
            requires_orphan,
        })
    }

    // ⚡: is_match przyjmuje środowisko do sprawdzania rodzeństwa
    pub fn is_match(&self, path: &str, env: &HashSet<&str>) -> bool {
        // TWARDA ZASADA: Jeśli wzorzec to plik, to natychmiastowo
        // odrzucamy każdą ścieżkę testową, która kończy się na ukośnik!
        if self.targets_file && path.ends_with('/') {
            return false;
        }

        let clean_path = path.strip_prefix("./").unwrap_or(path);

        // Jeśli Regex odrzuca ścieżkę, nie ma o czym gadać
        if !self.regex.is_match(clean_path) {
            return false;
        }

        // ⚡: Regex powiedział "TAK", ale wzorzec miał "@" lub "$", więc sprawdzamy brata!
        // --- ZASADA RODZEŃSTWA (@) LUB SIEROT ($) dla PLIKÓW ---
        // Obie zasady wymagają od pliku posiadania folderu-brata
        if (self.requires_sibling || self.requires_orphan) && !path.ends_with('/') {
            // Z "./interfaces/tui.rs" robimy folder nadrzędny i nazwę pliku
            let mut components: Vec<&str> = path.split('/').collect();
            if let Some(file_name) = components.pop() {
                let parent_dir = components.join("/"); // np. "./interfaces"
                
                // Z "tui.rs" bierzemy "tui"
                let core_name = file_name.split('.').next().unwrap_or(""); 
                
                // Budujemy docelową nazwę folderu: "./interfaces/tui/"
                let expected_folder = if parent_dir.is_empty() {
                    format!("{}/", core_name)
                } else {
                    format!("{}/{}/", parent_dir, core_name)
                };

                // Sprawdzamy, czy takie rodzeństwo istnieje na dysku
                if !env.contains(expected_folder.as_str()) {
                    return false; // Plik nie ma folderu -> Odrzucamy dla @ i $
                }
            }
        }

        // --- DODATKOWA ZASADA RODZEŃSTWA (@) DLA FOLDERÓW ---
        // Tylko @ wymaga, aby folder też miał plik-indeks
        if self.requires_sibling && path.ends_with('/') {
            let dir_no_slash = path.trim_end_matches('/');
            
            // Szukamy w środowisku pliku, który autoryzuje ten folder
            let has_file_sibling = env.iter().any(|&p| {
                p.starts_with(dir_no_slash) && 
                p[dir_no_slash.len()..].starts_with('.') && 
                !p.ends_with('/')
            });

            if !has_file_sibling {
                return false; // Folder nie ma pliku -> Odrzucamy TYLKO dla @
            }
        }

        true
    }

    /// Ewaluuje kolekcję ścieżek, wywołując odpowiedni callback dla dopasowania i braku dopasowania.
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

pub struct PathMatchers {
    matchers: Vec<PathMatcher>,
}

impl PathMatchers {
    /// Przyjmuje kolekcję wzorców i kompiluje je wszystkie
    pub fn new<I, S>(patterns: I, case_sensitive: bool) -> Result<Self, regex::Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut matchers = Vec::new();
        for pat in patterns {
            // ⚡ TUTAJ JEST GŁÓWNA ZMIANA:
            // Najpierw przepuszczamy wzorzec przez nasz nowy Middleware:
            let expanded_patterns = expand_braces(pat.as_ref());
            
            // Dopiero potem kompilujemy każdą z wygenerowanych wersji:
            for expanded_pat in expanded_patterns {
                matchers.push(PathMatcher::new(&expanded_pat, case_sensitive)?);
            }
        }
        Ok(Self { matchers })
    }

    /// Zwraca true, jeśli ścieżka pasuje do JAKIEGOKOLWIEK wzorca (logiczne OR)
    pub fn is_match(&self, path: &str, env: &HashSet<&str>) -> bool {
        for matcher in &self.matchers {
            // Przekazujemy `env` w dół do pojedynczego matchera
            if matcher.is_match(path, env) {
                return true;
            }
        }
        false
    }

    /// Ewaluuje kolekcję ścieżek względem wszystkich wzorców (logiczne OR), wywołując callbacki.
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

