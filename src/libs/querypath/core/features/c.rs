//DUPA AI
//! Path: `src/libs/querypath/features/handle_corpus.rs`
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
use crate::{timer::Timer, querypath::core::{
	features::{HandleVerifiedDirs, HandleWalkScan, VerboseDisplay},
	structs::{PathNode, PathsPatterns, WalkEnvIndex},
	traits::PattEnvIndex
}};

// ============================================================================
// ALGORYTM NAJNIŻSZEGO WSPÓLNEGO PRZODKA (LCA)
// ============================================================================

fn find_lowest_common_ancestor(paths: &[String]) -> Option<String> {
	if paths.is_empty() {
		return None;
	}
	if paths.len() == 1 {
		let trimmed = paths[0].trim_end_matches('/');
		return trimmed.rsplit_once('/').map(|(parent, _)| {
			if parent.is_empty() {
				"./".to_string()
			} else {
				format!("{parent}/")
			}
		});
	}

	let components: Vec<Vec<&str>> = paths
		.iter()
		.map(|p| p.trim_end_matches('/').split('/').collect())
		.collect();

	let min_len = components.iter().map(|c| c.len()).min().unwrap_or(0);
	let mut common: Vec<&str> = Vec::new();

	for i in 0..min_len {
		let first = components[0][i];
		if components.iter().all(|c| c[i] == first) {
			common.push(first);
		} else {
			break;
		}
	}

	if common.is_empty() {
		None
	} else {
		let mut res = common.join("/");
		if !res.ends_with('/') {
			res.push('/');
		}
		Some(res)
	}
}

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
	pub fn apply<E, P, W>(entry: E, patterns: P, walk: W, keep_parent: bool, debug: bool) -> Result<Self>
	where
		E: Borrow<HandleVerifiedDirs>,
		P: Borrow<PathsPatterns>,
		W: Borrow<HandleWalkScan>,
	{
		let entry_ref = entry.borrow();
		let patterns_ref = patterns.borrow();
		let walk_ref = walk.borrow();

		// 1. INDEX pamięciowy plików i katalogów
		let env = WalkEnvIndex::from_walk(walk_ref);

		// Zabezpieczenie: Identyfikacja głównych folderów skanowania (np. ./src/, ./examples/)
		let mut scan_roots: HashSet<&str> = HashSet::new();
		for d in walk_ref.iter_all_dirs() {
			let root_candidate = d.str.trim_start_matches("./").split('/').next().unwrap_or("");
			if !root_candidate.is_empty() {
				scan_roots.insert(root_candidate);
			}
		}

		// Pobranie katalogów zleceń bezpośrednio z HandleVerifiedDirs
		let mut entry_dirs: Vec<String> = entry_ref
			.handles
			.iter()
			.map(|h| {
				if h.str.ends_with('/') {
					h.str.clone()
				} else {
					format!("{}/", h.str)
				}
			})
			.collect();
		entry_dirs.sort_unstable_by(|a, b| b.len().cmp(&a.len()));

		// 2. Filtrowanie plików wzorcem oraz hydracja metadanych w PathNode
		let mut matched_files: Vec<PathNode> = Vec::new();
		for file in walk_ref.iter_all_files() {
			if patterns_ref.is_match(&file.str, &env) {
				let size = fs::metadata(&file.str).map(|m| m.len()).unwrap_or(0);
				let modified_at = Timer::from_path_modified(&file.str).format("[YW]W[WW][DDD] [hh]:[mm]:[ss]:[tt]");
				
				let mut file_node = file.clone();
				file_node.meta(Some(modified_at), Some(size), Some(size));
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

		// GWARANCJA: Katalogi zlecenia oraz korzenie ZAWSZE trafiają do wyniku
		for &root_name in &scan_roots {
			let dir_candidate = format!("./{root_name}/");
			if env.has_dir(&dir_candidate) {
				matched_dirs_set.insert(dir_candidate);
			}
		}
		for ed in &entry_dirs {
			matched_dirs_set.insert(ed.clone());
		}

		// UNIFIKACJA: Wszystkie znalezione węzły (pliki i katalogi) traktowane są w 100% identycznie
		let all_found_paths: Vec<String> = matched_files
			.iter()
			.map(|f| f.str.clone())
			.chain(matched_dirs_set.iter().cloned())
			.collect();

		let mut active_entry_dirs: HashSet<String> = HashSet::new();

		// KROK A: DOMYŚLNIE — uzupełnianie w górę od KAŻDEGO znalezionego węzła (pliku LUB katalogu) do jego zlecenia
		for path in all_found_paths {
			let matching_entry = entry_dirs.iter().find(|&ed| path.starts_with(ed));

			let ceiling = if let Some(ed) = matching_entry {
				active_entry_dirs.insert(ed.clone());
				ed.as_str()
			} else {
				"./"
			};

			if path.trim_end_matches('/') != ceiling.trim_end_matches('/') {
				let mut current = path.trim_end_matches('/');
				while let Some((parent, _)) = current.rsplit_once('/') {
					if parent.is_empty() {
						break;
					}
					let dir_candidate = format!("{parent}/");
					matched_dirs_set.insert(dir_candidate.clone());
					if dir_candidate == ceiling || dir_candidate == "./" {
						break;
					}
					current = parent;
				}
			}
		}

		// Jeśli w wynikach były same zlecenia bez podelementów, aktywne są wszystkie zlecenia
		if active_entry_dirs.is_empty() {
			for ed in &entry_dirs {
				active_entry_dirs.insert(ed.clone());
			}
		}

		// KROK B: GDY keep_parent JEST TRUE — łączymy katalogi zleceń do ich wspólnego przodka (LCA)
		// Działa bezwarunkowo, niezależnie czy w wynikach są pliki, czy same puste/dopasowane katalogi!
		if keep_parent && !active_entry_dirs.is_empty() {
			let entries_vec: Vec<String> = active_entry_dirs.into_iter().collect();
			if let Some(lca) = find_lowest_common_ancestor(&entries_vec) {
				matched_dirs_set.insert(lca.clone());
				for ed in entries_vec {
					let mut current = ed.trim_end_matches('/');
					while let Some((parent, _)) = current.rsplit_once('/') {
						if parent.is_empty() {
							break;
						}
						let dir_candidate = format!("{parent}/");
						matched_dirs_set.insert(dir_candidate.clone());
						if dir_candidate == lca || dir_candidate == "./" {
							break;
						}
						current = parent;
					}
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
				None => {
					let mut node = PathNode::new(
						PathBuf::from(&dir_path),
						true,
						false,
						dir_path.trim_start_matches("./").starts_with('.'),
						false,
						false,
					);
					node.set_no_copypast();
					node
				}
			};

			// Rzeczywisty rozmiar fizyczny z dysku (Właściwości folderu)
			let real_size: u64 = WalkDir::new(&dir_path)
				.into_iter()
				.filter_map(|e| e.ok())
				.filter(|e| e.file_type().is_file())
				.filter_map(|e| e.metadata().ok())
				.map(|m| m.len())
				.sum();

			// Rozmiar tylko elementów dopasowanych (w zakresie filtrów - 0 gdy brak plików)
			let mut matched_size: u64 = 0;
			for (matched_path, f_size) in &matched_files_map {
				if matched_path.starts_with(&dir_path) {
					matched_size += f_size;
				}
			}

			let modified_at = Timer::from_path_modified(&dir_path).format("[YW]W[WW][DDD] [hh]:[mm]:[ss]:[tt]");
			dir_node.meta(Some(modified_at), Some(real_size), Some(matched_size));

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
				"{}{}{}{} ⏳{}  🛤️ {}",
				if item.is_doted { "🕳️" } else { "👁️" },
				if item.is_empty { "🕸️" } else { "📦" },
				if item.is_dir { "📁" } else if item.is_binary { "🧊" } else { "📄" },
				if item.has_extension { "🏷️ " } else { "❔" },
				item.meta_modifed_at.as_deref().unwrap_or("-------- - - - -"),
				item.str,
			)?;
		}
		writeln!(f)?;
		writeln!(f, "1e📕 [ structs: HandleCorpus, PathNode ] 🏁")?;

		Ok(())
	}
}