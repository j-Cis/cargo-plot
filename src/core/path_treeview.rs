// [EN]: Logic for transforming flat paths into a hierarchical tree structure with icons and weights.
// [PL]: Logika przekształcania płaskich ścieżek w hierarchiczną strukturę drzewa z ikonami i wagami.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use colored::Colorize;
use super::super::theme::for_path_tree::{get_file_type, TreeStyle, DIR_ICON};

// ==========================================
// 1. STRUKTURY DANYCH I KONFIGURACJA
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitSystem {
    Decimal,
    Binary,
    Both,
    None,
}

#[derive(Debug, Clone)]
pub struct WeightConfig {
    pub system: UnitSystem,
    pub precision: usize,
    pub show_for_files: bool,
    pub show_for_dirs: bool,
    pub dir_sum_included: bool,
}

impl Default for WeightConfig {
    fn default() -> Self {
        Self {
            system: UnitSystem::Decimal,
            precision: 5,
            show_for_files: true,
            show_for_dirs: true,
            dir_sum_included: true,
        }
    }
}

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
    /// [PL]: Sortuje listę węzłów w miejscu.
    pub fn sort_slice(nodes: &mut [FileNode], method: &str) {
        match method {
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
}

// ==========================================
// 2. HERMETYCZNY SILNIK DRZEWA (PathTree)
// ==========================================

/// [PL]: Główna struktura zarządzająca budową i renderowaniem drzewa.
pub struct PathTree {
    roots: Vec<FileNode>,
    style: TreeStyle,
}

impl PathTree {
    /// [PL]: Inicjuje i buduje drzewo na podstawie płaskiej listy ścieżek.
    #[must_use]
    /// [PL]: Inicjuje i buduje drzewo na podstawie płaskiej listy ścieżek.
    #[must_use]
    pub fn build(
        paths_strings: &[String],
        base_dir: &str, // ⚡ Wstrzyknięta ścieżka bazowa
        sort_method: &str,
        weight_cfg: &WeightConfig,
    ) -> Self {
        // 1. Zdefiniowanie ścieżki bazowej (widocznej w całej funkcji build)
        let base_path_obj = Path::new(base_dir); 

        let paths: Vec<PathBuf> = paths_strings.iter().map(PathBuf::from).collect();
        let mut tree_map: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();

        for p in &paths {
            let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
            tree_map.entry(parent).or_default().push(p.clone());
        }

        // Wewnętrzna funkcja rekurencyjna
        fn build_node(
            path: &PathBuf,
            paths_map: &BTreeMap<PathBuf, Vec<PathBuf>>,
            base_path: &Path, // ⚡ Argument dla rekurencji
            sort_method: &str,
            weight_cfg: &WeightConfig,
        ) -> FileNode {
            let name = path
                .file_name()
                .map_or_else(|| "/".to_string(), |n| n.to_string_lossy().to_string());

            let is_dir = path.is_dir() || path.to_string_lossy().ends_with('/');
            let icon = if is_dir {
                DIR_ICON.to_string()
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                get_file_type(ext).icon.to_string()
            } else {
                "📄".to_string()
            };

            // ⚡ Tworzymy ścieżkę absolutną, żeby pobrać prawdziwą wagę z dysku
            let absolute_path = base_path.join(path);
            let mut weight_bytes = PathTree::get_path_weight(&absolute_path, weight_cfg.dir_sum_included);
            
            let mut children = vec![];

            if let Some(child_paths) = paths_map.get(path) {
                let mut child_nodes: Vec<FileNode> = child_paths
                    .iter()
                    .map(|c| build_node(c, paths_map, base_path, sort_method, weight_cfg)) // ⚡ Rekurencja z base_path
                    .collect();

                FileNode::sort_slice(&mut child_nodes, sort_method);

                if is_dir && weight_cfg.dir_sum_included {
                    weight_bytes = child_nodes.iter().map(|n| n.weight_bytes).sum();
                }

                children = child_nodes;
            }

            let weight_str = PathTree::format_weight(weight_bytes, is_dir, weight_cfg);

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
            .filter(|p| {
                let parent = p.parent();
                parent.is_none() || parent.unwrap() == Path::new("") || !paths.contains(&parent.unwrap().to_path_buf())
            })
            .cloned()
            .collect();

        let mut top_nodes: Vec<FileNode> = roots
            .into_iter()
            // ⚡ Tutaj przekazujemy obiekt zainicjowany na samym początku funkcji
            .map(|r| build_node(&r, &tree_map, base_path_obj, sort_method, weight_cfg))
            .collect();

        FileNode::sort_slice(&mut top_nodes, sort_method);

        Self {
            roots: top_nodes,
            style: TreeStyle::default(),
        }
    }

    /// [PL]: Opcjonalnie nadpisuje domyślny styl drzewa. (Wzorzec Builder)
    #[must_use]
    pub fn with_style(mut self, style: TreeStyle) -> Self {
        self.style = style;
        self
    }

    /// [PL]: Renderuje drzewo do formatu CLI (z kolorami).
    #[must_use]
    pub fn render_cli(&self) -> String {
        self.plot(&self.roots, "", true)
    }

    /// [PL]: Renderuje drzewo do czystego tekstu (np. do Markdown).
    #[must_use]
    pub fn render_txt(&self) -> String {
        self.plot(&self.roots, "", false)
    }

    // --- PRYWATNE METODY POMOCNICZE W IMPL ---

    fn plot(&self, nodes: &[FileNode], indent: &str, use_color: bool) -> String {
        let mut result = String::new();

        for (i, node) in nodes.iter().enumerate() {
            let is_last = i == nodes.len() - 1;
            let has_children = !node.children.is_empty();

            let branch = if node.is_dir {
                match (is_last, has_children) {
                    (true, true) => &self.style.dir_last_with_children,
                    (false, true) => &self.style.dir_mid_with_children,
                    (true, false) => &self.style.dir_last_no_children,
                    (false, false) => &self.style.dir_mid_no_children,
                }
            } else if is_last {
                &self.style.file_last
            } else {
                &self.style.file_mid
            };

            let weight_prefix = if node.weight_str.is_empty() {
                String::new()
            } else if use_color {
                node.weight_str.truecolor(120, 120, 120).to_string()
            } else {
                node.weight_str.clone()
            };

            let line = if use_color {
                if node.is_dir {
                    format!(
                        "{}{}{} {}{}/\n",
                        weight_prefix,
                        indent.green(),
                        branch.green(),
                        node.icon,
                        node.name.truecolor(200, 200, 50)
                    )
                } else {
                    format!(
                        "{}{}{} {}{}\n",
                        weight_prefix,
                        indent.green(),
                        branch.green(),
                        node.icon,
                        node.name.white()
                    )
                }
            } else {
                format!("{}{}{} {} {}\n", weight_prefix, indent, branch, node.icon, node.name)
            };

            result.push_str(&line);

            if has_children {
                let new_indent = if is_last {
                    format!("{}{}", indent, self.style.indent_last)
                } else {
                    format!("{}{}", indent, self.style.indent_mid)
                };
                result.push_str(&self.plot(&node.children, &new_indent, use_color));
            }
        }

        result
    }

    fn get_path_weight(path: &Path, sum_included_only: bool) -> u64 {
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => return 0,
        };

        if metadata.is_file() {
            return metadata.len();
        }

        if metadata.is_dir() && !sum_included_only {
            return Self::get_dir_size(path);
        }

        0
    }

    fn get_dir_size(path: &Path) -> u64 {
        fs::read_dir(path)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|e| {
                        let p = e.path();
                        if p.is_dir() {
                            Self::get_dir_size(&p)
                        } else {
                            e.metadata().map(|m| m.len()).unwrap_or(0)
                        }
                    })
                    .sum()
            })
            .unwrap_or(0)
    }

    fn format_weight(bytes: u64, is_dir: bool, config: &WeightConfig) -> String {
        if config.system == UnitSystem::None {
            return String::new();
        }

        let should_show = (is_dir && config.show_for_dirs) || (!is_dir && config.show_for_files);
        if !should_show {
            let empty_width = 7 + config.precision;
            return format!("{:width$}", "", width = empty_width);
        }

        let (base, units) = match config.system {
            UnitSystem::Binary => (1024.0_f64, vec!["B", "KiB", "MiB", "GiB", "TiB", "PiB"]),
            _ => (1000.0_f64, vec!["B", "kB", "MB", "GB", "TB", "PB"]),
        };

        if bytes == 0 {
            return format!("[{:>3} {:>width$}] ", units[0], "0", width = config.precision);
        }

        let bytes_f = bytes as f64;
        let exp = (bytes_f.ln() / base.ln()).floor() as usize;
        let exp = exp.min(units.len() - 1);
        let value = bytes_f / base.powi(exp as i32);
        let unit = units[exp];

        let mut formatted_value = format!("{value:.10}");
        if formatted_value.len() > config.precision {
            formatted_value = formatted_value[..config.precision].trim_end_matches('.').to_string();
        } else {
            formatted_value = format!("{formatted_value:>width$}", width = config.precision);
        }

        format!("[{unit:>3} {formatted_value}] ")
    }
}