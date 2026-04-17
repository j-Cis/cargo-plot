// 3
use std::{
	collections::{HashMap, HashSet},
	fs,
	io::{self, Read},
	marker::PhantomData,
};

use memchr::memchr;
use walkdir::WalkDir;

// Upewnij się, że PatternsQueries i PattEnvIndex są zaimportowane poprawnie
use crate::lib::logic::{AnchoredPathsDatum, PathNode, PattEnvIndex, PatternsQueries};

const CHUNK_SIZE: usize = 4096;
const MAX_SCAN: usize = 16 * 1024; // 16 KB wystarcza w 99.9%

#[inline(always)]
pub fn is_binary_file(path: &str) -> io::Result<bool> {
	let mut file = fs::File::open(path)?;
	let mut buf = [0u8; CHUNK_SIZE];

	let mut total_read = 0usize;
	let mut bad = 0usize;

	loop {
		let n = file.read(&mut buf)?;
		if n == 0 {
			break;
		}

		let data = &buf[..n];
		total_read += n;

		if memchr(0, data).is_some() {
			return Ok(true);
		}

		for &b in data {
			if is_control_byte(b) {
				bad += 1;
			}
		}

		if bad * 100 > total_read {
			return Ok(true);
		}
		if total_read >= MAX_SCAN {
			break;
		}
	}

	Ok(false)
}

#[inline(always)]
fn is_control_byte(b: u8) -> bool { matches!(b, 0x00..=0x08 | 0x0B | 0x0C | 0x0E..=0x1F) }

// ============================================================================
// PARTITIONS & MARKERS
// ============================================================================

pub trait MatchLabel {
	type Node;
	fn label() -> &'static str;
}

#[derive(Debug, Clone)]
pub struct MatchedFile;
#[derive(Debug, Clone)]
pub struct MismatchedFile;
#[derive(Debug, Clone)]
pub struct MatchedDir;
#[derive(Debug, Clone)]
pub struct MismatchedDir;

impl MatchLabel for MatchedFile {
	type Node = ScannedFileNode; // Związane z węzłem pliku!
	fn label() -> &'static str { "✔F" } // ✔️📄
}

impl MatchLabel for MismatchedFile {
	type Node = ScannedFileNode; // Związane z węzłem pliku!
	fn label() -> &'static str { "✖F" } // ✖️📄
}

impl MatchLabel for MatchedDir {
	type Node = ScannedDirNode; // Związane z węzłem katalogu!
	fn label() -> &'static str { "✔D" } // ✔️📂
}

impl MatchLabel for MismatchedDir {
	type Node = ScannedDirNode; // Związane z węzłem katalogu!
	fn label() -> &'static str { "✖D" } // ✖️📂
}

#[derive(Debug, Clone)]
pub struct Partition<L: MatchLabel> {
	pub nodes: Vec<L::Node>, // Kompilator sam podstawi tu ScannedFileNode lub ScannedDirNode
	pub label: &'static str,
	pub tier_max: usize,
	pub name_len_max: usize,
	pub path_len_max: usize,
	pub count: usize,
	_marker: PhantomData<L>,
}

impl<L: MatchLabel> Partition<L> {
	// Pomocniczy trait-like look dla L::Node, aby dobrać się do pól tier i name
	pub fn calculate_metadata(nodes: &[L::Node]) -> (usize, usize, usize)
	where L::Node: AsMetadataNode // Musimy zdefiniować ten pomocniczy trait poniżej
	{
		let t_max = nodes.iter().map(|n| n.get_tier()).max().unwrap_or(0);
		let n_max = nodes.iter().map(|n| n.get_name().chars().count()).max().unwrap_or(0);
		let p_max = nodes.iter().map(|n| n.get_path_len()).max().unwrap_or(0); // ⚡ NOWE
		(t_max, n_max, p_max)
	}
}
// Dodaj ten pomocniczy trait w fs_scanner.rs, aby Partition mogło operować na typach generycznych
pub trait AsMetadataNode {
	fn get_tier(&self) -> usize;
	fn get_name(&self) -> &str;
	fn get_path_len(&self) -> usize;
}

impl AsMetadataNode for ScannedFileNode {
	fn get_tier(&self) -> usize { self.tier }
	fn get_name(&self) -> &str { &self.name }
	fn get_path_len(&self) -> usize { self.path.str.chars().count() }
}

impl AsMetadataNode for ScannedDirNode {
	fn get_tier(&self) -> usize { self.tier }
	fn get_name(&self) -> &str { &self.name }
	fn get_path_len(&self) -> usize { self.path.str.chars().count() }
}
// ============================================================================
// STRUKTURY DANYCH I STATYSTYK
// ============================================================================

#[derive(Debug, Clone)]
pub struct StatsScannedTreeFs {
	pub count_dirs: usize,
	pub count_dirs_empty: usize,
	pub count_files: usize,
	pub count_files_empty: usize,
	pub count_files_text: usize,
	pub count_files_binary: usize,
	pub count_symlinks_skipped: usize,
}

#[derive(Debug, Clone)]
pub struct StatsPartitioning {
	pub count_m: usize,
	pub count_x: usize,
	pub count_m_f: usize,
	pub count_x_f: usize,
	pub count_m_d: usize,
	pub count_x_d: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeIs {
	File,
	Dir,
}

#[derive(Debug, Clone)]
pub struct ScannedNode {
	pub name: String,
	pub path: PathNode,
	pub node: NodeIs,
	pub tier: usize,
	pub id_self: usize,      // ⚡ Własne ID
	pub id_path: Vec<usize>, // ⚡ Ścieżka przodków [0, 1, 5...]
	pub dir_has_subdirs: Option<usize>,
	pub dir_has_files_binary: Option<usize>,
	pub dir_has_files_text: Option<usize>,
	pub dir_has_symlinks: Option<usize>,
	pub file_is_binary: Option<bool>,
	pub file_is_empty: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ScannedDirNode {
	pub name: String,
	pub path: PathNode,
	pub tier: usize,
	pub id_self: usize,      // ⚡
	pub id_path: Vec<usize>, // ⚡
	pub has_subdirs: usize,
	pub has_files_binary: usize,
	pub has_files_text: usize,
	pub has_symlinks: usize,
}

#[derive(Debug, Clone)]
pub struct ScannedFileNode {
	pub name: String,
	pub path: PathNode,
	pub tier: usize,
	pub id_self: usize,      // ⚡
	pub id_path: Vec<usize>, // ⚡
	pub is_binary: bool,
	pub is_empty: bool,
}

// ============================================================================
// ENV INDEX (Baza dla filtrowania)
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
// GŁÓWNY OBIEKT WYNIKOWY
// ============================================================================

/// Skaner i filter systemu plików (Zunifikowane)
#[derive(Debug, Clone)]
pub struct PartitionScanned {
	// Rozbite na konkretne typy partycje (pozbywamy się sztucznego markera Partition)
	pub m_f: Partition<MatchedFile>,
	pub x_f: Partition<MismatchedFile>,
	pub m_d: Partition<MatchedDir>,
	pub x_d: Partition<MismatchedDir>,

	pub stat_scan: StatsScannedTreeFs,
	pub stat_part: StatsPartitioning,
	pub stat_patt: Vec<String>,
	pub stat_work: String,
}

impl PartitionScanned {
	pub fn scan(p: &AnchoredPathsDatum, q: &PatternsQueries) -> Self {
		// ░░░░░░░░░░░░░░░ 🅰️ ZBIERANIE DANYCH ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
		let raw_data = gather_raw_nodes(p);
		let files = raw_data.files;
		let dirs = build_directory_nodes(&raw_data.dirs_raw, &files, &raw_data.symlink_counts);
		let stat_scan = raw_data.stat_scan;

		// Zbudowanie EnvIndex na referencjach (pożyczamy ścieżki)
		let env_index = EnvIndex {
			dirs: dirs.iter().map(|d| d.path.str.as_str()).collect(),
			files: files.iter().map(|f| f.path.str.as_str()).collect(),
		};

		// ░░░░░░░░░░░░░░░ 🅱️ PIERWSZY PRZEBIEG ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
		// zbieramy TYLKO wyniki true/false do masek
		let files_matches: Vec<bool> = files.iter().map(|f| q.is_match(&f.path.str, &env_index)).collect();
		let dirs_matches: Vec<bool> = dirs.iter().map(|d| q.is_match(&d.path.str, &env_index)).collect();

		// Uwalniamy referencje
		drop(env_index);

		let mut vec_m_f = Vec::new();
		let mut vec_x_f = Vec::new();
		let mut vec_m_d = Vec::new();
		let mut vec_x_d = Vec::new();

		// ░░░░░░░░░░░░░░░ 🆎 DRUGI PRZEBIEG ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
		// złączenie plików z zapisanymi wynikami
		for (f, is_matched) in files.into_iter().zip(files_matches) {
			if is_matched {
				vec_m_f.push(f);
			} else {
				vec_x_f.push(f);
			}
		}

		for (d, is_matched) in dirs.into_iter().zip(dirs_matches) {
			if is_matched {
				vec_m_d.push(d);
			} else {
				vec_x_d.push(d);
			}
		}

		// ░░░░░░░░░░░░░░░ 🅾️ MONTOWANIE WYNIKU ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
		let stat_part = StatsPartitioning {
			count_m: vec_m_f.len() + vec_m_d.len(),
			count_x: vec_x_f.len() + vec_x_d.len(),
			count_m_f: vec_m_f.len(),
			count_x_f: vec_x_f.len(),
			count_m_d: vec_m_d.len(),
			count_x_d: vec_x_d.len(),
		};

		let (mf_t, mf_n, mf_p) = Partition::<MatchedFile>::calculate_metadata(&vec_m_f);
		let (xf_t, xf_n, xf_p) = Partition::<MismatchedFile>::calculate_metadata(&vec_x_f);
		let (md_t, md_n, md_p) = Partition::<MatchedDir>::calculate_metadata(&vec_m_d);
		let (xd_t, xd_n, xd_p) = Partition::<MismatchedDir>::calculate_metadata(&vec_x_d);

		Self {
			m_f: Partition {
				nodes: vec_m_f,
				label: MatchedFile::label(),
				tier_max: mf_t,
				name_len_max: mf_n,
				path_len_max: mf_p,
				count: stat_part.count_m_f,
				_marker: PhantomData,
			},
			x_f: Partition {
				nodes: vec_x_f,
				label: MismatchedFile::label(),
				tier_max: xf_t,
				name_len_max: xf_n,
				path_len_max: xf_p,
				count: stat_part.count_x_f,
				_marker: PhantomData,
			},
			m_d: Partition {
				nodes: vec_m_d,
				label: MatchedDir::label(),
				tier_max: md_t,
				name_len_max: md_n,
				path_len_max: md_p,
				count: stat_part.count_m_d,
				_marker: PhantomData,
			},
			x_d: Partition {
				nodes: vec_x_d,
				label: MismatchedDir::label(),
				tier_max: xd_t,
				name_len_max: xd_n,
				path_len_max: xd_p,
				count: stat_part.count_x_d,
				_marker: PhantomData,
			},
			stat_scan,
			stat_part,
			stat_patt: q.patterns.0.clone(),
			stat_work: p.workspace_dir.str.clone(),
		}
	}

	// =========================================================================
	// HELPER
	// =========================================================================

	/// Zwraca połączone i posortowane struktury ScannedNode (pliki + katalogi).
	pub fn to_scanned_nodes(&self) -> Vec<ScannedNode> {
		let total_len = self.stat_part.count_m + self.stat_part.count_x;
		let mut unified = Vec::with_capacity(total_len);

		let map_dir = |d: &ScannedDirNode| ScannedNode {
			name: d.name.clone(), // NOWE
			path: d.path.clone(),
			node: NodeIs::Dir,
			tier: d.tier,
			id_self: d.id_self,         // ⚡
			id_path: d.id_path.clone(), // ⚡
			dir_has_subdirs: Some(d.has_subdirs),
			dir_has_files_binary: Some(d.has_files_binary),
			dir_has_files_text: Some(d.has_files_text),
			dir_has_symlinks: Some(d.has_symlinks),
			file_is_binary: None,
			file_is_empty: None,
		};

		let map_file = |f: &ScannedFileNode| ScannedNode {
			name: f.name.clone(), // NOWE
			path: f.path.clone(),
			node: NodeIs::File,
			tier: f.tier,
			id_self: f.id_self,         // ⚡
			id_path: f.id_path.clone(), // ⚡
			dir_has_subdirs: None,
			dir_has_files_binary: None,
			dir_has_files_text: None,
			dir_has_symlinks: None,
			file_is_binary: Some(f.is_binary),
			file_is_empty: Some(f.is_empty),
		};

		unified.extend(self.m_d.nodes.iter().chain(self.x_d.nodes.iter()).map(map_dir));
		unified.extend(self.m_f.nodes.iter().chain(self.x_f.nodes.iter()).map(map_file));

		unified.sort_unstable_by(|a, b| a.path.str.cmp(&b.path.str));
		unified
	}

	// =========================================================================
	// HELPERY I ITERATORY
	// =========================================================================

	pub fn iter_nodes(&self) -> impl Iterator<Item = ScannedNode> + '_ { self.to_scanned_nodes().into_iter() }

	pub fn iter_file_paths(&self) -> impl Iterator<Item = &str> + '_ {
		self.m_f.nodes.iter().chain(self.x_f.nodes.iter()).map(|n| n.path.str.as_str())
	}

	pub fn iter_dir_paths(&self) -> impl Iterator<Item = &str> + '_ {
		self.m_d.nodes.iter().chain(self.x_d.nodes.iter()).map(|n| n.path.str.as_str())
	}

	pub fn iter_paths(&self) -> impl Iterator<Item = &str> + '_ { self.iter_file_paths().chain(self.iter_dir_paths()) }
}

// ============================================================================
// HELPERY SKANERA (Przygotowanie danych)
// ============================================================================

struct RawDirNode {
	path: PathNode,
	tier: usize,
	id_self: usize,
	id_path: Vec<usize>,
}
struct RawGatherData {
	files: Vec<ScannedFileNode>,
	dirs_raw: Vec<RawDirNode>,
	symlink_counts: HashMap<String, usize>,
	stat_scan: StatsScannedTreeFs,
}

/// Helper 1: Przechodzi po dysku, kategoryzuje i zbiera surowe węzły
fn gather_raw_nodes(p: &AnchoredPathsDatum) -> RawGatherData {
	let mut files = Vec::new();
	let mut dirs_raw = Vec::new();
	let mut symlink_counts = HashMap::new();

	let mut count_dirs = 0;
	let mut count_dirs_empty = 0;
	let mut count_files = 0;
	let mut count_files_empty = 0;
	let mut count_files_text = 0;
	let mut count_files_binary = 0;
	let mut count_symlinks_skipped = 0;

	// ⚡ REJESTR ID I ŚCIEŻEK (Materialized Path)
	let mut current_id = 1; // Zaczynamy od 1, bo 0 to workspace_dir
	let mut path_registry: HashMap<std::path::PathBuf, (usize, Vec<usize>)> = HashMap::new();

	// Inicjalizujemy korzeń (workspace_dir) z ID 0 i pustą ścieżką przodków
	path_registry.insert(p.workspace_dir.buf.clone(), (0, vec![]));

	for e in WalkDir::new(&p.workspace_dir.buf).into_iter().filter_map(|e| e.ok()) {
		if e.depth() == 0 {
			continue;
		}

		let tier = e.depth();

		if e.path_is_symlink() {
			count_symlinks_skipped += 1;
			if let Some(parent) = e.path().strip_prefix(&p.workspace_dir.buf).ok().and_then(|p| p.parent()) {
				let parent_str = parent.to_string_lossy().replace('\\', "/");
				let formatted = if parent_str.is_empty() { "./".to_string() } else { format!("./{}/", parent_str) };
				*symlink_counts.entry(formatted).or_insert(0usize) += 1;
			}
			continue;
		}

		// ⚡ NADAWANIE TOŻSAMOŚCI I BUDOWANIE ID_PATH
		let id_self = current_id;
		current_id += 1;

		// Szukamy rodzica w rejestrze
		let parent_path = e.path().parent().expect("Błąd systemu plików: Węzeł nie ma rodzica");
		let (parent_id, parent_id_path) = path_registry.get(parent_path).unwrap_or(&(0, vec![])).clone();

		// Budujemy ścieżkę z ID: bierzemy ścieżkę rodzica i doklejamy do niej ID rodzica
		// Np. Rodzic to folder `src` (ID: 1, ścieżka: [0]). Nasza nowa ścieżka to [0, 1].
		let mut id_path = parent_id_path;
		id_path.push(parent_id);

		// Jeśli to folder, wpisujemy go do rejestru dla jego przyszłych dzieci
		if e.file_type().is_dir() {
			path_registry.insert(e.path().to_path_buf(), (id_self, id_path.clone()));
		}

		let Ok(rel_path) = e.path().strip_prefix(&p.workspace_dir.buf) else {
			continue;
		};
		let mut path = rel_path.to_string_lossy().replace('\\', "/");

		if e.file_type().is_dir() {
			path.push('/');
			count_dirs += 1;

			let is_empty = e.metadata().map(|m| m.is_dir()).unwrap_or(false)
				&& e.path().read_dir().map(|mut r| r.next().is_none()).unwrap_or(false);

			if is_empty {
				count_dirs_empty += 1;
			}
			dirs_raw.push(RawDirNode {
				path: PathNode::new(format!("./{}", path).into()),
				tier,
				id_self,
				id_path: id_path.clone(),
			});
		} else {
			count_files += 1;
			let abs_path = e.path();
			let is_empty = abs_path.metadata().map(|m| m.len() == 0).unwrap_or(false);
			let mut is_binary = false;

			if is_empty {
				count_files_empty += 1;
			} else {
				is_binary = is_binary_file(abs_path.to_str().unwrap_or("")).unwrap_or(false);
				if is_binary {
					count_files_binary += 1;
				} else {
					count_files_text += 1;
				}
			}

			files.push(ScannedFileNode {
				name: abs_path.file_name().unwrap_or_default().to_string_lossy().to_string(),
				path: PathNode::new(format!("./{}", path).into()),
				id_self,
				id_path,
				tier,
				is_binary,
				is_empty,
			});
		}
	}

	// Sortowanie gwarantujące stabilność dla logiki analizy dzieci
	files.sort_unstable_by(|a, b| a.path.str.cmp(&b.path.str));
	dirs_raw.sort_unstable_by(|a, b| a.path.str.cmp(&b.path.str));

	let stat_scan = StatsScannedTreeFs {
		count_dirs,
		count_dirs_empty,
		count_files,
		count_files_empty,
		count_files_text,
		count_files_binary,
		count_symlinks_skipped,
	};

	RawGatherData { files, dirs_raw, symlink_counts, stat_scan }
}

/// Helper 2: Analizuje relacje rodzic-dziecko i buduje pełne ScannedDirNode
fn build_directory_nodes(
	dirs_raw: &[RawDirNode],
	files: &[ScannedFileNode],
	symlink_counts: &HashMap<String, usize>,
) -> Vec<ScannedDirNode> {
	let mut dirs = Vec::with_capacity(dirs_raw.len());

	for i in 0..dirs_raw.len() {
		let raw = &dirs_raw[i];
		let mut count_subdirs = 0;
		let mut count_files_binary = 0;
		let mut count_files_text = 0;
		let count_symlinks = *symlink_counts.get(&raw.path.str).unwrap_or(&0);

		// Zliczanie plików bezpośrednich
		for f in files {
			if f.path.str.starts_with(&raw.path.str) {
				let remainder = &f.path.str[raw.path.str.len()..];
				if !remainder.contains('/') {
					if f.is_binary {
						count_files_binary += 1;
					} else if !f.is_empty {
						count_files_text += 1;
					}
				}
			}
		}

		// Zliczanie subkatalogów bezpośrednich
		for j in (i + 1)..dirs_raw.len() {
			if dirs_raw[j].path.str.starts_with(&raw.path.str) {
				let remainder = &dirs_raw[j].path.str[raw.path.str.len()..];
				if !remainder.is_empty() && !remainder.trim_end_matches('/').contains('/') {
					count_subdirs += 1;
				}
			} else {
				break;
			}
		}

		dirs.push(ScannedDirNode {
			name: raw.path.str.trim_end_matches('/').split('/').next_back().unwrap_or("").to_string(),
			path: raw.path.clone(),
			id_self: raw.id_self,         // ⚡ PRZEPISANIE ID
			id_path: raw.id_path.clone(), // ⚡ PRZEPISANIE ŚCIEŻKI
			tier: raw.tier,
			has_subdirs: count_subdirs,
			has_files_binary: count_files_binary,
			has_files_text: count_files_text,
			has_symlinks: count_symlinks,
		});
	}

	dirs
}
