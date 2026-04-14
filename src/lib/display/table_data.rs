use colored::Colorize;

use super::{Color, DrawTree, Icon};
use crate::lib::logic::{TabColumn, FileKind, TableData, TableOutput, TableRow,TabPathStructure};

fn format_size(bytes: u64) -> String {
	if bytes < 1024 {
		format!("{} B", bytes)
	} else if bytes < 1024 * 1024 {
		format!("{:.2} kB", bytes as f64 / 1024.0)
	} else {
		format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
	}
}

impl std::fmt::Display for TableData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let total = self.rows.len();
		if total == 0 {
			return write!(f, "{} {} Pusta tabela", DrawTree::ITEM_LAST, Icon::EMPTY);
		}

		let mut lines = Vec::new();

		for (i, row) in self.rows.iter().enumerate() {
			let prefix = DrawTree::list(i, total);

			let kind_icon = match row.kind {
				FileKind::Dir => Icon::FOLDER,
				FileKind::Binary => "💾",
				FileKind::Text => Icon::FILE,
				FileKind::Other => "❓",
			};

			lines.push(format!(
				"{} {} [{:>10}] | {} | {}",
				prefix,
				kind_icon,
				format_size(row.size),
				row.modified.format("%Y-%m-%d %H:%M"),
				row.path
			));
		}

		write!(f, "{}", lines.join("\n"))
	}
}

impl std::fmt::Display for TableOutput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let total_rows = self.data.rows.len();

		let (rows_to_show, index_offset) = if let Some(size) = self.trim_size {
            let start = self.trim_page.saturating_sub(1) * size;
            (self.data.rows.iter().skip(start).take(size).collect::<Vec<_>>(), start)
        } else {
            (self.data.rows.iter().collect::<Vec<_>>(), 0)
        };

		let current_view_count = rows_to_show.len();
		let num_width = total_rows.to_string().len();

		let mut prefix_map = std::collections::HashMap::new();

		if self.data.structure == TabPathStructure::Tree {
			use std::{
				collections::BTreeMap,
				path::{Path, PathBuf},
			};

			let clean_paths: Vec<PathBuf> =
				self.data.rows.iter().map(|r| PathBuf::from(r.path.trim_end_matches('/'))).collect();

			let mut tree_map: BTreeMap<PathBuf, Vec<&TableRow>> = BTreeMap::new();

			for (i, p) in clean_paths.iter().enumerate() {
				let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
				tree_map.entry(parent).or_default().push(&self.data.rows[i]);
			}

			let mut roots = Vec::new();
			for (i, p) in clean_paths.iter().enumerate() {
				let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
				let is_root = parent == Path::new(".") || parent == Path::new("") || !clean_paths.contains(&parent);
				if is_root {
					roots.push(&self.data.rows[i]);
				}
			}

			fn build_prefixes(
				nodes: &[&TableRow],
				tree_map: &BTreeMap<PathBuf, Vec<&TableRow>>,
				indent: &str,
				out_map: &mut std::collections::HashMap<String, String>,
			) {
				for (i, node) in nodes.iter().enumerate() {
					let clean_p = PathBuf::from(node.path.trim_end_matches('/'));
					let is_last = i == nodes.len() - 1;
					let children = tree_map.get(&clean_p);
					let has_children = children.is_some() && !children.unwrap().is_empty();
					let is_dir = node.kind == FileKind::Dir;

					let (branch, next_indent) = DrawTree::tree(is_dir, is_last, has_children);

					let prefix = format!("{}{}", indent, branch);
					out_map.insert(node.path.clone(), prefix);

					if let Some(child_nodes) = children {
						let new_indent = format!("{}{}", indent, next_indent);
						build_prefixes(child_nodes, tree_map, &new_indent, out_map);
					}
				}
			}

			build_prefixes(&roots, &tree_map, "", &mut prefix_map);
		}

		let mut output_lines = Vec::new();

		for (i, row) in rows_to_show.iter().enumerate() {
			let actual_idx = index_offset + i + 1;

			let tree_list_prefix_str = if self.data.structure == TabPathStructure::Tree {
				prefix_map.get(&row.path).map(|s| s.as_str()).unwrap_or("")
			} else {
				DrawTree::list(i, current_view_count)
			};
			let tree_list_prefix = Color::tree(tree_list_prefix_str);

			let icon_str = if self.more_icons {
				let is_dir = row.kind == FileKind::Dir;
				let file_name = row.path.trim_end_matches('/').split('/').next_back().unwrap_or("");
				let is_hidden = file_name.starts_with('.');

				if is_dir {
					if is_hidden { Icon::FOLDER2_HIDDEN } else { Icon::FOLDER2 }
				} else if is_hidden {
					Icon::FILE2_HIDDEN
				} else {
					let path_obj = std::path::Path::new(&row.path);
					if let Some(ext) = path_obj.extension().and_then(|e| e.to_str()) {
						match ext.to_lowercase().as_str() {
							"rs" => Icon::LANG_RUST,
							"toml" => "⚙️",
							"slint" => "🎨",
							"md" => "📝",
							"json" => "🔣",
							"yaml" | "yml" => "🛠️",
							"html" => "📖",
							"css" => "🖌️",
							"js" => "📜",
							"ts" => "📘",
							_ => {
								if row.kind == FileKind::Binary {
									"💾"
								} else {
									Icon::FILE2
								}
							}
						}
					} else {
						if row.kind == FileKind::Binary { "💾" } else { Icon::FILE2 }
					}
				}
			} else {
				match row.kind {
					FileKind::Dir => Icon::FOLDER,
					FileKind::Binary => "💾",
					FileKind::Text => Icon::FILE,
					FileKind::Other => "❓",
				}
			};

			let path_styled = match row.kind {
				FileKind::Dir => Color::folder(&row.path),
				FileKind::Binary => Color::binary(&row.path),
				FileKind::Text => Color::file(&row.path),
				FileKind::Other => row.path.dimmed(),
			};

			let icon = match row.kind {
				FileKind::Dir => icon_str.yellow(),
				FileKind::Binary => icon_str.magenta(),
				FileKind::Text => icon_str.normal(),
				FileKind::Other => icon_str.red(),
			};

			let size_str = Color::size(&format!("{:>10}", format_size(row.size)));
			let pos_num = Color::num(&format!("{:>width$}.", actual_idx, width = num_width));
			let date_str = Color::date(&row.modified.format("%Y W%V %a").to_string());
			let time_str = Color::time(&row.modified.format("%H:%M:%S.%3f").to_string());

			let mut line_parts = Vec::new();
			for (col_idx, col) in self.columns.iter().enumerate() {
				let part = match col {
					TabColumn::TreeList => tree_list_prefix.to_string(),
					TabColumn::Number => pos_num.to_string(),
					TabColumn::Icon => icon.to_string(),
					TabColumn::Size => {
						format!("{}{}{}", Color::border("[ "), size_str, Color::border(" ]"))
					}
					TabColumn::Date => {
						let pipe = Color::border("|");
						let next_is_time = matches!(self.columns.get(col_idx + 1), Some(TabColumn::Time));
						let prev_is_time = col_idx > 0 && self.columns[col_idx - 1] == TabColumn::Time;
						match (prev_is_time, next_is_time) {
							(false, true) => format!("{} {}", pipe, date_str),
							(true, false) => format!("{} {}", date_str, pipe),
							(true, true) => format!("{}", date_str),
							(false, false) => format!("{} {} {}", pipe, date_str, pipe),
						}
					}
					TabColumn::Time => {
						let pipe = Color::border("|");
						let next_is_date = matches!(self.columns.get(col_idx + 1), Some(TabColumn::Date));
						let prev_is_date = col_idx > 0 && self.columns[col_idx - 1] == TabColumn::Date;
						match (prev_is_date, next_is_date) {
							(false, true) => format!("{} {}", pipe, time_str),
							(true, false) => format!("{} {}", time_str, pipe),
							(true, true) => format!("{}", time_str),
							(false, false) => format!("{} {} {}", pipe, time_str, pipe),
						}
					}
					TabColumn::Path => path_styled.to_string(),
				};
				line_parts.push(part);
			}

			output_lines.push(line_parts.join(" "));
		}

		if let Some(size) = self.trim_size {
            let total_pages = total_rows.saturating_add(size - 1) / size;
            if total_pages > 1 {
                output_lines.push(format!(
                    "          {}",
                    format!("... Strona {} z {} (łącznie {} pozycji)", self.trim_page, total_pages, total_rows)
                        .italic()
                        .dimmed()
                ));
            } else if total_rows > size {
                output_lines.push(format!(
                    "          {}",
                    format!("... i {} innych pozycji", total_rows - size).italic().dimmed()
                ));
            }
        }

		write!(f, "{}", output_lines.join("\n"))
	}
}
