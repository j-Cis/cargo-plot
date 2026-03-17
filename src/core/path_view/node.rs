use crate::core::path_matcher::SortStrategy;
use std::path::PathBuf;

/// [POL]: Reprezentuje pojedynczy węzeł w drzewie systemu plików.
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub icon: String,
    pub weight_str: String,
    pub weight_bytes: u64,
    pub children: Vec<FileNode>,
}

impl FileNode {
    /// [POL]: Sortuje listę węzłów w miejscu zgodnie z wybraną strategią.
    pub fn sort_slice(nodes: &mut [FileNode], strategy: SortStrategy) {
        if strategy == SortStrategy::None {
            return;
        }

        nodes.sort_by(|a, b| {
            let a_is_dir = a.is_dir;
            let b_is_dir = b.is_dir;

            // Klucz Merge: "interfaces.rs" -> "interfaces", "interfaces/" -> "interfaces"
            let a_merge = Self::get_merge_key(&a.name);
            let b_merge = Self::get_merge_key(&b.name);

            match strategy {
                // 1. CZYSTE ALFANUMERYCZNE
                SortStrategy::Az => a.name.cmp(&b.name),
                SortStrategy::Za => b.name.cmp(&a.name),

                // 2. PLIKI PIERWSZE (Globalnie)
                SortStrategy::AzFileFirst => (a_is_dir, &a.name).cmp(&(b_is_dir, &b.name)),
                SortStrategy::ZaFileFirst => (a_is_dir, &b.name).cmp(&(b_is_dir, &a.name)),

                // 3. KATALOGI PIERWSZE (Globalnie)
                SortStrategy::AzDirFirst => (!a_is_dir, &a.name).cmp(&(!b_is_dir, &b.name)),
                SortStrategy::ZaDirFirst => (!a_is_dir, &b.name).cmp(&(!b_is_dir, &a.name)),

                // 4. PLIKI PIERWSZE + MERGE (Grupowanie modułów)
                SortStrategy::AzFileFirstMerge => {
                    (a_merge, a_is_dir, &a.name).cmp(&(b_merge, b_is_dir, &b.name))
                }
                SortStrategy::ZaFileFirstMerge => {
                    (b_merge, a_is_dir, &b.name).cmp(&(a_merge, b_is_dir, &a.name))
                }

                // 5. KATALOGI PIERWSZE + MERGE (Zgodnie z Twoją notatką: fallback do DirFirst)
                SortStrategy::AzDirFirstMerge => (!a_is_dir, &a.name).cmp(&(!b_is_dir, &b.name)),
                SortStrategy::ZaDirFirstMerge => (!a_is_dir, &b.name).cmp(&(!b_is_dir, &a.name)),

                _ => a.name.cmp(&b.name),
            }
        });
    }

    /// [POL]: Wyciąga rdzeń nazwy do grupowania (np. "main.rs" -> "main").
    fn get_merge_key(name: &str) -> &str {
        if let Some(idx) = name.rfind('.')
            && idx > 0
        {
            return &name[..idx];
        }
        name
    }
}
