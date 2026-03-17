use super::node::FileNode;
use crate::core::file_stats::weight::{self, WeightConfig};
use crate::core::path_matcher::SortStrategy;
use crate::theme::for_path_list::get_icon_for_path;
use colored::Colorize;
/// [POL]: Zarządca wyświetlania wyników w formie płaskiej listy.
pub struct PathList {
    items: Vec<FileNode>,
}

impl PathList {
    /// [POL]: Buduje listę na podstawie zbioru ścieżek i statystyk.
    pub fn build(
        paths_strings: &[String],
        base_dir: &str,
        sort_strategy: SortStrategy,
        weight_cfg: &WeightConfig,
    ) -> Self {
        // Wykorzystujemy istniejącą logikę węzłów, ale bez rekurencji (płaska lista)
        let mut items: Vec<FileNode> = paths_strings
            .iter()
            .map(|p_str| {
                let absolute = std::path::Path::new(base_dir).join(p_str);
                let is_dir = p_str.ends_with('/');
                let weight_bytes =
                    crate::core::file_stats::weight::get_path_weight(&absolute, true);
                let weight_str = weight::format_weight(weight_bytes, is_dir, weight_cfg);

                FileNode {
                    name: p_str.clone(),
                    path: absolute,
                    is_dir,
                    icon: get_icon_for_path(p_str).to_string(),
                    weight_str,
                    weight_bytes,
                    children: vec![], // Lista nie ma dzieci
                }
            })
            .collect();

        FileNode::sort_slice(&mut items, sort_strategy);

        Self { items }
    }

    /// [POL]: Renderuje listę dla terminala (z kolorami i ikonami).
    pub fn render_cli(&self, _is_match: bool) -> String {
        let mut out = String::new();
        // let tag = if is_match { "✅ MATCH: ".green() } else { "❌ REJECT:".red() };

        for item in &self.items {
            let line = format!(
                "{} {} {}\n",
                item.weight_str.truecolor(120, 120, 120),
                item.icon,
                if item.is_dir {
                    item.name.yellow()
                } else {
                    item.name.white()
                }
            );
            out.push_str(&line);
        }
        out
    }

    #[must_use]
    pub fn render_txt(&self) -> String {
        let mut out = String::new();
        for item in &self.items {
            // Brak formatowania ANSI
            let line = format!("{} {} {}\n", item.weight_str, item.icon, item.name);
            out.push_str(&line);
        }
        out
    }
}
