//! Path: `src/libs/querypath/features/step04_execute_query.rs`
//! Last significant modified: 2026-W43-3 19:07:00 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::{
	borrow::Borrow,
	collections::{HashMap, HashSet},
	fmt, fs,
};

use crate::querypath::{
	features::{HandleWalkScan, VerboseDisplay},
	structs::{DirItem, FileItem, PathsPatterns, WalkEnvIndex},
	traits::PattEnvIndex,
	utils::{check_is_binary, get_modified_time},
};
use anyhow::Result;

// ============================================================================
// GŁÓWNY EGZEKUTOR
// ============================================================================

pub struct QueryExecute {
	pub matched_files: Vec<FileItem>,
	pub matched_dirs: Vec<DirItem>,
	pub scanned_files: usize,
	pub scanned_dirs: usize,
}

impl QueryExecute {
	pub fn apply<P, W>(patterns: P, walk: W, keep_parent: bool, debug: bool) -> Result<Self>
	where
		P: Borrow<PathsPatterns>,
		W: Borrow<HandleWalkScan>,
	{
		// Odpakowanie referencji z generyków
		let patterns_ref = patterns.borrow();
		let walk_ref = walk.borrow();

		// 0. Zbieranie rozmiarów fizycznych wszystkich zeskanowanych plików
		let mut file_sizes: HashMap<String, u64> = HashMap::new();
		for file in walk_ref.iter_all_files() {
			let meta_size = fs::metadata(&file.str).map(|m| m.len()).unwrap_or(0);
			file_sizes.insert(file.str.clone(), meta_size);
		}

		// 1. env to pamięciowy indeks (cache) plików i katalogów
		//    przeskanowanych przez HandleWalkScan.
		let env = WalkEnvIndex::from_walk(walk_ref);

		// 2. Filtrowanie plików wzorcem oraz budowa FileItem
		let mut matched_files: Vec<FileItem> = Vec::new();
		for file in walk_ref.iter_all_files() {
			if patterns_ref.is_match(&file.str, &env) {
				let size = *file_sizes.get(&file.str).unwrap_or(&0);
				let is_binary = check_is_binary(&file.str);
				let modified_at = get_modified_time(&file.str);

				matched_files.push(FileItem { path: file.str.clone(), size, is_binary, modified_at });
			}
		}

		// 3. Budowa zbioru katalogów i ich wyliczanie
		let mut matched_dirs_set: HashSet<String> = walk_ref
			.iter_all_dirs()
			.map(|d| d.str.as_str())
			.filter(|d| patterns_ref.is_match(d, &env))
			.map(String::from)
			.collect();

		if keep_parent {
			for file_item in &matched_files {
				let mut current: &str = file_item.path.as_str();
				while let Some((parent, _)) = current.rsplit_once('/') {
					if parent.is_empty() {
						break;
					}
					let dir_candidate = format!("{parent}/");
					if env.has_dir(&dir_candidate) {
						matched_dirs_set.insert(dir_candidate);
					}
					current = parent;
				}
			}
		}

		// 4. Kalkulacja real_size oraz matched_size dla każdego katalogu
		let matched_files_map: HashMap<&str, u64> = matched_files.iter().map(|f| (f.path.as_str(), f.size)).collect();

		let mut matched_dir_items: Vec<DirItem> = Vec::new();

		for dir_path in matched_dirs_set {
			let mut real_size: u64 = 0;
			let mut matched_size: u64 = 0;

			for file in walk_ref.iter_all_files() {
				if file.str.starts_with(&dir_path) {
					let f_size = *file_sizes.get(&file.str).unwrap_or(&0);
					real_size += f_size;

					if matched_files_map.contains_key(file.str.as_str()) {
						matched_size += f_size;
					}
				}
			}

			let modified_at = get_modified_time(&dir_path);

			matched_dir_items.push(DirItem { path: dir_path, real_size, matched_size, modified_at });
		}

		matched_dir_items.sort_by(|a, b| a.path.cmp(&b.path));

		// Kompilujemy ostateczną strukturę
		let instance = Self {
			matched_files,
			matched_dirs: matched_dir_items,
			scanned_files: walk_ref.count_all_files(),
			scanned_dirs: walk_ref.count_all_dirs(),
		};

		// 004📕 Self-Logowanie procesu egzekucji
		if debug {
			println!("{}", instance.display_debug());
		}

		Ok(instance)
	}
}

impl VerboseDisplay for QueryExecute {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "004📕 [ structs: QueryExecute, FileItem, DirItem ] 🔰")?;

		writeln!(f, "📄📄📄📄📄📄📄")?;
		for file in &self.matched_files {
			writeln!(
				f,
				"📄  {:?} (size: {} B, bin: {}, mod: {:?})",
				file.path, file.size, file.is_binary, file.modified_at
			)?;
		}

		writeln!(f, "📁📁📁📁📁📁📁")?;
		for dir in &self.matched_dirs {
			writeln!(
				f,
				"📁  {:?} (matched: {} B, real: {} B, mod: {:?})",
				dir.path, dir.matched_size, dir.real_size, dir.modified_at
			)?;
		}

		writeln!(f, "📊📊📊📊📊📊📊")?;
		writeln!(f, "📊 count matched: 📄 = {};  📁 = {};", self.matched_files.len(), self.matched_dirs.len())?;
		writeln!(f, "📊 count scanned: 📄 = {};  📁 = {};", self.scanned_files, self.scanned_dirs)?;

		writeln!(f, "004📕 [ structs: QueryExecute, FileItem, DirItem ] 🏁")?;

		Ok(())
	}
}
