use std::collections::BTreeMap;
use std::path::PathBuf;
use std::cmp::Ordering;
use crate::fn_pathtype::{get_file_type, DIR_ICON};

/// Struktura węzła drzewa
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub icon: String,
    pub children: Vec<FileNode>,
}

/// Helper do sortowania węzłów zgodnie z wybraną metodą
fn sort_nodes(nodes: &mut [FileNode], sort_method: &str) {
    match sort_method {
        "files-first" => nodes.sort_by(|a, b| {
            if a.is_dir == b.is_dir {
                a.name.cmp(&b.name)
            } else if !a.is_dir {
                Ordering::Less // Jeśli 'a' to plik, ma być wcześniej
            } else {
                Ordering::Greater // Jeśli 'a' to folder, spada na dół
            }
        }),
        "dirs-first" => nodes.sort_by(|a, b| {
            if a.is_dir == b.is_dir {
                a.name.cmp(&b.name)
            } else if a.is_dir {
                Ordering::Less // Jeśli 'a' to folder, ma być wcześniej
            } else {
                Ordering::Greater
            }
        }),
        _ => nodes.sort_by(|a, b| a.name.cmp(&b.name)),
    }
}

/// Funkcja formatująca - buduje drzewo i przypisuje ikony
pub fn filestree(paths: Vec<PathBuf>, sort_method: &str) -> Vec<FileNode> {
    let mut tree_map: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    for p in &paths {
        let parent = p.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from("/"));
        tree_map.entry(parent).or_default().push(p.clone());
    }

    // ZMIANA: Usuwamy file_icons z funkcji wewnętrznej
    fn build_node(
        path: &PathBuf,
        paths: &BTreeMap<PathBuf, Vec<PathBuf>>,
        sort_method: &str,
    ) -> FileNode {
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        let is_dir = path.is_dir();

        // Pobieramy ikony z naszego pliku SSoT
        let icon = if is_dir {
            DIR_ICON.to_string()
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            get_file_type(ext).icon.to_string()
        } else {
            "📄".to_string()
        };

        let mut children = vec![];
        if let Some(child_paths) = paths.get(path) {
            let mut child_nodes: Vec<FileNode> = child_paths
                .iter()
                .map(|c| build_node(c, paths, sort_method)) // Usuwamy argument file_icons
                .collect();

            crate::fn_filestree::sort_nodes(&mut child_nodes, sort_method);
            children = child_nodes;
        }

        FileNode { name, path: path.clone(), is_dir, icon, children }
    }

    let roots: Vec<PathBuf> = paths.iter()
        .filter(|p| p.parent().is_none() || !paths.contains(&p.parent().unwrap().to_path_buf()))
        .cloned()
        .collect();

    let mut top_nodes: Vec<FileNode> = roots
        .into_iter()
        .map(|r| build_node(&r, &tree_map, sort_method)) // Usuwamy argument file_icons
        .collect();

    crate::fn_filestree::sort_nodes(&mut top_nodes, sort_method);

    top_nodes
}