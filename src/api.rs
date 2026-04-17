pub mod lib {
	pub mod job {
		pub mod config;
		pub use config::*;
		pub mod step1;
		pub use step1::*;
		pub mod gens {
			pub mod item_icon;
			pub mod item_list;
			pub use item_icon::*;
			pub use item_list::*;
		}
		pub mod step2;
		pub use step2::*;
		pub mod step3;
		pub use step3::*;
	}
	pub mod logic {
		pub mod anchored_paths_datum;
		pub use anchored_paths_datum::{AnchoredPathsDatum, PathNode};
		pub mod path_patterns;
		pub use path_patterns::{PattEnvIndex, PattExp, PattRaw, PatternsQueries};
		pub mod path_context;
		pub use path_context::PathContext;
		pub mod fs_scanner;
		pub use fs_scanner::{
			MatchLabel,
			MatchedDir,
			MatchedFile,
			MismatchedDir,
			MismatchedFile,
			NodeIs,
			Partition,
			PartitionScanned,
			ScannedDirNode,
			ScannedFileNode,
			ScannedNode,
			StatsPartitioning,
			StatsScannedTreeFs,
			is_binary_file,
		};
		pub mod path_sort;
		pub use path_sort::SortQueries;
		// pub mod paths_table;
		// pub use paths_table::{FileKind, TableData, TableRow, TableSotcTreeOutput};
		// pub mod paths_result;
		// pub use paths_result::{Matched, Mismatched,PartitioningResult};

		pub mod specification;
		pub use specification::{
			JobMode,
			JobSpec,
			ScanSpec,
			TabColumn,
			TabPathStructure,
			TabSortBy,
			TabSortOrder,
			TabSpec,
		};
		pub mod tag_time;
		pub use tag_time::{TagTime, tag_time};
		pub mod mapper_lang_type;
		pub use mapper_lang_type::LangMapper;
		// pub mod engine;
		// pub use engine::{JobEngine, MX, RenderFlags};
		// pub mod engine_multiple;
		// pub use engine_multiple::DocEngineMultiple;
		pub mod config_model;
		pub use config_model::{
			ConfigExport,
			ConfigJob,
			ConfigLayout,
			ConfigManifest,
			ConfigSpec,
			ConfigTrimming,
			ScanRawJobNew,
		};
		pub mod file_toml_config;
		pub use file_toml_config::IoConfig;
	}
	// pub mod command {
	// 	pub mod args;
	// 	pub mod help;
	// }
	pub mod display {

		pub mod anchored_paths_datum;
		pub mod config;
		pub mod job_spec;
		// pub mod fs_scanner;
		// pub mod paths_patterns;
		// pub mod paths_result;
		// pub mod table_data;

		use colored::*;

		pub struct Color;
		impl Color {
			pub fn tree(s: &str) -> ColoredString { s.truecolor(41, 211, 152) }
			pub fn num(s: &str) -> ColoredString { s.bright_magenta() }
			pub fn size(s: &str) -> ColoredString { s.cyan() }
			pub fn date(s: &str) -> ColoredString { s.truecolor(140, 120, 100) }
			pub fn time(s: &str) -> ColoredString { s.truecolor(100, 70, 100) }
			pub fn folder(s: &str) -> ColoredString { s.truecolor(200, 200, 50).bold() }
			pub fn file(s: &str) -> ColoredString { s.bright_white() }
			pub fn binary(s: &str) -> ColoredString { s.bright_red() }
			pub fn border(s: &str) -> ColoredString { s.truecolor(20, 20, 20).dimmed() }
		}

		/// [POL]: Scentralizowane symbole formatowania dla projektu.
		pub struct Icon;

		impl Icon {
			pub const EMPTY: &'static str = "⭕";
			// --- Symbole zasobów ---
			pub const ENTRY: &'static str = "🗃️ ";
			// --- Symbole dla Entry (Twój styl z obrazka) ---
			pub const EXPAND: &'static str = "🔀";
			pub const BOOL_FALSE: &'static str = "✖️ ";
			pub const FILE: &'static str = "📝";
			pub const FOLDER: &'static str = "📂";
			// --- Symbole sekcji ---
			pub const H2: &'static str = "📚";
			pub const FILE2: &'static str = "📄";
			pub const FILE2_HIDDEN: &'static str = "⚙️ ";
			// ⚡ --- Symbole zasobów (Rozszerzone - z wersji OLD) ---
			pub const FOLDER2: &'static str = "📁";
			pub const FOLDER2_HIDDEN: &'static str = "🗃️";
			pub const LANG_RUST: &'static str = "🦀";
			// --- Symbole logiczne (Bool) ---
			pub const BOOL_TRUE: &'static str = "✔️ ";

			#[inline]
			pub fn bool(val: bool) -> &'static str { if val { Self::BOOL_TRUE } else { Self::BOOL_FALSE } }
		}

		/// [POL]: Rozszerzenie dla bool, aby pisać `val.as_symbol()`.
		pub trait BoolExt {
			fn as_symbol(&self) -> &'static str;
		}

		impl BoolExt for bool {
			fn as_symbol(&self) -> &'static str { Icon::bool(*self) }
		}
		pub struct TreeLast;
		impl TreeLast {
			pub const DIR_NO_CHILDREN: &'static str = "└───";
			pub const DIR_WITH_CHILDREN: &'static str = "└──┬";
			pub const FILE: &'static str = "└──•";
			pub const INDENT: &'static str = "   ";
		}
		pub struct TreeMid;
		impl TreeMid {
			pub const DIR_NO_CHILDREN: &'static str = "├───";
			pub const DIR_WITH_CHILDREN: &'static str = "├──┬";
			pub const FILE: &'static str = "├──•";
			pub const INDENT: &'static str = "│  ";
		}

		pub struct DrawTree;
		impl DrawTree {
			pub const ENTRY_BRANCH: &'static str = "  ├───";
			// view::fs::entry
			pub const ENTRY_TERMINAL: &'static str = "  └───";
			// view::fs::entry
			pub const ITEM: &'static str = "     •";
			// view::path::pattern
			pub const ITEM_BETWEEN: &'static str = "  ├──•";
			// self
			pub const ITEM_FIRST: &'static str = "  ┌──•";
			// view::fs::filter
			pub const ITEM_LAST: &'static str = "  └──•";
			// self
			pub const ITEM_ONEFOLD: &'static str = "   ──•";

			// self

			/// Automatycznie dobiera symbol gałęzi na podstawie indeksu i
			/// rozmiaru listy.
			pub fn list(index: usize, total: usize) -> &'static str {
				if total == 1 {
					Self::ITEM_ONEFOLD
				} else if index == 0 {
					Self::ITEM_FIRST
				} else if index == total - 1 {
					Self::ITEM_LAST
				} else {
					Self::ITEM_BETWEEN
				}
			}

			/// Zwraca krotkę: (symbol_gałęzi_dla_siebie, wcięcie_dla_dzieci)
			pub fn tree(is_dir: bool, is_last: bool, has_children: bool) -> (&'static str, &'static str) {
				let branch = if is_dir {
					match (is_last, has_children) {
						(true, true) => TreeLast::DIR_WITH_CHILDREN,
						(false, true) => TreeMid::DIR_WITH_CHILDREN,
						(true, false) => TreeLast::DIR_NO_CHILDREN,
						(false, false) => TreeMid::DIR_NO_CHILDREN,
					}
				} else if is_last {
					TreeLast::FILE
				} else {
					TreeMid::FILE
				};

				let next_indent = if is_last { TreeLast::INDENT } else { TreeMid::INDENT };

				(branch, next_indent)
			}
		}
	}
}
