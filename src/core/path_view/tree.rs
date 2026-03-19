use colored::Colorize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

// Importy z rodzeństwa i innych modułów core
use super::node::FileNode;
use crate::core::file_stats::weight::{self, UnitSystem, WeightConfig};
use crate::core::path_matcher::SortStrategy;
use crate::theme::for_path_tree::{DIR_ICON, FILE_ICON, TreeStyle, get_file_type};
pub struct PathTree {
    roots: Vec<FileNode>,
    style: TreeStyle,
}

impl PathTree {
    #[must_use]
    pub fn build(
        paths_strings: &[String],
        base_dir: &str,
        sort_strategy: SortStrategy,
        weight_cfg: &WeightConfig,
        root_name: Option<&str>,
        no_emoji: bool,
    ) -> Self {
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
            no_emoji: bool,
        ) -> FileNode {
            let name = path
                .file_name()
                .map_or_else(|| "/".to_string(), |n| n.to_string_lossy().to_string());

            let is_dir = path.is_dir() || path.to_string_lossy().ends_with('/');
            let icon = if no_emoji {
                String::new()
            } else if is_dir {
                DIR_ICON.to_string()
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                get_file_type(ext).icon.to_string()
            } else {
                FILE_ICON.to_string()
            };

            let absolute_path = base_path.join(path);
            let mut weight_bytes =
                weight::get_path_weight(&absolute_path, weight_cfg.dir_sum_included);
            let mut children = vec![];

            if let Some(child_paths) = paths_map.get(path) {
                let mut child_nodes: Vec<FileNode> = child_paths
                    .iter()
                    .map(|c| build_node(c, paths_map, base_path, sort_strategy, weight_cfg, no_emoji))
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
            .map(|r| build_node(&r, &tree_map, base_path_obj, sort_strategy, weight_cfg, no_emoji))
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
                icon: if no_emoji { String::new() } else { DIR_ICON.to_string() },
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
        self.plot(&self.roots, "", true)
    }

    #[must_use]
    pub fn render_txt(&self) -> String {
        self.plot(&self.roots, "", false)
    }

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
                        "{weight_prefix}{}{branch_color} {icon} {name}\n",
                        indent.green(),
                        branch_color = branch.green(),
                        icon = node.icon,
                        name = node.name.truecolor(200, 200, 50)
                    )
                } else {
                    format!(
                        "{weight_prefix}{}{branch_color} {icon} {name}\n",
                        indent.green(),
                        branch_color = branch.green(),
                        icon = node.icon,
                        name = node.name.white()
                    )
                }
            } else {
                format!(
                    "{weight_prefix}{indent}{branch} {icon} {name}\n",
                    icon = node.icon,
                    name = node.name
                )
            };

            result.push_str(&line);

            if has_children {
                let new_indent = if is_last {
                    format!("{indent}{}", self.style.indent_last)
                } else {
                    format!("{indent}{}", self.style.indent_mid)
                };
                result.push_str(&self.plot(&node.children, &new_indent, use_color));
            }
        }
        result
    }
}
