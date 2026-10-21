//! Path: `src/libs/querypath/structs/tree_bundle_item.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Pojedynczy plik/folder z nadanym numerem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleItem {
	pub number: usize,
	pub path: String,
	pub is_dir: bool,
	pub is_binary: bool,
	pub size: u64,
	pub modified_at: Option<u64>,
}
