use crate::lib::job::{self, TraitParseFromFlags};

impl job::schema::ValidPreparedJobParams {
	/// Wczytuje konfigurację z pliku TOML.
	/// Sztuczka: `path: None` użyje domyślnej lokalizacji, `Some("...")` użyje własnej.
	/// Główna metoda ładująca. Używa jednej ścieżki docelowej jako źródła prawdy.
	pub fn from_toml(path: Option<&str>) -> Result<Self, String> {
		// 1. Definiujemy jedną docelową ścieżkę (domyślną lub podaną przez użytkownika)
		let target_path = path.unwrap_or("./target/.cargo-plot/task.toml");

		// 2. Krok I: Jeśli pliku nie ma - zostanie natychmiast utworzony
		Self::ensure_config_exists(target_path)?;

		// 3. Krok II: Odczyt i ratunek
		let content = std::fs::read_to_string(target_path)
			.map_err(|e| format!("Nie można odczytać pliku {}: {}", target_path, e))?;

		let toml_val: toml::Value = match toml::from_str(&content) {
			Ok(val) => val,
			Err(_) => {
				// Składnia jest nieczytelna/zepsuta -> Odpalamy walec naprawczy!
				Self::backup_and_reset_config(target_path)?;

				// Po resecie wczytujemy na bezczelnego - to nasz świeży plik, więc na 100% zadziała
				let fresh_content = std::fs::read_to_string(target_path).unwrap();
				toml::from_str(&fresh_content).unwrap()
			}
		};

		// 4. Krok III: Wyciąganie wartości (logika z poprzednich etapów)
		let job = toml_val
			.get("job")
			.and_then(|j| j.as_array())
			.and_then(|arr| arr.get(0))
			.ok_or("Brak sekcji [[job]] w pliku TOML")?;

		// =====================================================================
		// ZWINNE HELPERY DO WYCIĄGANIA DANYCH Z TOML
		// =====================================================================
		let get_str = |parent: Option<&toml::Value>, key: &str, default: &str| -> String {
			parent.and_then(|p| p.get(key)).and_then(|v| v.as_str()).unwrap_or(default).to_string()
		};

		let get_bool = |parent: Option<&toml::Value>, key: &str, default: bool| -> bool {
			parent.and_then(|p| p.get(key)).and_then(|v| v.as_bool()).unwrap_or(default)
		};

		let get_arr = |parent: Option<&toml::Value>, key: &str| -> Vec<String> {
			parent
				.and_then(|p| p.get(key))
				.and_then(|v| v.as_array())
				.map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
				.unwrap_or_default()
		};

		// =====================================================================
		// NAWIGACJA PO SEKCJACH
		// =====================================================================
		let explorer = job.get("explorer");
		let attributes = job.get("attributes");
		let config = attributes.and_then(|a| a.get("config"));
		let tuples = job.get("tuples");
		let save_cfg = tuples.and_then(|t| t.get("save"));

		// =====================================================================
		// MAPOWANIE NA FLAGI CLI (Przygotowanie wsadu dla TraitParseFromFlags)
		// =====================================================================

		let run_mode = get_arr(Some(job), "run_mode");

		// Work: Budujemy z pojedynczych wartości naszą flagową strukturę wejściową
		let workspace = get_str(explorer, "workspace", ".");
		let ignore_case = get_bool(explorer, "ignore_case", false);
		let mut work_flags = vec![workspace];
		if ignore_case {
			work_flags.push("ignore-case".to_string());
		}

		let patt_flags = get_arr(explorer, "patterns");
		let part_flags = get_arr(explorer, "parts");

		let cols_flags = get_arr(attributes, "select");

		let item_flags = get_arr(config, "item");
		let date_flags = vec![get_str(config, "date", "default")];
		let time_flags = vec![get_str(config, "time", "default")];
		let size_flags = vec![get_str(config, "size", "decimal")];

		let sort_flags = get_arr(tuples, "sort");

		// Save: Budujemy flagi typu `klucz=wartosc`
		let out_dir = get_str(save_cfg, "out_dir", "./target/.cargo-plot/");
		let title = get_str(save_cfg, "title", "Project Snapshot");
		let name = get_str(save_cfg, "name", "");
		let name_is_prefix = get_bool(save_cfg, "name_is_prefix", false);
		let not_separately = get_bool(save_cfg, "not_separately", false);

		let mut save_flags = Vec::new();
		if !out_dir.is_empty() {
			save_flags.push(format!("outdir={}", out_dir));
		}
		if !title.is_empty() {
			save_flags.push(format!("title={}", title));
		}
		if !name.is_empty() {
			save_flags.push(format!("name={}", name));
		}
		if name_is_prefix {
			save_flags.push("name-prefix".to_string());
		} else {
			save_flags.push("name-suffix".to_string());
		}
		if not_separately {
            save_flags.push("not-separately".to_string()); 
        }
		// =====================================================================
		// BUDOWANIE OSTATECZNEJ STRUKTURY (Recykling logiki!)
		// =====================================================================
		Ok(job::schema::ValidPreparedJobParams {
			hash: crate::lib::logic::tag_time().0,
			exec: job::schema::ValidExecutionFlags::parse_vec_as_config(&run_mode),
			work: job::schema::ValidWorkspaceFlags::parse_vec_as_config(&work_flags),
			patt: job::schema::ValidPatternFlags::parse_vec_as_config(&patt_flags),
			part: job::schema::ValidTablePartFlags::parse_vec_as_config(&part_flags),
			cols: job::schema::ValidTableColumnsFlags::parse_vec_as_config(&cols_flags),
			item: job::schema::ValidColumnItemFlags::parse_vec_as_config(&item_flags),
			date: job::schema::ValidColumnDateFlags::parse_vec_as_config(&date_flags),
			time: job::schema::ValidColumnTimeFlags::parse_vec_as_config(&time_flags),
			size: job::schema::ValidColumnSizeFlags::parse_vec_as_config(&size_flags),
			sort: job::schema::ValidSortByFlags::parse_vec_as_config(&sort_flags),
			save: job::schema::ValidSaveAsFlags::parse_vec_as_config(&save_flags),
		})
	}
}
