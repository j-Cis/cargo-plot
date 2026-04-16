use plot::lib::job;

fn main() {
	println!("=== TEST STEP 2 (ALPHA) ===\n");

	// =========================================================================
	// STEP 0: Inicjalizacja
	// =========================================================================

	let ws_cfg = job::ValidWorkspace::parse_vec_as_config(["."]);
	let pt_cfg = job::ValidPattern::parse_vec_as_config(Vec::<&str>::new(), None);
	// let pt_cfg = job::ValidPattern::parse_vec_as_config::<[&str; 0]>([], None);

	// =========================================================================
	// STEP 1: Skanowanie i pobranie metadanych z dysku
	// =========================================================================
	let part_cfg = job::ValidTablePart::parse_vec_as_config(["md", "mf"]);
	let raw_rows = job::engine_step1_scanner(part_cfg, &ws_cfg, &pt_cfg);

	println!("Skanowanie zakończone. Zabrano wierszy (RawRow): {}\n", raw_rows.rows.len());

	// =========================================================================
	// STEP 2: Formatowanie komórek
	// =========================================================================

	// Ustawiamy dość bogatą konfigurację, żeby przetestować wszystkie mechanizmy
	let item_cfg =
		job::ValidColumnItem::parse_vec_as_config(["list-tree", "name-show", "ws-show", "icons-lite"]).unwrap();

	let cols_cfg = job::ValidTableColumns::parse_vec_as_config(["date", "time", "size", "item", "path"]);

	let date_cfg = job::ValidColumnDate::parse_vec_as_config(["default"]);
	let time_cfg = job::ValidColumnTime::parse_vec_as_config(["default"]);
	let size_cfg = job::ValidColumnSize::parse_vec_as_config(["dec"]);

	let formatted_rows =
		job::engine_step2_data_formater((&item_cfg, &cols_cfg), (&date_cfg, &time_cfg, &size_cfg), &raw_rows);

	// =========================================================================
	// WYDRUK KONTROLNY (Dla naszych oczu)
	// =========================================================================
	let sample_rows: Vec<_> = formatted_rows.into_iter().take(75).collect();

	// Przekazujemy wiersze i konfigurację kolumn (żeby renderer wiedział, czym są komórki)
	mock_render(&sample_rows, &cols_cfg);
}

// =========================================================================
// FUNKCJA POMOCNICZA (MOCK RENDERER - zalążek Step 5)
// =========================================================================
fn mock_render(rows: &[job::FormattedRow], cols_cfg: &job::ValidTableColumnsConfig) {
	for row in rows {
		let mut line = String::new();

		for (col_idx, cell) in row.cells.iter().enumerate() {
			line.push_str(cell);

			// Jeśli to nie jest ostatnia kolumna, dodajemy separator
			if col_idx < row.cells.len() - 1 {
				let current_col = &cols_cfg.columns[col_idx];
				let next_col = &cols_cfg.columns[col_idx + 1];

				// ⚡ Magia: Jeśli obok siebie stoją Data i Czas (w dowolnej kolejności),
				// rezygnujemy z pionowej kreski na rzecz zwykłej spacji.
				let is_date_time_pair = (current_col == &job::ValidTableColumns::Date
					&& next_col == &job::ValidTableColumns::Time)
					|| (current_col == &job::ValidTableColumns::Time && next_col == &job::ValidTableColumns::Date);

				if is_date_time_pair {
					line.push_str(" "); // Sklejenie Daty i Czasu
				} else {
					line.push_str(" | "); // Standardowy podział kolumn
				}
			}
		}
		println!("{}", line);
	}
}
