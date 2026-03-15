use regex::Regex;

pub struct PathMatcher {
    regex: Regex,
    targets_file: bool, 
}

impl PathMatcher {
    pub fn new(pattern: &str, case_sensitive: bool) -> Result<Self, regex::Error> {
        let mut re = String::new();

        // 🔥 MAGIA: Wstrzykujemy flagę niewrażliwości na wielkość liter
        if !case_sensitive {
            re.push_str("(?i)"); 
        }
        
        let mut is_anchored = false;
        let mut p = pattern;

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
        })
    }

    pub fn is_match(&self, path: &str) -> bool {
        // TWARDA ZASADA: Jeśli wzorzec to plik, to natychmiastowo
        // odrzucamy każdą ścieżkę testową, która kończy się na ukośnik!
        if self.targets_file && path.ends_with('/') {
            return false;
        }

        let clean_path = path.strip_prefix("./").unwrap_or(path);
        self.regex.is_match(clean_path)
    }

    /// Ewaluuje kolekcję ścieżek, wywołując odpowiedni callback dla dopasowania i braku dopasowania.
    pub fn evaluate<I, S, OnMatch, OnMismatch>(
        &self,
        paths: I,
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
            if self.is_match(path_ref) {
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
            // Używamy "świętej" funkcji do skompilowania każdego z nich
            matchers.push(PathMatcher::new(pat.as_ref(), case_sensitive)?);
        }
        Ok(Self { matchers })
    }

    /// Zwraca true, jeśli ścieżka pasuje do JAKIEGOKOLWIEK wzorca (logiczne OR)
    pub fn is_match(&self, path: &str) -> bool {
        for matcher in &self.matchers {
            if matcher.is_match(path) {
                return true;
            }
        }
        false
    }

    /// Ewaluuje kolekcję ścieżek względem wszystkich wzorców (logiczne OR), wywołując callbacki.
    pub fn evaluate<I, S, OnMatch, OnMismatch>(
        &self,
        paths: I,
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
            if self.is_match(path_ref) {
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