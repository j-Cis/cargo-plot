//! Path: `src/libs/querypath/structs/tree_item.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

#[derive(Debug, Clone)]
pub enum TreeItem {
	Root {
		label: String,
		path: String,
		matched_size: u64,
		real_size: u64,
		modified_at: Option<u64>,
		children: Vec<TreeItem>,
	},
	Node {
		label: String,
		is_dir: bool,
		is_binary: bool,
		path: String,
		number: Option<usize>,
		size: u64,
		matched_size: u64,
		real_size: u64,
		modified_at: Option<u64>,
		children: Vec<TreeItem>,
	},
	Spacer,
}

// Akcesory używane przez silnik sortujący
impl TreeItem {
	pub fn is_dir(&self) -> bool {
		match self {
			TreeItem::Root { .. } => true,
			TreeItem::Node { is_dir, .. } => *is_dir,
			TreeItem::Spacer => false,
		}
	}

	pub fn label(&self) -> &str {
		match self {
			TreeItem::Root { label, .. } => label,
			TreeItem::Node { label, .. } => label,
			TreeItem::Spacer => "",
		}
	}
}
