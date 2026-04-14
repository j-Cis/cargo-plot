pub mod lib {
	pub mod logic {
		pub mod path_canonical_ctx;
		pub use path_canonical_ctx::{PathCanonicalCtx, PathNode};
		pub mod path_scan;
		pub use path_scan::{PathScan, PathScanStat};
		pub mod path_context;
		pub use path_context::PathContext;
		pub mod paths_patterns;
		pub use paths_patterns::{PathsPatterns, PattEnvIndex, PattExp, PattRaw};
		pub mod table_spec;
		pub use table_spec::{TabColumn, TabSortBy, TabSortOrder, TableSpec};
		pub mod table_data;
		pub use table_data::{FileKind, TableData, TableOutput, TableRow};
		pub mod paths_result;
		pub use paths_result::{FilterList, MatchLabel, Matched, Mismatched, ResultScanPatterns};
		pub mod tag_time;
		pub use tag_time::{TagTime, tag_time};
		pub mod lang_mapper;
		pub use lang_mapper::LangMapper;
		pub mod doc_markdown;
		pub use doc_markdown::DocMarkdown;
		pub mod doc_engine;
		pub use doc_engine::{DocEngine, MX, RenderFlags};
	}
	pub mod command {
		pub mod args;
		pub mod table;
		pub use table::{parse_column, parse_sort};
		pub mod help;
	}
	pub mod display {

		pub mod path_canonical_ctx;
		pub mod path_scan;
		pub mod paths_patterns;
		pub mod paths_result;
		pub mod table_data;

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
