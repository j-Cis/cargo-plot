use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

// use std::fs;
// use std::path::Path;

// [ENG]: Container for scanned paths and their searchable pool.
// [POL]: Kontener na zeskanowane ścieżki i ich przeszukiwalną pulę.
#[derive(Debug)]
pub struct PathStore {
    pub list: Vec<String>,
}
impl PathStore {
    /// [POL]: Skanuje katalog rekurencyjnie i zwraca znormalizowane ścieżki (prefix "./", separator "/", suffix "/" dla folderów).
    /// [ENG]: Scans the directory recursively and returns normalised paths (prefix "./", separator "/", suffix "/" for directories).
    pub fn scan<P: AsRef<Path>>(dir_path: P) -> Self {
        let mut list = Vec::new();
        let entry_path = dir_path.as_ref();

        for entry in WalkDir::new(entry_path).into_iter().filter_map(|e| e.ok()) {
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

            if let Ok(rel_path) = entry.path().strip_prefix(entry_path) {
                // [POL]: Normalizacja separatorów systemowych do formatu uniwersalnego.
                // [ENG]: Normalisation of system separators to a universal format.
                let relative_str = rel_path.to_string_lossy().replace('\\', "/");
                let mut final_path = format!("./{}", relative_str);

                if entry.file_type().is_dir() {
                    final_path.push('/');
                }

                list.push(final_path);
            }
        }

        Self { list }
    }

    // [EN]: Creates a temporary pool of references for the matcher.
    // [PL]: Tworzy tymczasową pulę referencji (paths_pool) dla matchera.
    pub fn get_index(&self) -> HashSet<&str> {
        self.list.iter().map(|s| s.as_str()).collect()
    }
} 
