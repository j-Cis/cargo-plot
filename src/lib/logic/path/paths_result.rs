use std::{collections::HashSet, marker::PhantomData};

// Wciągamy czyste, domenowe nazwy
use crate::lib::logic::{
	AnchoredPathsDatum,
	PattEnvIndex,
	PatternsToApply,
	ScannedToApply,
	TabColumn,
	TabPathStructure,
	TabSortBy,
	TabSortOrder,
	TabSpec,
	TableData,
	TableSotcTreeOutput,
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
pub struct Partition<L: MatchLabel> {
	pub paths: Vec<String>,
	pub label: &'static str,
	pub entry: AnchoredPathsDatum,
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
pub struct PartitioningResult {
	pub scanner: ScannedToApply,
	pub patterns: PatternsToApply,

	pub m: Partition<Matched>,
	pub x: Partition<Mismatched>,

	// Przechowuje recepturę jak zbudować wyjściową tabelę
	pub spec: TabSpec,
}
impl PartitioningResult {
	pub fn new(scanner: ScannedToApply, patterns: PatternsToApply, anchored: AnchoredPathsDatum) -> Self {
		let env_index =
			EnvIndex { dirs: scanner.iter_dir_paths().collect(), files: scanner.iter_file_paths().collect() };

		let mut m_vec = Vec::new();
		let mut x_vec = Vec::new();

		let all_paths = scanner.iter_file_paths().chain(scanner.iter_dir_paths());

		for p in all_paths {
			// p jest tutaj &str pochodzącym z node.path.str
			if patterns.is_match(p, &env_index) {
				m_vec.push(p.to_string());
			} else {
				x_vec.push(p.to_string());
			}
		}

		Self {
			scanner,
			patterns,
			m: Partition { paths: m_vec, label: Matched::label(), entry: anchored.clone(), _marker: PhantomData },
			x: Partition { paths: x_vec, label: Mismatched::label(), entry: anchored.clone(), _marker: PhantomData },
			spec: TabSpec::default(),
		}
	}

	// ============================================================================
	// BUILDER API (Konfiguracja specyfikacji w miejscu)
	// ============================================================================

	pub fn sort(mut self, by: TabSortBy, order: TabSortOrder, structure: TabPathStructure) -> Self {
		self.spec = self.spec.sort(by, order, structure);
		self
	}

	pub fn columns(mut self, cols: &[TabColumn]) -> Self {
		self.spec = self.spec.columns(cols);
		self
	}

	pub fn trim(mut self, size: usize, page: Option<usize>) -> Self {
		self.spec = self.spec.trim(size, page);
		self
	}

	// ============================================================================
	// LENIWA MATERIALIZACJA (Fizyczny odczyt FS na wybranej grupie)
	// ============================================================================

	pub fn build_matched(&self) -> TableSotcTreeOutput {
		TableData::gather(&self.m)
			.sort(self.spec.sort_by, self.spec.sort_order, self.spec.structure)
			.into_output(&self.spec)
	}

	pub fn build_mismatched(&self) -> TableSotcTreeOutput {
		TableData::gather(&self.x)
			.sort(self.spec.sort_by, self.spec.sort_order, self.spec.structure)
			.into_output(&self.spec)
	}
}
