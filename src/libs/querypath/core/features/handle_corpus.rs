//! Path: `src/libs/querypath/features/handle_corpus.rs`
//! Last significant modified: 2026-W43-3 19:07:00 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::{
	borrow::Borrow,
	collections::{HashMap, HashSet},
	fmt, fs,
	path::PathBuf,
};
use walkdir::WalkDir;
use anyhow::Result;
use crate::querypath::core::{
	features::{HandleWalkScan, VerboseDisplay},
	structs::{PathNode, PathsPatterns, WalkEnvIndex},
	traits::PattEnvIndex,
	utils::get_modified_time,
};


// ============================================================================
// GŁÓWNY EGZEKUTOR
// ============================================================================

pub struct HandleCorpus {
	pub matched_files: Vec<PathNode>,
	pub matched_dirs: Vec<PathNode>,
	pub scanned_files: usize,
	pub scanned_dirs: usize,
}

impl HandleCorpus {
	pub fn apply<P, W>(patterns: P, walk: W, keep_parent: bool, debug: bool) -> Result<Self>
	where
		P: Borrow<PathsPatterns>,
		W: Borrow<HandleWalkScan>,
	{
		// Odpakowanie referencji z generyków
		let patterns_ref = patterns.borrow();
		let walk_ref = walk.borrow();

		// 1. INDEX pamięciowy plików i katalogów
		let env = WalkEnvIndex::from_walk(walk_ref);

		// 2. Filtrowanie plików wzorcem oraz hydracja metadanych w PathNode
		let mut matched_files: Vec<PathNode> = Vec::new();
		for file in walk_ref.iter_all_files() {
			if patterns_ref.is_match(&file.str, &env) {
				let size = fs::metadata(&file.str).map(|m| m.len()).unwrap_or(0);
				let modified_at = get_modified_time(&file.str);

				let mut file_node = file.clone();
				file_node.meta(modified_at, Some(size), Some(size));
				matched_files.push(file_node);
			}
		}

		// 3. Budowa zbioru katalogów dopasowanych bezpośrednio
		let mut matched_dirs_set: HashSet<String> = walk_ref
			.iter_all_dirs()
			.map(|d| d.str.as_str())
			.filter(|d| patterns_ref.is_match(d, &env))
			.map(String::from)
			.collect();

		
		// Odtwarzanie katalogów nadrzędnych dla dopasowanych plików
		if keep_parent {
			for file_item in &matched_files {
				let mut current: &str = file_item.str.as_str();
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

		// 4. Mapa pomocnicza dla kalkulacji matched_size
		let matched_files_map: HashMap<&str, u64> = matched_files
			.iter()
			.map(|f| (f.str.as_str(), f.meta_size_scope.unwrap_or(0)))
			.collect();

		let all_dirs_map: HashMap<&str, &PathNode> =
			walk_ref.iter_all_dirs().map(|d| (d.str.as_str(), d)).collect();

		let mut matched_dir_items: Vec<PathNode> = Vec::new();

		for dir_path in matched_dirs_set {
			// Pobieramy gotowy węzeł ze skanu lub tworzymy go awaryjnie
			let mut dir_node = match all_dirs_map.get(dir_path.as_str()) {
				Some(existing) => (*existing).clone(),
				None => PathNode::new(
					PathBuf::from(&dir_path),
					true,
					false,
					dir_path.trim_start_matches("./").starts_with('.'),
					false,
					false,
				),
			};

			// Rzeczywisty rozmiar fizyczny z dysku (Właściwości folderu)
			let real_size: u64 = WalkDir::new(&dir_path)
				.into_iter()
				.filter_map(|e| e.ok())
				.filter(|e| e.file_type().is_file())
				.filter_map(|e| e.metadata().ok())
				.map(|m| m.len())
				.sum();

			// Rozmiar tylko elementów dopasowanych (w zakresie filtrów)
			let mut matched_size: u64 = 0;
			for (matched_path, f_size) in &matched_files_map {
				if matched_path.starts_with(&dir_path) {
					matched_size += f_size;
				}
			}

			let modified_at = get_modified_time(&dir_path);
			dir_node.meta(modified_at, Some(real_size), Some(matched_size));

			matched_dir_items.push(dir_node);
		}

		matched_dir_items.sort_by(|a, b| a.str.cmp(&b.str));

		let instance = Self {
			matched_files,
			matched_dirs: matched_dir_items,
			scanned_files: walk_ref.count_all_files(),
			scanned_dirs: walk_ref.count_all_dirs(),
		};

		if debug {
			println!("{}", instance.display_debug());
		}

		Ok(instance)
	}
}
		


impl VerboseDisplay for HandleCorpus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

		writeln!(f, "1e📕 [ structs: HandleCorpus, PathNode ] 🔰")?;	

		let mut all_items: Vec<&PathNode> = self.matched_dirs.iter().chain(self.matched_files.iter()).collect();
		all_items.sort_unstable_by(|a, b| a.str.cmp(&b.str));
		for item in all_items {
			writeln!(
				f,
				"{}{}{}{}  🛤️ {}",
				if item.is_doted { "🕳️" } else { "👁️" },
				if item.is_empty { "🕸️" } else { "📦" },//🫙
				if item.is_dir { "📁" } else if item.is_binary { "🧊" } else { "📄" },
				if item.has_extension { "🏷️ " } else { "❔" },
				item.str,
			)?;
		}
		writeln!(f)?;
		writeln!(f, "1e📕 [ structs: HandleCorpus, PathNode ] 🏁")?;

		Ok(())
	}
}