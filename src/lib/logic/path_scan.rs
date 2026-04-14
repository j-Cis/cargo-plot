use walkdir::WalkDir;

use super::{PathCanonicalCtx, PathNode};

/// Statystyki skanowania systemu plików
#[derive(Debug, Clone)]
pub struct PathScanStat {
	pub count_files: usize,
	pub count_folder: usize,
	pub count_empty: usize,
	pub relation: PathCanonicalCtx,
}

/// Skaner systemu plików (warstwa IO)
#[derive(Debug, Clone)]
pub struct PathScan {
	pub files: Vec<PathNode>,
	pub dirs: Vec<PathNode>,
	pub stat: PathScanStat,
}

impl PathScan {
	pub fn scan(relation: &PathCanonicalCtx) -> Self {
		let mut files = Vec::new();
		let mut dirs = Vec::new();

		let mut count_files = 0;
		let mut count_folder = 0;
		let mut count_empty = 0;

		let root = &relation.select_dir.buf;

		for e in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
			if e.depth() == 0 {
				continue;
			}

			if e.path_is_symlink() {
				continue;
			}

			let Ok(rel_path) = e.path().strip_prefix(root) else {
				continue;
			};

			let mut path = rel_path.to_string_lossy().replace('\\', "/");

			if e.file_type().is_dir() {
				path.push('/');
				count_folder += 1;

				let is_empty = e.metadata().map(|m| m.is_dir()).unwrap_or(false)
					&& e.path().read_dir().map(|mut r| r.next().is_none()).unwrap_or(false);

				if is_empty {
					count_empty += 1;
				}

				dirs.push(PathNode::new(format!("./{}", path).into()));
			} else {
				count_files += 1;

				files.push(PathNode::new(format!("./{}", path).into()));
			}
		}

		files.sort_unstable_by(|a, b| a.str.cmp(&b.str));
		dirs.sort_unstable_by(|a, b| a.str.cmp(&b.str));

		let stat = PathScanStat { count_files, count_folder, count_empty, relation: relation.clone() };

		Self { files, dirs, stat }
	}

	pub fn files(&self) -> &[PathNode] { &self.files }

	pub fn dirs(&self) -> &[PathNode] { &self.dirs }

	fn to_strs(v: &[PathNode]) -> impl Iterator<Item = &str> + '_ { v.iter().map(|p| p.str.as_str()) }

	pub fn file_strs(&self) -> impl Iterator<Item = &str> + '_ { Self::to_strs(&self.files) }

	pub fn dir_strs(&self) -> impl Iterator<Item = &str> + '_ { Self::to_strs(&self.dirs) }
}
