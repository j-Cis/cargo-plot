use std::{fs, io, path::Path};

use crate::lib::{
    logic::{ScanNodeIs, file_save_safe_if_changed, tag_from_time_now},
    mapping::LangMapper,
    pipelines::step2::{FormattedRow, render_table},
    schema::{ReadyJob, SharedJobRunMode},
};

// ============================================================================
// EXPORTER (Step 6) - ZAPIS DO PLIKU MARKDOWN
// ============================================================================

pub fn engine_step6_data_save(job: &ReadyJob, formatted_rows: &[FormattedRow]) -> io::Result<()> {

    // Funkcja prepare() dba o to, żeby DryRun usuwał flagi Save.
    // My tu sprawdzamy po prostu, czy program ma autoryzację do zapisu.
    let should_save = job.run_modes().contains(&SharedJobRunMode::Save)
        || job.run_modes().contains(&SharedJobRunMode::SaveWithInspection);

    if !should_save {
        return Ok(()); // Tryb cichy/suchy bieg - wychodzimy bez błędu
    }

    let export = job.export();
    let out_dir = export.out_dir();

	// 1. Gwarantujemy istnienie folderu docelowego
    if !out_dir.exists() {
        fs::create_dir_all(out_dir)?;
    }

	// ========================================================================
	// GENEROWANIE TABELI (Używamy gotowego generatora mock_render)
	// ========================================================================
	let table_plain_text = render_table(formatted_rows, job.attributes().select());

	// ========================================================================
	// GENEROWANIE TREŚCI
	// ========================================================================
	let tag = tag_from_time_now().0;
	let name_is_prefix = export.name_is_first();
    let export_name = export.name();

	let name_index_sotc = build_filename(export_name, &tag, name_is_prefix, "SOTC-INDEX");
    let name_files_cots = build_filename(export_name, &tag, name_is_prefix, "COTS-FILES");
    let name_combined = build_filename(export_name, &tag, name_is_prefix, "COTS");

	//-------------------------

	let h1_title = format!("# {}\n\n", format!("{} (v:{})", export.title(), tag).trim());
    let h2_title_sotc = format!("## INDEX [SOTC - Structure of the content] (v:{})\n", tag);
    let h2_title_sotc2 = format!("## INDEX OF THE CONTENT\n");
    let h2_title_cots = format!("## FILES [COTS - Content of the structure] (v:{})\n", tag);
    let h3_title_file = |idx: usize, path: &str| -> String { format!("### {:03}: `{}`\n\n", idx, path) };

	let mut sotc_index = String::new();
	sotc_index.push_str(&h2_title_sotc);
	sotc_index.push_str("\n```plaintext\n");
	sotc_index.push_str(&table_plain_text);
	sotc_index.push_str("```\n");

	let mut sotc_index2 = String::new();
	sotc_index2.push_str(&h2_title_sotc2);
	sotc_index2.push_str("\n```plaintext\n");
	sotc_index2.push_str(&table_plain_text);
	sotc_index2.push_str("```\n");

	let mut cots_files = String::new();
	cots_files.push_str(&h2_title_cots);
	cots_files.push_str("\n");
	cots_files.push_str(&generate_cots_files(
        formatted_rows,
        h3_title_file,
        job.explorer().workspace_dir(),
    ));

	// ========================================================================
	// ZAPIS TREŚCI
	// ========================================================================
	if !export.save_separately() {
		let path: std::path::PathBuf = out_dir.join(name_combined);
		let mut text: String = String::new();

		text.push_str(&h1_title);
		text.push_str(&sotc_index);
		text.push_str("\n");
		text.push_str(&cots_files);
		text.push_str(&sotc_index2);
		text.push_str("\n---\n");

		file_save_safe_if_changed(&path, &text);
		println!("📦 Zapisano COTS (INDEX & FILES) do: {}", path.display());
	}
	else {
		let path: (std::path::PathBuf,std::path::PathBuf) = (out_dir.join(name_index_sotc),out_dir.join(name_files_cots));
		let mut text: (String,String) = (String::new(), String::new());

		text.0.push_str(&h1_title);
		text.1.push_str(&h1_title);
		text.0.push_str(&sotc_index);
		text.1.push_str(&cots_files);
		text.0.push_str("\n---\n");
		text.1.push_str("\n---\n");

		file_save_safe_if_changed(&path.0, &text.0);
		println!("📦 Zapisano SOTC (INDEX) do: {}", path.0.display());
		file_save_safe_if_changed(&path.1, &text.1);
		println!("📦 Zapisano COTS (FILES) do: {}", path.1.display());
	}




	Ok(())
}



/// ⚡ Pomocnicza funkcja generująca WSZYSTKIE bloki z kodem.
fn generate_cots_files<F>(rows: &[FormattedRow], mut format_title: F, workspace_dir: &Path) -> String
where
    F: FnMut(usize, &str) -> String,
{
    let mut content = String::new();

    // Enumerate() działa na całości, zachowując zgodność numeracji plików z tabelą ("dziury" dla folderów)
    for (i, row) in rows.iter().enumerate() {
        let n = &row.raw.node;

        if n.node == ScanNodeIs::Dir {
            continue;
        }

        let rel_path = &n.path.str;
        let actual_idx = i + 1; // Numer zgodny z wierszem tabeli

        // Użycie przekazanego callbacka do tytułu
        content.push_str(&format_title(actual_idx, rel_path));

        let ext = Path::new(rel_path).extension().unwrap_or_default().to_string_lossy().to_string();
        let lang = LangMapper::get_md_lang(&ext);
        let is_bin = n.file_is_binary.unwrap_or(false);

        // Zrzut kodu lub błędu
        if LangMapper::is_blacklisted(&ext) || is_bin {
            content.push_str("> *(Plik binarny/graficzny - pominięto zawartość)*\n\n");
        } else {
			// ⚡ Używamy workspace_dir jako bazy do odczytu zawartości
            let clean_rel = rel_path.strip_prefix("./").unwrap_or(rel_path);
            let absolute_path = workspace_dir.join(clean_rel);

            match fs::read_to_string(&absolute_path) {
                Ok(file_content) => {
                    let safe_content = file_content.replace('\t', "    ");
                    content.push_str(&format!("```{}\n{}\n```\n\n", lang, safe_content));
                }
                Err(_) => {
                    content.push_str("> *(Błąd odczytu / plik nie jest UTF-8)*\n\n");
                }
            }
        }
    }

    content
}

/// Helper: Generuje nazwę pliku w zależności od podanej nazwy, prefiksu i tagu.
fn build_filename(name: &str, hash: &str, is_prefix: bool, tag: &str) -> String {
    match (name.is_empty(), is_prefix) {
        (true, _)      => format!("{}{}.md", hash, tag),           // Brak nazwy
        (false, true)  => format!("{}_{}{}.md", name, hash, tag),  // Nazwa jako prefiks
        (false, false) => format!("{}{}_{}.md", hash, tag, name),  // Nazwa jako sufiks
    }
}
