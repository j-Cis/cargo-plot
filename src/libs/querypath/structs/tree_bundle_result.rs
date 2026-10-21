//! Path: `src/libs/querypath/structs/tree_bundle_result.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::{BundleItem, TreeItem};
use std::collections::HashMap;

/// Gotowy wynik: posortowane drzewo z przypisanymi numerami oraz lista
/// elementów.
#[derive(Debug, Clone)]
pub struct BundleResult {
	pub tree: TreeItem,
	pub items: Vec<BundleItem>,
	pub indices: HashMap<String, usize>,
	pub max_num: usize,
}

impl BundleResult {
	pub fn get(&self, path: &str) -> Option<usize> {
		self.indices.get(path).copied()
	}

	/// Iterator po plikach tekstowych w kolejności ułożenia w drzewie.
	pub fn text_files(&self) -> impl Iterator<Item = &BundleItem> {
		self.items.iter().filter(|i| !i.is_dir && !i.is_binary)
	}

	pub fn files(&self) -> impl Iterator<Item = &BundleItem> {
		self.items.iter().filter(|i| !i.is_dir)
	}

	pub fn dirs(&self) -> impl Iterator<Item = &BundleItem> {
		self.items.iter().filter(|i| i.is_dir)
	}

	pub fn len(&self) -> usize {
		self.items.len()
	}

	pub fn is_empty(&self) -> bool {
		self.items.is_empty()
	}
}
