use std::{
	collections::BTreeMap,
	path::{Path, PathBuf},
};

use crate::lib::job;

// ============================================================================
// SORTER (Step 3) - UKŁADACZ ELEMENTÓW W WORKU
// ============================================================================

pub fn engine_step3_data_sort(
	sort_cfg: &job::ValidSortByConfig,
	item_cfg: &job::ValidColumnItemConfig,
	table: &mut job::ValidResultMainTab,
) {
	if table.rows.is_empty() {
		return;
	}

	// Wyciągamy informacje o trybie płaskim bezpośrednio z konfiguratora
	let is_flat = match item_cfg.list {
		job::ModeListForValidColumnItem::Flat => true,
		job::ModeListForValidColumnItem::None => true,
		job::ModeListForValidColumnItem::Tree => false,
	};

	// 1. Inicjalizacja ciężkiej logiki sortowania
	let queries = sort_cfg.get();

	if is_flat {
		// --------------------------------------------------------------------
		// JEDEN WOREK: Globalne sortowanie płaskiej listy
		// --------------------------------------------------------------------
		table.rows.sort_by(|a, b| queries.compare(a, b));
	} else {
		// --------------------------------------------------------------------
		// WIELE WORKÓW: Sortowanie w obrębie własnych katalogów (Tree)
		// --------------------------------------------------------------------
		let clean_paths: Vec<PathBuf> =
			table.rows.iter().map(|r| PathBuf::from(r.node.path.str.trim_end_matches('/'))).collect();

		// Mapa powiązań (Klucz = Rodzic, Wartość = Indeksy dzieci)
		let mut tree_map: BTreeMap<PathBuf, Vec<usize>> = BTreeMap::new();

		for (i, p) in clean_paths.iter().enumerate() {
			let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
			tree_map.entry(parent).or_default().push(i);
		}

		// Sortujemy zawartość wewnątrz każdego worka z osobna!
		for indices in tree_map.values_mut() {
			indices.sort_by(|&a, &b| queries.compare(&table.rows[a], &table.rows[b]));
		}

		// Szukamy "Korzeni" (Głównych worków)
		let mut root_indices = Vec::new();
		for (i, p) in clean_paths.iter().enumerate() {
			let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
			let is_root = parent == Path::new(".") || parent == Path::new("") || !clean_paths.contains(&parent);
			if is_root {
				root_indices.push(i);
			}
		}

		// Sortujemy same korzenie między sobą
		root_indices.sort_by(|&a, &b| queries.compare(&table.rows[a], &table.rows[b]));

		// Zwijamy wszystko z powrotem do płaskiego wektora w idealnej kolejności
		let mut flat_indices = Vec::with_capacity(table.rows.len());
		flatten(&root_indices, &tree_map, &clean_paths, &mut flat_indices);

		// Błyskawiczna podmiana wierszy (in-place)
		let total_rows = table.rows.len(); // Najpierw czytamy długość...
		let old_rows = std::mem::replace(&mut table.rows, Vec::with_capacity(total_rows)); // ...potem mutujemy!

		let mut temp_rows: Vec<Option<job::ValidResultMainRow>> = old_rows.into_iter().map(Some).collect();

		for idx in flat_indices {
			if let Some(row) = temp_rows[idx].take() {
				table.rows.push(row);
			}
		}
	}
}

/// Helper do rekurencyjnego opróżniania worków w ustalonej kolejności
fn flatten(indices: &[usize], tree_map: &BTreeMap<PathBuf, Vec<usize>>, clean_paths: &[PathBuf], out: &mut Vec<usize>) {
	for &idx in indices {
		out.push(idx);
		if let Some(children) = tree_map.get(&clean_paths[idx]) {
			flatten(children, tree_map, clean_paths, out);
		}
	}
}
