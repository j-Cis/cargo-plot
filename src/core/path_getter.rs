use std::path::Path;
use walkdir::WalkDir;

/// [POL]: Skanuje katalog rekurencyjnie i zwraca znormalizowane ścieżki (prefix "./", separator "/", suffix "/" dla folderów).
/// [ENG]: Scans the directory recursively and returns normalised paths (prefix "./", separator "/", suffix "/" for directories).
pub fn get_paths<P: AsRef<Path>>(dir_path: P) -> Vec<String> {
    let mut result = Vec::new();
    let root_path = dir_path.as_ref();

    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        // [POL]: Pominięcie katalogu głównego (głębokość 0).
        // [ENG]: Skip the root directory (depth 0).
        if entry.depth() == 0 {
            continue;
        }

        // [POL]: Pominięcie dowiązań symbolicznych i punktów reparse.
        // [ENG]: Skip symbolic links and reparse points.
        if entry.path_is_symlink() {
            continue;
        }

        if let Ok(rel_path) = entry.path().strip_prefix(root_path) {
            // [POL]: Normalizacja separatorów systemowych do formatu uniwersalnego.
            // [ENG]: Normalisation of system separators to a universal format.
            let relative_str = rel_path.to_string_lossy().replace('\\', "/");
            let mut final_path = format!("./{}", relative_str);

            if entry.file_type().is_dir() {
                final_path.push('/');
            }

            result.push(final_path);
        }
    }

    result
}
