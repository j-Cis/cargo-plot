use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Rule {
    regex: Regex,
    only_dir: bool,
    // is_generic: bool,
    // raw_clean: String,
}

fn glob_to_regex(pattern: &str) -> Rule {
    let raw = pattern.trim();
    let mut p = raw.replace('\\', "/");
    
    // NAPRAWA: Jeśli użytkownik podał "./folder", ucinamy "./", 
    // ponieważ relatywna ścieżka (rel_path) nigdy tego nie zawiera.
    if p.starts_with("./") {
        p = p[2..].to_string();
    }

    // let is_generic = p == "*" || p == "**/*";
    let only_dir = p.ends_with('/');
    if only_dir {
        p.pop();
    }
    // let raw_clean = p.clone();

    let mut regex_str = regex::escape(&p);
    regex_str = regex_str.replace(r"\*\*", ".*");
    regex_str = regex_str.replace(r"\*", "[^/]*");
    regex_str = regex_str.replace(r"\?", "[^/]");
    regex_str = regex_str.replace(r"\[!", "[^");

    if regex_str.starts_with('/') {
        regex_str = format!("^{}", &regex_str[1..]);
    } else if regex_str.starts_with(".*") {
        regex_str = format!("^{}", regex_str);
    } else {
        regex_str = format!("(?:^|/){}", regex_str);
    }

    if only_dir {
        regex_str.push_str("(?:/.*)?$");
    } else {
        regex_str.push('$');
    }

    let final_regex = format!("(?i){}", regex_str);

    Rule {
        regex: Regex::new(&final_regex).unwrap_or_else(|_| Regex::new("(?i)$.^").unwrap()),
        only_dir,
        // is_generic,
        // raw_clean,
    }
}

/// Element tablicy wejściowej
/// Element tablicy wejściowej
pub struct Task<'a> {
    pub path_location: &'a str,
    pub path_exclude: Vec<&'a str>,
    pub path_include_only: Vec<&'a str>,
    pub filter_files: Vec<&'a str>,
    pub output_type: &'a str, // "dirs", "files", "dirs_and_files"
}

// Implementujemy wartości domyślne, co pozwoli nam pomijać nieużywane pola
impl<'a> Default for Task<'a> {
    fn default() -> Self {
        Self {
            path_location: ".",
            path_exclude: vec![],
            path_include_only: vec![],
            filter_files: vec![],
            output_type: "dirs_and_files",
        }
    }
}

pub fn filespath(tasks: &[Task]) -> Vec<PathBuf> {
    let mut all_results = HashSet::new();

    for task in tasks {
        let root_path = Path::new(task.path_location);
        let canonical_root = fs::canonicalize(root_path).unwrap_or_else(|_| root_path.to_path_buf());

        // Przygotowanie reguł
        let mut exclude_rules = Vec::new();
        for p in &task.path_exclude {
            if !p.trim().is_empty() { exclude_rules.push(glob_to_regex(p)); }
        }

        let mut include_only_rules = Vec::new();
        for p in &task.path_include_only {
            if !p.trim().is_empty() { include_only_rules.push(glob_to_regex(p)); }
        }

        let mut filter_files_rules = Vec::new();
        for p in &task.filter_files {
            if !p.trim().is_empty() { filter_files_rules.push(glob_to_regex(p)); }
        }

        // =========================================================
        // KROK 1: PEŁNY SKAN Z ODRZUCENIEM CAŁYCH GAŁĘZI EXCLUDE
        // =========================================================
        let mut scanned_paths = Vec::new();
        scan_step1(&canonical_root, &canonical_root, &exclude_rules, &mut scanned_paths);

        // =========================================================
        // KROK 2: ZACHOWANIE FOLDERÓW I FILTROWANIE PLIKÓW INCLUDE
        // =========================================================
        for path in scanned_paths {
            let rel_path = path.strip_prefix(&canonical_root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            let path_slash = format!("{}/", rel_path);

            if !include_only_rules.is_empty() {
                let mut matches = false;
                for rule in &include_only_rules {
                    if rule.only_dir {
                        // Jeśli reguła dotyczy TYLKO folderów
                        if path.is_dir() && rule.regex.is_match(&path_slash) {
                            matches = true; 
                            break;
                        }
                    } else {
                        // Jeśli reguła jest uniwersalna (pliki i foldery)
                        if rule.regex.is_match(&rel_path) || rule.regex.is_match(&path_slash) {
                            matches = true; 
                            break;
                        }
                    }
                }
                if !matches {
                    continue; 
                }
            }

            if path.is_dir() {
                // Jeśli tryb to NIE "files" (czyli "dirs" lub "dirs_and_files")
                // to dodajemy folder normalnie.
                if task.output_type != "files" {
                    all_results.insert(path);
                }
            } else {
                // Jeśli tryb to "dirs", całkowicie ignorujemy pliki
                if task.output_type == "dirs" {
                    continue;
                }

                // Pliki sprawdzamy pod kątem filter_files
                let mut is_file_matched = false;
                if filter_files_rules.is_empty() {
                    is_file_matched = true;
                } else {
                    for rule in &filter_files_rules {
                        if rule.only_dir { continue; } 
                        if rule.regex.is_match(&rel_path) {
                            is_file_matched = true;
                            break;
                        }
                    }
                }

                if is_file_matched {
                    all_results.insert(path.clone());
                    
                    // MAGIA DLA "files":
                    // Aby drzewo nie spłaszczyło się do zwykłej listy, musimy dodać foldery nadrzędne
                    // tylko dla TEGO KONKRETNEGO dopasowanego pliku. (Ukrywa to puste foldery!)
                    if task.output_type == "files" {
                        let mut current_parent = path.parent();
                        while let Some(p) = current_parent {
                            all_results.insert(p.to_path_buf());
                            if p == canonical_root { break; }
                            current_parent = p.parent();
                        }
                    }
                }
            }
        }
    }

    let result: Vec<PathBuf> = all_results.into_iter().collect();
    result
}

// Prywatna funkcja pomocnicza do wykonania Kroku 1
fn scan_step1(
    root_path: &Path,
    current_path: &Path,
    exclude_rules: &[Rule],
    scanned_paths: &mut Vec<PathBuf>
) {
    let read_dir = match fs::read_dir(current_path) {
        Ok(rd) => rd,
        Err(_) => return,
    };

    for entry in read_dir.filter_map(|e| e.ok()) {
        let path = entry.path();
        let is_dir = path.is_dir();

        let rel_path = match path.strip_prefix(root_path) {
            Ok(p) => p.to_string_lossy().replace('\\', "/"),
            Err(_) => continue,
        };

        if rel_path.is_empty() {
            continue;
        }
        
        let path_slash = format!("{}/", rel_path);

        // KROK 1.1: Czy wykluczone przez EXCLUDE?
        let mut is_excluded = false;
        for rule in exclude_rules {
            if rule.only_dir && !is_dir {
                continue;
            }
            if rule.regex.is_match(&rel_path) || (is_dir && rule.regex.is_match(&path_slash)) {
                is_excluded = true;
                break;
            }
        }

        // Jeśli folder/plik jest wykluczony - URYWAMY GAŁĄŹ I NIE WCHODZIMY GŁĘBIEJ
        if is_excluded {
            continue; 
        }

        // KROK 1.2: Dodajemy do tymczasowych wyników KROKU 1
        scanned_paths.push(path.clone());

        // KROK 1.3: Jeśli to bezpieczny folder, skanujemy jego zawartość
        if is_dir {
            scan_step1(root_path, &path, exclude_rules, scanned_paths);
        }
    }
}