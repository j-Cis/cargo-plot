use std::collections::BTreeMap;

use crate::lib::{job, job::TraitConfigGetSetup};

// ============================================================================
// SORTER (Step 3) - UKŁADACZ ELEMENTÓW W WORKU
// ============================================================================

pub fn engine_step3_data_sort<'a>(
	sort_cfg: &job::ValidSortByParams,
	item_cfg: &job::ValidColumnItemParams,
	table: &'a mut job::ValidResultMainTab, // ⚡ Zostawiamy referencję mutowalną!
) -> &'a mut job::ValidResultMainTab {
	// ⚡ Zwracamy referencję mutowalną!
	if table.rows.is_empty() {
		return table; // Teraz to zadziała bezbłędnie
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
		// Mapa powiązań (Klucz = ID Rodzica, Wartość = Indeksy dzieci)
		let mut tree_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

		for (i, row) in table.rows.iter().enumerate() {
			// Bezpośredni rodzic to ostatni element w id_path.
			// Jeśli id_path jest puste (korzeń), przypisujemy do ID 0.
			let parent_id = row.node.id_path.last().copied().unwrap_or(0);
			tree_map.entry(parent_id).or_default().push(i);
		}

		// Sortujemy zawartość wewnątrz każdego worka (rodzica) z osobna!
		for indices in tree_map.values_mut() {
			indices.sort_by(|&a, &b| queries.compare(&table.rows[a], &table.rows[b]));
		}

		// Szukamy "Korzeni" (elementy, których rodzicem jest workspace_dir z ID 0)
		let mut root_indices = tree_map.remove(&0).unwrap_or_default();

		// Sortujemy same korzenie między sobą
		root_indices.sort_by(|&a, &b| queries.compare(&table.rows[a], &table.rows[b]));

		// Zwijamy wszystko rekurencyjnie do płaskiego wektora w idealnej kolejności
		let mut flat_indices = Vec::with_capacity(table.rows.len());
		flatten(&root_indices, &tree_map, &table.rows, &mut flat_indices);

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
	table
}

/// Helper do rekurencyjnego opróżniania worków w ustalonej kolejności (oparty na ID) 🔙📂
fn flatten(
	indices: &[usize],
	tree_map: &BTreeMap<usize, Vec<usize>>,
	rows: &[job::ValidResultMainRow],
	out: &mut Vec<usize>,
) {
	for &idx in indices {
		out.push(idx);

		let my_id = rows[idx].node.id_self; // Pobieram swoje własne ID

		// Jeśli jestem rodzicem dla jakichś dzieci w mapie, wchodzimy głębiej
		if let Some(children) = tree_map.get(&my_id) {
			flatten(children, tree_map, rows, out);
		}
	}
}
