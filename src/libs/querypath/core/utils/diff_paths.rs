//! Path: `src/libs/querypath/utils/diff_paths.rs`
//! Last significant modified: 2026-W43-3 13:06:18 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::path::{Component, Path, PathBuf};

/// Ścieżka `target` względem `base`, z `..` gdy trzeba.
/// `None`, gdy nie ma wspólnego korzenia (np. inny dysk na Windows).
pub fn diff_paths(target: &Path, base: &Path) -> Option<PathBuf> {
	let t: Vec<Component> = target.components().collect();
	let b: Vec<Component> = base.components().collect();

	let common = t.iter().zip(&b).take_while(|(x, y)| x == y).count();
	if common == 0 {
		return None;
	}

	let mut out = PathBuf::new();
	for _ in common..b.len() {
		out.push("..");
	}
	for c in &t[common..] {
		out.push(c.as_os_str());
	}
	Some(out)
}
