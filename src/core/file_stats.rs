// use std::fs;
use std::path::{Path, PathBuf};
pub mod weight;

use self::weight::get_path_weight;

/// [PL]: Struktura przechowująca metadane (statystyki) pliku lub folderu.
#[derive(Debug, Clone)]
pub struct FileStats {
    pub path: String,      // Oryginalna ścieżka relatywna (np. "src/main.rs")
    pub absolute: PathBuf, // Pełna ścieżka absolutna na dysku
    pub weight_bytes: u64, // Rozmiar w bajtach

                           // ⚡ Miejsce na przyszłe parametry:
                           // pub created_at: Option<std::time::SystemTime>,
                           // pub modified_at: Option<std::time::SystemTime>,
}

impl FileStats {
    /// [PL]: Pobiera statystyki pliku bezpośrednio z dysku.
    pub fn fetch(path: &str, entry_absolute: &str) -> Self {
        let absolute = Path::new(entry_absolute).join(path);

        let weight_bytes = get_path_weight(&absolute, true);
        // let weight_bytes = fs::metadata(&absolute)
        //     .map(|m| m.len())
        //     .unwrap_or(0);

        Self {
            path: path.to_string(),
            absolute,
            weight_bytes,
        }
    }
}
