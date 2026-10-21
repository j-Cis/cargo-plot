//! Path: `src/libs/querypath/structs/path_context.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Analiza relacji wewnątrz ścieżki tekstowej bez ani jednej alokacji na
/// stercie. Działa w 100% na pożyczonych wycinkach referencji (`&str`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathContext<'a> {
	pub parent: &'a str,
	pub file: &'a str,
}

impl<'a> PathContext<'a> {
	/// Dzieli ścieżkę na katalog nadrzędny i plik/katalog docelowy.
	#[must_use]
	pub fn from_path(path: &'a str) -> Self {
		let clean_path = path.trim_start_matches("./");

		let (parent, file) = match clean_path.rsplit_once('/') {
			Some((p, f)) => (p, f),
			std::option::Option::None => ("", clean_path),
		};

		Self { parent, file }
	}

	pub fn name(&self) -> &'a str {
		self.file
	}

	pub fn parent(&self) -> &'a str {
		self.parent
	}

	pub fn is_root_level(&self) -> bool {
		self.parent.is_empty()
	}
}
