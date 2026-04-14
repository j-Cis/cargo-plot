use std::collections::HashSet;

// Wciągamy czyste, domenowe nazwy
use super::table_data::{TableData, TableOutput};
use super::{
	PathCanonicalCtx,
	PathScan,
	PathsPatterns,
	PattEnvIndex,
	table_spec::{TabColumn, TabSortBy, TabSortOrder, TableSpec},
};

// ============================================================================
// SEMANTYKA TRUE/FALSE
// ============================================================================

pub trait MatchLabel {
	fn label() -> &'static str;
}

#[derive(Debug, Clone)]
pub struct Matched;
#[derive(Debug, Clone)]
pub struct Mismatched;

impl MatchLabel for Matched {
	fn label() -> &'static str { "✔" } // ✔️
}

impl MatchLabel for Mismatched {
	fn label() -> &'static str { "✖" } // ✖️
}

/// ============================================================================
/// FILTER LIST
/// ============================================================================

#[derive(Debug, Clone)]
pub struct FilterList<L: MatchLabel> {
	pub paths: Vec<String>,
	pub label: &'static str,
	pub entry: PathCanonicalCtx,
	pub _marker: std::marker::PhantomData<L>,
}

// ============================================================================
// ENV INDEX IMPLEMENTATION (Zero-copy, Binary Search)
// ============================================================================

struct EnvIndex<'a> {
	pub dirs: HashSet<&'a str>,
	pub files: Vec<&'a str>,
}

impl<'a> PattEnvIndex for EnvIndex<'a> {
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

// ============================================================================
// RESULT SCAN PATTERNS
// ============================================================================

#[derive(Debug, Clone)]
pub struct ResultScanPatterns {
	pub scanner: PathScan,
	pub patterns: PathsPatterns,

	pub m: FilterList<Matched>,
	pub x: FilterList<Mismatched>,

	// Przechowuje recepturę jak zbudować wyjściową tabelę
	pub spec: TableSpec,
}

impl ResultScanPatterns {
	pub fn new(scanner: PathScan, patterns: PathsPatterns) -> Self {
		let env_index = EnvIndex {
			dirs: scanner.dirs.iter().map(|n| n.str.as_str()).collect(),
			files: scanner.files.iter().map(|n| n.str.as_str()).collect(),
		};

		let mut m_vec = Vec::new();
		let mut x_vec = Vec::new();

		let all_paths = scanner.files.iter().chain(scanner.dirs.iter());

		for node in all_paths {
			let p = node.str.as_str();
			if patterns.is_match(p, &env_index) {
				m_vec.push(node.str.clone());
			} else {
				x_vec.push(node.str.clone());
			}
		}

		let entry = scanner.stat.relation.clone();

		Self {
			scanner,
			patterns,
			m: FilterList {
				paths: m_vec,
				label: Matched::label(),
				entry: entry.clone(),
				_marker: std::marker::PhantomData,
			},
			x: FilterList { paths: x_vec, label: Mismatched::label(), entry, _marker: std::marker::PhantomData },
			spec: TableSpec::default(),
		}
	}

	// ============================================================================
	// BUILDER API (Konfiguracja specyfikacji w miejscu)
	// ============================================================================

	pub fn sort(mut self, by: TabSortBy, order: TabSortOrder, is_tree: bool) -> Self {
		self.spec = self.spec.sort(by, order, is_tree);
		self
	}

	pub fn columns(mut self, cols: &[TabColumn]) -> Self {
		self.spec = self.spec.columns(cols);
		self
	}

	pub fn limit(mut self, n: usize) -> Self {
		self.spec = self.spec.limit(n);
		self
	}

	// ============================================================================
	// LENIWA MATERIALIZACJA (Fizyczny odczyt FS na wybranej grupie)
	// ============================================================================

	pub fn build_matched(&self) -> TableOutput {
		TableData::gather(&self.m)
			.sort(self.spec.sort_by, self.spec.sort_order, self.spec.is_tree)
			.into_output(&self.spec)
	}

	pub fn build_mismatched(&self) -> TableOutput {
		TableData::gather(&self.x)
			.sort(self.spec.sort_by, self.spec.sort_order, self.spec.is_tree)
			.into_output(&self.spec)
	}
}
