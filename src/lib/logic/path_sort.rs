use std::cmp::Ordering;

use crate::lib::{
	job::{ModeFileGroupForValidSortBy, StrategyForValidSortBy, ValidResultMainRow, ValidSortByParams},
	logic::NodeIs,
};

// ============================================================================
// SORT QUERIES (Silnik Sortowania)
// ============================================================================

#[derive(Debug, Clone)]
pub struct SortQueries {
	pub raw_config: ValidSortByParams,
}

impl SortQueries {
	pub fn new(config: ValidSortByParams) -> Self { Self { raw_config: config } }

	/// Główny komparator. Zwraca Ordering (Mniej, Więcej, Równe).
	pub fn compare(&self, a: &ValidResultMainRow, b: &ValidResultMainRow) -> Ordering {
		let a_is_dir = a.node.node == NodeIs::Dir;
		let b_is_dir = b.node.node == NodeIs::Dir;
		let a_name = &a.node.name;
		let b_name = &b.node.name;

		// 1. Wykonanie priorytetu sortowania z konfiguracji
		let mut cmp = match &self.raw_config.strategy {
			StrategyForValidSortBy::Date { .. } => a.dt_modified.cmp(&b.dt_modified),
			StrategyForValidSortBy::Size { .. } => a.size_real.cmp(&b.size_real),

			StrategyForValidSortBy::Name { file_group, .. } | StrategyForValidSortBy::Path { file_group, .. } => {
				match file_group {
					ModeFileGroupForValidSortBy::Name => {
						// Sortowanie "Merge": Folder "mod" i plik "mod.rs" lądują obok siebie
						let a_core = Self::get_core_name(a_name);
						let b_core = Self::get_core_name(b_name);
						let core_cmp = a_core.cmp(b_core);

						if core_cmp == Ordering::Equal {
							// Domyślnie foldery idą nad plikami o tej samej nazwie bazowej
							b_is_dir.cmp(&a_is_dir).then(a_name.cmp(b_name))
						} else {
							core_cmp
						}
					}
					ModeFileGroupForValidSortBy::Exte => {
						// Grupowanie po rozszerzeniu
						let a_ext = Self::get_extension(a_name);
						let b_ext = Self::get_extension(b_name);
						a_ext.cmp(b_ext).then(a_name.cmp(b_name))
					}
					ModeFileGroupForValidSortBy::None => {
						// Klasyczne sortowanie (Foldery nad plikami)
						b_is_dir.cmp(&a_is_dir).then(a_name.cmp(b_name))
					}
				}
			}
		};

		// 2. Obsługa flagi Reverse
		let reverse = match &self.raw_config.strategy {
			StrategyForValidSortBy::Date { reverse } => *reverse,
			StrategyForValidSortBy::Size { reverse } => *reverse,
			StrategyForValidSortBy::Name { reverse, .. } => *reverse,
			StrategyForValidSortBy::Path { reverse, .. } => *reverse,
		};

		if reverse {
			cmp = cmp.reverse();
		}

		cmp
	}

	// Helper: Wydobywa nazwę bez rozszerzenia (np. "main.rs" -> "main")
	fn get_core_name(name: &str) -> &str {
		if let Some(idx) = name.rfind('.') {
			if idx > 0 {
				return &name[..idx];
			}
		}
		name
	}

	// Helper: Wydobywa samo rozszerzenie (np. "main.rs" -> "rs")
	fn get_extension(name: &str) -> &str {
		if let Some(idx) = name.rfind('.') {
			if idx > 0 {
				return &name[idx + 1..];
			}
		}
		""
	}
}
