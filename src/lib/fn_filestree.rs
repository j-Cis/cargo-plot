// Zaktualizowany Plik-004: src/lib/fn_filestree.rs
use crate::fn_pathtype::{DIR_ICON, get_file_type};
use crate::fn_weight::{WeightConfig, format_weight, get_path_weight};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Struktura węzła drzewa
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub icon: String,
    pub weight_str: String, // Nowe pole na sformatowaną wagę [qq xxxxx]
    pub weight_bytes: u64,  // Surowa waga do obliczeń sumarycznych
    pub children: Vec<FileNode>,
}

/// Helper do sortowania węzłów zgodnie z wybraną metodą
fn sort_nodes(nodes: &mut [FileNode], sort_method: &str) {
    match sort_method {
        "files-first" => nodes.sort_by(|a, b| {
            if a.is_dir == b.is_dir {
                a.name.cmp(&b.name)
            } else if !a.is_dir {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }),
        "dirs-first" => nodes.sort_by(|a, b| {
            if a.is_dir == b.is_dir {
                a.name.cmp(&b.name)
            } else if a.is_dir {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }),
        _ => nodes.sort_by(|a, b| a.name.cmp(&b.name)),
    }
}

/// Funkcja formatująca - buduje drzewo i przypisuje ikony oraz wagi
pub fn filestree(
    paths: Vec<PathBuf>,
    sort_method: &str,
    weight_cfg: &WeightConfig, // NOWY ARGUMENT
) -> Vec<FileNode> {
    let mut tree_map: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    for p in &paths {
        let parent = p
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("/"));
        tree_map.entry(parent).or_default().push(p.clone());
    }

    fn build_node(
        path: &PathBuf,
        paths: &BTreeMap<PathBuf, Vec<PathBuf>>,
        sort_method: &str,
        weight_cfg: &WeightConfig, // NOWY ARGUMENT
    ) -> FileNode {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        let is_dir = path.is_dir();

        let icon = if is_dir {
            DIR_ICON.to_string()
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            get_file_type(ext).icon.to_string()
        } else {
            "📄".to_string()
        };

        // KROK A: Pobieramy bazową wagę (0 dla folderów w trybie sumy uwzględnionych)
        let mut weight_bytes = get_path_weight(path, weight_cfg.dir_sum_included);

        let mut children = vec![];
        if let Some(child_paths) = paths.get(path) {
            let mut child_nodes: Vec<FileNode> = child_paths
                .iter()
                .map(|c| build_node(c, paths, sort_method, weight_cfg))
                .collect();

            crate::fn_filestree::sort_nodes(&mut child_nodes, sort_method);

            // KROK B: Jeśli to folder i sumujemy tylko ujęte pliki, zsumuj wagi dzieci
            if is_dir && weight_cfg.dir_sum_included {
                weight_bytes = child_nodes.iter().map(|n| n.weight_bytes).sum();
            }

            children = child_nodes;
        }

        // KROK C: Formatowanie wagi do ciągu "[qq xxxxx]"
        let mut weight_str = String::new();
        
        // Sprawdzamy czy system wag jest w ogóle włączony
        if weight_cfg.system != crate::fn_weight::UnitSystem::None {
            let should_show = (is_dir && weight_cfg.show_for_dirs) || (!is_dir && weight_cfg.show_for_files);
            
            if should_show {
                weight_str = format_weight(weight_bytes, weight_cfg);
            } else {
                // Jeśli ukrywamy wagę dla tego węzła, wstawiamy puste spacje 
                // szerokość = 7 (nawiasy, jednostka, spacje) + precyzja
                let empty_width = 7 + weight_cfg.precision;
                weight_str = format!("{:width$}", "", width = empty_width);
            }
        }

        FileNode {
            name,
            path: path.clone(),
            is_dir,
            icon,
            weight_str,
            weight_bytes,
            children,
        }
    }

    let roots: Vec<PathBuf> = paths
        .iter()
        .filter(|p| p.parent().is_none() || !paths.contains(&p.parent().unwrap().to_path_buf()))
        .cloned()
        .collect();

    let mut top_nodes: Vec<FileNode> = roots
        .into_iter()
        .map(|r| build_node(&r, &tree_map, sort_method, weight_cfg))
        .collect();

    crate::fn_filestree::sort_nodes(&mut top_nodes, sort_method);

    top_nodes
}