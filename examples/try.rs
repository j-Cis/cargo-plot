//use plot::lib::{job, job::ParseFromFlags};
use plot::lib::job;
fn main() {
	let mode_is_job_from_toml: bool = true;
	// =========================================================================
	// ZBIERANIE USTAWIEŃ (Wszystkie dostępne parametry z Valid<NAZWA>Config)
	// =========================================================================
	let j: job::ValidPreparedJobConfig = if !mode_is_job_from_toml {
		//let blank: Vec<&str> = Vec::<&str>::new();
		//job::ValidPreparedJobConfig {
		//    // 0. Execution Config (Domyślnie: Debug: true, SaveAs: true, CliColor: true)
		//    exec: job::ValidExecutionParse::parse_vec_as_config(&blank),
		//    // 1. Workspace (Domyślnie ".")
		//    work: job::ValidWorkspaceParse::parse_vec_as_config(["."]),
		//    // 2. Patterns (Domyślnie zbiór filtrów zdefiniowany w config.rs)
		//    patt: job::ValidPatternParse::parse_vec_as_config(&blank),
		//    // 3. Table Parts (Domyślnie md + mf)
		//    part: job::ValidTablePartParse::parse_vec_as_config(&blank),
		//    // 4. Column Item (Domyślnie Tree, IconsLite, Name: false, Align: false)
		//    item: job::ValidColumnItemParse::parse_vec_as_config(["list-tree", "name-show", "ws-show", "icons-lite"]),
		//    // 5. Table Columns (Domyślnie Date, Time, Size, Item, Path)
		//    cols: job::ValidTableColumnsParse::parse_vec_as_config(&blank),
		//    // 6. Column Date (Domyślnie "%Y W%V %u-%a")
		//    date: job::ValidColumnDateParse::parse_vec_as_config(&blank),
		//    // 7. Column Time (Domyślnie "%H:%M:%S.%3f")
		//    time: job::ValidColumnTimeParse::parse_vec_as_config(&blank),
		//    // 8. Column Size (Domyślnie Decimal / SI)
		//    size: job::ValidColumnSizeParse::parse_vec_as_config(&blank),
		//    // 9. Sort By (Domyślnie Name, [Spec][Num][AZaz], Reverse: false)
		//    sort: job::ValidSortByParse::parse_vec_as_config(&blank),
		//    // 10. Save As (Domyślnie "./target/.cargo-plot/", "Project Snapshot", SOTC/COTS names)
		//    save: job::ValidSaveAsParse::parse_vec_as_config(&blank),
		//}
		job::ValidPreparedJobConfig::default()
	} else {
		job::ValidPreparedJobConfig::from_toml(None).unwrap()
	};

	// =========================================================================
	// STEP 0: Inicjalizacja
	// =========================================================================
	// =========================================================================
	// STEP 1: Skanowanie i pobranie metadanych z dysku
	// =========================================================================
	let step1 = job::engine_step1_scanner(&j);

	println!("Skanowanie zakończone. Zabrano wierszy (ValidResultMainRow): {}\n", step1.rows.len());

	// =========================================================================
	// STEP 2: Formatowanie komórek
	// =========================================================================
	let step2: Vec<job::FormattedRow> = job::engine_step2_data_formater(&j, &step1);

	// =========================================================================
	// STEP 6: ZAPISYWANIE
	// =========================================================================
	if j.exec.save_as && !j.exec.only_dry {
		job::engine_step6_data_save(&j, &step2).unwrap();
	}
	// =========================================================================
	// STEP 7: PODGLĄD
	// =========================================================================
	if !j.exec.mute {
		let limit = step2.len().min(75);
		println!("{}", job::gens::mock_render(&step2[..limit], &j.cols));
	}
	// =========================================================================
	println!("\n=== KONIEC ===\n");
}
