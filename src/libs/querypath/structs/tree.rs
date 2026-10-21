//! Path: `src/libs/querypath/structs/tree.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::{super::types::consts::TREE_SYMBOLS, TreeItem};

impl Default for Tree {
	fn default() -> Self {
		Self::new()
	}
}

pub struct Tree {
	pub name_width: usize,
}

impl Tree {
	pub fn new() -> Self {
		Self { name_width: 25 } // <-- Domyślnie 25
	}

	pub fn name_width(mut self, width: usize) -> Self {
		self.name_width = width.max(15); // <-- Minimum 15
		self
	}

	pub fn calculate_max_prefix_len_for_item(&self, item: &TreeItem) -> usize {
		let mut max_len = 0;
		self.walk_prefix_len(item, "", &mut max_len);
		max_len
	}

	fn walk_prefix_len(&self, item: &TreeItem, prefix: &str, max_len: &mut usize) {
		match item {
			TreeItem::Spacer => {
				*max_len = (*max_len).max(prefix.chars().count());
			}
			TreeItem::Root { children, .. } => {
				*max_len = (*max_len).max(3); // "▣─┬"
				for child in children {
					self.walk_prefix_len(child, "  ", max_len);
				}
			}
			TreeItem::Node { children, .. } => {
				let symbol_len = 4; // np. "├──┬"
				let line_prefix_len = prefix.chars().count() + symbol_len;
				*max_len = (*max_len).max(line_prefix_len);

				let child_prefix = format!("{}{}", prefix, TREE_SYMBOLS[7]);
				for child in children {
					self.walk_prefix_len(child, &child_prefix, max_len);
				}
			}
		}
	}
}
