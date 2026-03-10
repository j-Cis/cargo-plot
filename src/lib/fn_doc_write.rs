use crate::fn_files_blacklist::is_blacklisted_extension;
use crate::fn_path_utils::to_display_path;
use crate::fn_pathtype::get_file_type;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn write_md(
    out_path: &str,
    files: &[PathBuf],
    id_map: &HashMap<PathBuf, String>,
    tree_text: Option<String>,
    _stamp: &str,
    id_style: &str,
) -> io::Result<()> {
    let mut content = String::new();

    content.push_str(&format!("# RAPORT ({})\n\n", out_path));
    // content.push_str(&format!("# RAPORT {} ({})\n\n", stamp, out_path));

    if let Some(tree) = tree_text {
        content.push_str("```text\n");
        content.push_str(&tree);
        content.push_str("```\n\n");
    }

    let current_dir = std::env::current_dir().unwrap_or_default();
    let mut file_counter = 1;

    for path in files {
        if path.is_dir() {
            continue;
        }

        let display_path = to_display_path(path, &current_dir);

        if path.exists() {
            let original_id = id_map
                .get(path)
                .cloned()
                .unwrap_or_else(|| "BrakID".to_string());

            // <-- POPRAWIONE: używamy id_style bezpośrednio
            let header_name = match id_style {
                "id-num" => format!("Plik-{:03}", file_counter),
                "id-non" => "Plik".to_string(),
                _ => format!("Plik-{}", original_id),
            };
            file_counter += 1;

            let ext = path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();
            let lang = get_file_type(&ext).md_lang;

            // KROK 1: Sprawdzenie czarnej listy rozszerzeń
            if is_blacklisted_extension(&ext) {
                content.push_str(&format!(
                    "## {}: `{}`\n\n> *(Plik binarny/graficzny - pominięto zawartość)*\n\n",
                    header_name, display_path
                ));
                continue;
            }

            // KROK 2: Bezpieczna próba odczytu zawartości
            match fs::read_to_string(path) {
                Ok(file_content) => {
                    if lang == "markdown" {
                        content.push_str(&format!("## {}: `{}`\n\n", header_name, display_path));
                        for line in file_content.lines() {
                            if line.trim().is_empty() {
                                content.push_str(">\n");
                            } else {
                                content.push_str(&format!("> {}\n", line));
                            }
                        }
                        content.push_str("\n\n");
                    } else {
                        content.push_str(&format!(
                            "## {}: `{}`\n\n```{}\n{}\n```\n\n",
                            header_name, display_path, lang, file_content
                        ));
                    }
                }
                Err(_) => {
                    // Fallback: Plik nie ma rozszerzenia binarnego, ale jego zawartość to nie jest czysty tekst UTF-8
                    content.push_str(&format!("## {}: `{}`\n\n> *(Nie można odczytać pliku jako tekst UTF-8 - pominięto)*\n\n", header_name, display_path));
                }
            }
        } else {
            content.push_str(&format!(
                "## BŁĄD: `{}` (Plik nie istnieje)\n\n",
                display_path
            ));
        }
    }

    fs::write(out_path, &content)?;
    Ok(())
}
