use colored::Colorize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::node::FileNode;
use crate::core::file_stats::weight::{self, UnitSystem, WeightConfig};
use crate::core::path_matcher::SortStrategy;
use crate::theme::for_path_tree::{DIR_ICON, TreeStyle, get_file_type};

pub struct PathGrid {
    roots: Vec<FileNode>,
    style: TreeStyle,
}

impl PathGrid {
    #[must_use]
    pub fn build(
        paths_strings: &[String],
        base_dir: &str,
        sort_strategy: SortStrategy,
        weight_cfg: &WeightConfig,
        root_name: Option<&str>,
    ) -> Self {
        // Dokładnie taka sama logika budowania struktury węzłów jak w PathTree::build
        let base_path_obj = Path::new(base_dir);
        let paths: Vec<PathBuf> = paths_strings.iter().map(PathBuf::from).collect();
        let mut tree_map: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();

        for p in &paths {
            let parent = p
                .parent()
                .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
            tree_map.entry(parent).or_default().push(p.clone());
        }

        fn build_node(
            path: &PathBuf,
            paths_map: &BTreeMap<PathBuf, Vec<PathBuf>>,
            base_path: &Path,
            sort_strategy: SortStrategy,
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

            let absolute_path = base_path.join(path);
            let mut weight_bytes =
                weight::get_path_weight(&absolute_path, weight_cfg.dir_sum_included);
            let mut children = vec![];

            if let Some(child_paths) = paths_map.get(path) {
                let mut child_nodes: Vec<FileNode> = child_paths
                    .iter()
                    .map(|c| build_node(c, paths_map, base_path, sort_strategy, weight_cfg))
                    .collect();

                FileNode::sort_slice(&mut child_nodes, sort_strategy);

                if is_dir && weight_cfg.dir_sum_included {
                    weight_bytes = child_nodes.iter().map(|n| n.weight_bytes).sum();
                }
                children = child_nodes;
            }

            let weight_str = weight::format_weight(weight_bytes, is_dir, weight_cfg);
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

        let roots_paths: Vec<PathBuf> = paths
            .iter()
            .filter(|p| {
                let parent = p.parent();
                parent.is_none()
                    || parent.unwrap() == Path::new("")
                    || !paths.contains(&parent.unwrap().to_path_buf())
            })
            .cloned()
            .collect();

        let mut top_nodes: Vec<FileNode> = roots_paths
            .into_iter()
            .map(|r| build_node(&r, &tree_map, base_path_obj, sort_strategy, weight_cfg))
            .collect();

        FileNode::sort_slice(&mut top_nodes, sort_strategy);

        let final_roots = if let Some(r_name) = root_name {
            let empty_weight = if weight_cfg.system != UnitSystem::None {
                " ".repeat(7 + weight_cfg.precision)
            } else {
                String::new()
            };

            vec![FileNode {
                name: r_name.to_string(),
                path: PathBuf::from(r_name),
                is_dir: true,
                icon: DIR_ICON.to_string(),
                weight_str: empty_weight,
                weight_bytes: 0,
                children: top_nodes,
            }]
        } else {
            top_nodes
        };

        Self {
            roots: final_roots,
            style: TreeStyle::default(),
        }
    }

    #[must_use]
    pub fn render_cli(&self) -> String {
        let max_width = self.calc_max_width(&self.roots, 0);
        self.plot(&self.roots, "", true, max_width)
    }

    #[must_use]
    pub fn render_txt(&self) -> String {
        let max_width = self.calc_max_width(&self.roots, 0);
        self.plot(&self.roots, "", false, max_width)
    }

    fn calc_max_width(&self, nodes: &[FileNode], indent_len: usize) -> usize {
        let mut max = 0;
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

            let current_len = node.weight_str.chars().count()
                + indent_len
                + branch.chars().count()
                + 1
                + node.icon.chars().count()
                + 1
                + node.name.chars().count();
            if current_len > max {
                max = current_len;
            }

            if has_children {
                let next_indent = indent_len
                    + if is_last {
                        self.style.indent_last.chars().count()
                    } else {
                        self.style.indent_mid.chars().count()
                    };
                let child_max = self.calc_max_width(&node.children, next_indent);
                if child_max > max {
                    max = child_max;
                }
            }
        }
        max
    }

    fn plot(&self, nodes: &[FileNode], indent: &str, use_color: bool, max_width: usize) -> String {
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

            let raw_left_len = node.weight_str.chars().count()
                + indent.chars().count()
                + branch.chars().count()
                + 1
                + node.icon.chars().count()
                + 1
                + node.name.chars().count();
            let pad_len = max_width.saturating_sub(raw_left_len) + 4;
            let padding = " ".repeat(pad_len);

            let rel_path_str = node.path.to_string_lossy().replace('\\', "/");
            let display_path = if node.is_dir && !rel_path_str.ends_with('/') {
                format!("./{}/", rel_path_str)
            } else if !rel_path_str.starts_with("./") && !rel_path_str.starts_with('.') {
                format!("./{}", rel_path_str)
            } else {
                rel_path_str
            };

            let right_colored = if use_color {
                if node.is_dir {
                    display_path.truecolor(200, 200, 50).to_string()
                } else {
                    display_path.white().to_string()
                }
            } else {
                display_path
            };

            let left_colored = if use_color {
                if node.is_dir {
                    format!(
                        "{}{}{} {}{}",
                        weight_prefix,
                        indent.green(),
                        branch.green(),
                        node.icon,
                        node.name.truecolor(200, 200, 50)
                    )
                } else {
                    format!(
                        "{}{}{} {}{}",
                        weight_prefix,
                        indent.green(),
                        branch.green(),
                        node.icon,
                        node.name.white()
                    )
                }
            } else {
                format!(
                    "{}{}{} {} {}",
                    weight_prefix, indent, branch, node.icon, node.name
                )
            };

            result.push_str(&format!("{}{}{}\n", left_colored, padding, right_colored));

            if has_children {
                let new_indent = if is_last {
                    format!("{}{}", indent, self.style.indent_last)
                } else {
                    format!("{}{}", indent, self.style.indent_mid)
                };
                result.push_str(&self.plot(&node.children, &new_indent, use_color, max_width));
            }
        }
        result
    }
}
