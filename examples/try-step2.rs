use plot::lib::job;

fn main() {
	println!("=== TEST STEP 2 (ALPHA) ===\n");

	// =========================================================================
	// STEP 0: Inicjalizacja
	// =========================================================================

	let ws_cfg = job::ValidWorkspaceParse::parse_vec_as_config(["."]);
	let pt_cfg = job::ValidPatternParse::parse_vec_as_config(Vec::<&str>::new(), None);
	// let pt_cfg = job::ValidPatternParse::parse_vec_as_config::<[&str; 0]>([], None);

	// =========================================================================
	// STEP 1: Skanowanie i pobranie metadanych z dysku
	// =========================================================================
	let part_cfg = job::ValidTablePartParse::parse_vec_as_config(["md", "mf"]);
	let raw_rows = job::engine_step1_scanner(part_cfg, &ws_cfg, &pt_cfg);

	println!("Skanowanie zakończone. Zabrano wierszy (ValidResultMainRow): {}\n", raw_rows.rows.len());

	// =========================================================================
	// STEP 2: Formatowanie komórek
	// =========================================================================

	// Ustawiamy dość bogatą konfigurację, żeby przetestować wszystkie mechanizmy
	let item_cfg =
		job::ValidColumnItemParse::parse_vec_as_config(["list-tree", "name-show", "ws-show", "icons-lite"]).unwrap();

	let cols_cfg = job::ValidTableColumnsParse::parse_vec_as_config(["date", "time", "size", "item", "path"]);

	let date_cfg = job::ValidColumnDateParse::parse_vec_as_config(["default"]);
	let time_cfg = job::ValidColumnTimeParse::parse_vec_as_config(["default"]);
	let size_cfg = job::ValidColumnSizeParse::parse_vec_as_config(["dec"]);

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

			// Jeśli to nie jest ostatnia komórka, dodajemy separator
			if col_idx < row.cells.len() - 1 {
				// Używamy bezpiecznego .get(), bo kolumny debug nie mają swojej definicji w configu
				let current_col = cols_cfg.columns.get(col_idx);
				let next_col = cols_cfg.columns.get(col_idx + 1);

				// ⚡ Magia: Jeśli obok siebie stoją Data i Czas (w dowolnej kolejności)
				let is_date_time_pair = match (current_col, next_col) {
					(Some(job::ValidTableColumnsParse::Date), Some(job::ValidTableColumnsParse::Time)) => true,
					(Some(job::ValidTableColumnsParse::Time), Some(job::ValidTableColumnsParse::Date)) => true,
					_ => false,
				};

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
