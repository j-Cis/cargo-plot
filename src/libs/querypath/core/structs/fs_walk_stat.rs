//! Path: `src/libs/querypath/structs/fs_walk_stat.rs`
//! Last significant modified: 2026-W43-3 14:54:13 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Statystyki surowego skanowania dysku (przed filtrowaniem wzorcami i
/// opcjami).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FsWalkStat {
	pub count_dirs_all: usize,
	pub count_dirs_empty: usize,
	pub count_files_all: usize,
	pub count_files_empty: usize,
	pub count_files_binary: usize,
}
