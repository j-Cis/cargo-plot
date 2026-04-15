use std::{
	collections::HashSet,
	fs,
	io::{self, Read},
};

use memchr::memchr;
use walkdir::WalkDir;

use crate::lib::logic::{AnchoredPathsDatum, PathNode};

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

		// 1. natychmiastowy kill switch
		if memchr(0, data).is_some() {
			return Ok(true);
		}

		// 2. kontrolne bajty
		for &b in data {
			if is_control_byte(b) {
				bad += 1;
			}
		}

		// 3. early exit: jeśli już wiemy
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
fn is_control_byte(b: u8) -> bool {
	// allow: tab, lf, cr
	matches!(b,
		0x00..=0x08 |
		0x0B |
		0x0C |
		0x0E..=0x1F
	)
}

//=======================================================

/// Statystyki skanowania systemu plików
#[derive(Debug, Clone)]
pub struct StatsScannedTreeFs {
	pub count_dirs: usize,
	pub count_dirs_empty: usize,
	pub count_files: usize,
	pub count_files_empty: usize,
	pub count_files_text: usize,
	pub count_files_binary: usize,
	pub count_symlinks_skipped: usize,
	anchored: AnchoredPathsDatum,
}

impl StatsScannedTreeFs {
	/// Zwraca informację, do jakiej relacji ścieżek należą te statystyki
	pub fn where_scanned(&self) -> &PathNode { &self.anchored.workspace_dir }
	pub fn where_runtime(&self) -> &PathNode { &self.anchored.execution_dir }
}

//=======================================================

/* #[derive(Debug, Clone)]
 * pub struct ScannedNode {
 * pub path: PathNode,
 * pub is_symlink: bool,
 * pub is_file: bool,
 * pub is_dir: bool,
 * }
 * */

/// Reprezentacja foldery z wczesną informacją o jego zawartości
#[derive(Debug, Clone)]
pub struct ScannedDirNode {
	pub path: PathNode,
	pub has_subdirs: bool,
	pub has_files_binary: bool,
	pub has_files_text: bool,
	pub has_symlinks: bool,
}
/// Reprezentacja pliku z wczesną informacją o jego zawartości
#[derive(Debug, Clone)]
pub struct ScannedFileNode {
	pub path: PathNode,
	pub is_binary: bool,
	pub is_empty: bool,
}

/// Skaner systemu plików (warstwa IO)
#[derive(Debug, Clone)]
pub struct ScannedToApply {
	pub files: Vec<ScannedFileNode>,
	pub dirs: Vec<ScannedDirNode>,
	pub stat: StatsScannedTreeFs,
}

impl ScannedToApply {
	pub fn scan(p: &AnchoredPathsDatum) -> Self {
		let mut files = Vec::new();
		let mut dirs_raw = Vec::new();
		let mut symlink_parents = HashSet::new();

		let mut count_dirs = 0;
		let mut count_dirs_empty = 0;
		let mut count_files = 0;
		let mut count_files_empty = 0;
		let mut count_files_text = 0;
		let mut count_files_binary = 0;
		let mut count_symlinks_skipped = 0;

		for e in WalkDir::new(&p.workspace_dir.buf).into_iter().filter_map(|e| e.ok()) {
			if e.depth() == 0 {
				continue;
			}

			if e.path_is_symlink() {
				count_symlinks_skipped += 1;
				// Śledzenie w którym katalogu był symlink
				if let Some(parent) = e.path().strip_prefix(&p.workspace_dir.buf).ok().and_then(|p| p.parent()) {
					let parent_str = parent.to_string_lossy().replace('\\', "/");
					let formatted = if parent_str.is_empty() { "./".to_string() } else { format!("./{}/", parent_str) };
					symlink_parents.insert(formatted);
				}
				continue;
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

				dirs_raw.push(PathNode::new(format!("./{}", path).into()));
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

				files.push(ScannedFileNode { path: PathNode::new(format!("./{}", path).into()), is_binary, is_empty });
			}
		}

		files.sort_unstable_by(|a, b| a.path.str.cmp(&b.path.str));
		dirs_raw.sort_unstable_by(|a, b| a.str.cmp(&b.str));

		// Post-processing folderów
		let mut dirs = Vec::with_capacity(dirs_raw.len());
		for i in 0..dirs_raw.len() {
			let path = dirs_raw[i].clone();
			let mut has_subdirs = false;
			let mut has_files_binary = false;
			let mut has_files_text = false;
			let has_symlinks = symlink_parents.contains(&path.str);

			for f in &files {
				if f.path.str.starts_with(&path.str) {
					let remainder = &f.path.str[path.str.len()..];
					if !remainder.contains('/') {
						if f.is_binary {
							has_files_binary = true;
						} else if !f.is_empty {
							has_files_text = true;
						}
					}
				}
			}

			for j in (i + 1)..dirs_raw.len() {
				if dirs_raw[j].str.starts_with(&path.str) {
					let remainder = &dirs_raw[j].str[path.str.len()..];
					if !remainder.is_empty() {
						has_subdirs = true;
						break;
					}
				} else {
					break;
				}
			}

			dirs.push(ScannedDirNode { path, has_subdirs, has_files_binary, has_files_text, has_symlinks });
		}

		let stat = StatsScannedTreeFs {
			count_dirs,
			count_dirs_empty,
			count_files,
			count_files_empty,
			count_files_text,
			count_files_binary,
			count_symlinks_skipped,
			anchored: p.clone(),
		};

		Self { files, dirs, stat }
	}
	// =========================================================================
	// DOSTĘP TYLKO DO ŚCIEŻEK (Zwracają wygodne iteratory tekstowe)
	// =========================================================================

	/// Zwraca iterator przechodzący wyłącznie po tekstowych ścieżkach plików
	pub fn iter_file_paths(&self) -> impl Iterator<Item = &str> + '_ {
		self.files.iter().map(|node| node.path.str.as_str())
	}

	/// Zwraca iterator przechodzący wyłącznie po tekstowych ścieżkach katalogów
	pub fn iter_dir_paths(&self) -> impl Iterator<Item = &str> + '_ {
		self.dirs.iter().map(|node| node.path.str.as_str())
	}
}
