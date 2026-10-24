//! Path: `src/libs/querypath/structs/walk_env_index.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::collections::HashSet;

use crate::querypath::core::{features::HandleWalkScan, traits::PattEnvIndex};

// ============================================================================
// INDEX I POMOCNICZE
// ============================================================================

pub struct WalkEnvIndex<'a> {
	dirs: HashSet<&'a str>,
	files: Vec<&'a str>,
}

impl<'a> WalkEnvIndex<'a> {
	pub fn from_walk(walk: &'a HandleWalkScan) -> Self {
		// Używamy nowych, płaskich iteratorów generujących się na żywo
		let dirs = walk.iter_all_dirs().map(|d| d.str.as_str()).collect();
		let mut files: Vec<&str> = walk.iter_all_files().map(|f| f.str.as_str()).collect();
		files.sort_unstable();
		Self { dirs, files }
	}
}

impl<'a> PattEnvIndex for WalkEnvIndex<'a> {
	fn has_dir(&self, dir: &str) -> bool { self.dirs.contains(dir) }

	fn has_file_with_prefix(&self, prefix: &str) -> bool {
		let start = self.files.partition_point(|&f| f < prefix);
		start < self.files.len() && self.files[start].starts_with(prefix)
	}

	fn any_file_in_dir(&self, dir: &str, check: &mut dyn FnMut(&str) -> bool) -> bool {
		let start = self.files.partition_point(|&f| f < dir);
		for &f in &self.files[start..] {
			if !f.starts_with(dir) {
				break;
			}
			if check(f) {
				return true;
			}
		}
		false
	}
}
