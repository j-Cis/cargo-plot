//! Path: `src/libs/querypath/structs/path_node.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::path::PathBuf;

use crate::querypath::core::utils::normalize_path;

/// Reprezentacja pojedynczego węzła ścieżki przechowująca zarówno
/// oryginalny obiekt systemowy (`PathBuf`), jak i szybką, znormalizowaną
/// postać tekstową (`String`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathNode {
	pub buf: PathBuf,
	pub str: String,
	pub is_dir: bool,
	pub is_empty: bool,
	pub is_doted: bool,
	pub is_binary: bool,
	pub has_extension: bool,
	pub no_numerate: bool,
	pub no_copypast: bool,
	pub meta_modifed_at: Option<String>,
	pub meta_size_total: Option<u64>,
	pub meta_size_scope: Option<u64>,
}

impl PathNode {
	pub fn new(
		buf: PathBuf,
		is_dir: bool,
		is_empty: bool,
		is_doted: bool,
		is_binary: bool,
		has_extension: bool,
	) -> Self {
		let str = normalize_path(&buf);
		Self {
			buf,
			str,
			is_dir,
			is_empty,
			is_doted,
			is_binary,
			has_extension,
			no_numerate: false,
			no_copypast: false,
			meta_modifed_at: None,
			meta_size_total: None,
			meta_size_scope: None,
		}
	}

	/// Zbiorcza aktualizacja metadanych węzła na późniejszych etapach potoku.
	pub fn meta(&mut self, modifed_at: Option<String>, size_total: Option<u64>, size_scope: Option<u64>) -> &mut Self {
		self.meta_modifed_at = modifed_at;
		self.meta_size_total = size_total;
		self.meta_size_scope = size_scope;
		self
	}

	/// Selektywna aktualizacja wyłącznie rozmiaru dopasowanego (po filtracji).
	pub fn set_meta_scope(&mut self, size_scope: Option<u64>) -> &mut Self {
		self.meta_size_scope = size_scope;
		self
	}

	/// Aktywuje flagę zakazującą numeracji dla tego węzła. Zwraca siebie do łańcuchowania.
	pub fn set_no_numerate(&mut self) -> &mut Self {
		self.no_numerate = true;
		self
	}

	/// Aktywuje flagę zakazującą kopiowania kodu dla tego węzła. Zwraca siebie do łańcuchowania.
	pub fn set_no_copypast(&mut self) -> &mut Self {
		self.no_copypast = true;
		self
	}
}
