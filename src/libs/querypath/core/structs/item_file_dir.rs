//! Path: `src/libs/querypath/structs/item_file_dir.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Metadane dopasowanego pliku.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FileItem {
	pub path: String,
	pub size: u64,
	pub is_binary: bool,
	pub modified_at: Option<u64>,
}

/// Metadane dopasowanego katalogu z podwójnym rozliczeniem rozmiaru.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DirItem {
	pub path: String,
	/// Rzeczywisty, pełny rozmiar fizyczny wszystkich plików wewnątrz katalogu.
	pub real_size: u64,
	/// Sumaryczny rozmiar tylko tych plików, które spełniły kryteria wzorca.
	pub matched_size: u64,
	pub modified_at: Option<u64>,
}
