use std::{fs, io, path::Path};

use crate::lib::{
	job,
	logic::{LangMapper, NodeIs},
};

// ============================================================================
// EXPORTER (Step 6) - ZAPIS DO PLIKU MARKDOWN
// ============================================================================

pub fn engine_step6_data_save(j: &job::ValidPreparedJobParams, formatted_rows: &[job::FormattedRow]) -> io::Result<()> {
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
	let table_plain_text = job::view::mock_render(formatted_rows, &j.cols);
	
	// ========================================================================
	// GENEROWANIE TREŚCI
	// ========================================================================
	

		// 4. Generowanie nazw plików (z zabezpieczeniem przed "wiszącym" podkreślnikiem gdy name to "")
	let name_index_sotc = build_filename(&j.save.name, &j.hash, j.save.name_is_prefix, "SOTC-INDEX");
	let name_files_cots = build_filename(&j.save.name, &j.hash, j.save.name_is_prefix, "COTS-FILES");
	let name_combined   = build_filename(&j.save.name, &j.hash, j.save.name_is_prefix, "COTS");	

	//-------------------------
		
	let h1_title = format!("# {} (v:{})\n\n", &j.save.title, &j.hash);
	let h2_title_sotc = format!("## INDEX [SOTC - Structure of the content] (v:{})\n", &j.hash);
    let h2_title_cots = format!("## FILES [COTS - Content of the structure] (v:{})\n", &j.hash);
	let h3_title_file = |idx: usize, path: &str| -> String 
		{ format!("### {:03}: `{}`\n\n", idx, path) };

	let mut sotc_index = String::new();
	sotc_index.push_str(&h2_title_sotc);
	sotc_index.push_str("\n```plaintext\n");
	sotc_index.push_str(&table_plain_text);
	sotc_index.push_str("```\n");
	
	let mut cots_files = String::new();
	cots_files.push_str(&h2_title_cots);
	cots_files.push_str("\n");
	cots_files.push_str(&generate_cots_files(formatted_rows, h3_title_file));

	// ========================================================================
	// ZAPIS TREŚCI
	// ========================================================================
	if j.save.not_separately {
		let path: std::path::PathBuf = out_dir.join(name_combined);
		let mut text: String = String::new();
		
		text.push_str(&h1_title);
		text.push_str(&sotc_index);
		text.push_str(&cots_files);
		text.push_str(&sotc_index);

		fs::write(&path, text)?;
		println!("📦 Zapisano COTS (INDEX & FILES) do: {}", path.display());
	}
	else {
		let path: (std::path::PathBuf,std::path::PathBuf) = (out_dir.join(name_index_sotc),out_dir.join(name_files_cots));
		let mut text: (String,String) = (String::new(), String::new());

		text.0.push_str(&h1_title);
		text.1.push_str(&h1_title);
		text.0.push_str(&sotc_index);
		text.1.push_str(&cots_files);

		fs::write(&path.0, text.0)?;
		println!("📦 Zapisano SOTC (INDEX) do: {}", path.0.display());
		fs::write(&path.1, text.1)?;
		println!("📦 Zapisano COTS (FILES) do: {}", path.1.display());
	}


	

	Ok(())
}



/// ⚡ Pomocnicza funkcja generująca WSZYSTKIE bloki. Pętla wylądowała tutaj!
fn generate_cots_files<F>(rows: &[job::FormattedRow], mut format_title: F) -> String 
where
    F: FnMut(usize, &str) -> String,
{
    let mut content = String::new();

    // Enumerate() działa na całości, zachowując zgodność numeracji plików z tabelą ("dziury" dla folderów)
    for (i, row) in rows.iter().enumerate() {
        let n = &row.raw.node;
        
        if n.node == NodeIs::Dir {
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
            match fs::read_to_string(rel_path) {
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