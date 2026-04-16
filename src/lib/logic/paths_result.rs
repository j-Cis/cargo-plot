/*
 * use std::{collections::HashSet, marker::PhantomData};
 *
 * // Wciągamy czyste, domenowe nazwy
 * use crate::lib::logic::{
 * 	AnchoredPathsDatum,
 * 	PattEnvIndex,
 * 	PatternsQueries,
 * 	PartitionScanned,
 * 	TabColumn,
 * 	TabPathStructure,
 * 	TabSortBy,
 * 	TabSortOrder,
 * 	TabSpec,
 * 	TableData,
 * 	TableSotcTreeOutput,
 * };
 *
 * // ============================================================================
 * // SEMANTYKA TRUE/FALSE
 * // ============================================================================
 *
 * pub trait MatchLabel {
 * 	fn label() -> &'static str;
 * }
 *
 * #[derive(Debug, Clone)]
 * pub struct Matched;
 * #[derive(Debug, Clone)]
 * pub struct Mismatched;
 *
 * impl MatchLabel for Matched {
 * 	fn label() -> &'static str { "✔" } // ✔️
 * }
 *
 * impl MatchLabel for Mismatched {
 * 	fn label() -> &'static str { "✖" } // ✖️
 * }
 *
 * /// ============================================================================
 * /// FILTER LIST
 * /// ============================================================================
 *
 * #[derive(Debug, Clone)]
 * pub struct Partition<L: MatchLabel> {
 * 	pub paths: Vec<String>,
 * 	pub label: &'static str,
 * 	pub _marker: std::marker::PhantomData<L>,
 * }
 *
 * // ============================================================================
 * // ENV INDEX IMPLEMENTATION (Zero-copy, Binary Search)
 * // ============================================================================
 *
 * struct EnvIndex<'a> {
	pub dirs: HashSet<&'a str>,
	pub files: Vec<&'a str>,
}
 *
 * impl<'a> PattEnvIndex for EnvIndex<'a> {
 * 	fn has_dir(&self, dir: &str) -> bool { self.dirs.contains(dir) }
 *
 * 	fn has_file_with_prefix(&self, prefix: &str) -> bool {
 * 		let start = self.files.partition_point(|&f| f < prefix);
 * 		start < self.files.len() && self.files[start].starts_with(prefix)
 * 	}
 *
 * 	fn any_file_in_dir(&self, dir: &str, check: &mut dyn FnMut(&str) -> bool) -> bool {
 * 		let start = self.files.partition_point(|&f| f < dir);
 * 		for &f in &self.files[start..] {
 * 			if !f.starts_with(dir) {
 * 				break;
 * 			}
 * 			if check(f) {
 * 				return true;
 * 			}
 * 		}
 * 		false
 * 	}
 * }
 *
 * // ============================================================================
 * // STATS
 * // ============================================================================
 *
 * #[derive(Debug, Clone)]
 * pub struct StatsPartitioning {
 *     pub count_m: usize,
 *     pub count_x: usize,
 * 	pub count_m_f: usize,
 * 	pub count_x_f: usize,
 * 	pub count_m_d: usize,
 * 	pub count_x_d: usize,
 * }
 * // ============================================================================
 * // RESULT SCAN PATTERNS
 * // ============================================================================
 *
 * #[derive(Debug, Clone)]
 * pub struct PartitioningResult {
 * 	pub scanner: PartitionScanned,
 * 	pub patterns: PatternsQueries,
 * 	pub stat: StatsPartitioning,
 *
 * 	pub m: Partition<Matched>,
 * 	pub x: Partition<Mismatched>,
 *
 * 	// Przechowuje recepturę jak zbudować wyjściową tabelę
 * 	pub spec: TabSpec,
 * }
 * impl PartitioningResult {
 * 	pub fn new(raw_scanned: PartitionScanned, raw_patterns: PatternsQueries, anchored: AnchoredPathsDatum) -> Self {
 * 		let env_index =
 * 			EnvIndex {
 * 				dirs: raw_scanned.iter_dir_paths().collect(),
 * 				files: raw_scanned.iter_file_paths().collect()
 * 			};
 *
 * 		let mut m_vec = Vec::new();
 * 		let mut x_vec = Vec::new();
 * 		let mut m_vec_f = Vec::new();
 *         let mut m_vec_d = Vec::new();
 *         let mut x_vec_f = Vec::new();
 *         let mut x_vec_d = Vec::new();
 *
 * 		// for n in raw_scanned.iter_nodes() {
 * 		for p in raw_scanned.iter_paths() {
 * 			// p jest tutaj &str pochodzącym z node.path.str
 * 			if raw_patterns.is_match(p, &env_index) {
 * 				m_vec.push(p.to_string());
 * 			} else {
 * 				x_vec.push(p.to_string());
 * 			}
 * 		}
 *
 * 		// Iterujemy po zdenormalizowanych węzłach z fs_scanner.rs
 *         for n in raw_scanned.iter_nodes() {
 *             let p_str = n.path.str.as_str();
 *
 *             // Decydujemy, do którego "worka" trafia element
 *             let is_matched = raw_patterns.is_match(p_str, &env_index);
 *
 *             // Odtwarzamy konkretny typ węzła na podstawie flagi node
 *             match n.node {
 *                 crate::lib::logic::NodeIs::File => {
 *                     let file_node = crate::lib::logic::ScannedFileNode {
 *                         path: n.path.clone(),
 *                         is_binary: n.file_is_binary.unwrap_or(false),
 *                         is_empty: n.file_is_empty.unwrap_or(false),
 *                     };
 *                     if is_matched {
 *                         m_vec_f.push(file_node);
 *                     } else {
 *                         x_vec_f.push(file_node);
 *                     }
 *                 }
 *                 crate::lib::logic::NodeIs::Dir => {
 *                     let dir_node = crate::lib::logic::ScannedDirNode {
 *                         path: n.path.clone(),
 *                         has_subdirs: n.dir_has_subdirs.unwrap_or(false),
 *                         has_files_binary: n.dir_has_files_binary.unwrap_or(false),
 *                         has_files_text: n.dir_has_files_text.unwrap_or(false),
 *                         has_symlinks: n.dir_has_symlinks.unwrap_or(false),
 *                     };
 *                     if is_matched {
 *                         m_vec_d.push(dir_node);
 *                     } else {
 *                         x_vec_d.push(dir_node);
 *                     }
 *                 }
 *             }
 *         }
 *
 * 		let stat = StatsPartitioning {
 *             count_m: m_vec_f.len() + m_vec_d.len(),
 *             count_x: x_vec_f.len() + x_vec_d.len(),
 *             count_m_f: m_vec_f.len(),
 *             count_x_f: x_vec_f.len(),
 *             count_m_d: m_vec_d.len(),
 *             count_x_d: x_vec_d.len(),
 *         };
 *
 * 		Self {
 * 			scanner: raw_scanned,
 * 			patterns: raw_patterns,
 * 			stat,
 * 			m: Partition { paths: m_vec, label: Matched::label(), _marker: PhantomData },
 * 			x: Partition { paths: x_vec, label: Mismatched::label(), _marker: PhantomData },
 * 			spec: TabSpec::default(),
 * 		}
 * 	}
 *
 * 	// ============================================================================
 * 	// BUILDER API (Konfiguracja specyfikacji w miejscu)
 * 	// ============================================================================
 *
 * 	pub fn sort(mut self, by: TabSortBy, order: TabSortOrder, structure: TabPathStructure) -> Self {
 * 		self.spec = self.spec.sort(by, order, structure);
 * 		self
 * 	}
 *
 * 	pub fn columns(mut self, cols: &[TabColumn]) -> Self {
 * 		self.spec = self.spec.columns(cols);
 * 		self
 * 	}
 *
 * 	pub fn trim(mut self, size: usize, page: Option<usize>) -> Self {
 * 		self.spec = self.spec.trim(size, page);
 * 		self
 * 	}
 *
 * 	// ============================================================================
 * 	// LENIWA MATERIALIZACJA (Fizyczny odczyt FS na wybranej grupie)
 * 	// ============================================================================
 *
 * 	pub fn build_matched(&self) -> TableSotcTreeOutput {
 * 		TableData::gather(&self.m)
 * 			.sort(self.spec.sort_by, self.spec.sort_order, self.spec.structure)
 * 			.into_output(&self.spec)
 * 	}
 *
 * 	pub fn build_mismatched(&self) -> TableSotcTreeOutput {
 * 		TableData::gather(&self.x)
 * 			.sort(self.spec.sort_by, self.spec.sort_order, self.spec.structure)
 * 			.into_output(&self.spec)
 * 	}
 * }
 *
 * */
