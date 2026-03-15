use std::path::Path;
use walkdir::WalkDir;

/// Skanuje podany katalog i zwraca listę ścieżek znormalizowanych do formatu:
/// - zaczynają się od "./"
/// - mają ukośniki "/"
/// - foldery kończą się na "/"
/// - ignoruje symlinki/junctions (!ReparsePoint)
pub fn get_paths<P: AsRef<Path>>(dir_path: P) -> Vec<String> {
    let mut result = Vec::new();
    let root_path = dir_path.as_ref();

    // WalkDir::new domyślnie iteruje rekurencyjnie (odpowiednik -Recurse)
    // filter_map(|e| e.ok()) bezpiecznie ignoruje błędy braku uprawnień (Access Denied)
    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        
        // Pomijamy sam katalog główny (depth == 0), interesuje nas tylko zawartość
        if entry.depth() == 0 {
            continue;
        }

        // Pomijamy symlinki i punkty reparse (odpowiednik !ReparsePoint)
        if entry.path_is_symlink() {
            continue;
        }

        // Ucinamy bezwzględną część ścieżki (zostaje nam np. "src\main.rs")
        if let Ok(rel_path) = entry.path().strip_prefix(root_path) {
            
            // Konwersja ścieżki i ujednolicenie ukośników (Windows '\' -> '/')
            let relative_str = rel_path.to_string_lossy().replace('\\', "/");
            
            // Doklejamy wymagany prefix "./"
            let mut final_path = format!("./{}", relative_str);

            // Jeśli to folder (odpowiednik PSIsContainer), dodajemy "/" na końcu
            if entry.file_type().is_dir() {
                final_path.push('/');
            }

            result.push(final_path);
        }
    }

    result
}