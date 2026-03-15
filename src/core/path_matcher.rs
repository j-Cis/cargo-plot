use regex::Regex;
use std::collections::HashSet;

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

pub struct PathMatcher {
    regex: Regex,
    targets_file: bool, 
    requires_sibling: bool, // ⚡ : Flaga świadomości kontekstu
}

impl PathMatcher {
    pub fn new(pattern: &str, case_sensitive: bool) -> Result<Self, regex::Error> {
        // ⚡: Wykrywamy '@' i od razu usuwamy go ze wzorca!
        let requires_sibling = pattern.contains('@');
        let clean_pattern_str = pattern.replace('@', "");

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
        let targets_file = !pattern.ends_with('/') && !pattern.ends_with("**");

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

        // ⚡: Regex powiedział "TAK", ale wzorzec miał "@", więc sprawdzamy brata!
        if self.requires_sibling && !path.ends_with('/') {
            // Z "./interfaces/tui.rs" robimy folder nadrzędny i nazwę pliku
            let mut components: Vec<&str> = path.split('/').collect();
            if let Some(file_name) = components.pop() {
                let parent_dir = components.join("/"); // np. "./interfaces"
                
                // Z "tui.rs" bierzemy "tui"
                let core_name = file_name.split('.').next().unwrap_or(""); 
                
                // Budujemy docelową nazwę folderu: "./interfaces/tui/"
                let expected_sibling = format!("{}/{}/", parent_dir, core_name);

                // Sprawdzamy, czy takie rodzeństwo istnieje na dysku
                if !env.contains(expected_sibling.as_str()) {
                    return false; // Sierota! Odrzucamy.
                }
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


/// Zwraca odpowiednią ikonę (emoji) dla podanej ścieżki,
/// rozpoznając foldery (końcówka '/') oraz elementy ukryte (kropka na początku nazwy).
pub fn get_icon_for_path(path: &str) -> &'static str {
    let is_dir = path.ends_with('/');
    
    // Wyciągamy samą nazwę pliku/folderu:
    // 1. Usuwamy ew. ukośnik z końca (żeby folder nie zwrócił pustego stringa)
    // 2. Dzielimy przez ukośniki i bierzemy ostatni element
    let nazwa = path.trim_end_matches('/').split('/').last().unwrap_or("");
    let is_hidden = nazwa.starts_with('.');

    // Dobieramy odpowiednią ikonę na podstawie dwóch cech
    match (is_dir, is_hidden) {
        (true, false) => "📁",  // Zwykły folder
        (true, true)  => "🗃️",  // Ukryty folder (z kropką)
        (false, false)=> "📄",  // Zwykły plik
        (false, true) => "⚙️ ", // Ukryty plik (konfiguracyjny z kropką)
    }
}