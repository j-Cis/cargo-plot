use std::{fs, io, path::Path};

use crate::lib::{
	job,
	logic::{LangMapper, NodeIs},
};

// ============================================================================
// EXPORTER (Step 6) - ZAPIS DO PLIKU MARKDOWN
// ============================================================================

pub fn engine_step6_data_save(j: &job::ValidPreparedJobConfig, formatted_rows: &[job::FormattedRow]) -> io::Result<()> {
	// 1. Gwarantujemy istnienie folderu docelowego
	let out_dir = &j.save.ensure_out_dir()?;

	// ========================================================================
	// ZAPIS KONFIGURACJI ZADANIA (Task.toml)
	// ========================================================================
	let toml_path = out_dir.join("Task.toml");
	let toml_content = j.to_toml(
		"auto-generated",
		Some("Cargo-Plot Export"),
		Some("Pełna konfiguracja użyta do wygenerowania tego zrzutu"),
	);
	fs::write(&toml_path, toml_content)?;
	println!("📄 Zapisano konfigurację zadania: {}", toml_path.display());

	// ========================================================================
	// GENEROWANIE TABELI (Używamy gotowego generatora mock_render)
	// ========================================================================
	let table_plain_text = job::gens::mock_render(formatted_rows, &j.cols);

	// ========================================================================
	// 2. ZAPIS SOTC (INDEX) - Struktura zawartości
	// ========================================================================
	let sotc_path = out_dir.join(&j.save.name_index_sotc);
	let mut sotc_content = String::new();

	sotc_content.push_str(&j.save.title_index_sotc);
	sotc_content.push_str("\n\n```plaintext\n");
	sotc_content.push_str(&table_plain_text);
	sotc_content.push_str("```\n");

	fs::write(&sotc_path, sotc_content)?;
	println!("📦 Zapisano SOTC (INDEX) do: {}", sotc_path.display());

	// ========================================================================
	// 3. ZAPIS COTS (FILES) - Zawartość struktury
	// ========================================================================
	let cots_path = out_dir.join(&j.save.name_files_cots);
	let mut cots_content = String::new();

	cots_content.push_str(&j.save.title_files_cots);
	cots_content.push_str("\n\n");

	// Parsowanie i dołączanie fizycznej zawartości plików
	for (i, row) in formatted_rows.iter().enumerate() {
		let n = &row.raw.node;
		let rel_path = &n.path.str; // Ścieżka relatywna płynąca bezpośrednio z potoku
		let actual_idx = i + 1; // Offset dla paginacji (Step 4)

		if n.node == NodeIs::Dir {
			continue;
		}

		let ext = Path::new(rel_path).extension().unwrap_or_default().to_string_lossy().to_string();
		let lang = LangMapper::get_md_lang(&ext);
		let is_bin = n.file_is_binary.unwrap_or(false);

		if LangMapper::is_blacklisted(&ext) || is_bin {
			cots_content.push_str(&format!(
				"## {:03}: `{}`\n\n> *(Plik binarny/graficzny - pominięto zawartość)*\n\n",
				actual_idx, rel_path
			));
			continue;
		}

		// Odczyt pliku natywnie po ścieżce z noda
		match fs::read_to_string(rel_path) {
			Ok(file_content) => {
				let safe_content = file_content.replace('\t', "    ");
				cots_content.push_str(&format!(
					"## {:03}: `{}`\n\n```{}\n{}\n```\n\n",
					actual_idx, rel_path, lang, safe_content
				));
			}
			Err(_) => {
				cots_content.push_str(&format!(
					"## {:03}: `{}`\n\n> *(Błąd odczytu / plik nie jest UTF-8)*\n\n",
					actual_idx, rel_path
				));
			}
		}
	}

	fs::write(&cots_path, cots_content)?;
	println!("📦 Zapisano COTS (FILES) do: {}", cots_path.display());

	Ok(())
}
