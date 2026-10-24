//! Path: `src/libs/querypath/features/handle_walk_scan.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::{
	fmt,
	path::{Path, PathBuf},
};

use anyhow::Result;
use walkdir::WalkDir;

use crate::querypath::{
	controllers::types::ClaimRefining,
	core::{
		features::{HandleVerifiedDirs, VerboseDisplay},
		structs::{PathAnchored, PathNode},
		utils::{check_is_binary, normalize_path},
	},
};

/// Surowy rezultat przejścia po drzewie katalogów.
#[derive(Debug, Clone, Default)]
pub struct HandleWalkScan {
	// katalogi puste
	pub dirs_empty: Vec<PathNode>,
	pub dotdirs_empty: Vec<PathNode>,
	pub count_dirs_empty: usize,
	pub count_dotdirs_empty: usize,

	// katalogi nie puste
	pub dirs_no_empty: Vec<PathNode>,
	pub dotdirs_no_empty: Vec<PathNode>,
	pub count_dirs_no_empty: usize,
	pub count_dotdirs_no_empty: usize,

	// pliki puste
	pub files_empty_no_ext: Vec<PathNode>,
	pub dotfiles_empty_no_ext: Vec<PathNode>,
	pub files_empty_has_ext: Vec<PathNode>,
	pub dotfiles_empty_has_ext: Vec<PathNode>,
	pub count_files_empty_no_ext: usize,
	pub count_dotfiles_empty_no_ext: usize,
	pub count_files_empty_has_ext: usize,
	pub count_dotfiles_empty_has_ext: usize,

	// pliki nie textowe - czyli binarne
	pub files_binary_no_ext: Vec<PathNode>,
	pub dotfiles_binary_no_ext: Vec<PathNode>,
	pub files_binary_has_ext: Vec<PathNode>,
	pub dotfiles_binary_has_ext: Vec<PathNode>,
	pub count_files_binary_no_ext: usize,
	pub count_dotfiles_binary_no_ext: usize,
	pub count_files_binary_has_ext: usize,
	pub count_dotfiles_binary_has_ext: usize,

	// pliki nie binarne - czyli prawdopodobnie tekstowe
	pub files_text_no_empty_no_ext: Vec<PathNode>,
	pub dotfiles_text_no_empty_no_ext: Vec<PathNode>,
	pub files_text_no_empty_has_ext: Vec<PathNode>,
	pub dotfiles_text_no_empty_has_ext: Vec<PathNode>,
	pub count_files_text_no_empty_no_ext: usize,
	pub count_dotfiles_text_no_empty_no_ext: usize,
	pub count_files_text_no_empty_has_ext: usize,
	pub count_dotfiles_text_no_empty_has_ext: usize,
}

impl HandleWalkScan {
	pub fn count_all_dirs(&self) -> usize {
		self.count_dirs_empty + self.count_dotdirs_empty + self.count_dirs_no_empty + self.count_dotdirs_no_empty
	}

	pub fn count_all_files(&self) -> usize {
		self.count_files_empty_no_ext
			+ self.count_dotfiles_empty_no_ext
			+ self.count_files_empty_has_ext
			+ self.count_dotfiles_empty_has_ext
			+ self.count_files_binary_no_ext
			+ self.count_dotfiles_binary_no_ext
			+ self.count_files_binary_has_ext
			+ self.count_dotfiles_binary_has_ext
			+ self.count_files_text_no_empty_no_ext
			+ self.count_dotfiles_text_no_empty_no_ext
			+ self.count_files_text_no_empty_has_ext
			+ self.count_dotfiles_text_no_empty_has_ext
	}

	/// Iterator scalający wszystkie 4 kategorie katalogów (Zero-Copy)
	pub fn iter_all_dirs(&self) -> impl Iterator<Item = &PathNode> {
		self.dirs_empty
			.iter()
			.chain(self.dotdirs_empty.iter())
			.chain(self.dirs_no_empty.iter())
			.chain(self.dotdirs_no_empty.iter())
	}

	/// Iterator scalający wszystkie 12 kategorii plików (Zero-Copy)
	pub fn iter_all_files(&self) -> impl Iterator<Item = &PathNode> {
		self.files_empty_no_ext
			.iter()
			.chain(self.dotfiles_empty_no_ext.iter())
			.chain(self.files_empty_has_ext.iter())
			.chain(self.dotfiles_empty_has_ext.iter())
			.chain(self.files_binary_no_ext.iter())
			.chain(self.dotfiles_binary_no_ext.iter())
			.chain(self.files_binary_has_ext.iter())
			.chain(self.dotfiles_binary_has_ext.iter())
			.chain(self.files_text_no_empty_no_ext.iter())
			.chain(self.dotfiles_text_no_empty_no_ext.iter())
			.chain(self.files_text_no_empty_has_ext.iter())
			.chain(self.dotfiles_text_no_empty_has_ext.iter())
	}

	/// Czyści konkretne szufladki na podstawie flag ClaimRefining
	pub fn refine(&mut self, config: &ClaimRefining, debug: bool) {
		let mut active = 0;
		let mut log_out = String::new();

		macro_rules! cut {
			($flag:expr, $list:expr, $msg:expr) => {
				if $flag {
					$list.clear();
					if debug {
						log_out.push_str(&format!("  ✂️ CUT: {}\n", $msg));
						active += 1;
					}
				}
			};
		}

		// Katalogi
		cut!(config.skip_doted_dirs_empty, self.dotdirs_empty, "skip_doted_dirs_empty");
		cut!(config.skip_nodot_dirs_empty, self.dirs_empty, "skip_nodot_dirs_empty");
		cut!(config.skip_doted_dirs_laden, self.dotdirs_no_empty, "skip_doted_dirs_laden");
		cut!(config.skip_nodot_dirs_laden, self.dirs_no_empty, "skip_nodot_dirs_laden");

		// Pliki puste
		cut!(config.skip_doted_noext_files_empty, self.dotfiles_empty_no_ext, "skip_doted_noext_files_empty");
		cut!(config.skip_doted_withext_files_empty, self.dotfiles_empty_has_ext, "skip_doted_withext_files_empty");
		cut!(config.skip_nodoted_noext_files_empty, self.files_empty_no_ext, "skip_nodoted_noext_files_empty");
		cut!(
			config.skip_nodoted_withext_files_empty,
			self.files_empty_has_ext,
			"skip_nodoted_withext_files_empty"
		);

		// Pliki binarne
		cut!(config.skip_doted_noext_files_binary, self.dotfiles_binary_no_ext, "skip_doted_noext_files_binary");
		cut!(
			config.skip_doted_withext_files_binary,
			self.dotfiles_binary_has_ext,
			"skip_doted_withext_files_binary"
		);
		cut!(config.skip_nodoted_noext_files_binary, self.files_binary_no_ext, "skip_nodoted_noext_files_binary");
		cut!(
			config.skip_nodoted_withext_files_binary,
			self.files_binary_has_ext,
			"skip_nodoted_withext_files_binary"
		);

		// Pliki tekstowe
		cut!(
			config.skip_doted_noext_files_text,
			self.dotfiles_text_no_empty_no_ext,
			"skip_doted_noext_files_text"
		);
		cut!(
			config.skip_doted_withext_files_text,
			self.dotfiles_text_no_empty_has_ext,
			"skip_doted_withext_files_text"
		);
		cut!(
			config.skip_nodoted_noext_files_text,
			self.files_text_no_empty_no_ext,
			"skip_nodoted_noext_files_text"
		);
		cut!(
			config.skip_nodoted_withext_files_text,
			self.files_text_no_empty_has_ext,
			"skip_nodoted_withext_files_text"
		);

		self.update_counts();

		if debug {
			println!("1c📕 [ structs: ClaimRefining applied to HandleWalkScan ] 🔰");
			if active == 0 {
				println!("  (Brak aktywnych filtrów ClaimRefining - nic nie odcięto)");
			} else {
				print!("{}", log_out);
			}
			println!("1c📕 [ structs: ClaimRefining applied to HandleWalkScan ] 🏁\n");
		}
	}

	/// Wykonuje surowe skanowanie dla wszystkich celów w `HandleVerifiedDirs`.
	pub fn scan(entry: &HandleVerifiedDirs, refining: &ClaimRefining, debug: bool) -> Result<Self> {
		let mut instance = Self::default();

		for handle in &entry.handles {
			instance.scan_target(handle, &entry.cwd_buf)?;
		}

		instance.sort_and_dedup_all();

		// Bezpośrednie wyłączenie szufladek wewnątrz skanera
		instance.refine(refining, debug);

		instance.update_counts();

		if debug {
			println!("{}", instance.display_debug());
		}

		Ok(instance)
	}

	fn scan_target(&mut self, handle: &PathAnchored, cwd: &Path) -> Result<()> {
		let root = &handle.buf;

		for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
			if entry.depth() == 0 {
				continue;
			}
			if entry.path_is_symlink() {
				continue;
			}

			let Ok(rel_prefix) = entry.path().strip_prefix(cwd) else {
				continue;
			};

			let norm_rel = normalize_path(rel_prefix);
			let clean_rel = norm_rel.trim_start_matches("./");

			let file_name = entry.file_name().to_string_lossy();
			let is_dot = file_name.starts_with('.');

			if entry.file_type().is_dir() {
				let is_empty = entry.path().read_dir().map(|mut r| r.next().is_none()).unwrap_or(false);
				let dir_str = format!("./{}/", clean_rel.trim_end_matches('/'));
				let mut node = PathNode::new(
					PathBuf::from(dir_str),
					true,     // is_dir
					is_empty, // is_empty
					is_dot,   // is_doted
					false,    // is_binary
					false,    // has_extension
				);
				// Foldery na start otrzymują no_copypast = true
				node.set_no_copypast();

				match (is_empty, is_dot) {
					(true, false) => self.dirs_empty.push(node),
					(true, true) => self.dotdirs_empty.push(node),
					(false, false) => self.dirs_no_empty.push(node),
					(false, true) => self.dotdirs_no_empty.push(node),
				}
			} else {
				let has_ext = entry.path().extension().is_some();
				let meta = entry.metadata().ok();
				let is_empty = meta.as_ref().map(|m| m.len() == 0).unwrap_or(false);

				let is_bin = if !is_empty {
					if let Some(path_str) = entry.path().to_str() { check_is_binary(path_str) } else { false }
				} else {
					false
				};

				let file_str = format!("./{}", clean_rel);
				let mut node = PathNode::new(
					PathBuf::from(file_str),
					false,    // is_dir
					is_empty, // is_empty
					is_dot,   // is_doted
					is_bin,   // is_binary
					has_ext,  // has_extension
				);

				// Pliki puste oraz pliki binarne na start otrzymują no_copypast = true
				if is_empty || is_bin {
					node.set_no_copypast();
				}

				match (is_empty, is_bin, is_dot, has_ext) {
					(true, _, false, false) => self.files_empty_no_ext.push(node),
					(true, _, false, true) => self.files_empty_has_ext.push(node),
					(true, _, true, false) => self.dotfiles_empty_no_ext.push(node),
					(true, _, true, true) => self.dotfiles_empty_has_ext.push(node),

					(false, true, false, false) => self.files_binary_no_ext.push(node),
					(false, true, false, true) => self.files_binary_has_ext.push(node),
					(false, true, true, false) => self.dotfiles_binary_no_ext.push(node),
					(false, true, true, true) => self.dotfiles_binary_has_ext.push(node),

					(false, false, false, false) => self.files_text_no_empty_no_ext.push(node),
					(false, false, false, true) => self.files_text_no_empty_has_ext.push(node),
					(false, false, true, false) => self.dotfiles_text_no_empty_no_ext.push(node),
					(false, false, true, true) => self.dotfiles_text_no_empty_has_ext.push(node),
				}
			}
		}

		Ok(())
	}

	fn sort_and_dedup_all(&mut self) {
		let process_vec = |v: &mut Vec<PathNode>| {
			v.sort_unstable_by(|a, b| a.str.cmp(&b.str));
			v.dedup_by(|a, b| a.str == b.str);
		};

		process_vec(&mut self.dirs_empty);
		process_vec(&mut self.dotdirs_empty);
		process_vec(&mut self.dirs_no_empty);
		process_vec(&mut self.dotdirs_no_empty);

		process_vec(&mut self.files_empty_no_ext);
		process_vec(&mut self.dotfiles_empty_no_ext);
		process_vec(&mut self.files_empty_has_ext);
		process_vec(&mut self.dotfiles_empty_has_ext);

		process_vec(&mut self.files_binary_no_ext);
		process_vec(&mut self.dotfiles_binary_no_ext);
		process_vec(&mut self.files_binary_has_ext);
		process_vec(&mut self.dotfiles_binary_has_ext);

		process_vec(&mut self.files_text_no_empty_no_ext);
		process_vec(&mut self.dotfiles_text_no_empty_no_ext);
		process_vec(&mut self.files_text_no_empty_has_ext);
		process_vec(&mut self.dotfiles_text_no_empty_has_ext);
	}

	fn update_counts(&mut self) {
		self.count_dirs_empty = self.dirs_empty.len();
		self.count_dotdirs_empty = self.dotdirs_empty.len();
		self.count_dirs_no_empty = self.dirs_no_empty.len();
		self.count_dotdirs_no_empty = self.dotdirs_no_empty.len();

		self.count_files_empty_no_ext = self.files_empty_no_ext.len();
		self.count_dotfiles_empty_no_ext = self.dotfiles_empty_no_ext.len();
		self.count_files_empty_has_ext = self.files_empty_has_ext.len();
		self.count_dotfiles_empty_has_ext = self.dotfiles_empty_has_ext.len();

		self.count_files_binary_no_ext = self.files_binary_no_ext.len();
		self.count_dotfiles_binary_no_ext = self.dotfiles_binary_no_ext.len();
		self.count_files_binary_has_ext = self.files_binary_has_ext.len();
		self.count_dotfiles_binary_has_ext = self.dotfiles_binary_has_ext.len();

		self.count_files_text_no_empty_no_ext = self.files_text_no_empty_no_ext.len();
		self.count_dotfiles_text_no_empty_no_ext = self.dotfiles_text_no_empty_no_ext.len();
		self.count_files_text_no_empty_has_ext = self.files_text_no_empty_has_ext.len();
		self.count_dotfiles_text_no_empty_has_ext = self.dotfiles_text_no_empty_has_ext.len();
	}
}

impl VerboseDisplay for HandleWalkScan {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "1c📕 [ structs: HandleWalkScan ] 🔰\n")?;

		let total_dirs = self.count_all_dirs();
		let total_files = self.count_all_files();

		// --- PODSUMOWANIE KATALOGÓW ---
		writeln!(f, "📁 TOTAL {} DIRECTORIES SCANNED", total_dirs)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, NOT HIDDEN", self.count_dirs_empty)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, HIDDEN (DOT)", self.count_dotdirs_empty)?;
		writeln!(f, "  ▪️ INCLUDING {} NOT EMPTY, NOT HIDDEN", self.count_dirs_no_empty)?;
		writeln!(f, "  ▪️ INCLUDING {} NOT EMPTY, HIDDEN (DOT)", self.count_dotdirs_no_empty)?;
		writeln!(f)?;

		// --- PODSUMOWANIE PLIKÓW ---
		writeln!(f, "📄 TOTAL {} FILES SCANNED", total_files)?;
		writeln!(f)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, NO EXTENSION", self.count_files_empty_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, HIDDEN (DOT), NO EXTENSION", self.count_dotfiles_empty_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, WITH EXTENSION", self.count_files_empty_has_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, HIDDEN (DOT), WITH EXTENSION", self.count_dotfiles_empty_has_ext)?;
		writeln!(f)?;
		writeln!(f, "  ▪️ INCLUDING {} BINARY, NO EXTENSION", self.count_files_binary_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} BINARY, HIDDEN (DOT), NO EXTENSION", self.count_dotfiles_binary_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} BINARY, WITH EXTENSION", self.count_files_binary_has_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} BINARY, HIDDEN (DOT), WITH EXTENSION", self.count_dotfiles_binary_has_ext)?;
		writeln!(f)?;
		writeln!(f, "  ▪️ INCLUDING {} TEXT, NO EXTENSION", self.count_files_text_no_empty_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} TEXT, HIDDEN (DOT), NO EXTENSION", self.count_dotfiles_text_no_empty_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} TEXT, WITH EXTENSION", self.count_files_text_no_empty_has_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} TEXT, HIDDEN (DOT), WITH EXTENSION", self.count_dotfiles_text_no_empty_has_ext)?;
		writeln!(f)?;

		// --- JEDNOLITY WYKAZ WSZYSTKICH WĘZŁÓW ---
		let mut all_items: Vec<&PathNode> = self.iter_all_dirs().chain(self.iter_all_files()).collect();
		all_items.sort_unstable_by(|a, b| a.str.cmp(&b.str));

		for item in all_items {
			writeln!(
				f,
				"{}{}{}{}  🛤️ {}",
				if item.is_doted { "🕳️" } else { "👁️" },
				if item.is_empty { "🕸️" } else { "📦" }, //🫙
				if item.is_dir {
					"📁"
				} else if item.is_binary {
					"🧊"
				} else {
					"📄"
				},
				if item.has_extension { "🏷️ " } else { "❔" },
				item.str,
			)?;
		}
		writeln!(f)?;

		writeln!(f, "1c📕 [ structs: HandleWalkScan ] 🏁")?;
		Ok(())
	}
}
