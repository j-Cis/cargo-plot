// src/lib/fn_path_utils.rs
use std::path::Path;

/// Standaryzuje ścieżkę: zamienia ukośniki na uniksowe i usuwa windowsowy prefiks rozszerzony.
pub fn standardize_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .trim_start_matches("//?/")
        .to_string()
}

/// Formatuje ścieżkę względem podanego katalogu bazowego (np. obecnego katalogu roboczego).
/// Jeśli ścieżka zawiera się w bazowej, zwraca ładny format `./relatywna/sciezka`.
pub fn to_display_path(path: &Path, base_dir: &Path) -> String {
    match path.strip_prefix(base_dir) {
        Ok(rel_path) => format!("./{}", standardize_path(rel_path)),
        Err(_) => standardize_path(path),
    }
}
