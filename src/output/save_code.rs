use std::fs;
use std::path::Path;
use crate::output::generator::config_backlist::is_blacklisted_extension;
use crate::theme::for_path_tree::get_file_type;

pub fn save(tree_text: &str, paths: &[String], base_dir: &str, filepath: &str) {
    let path = Path::new(filepath);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            let _ = fs::create_dir_all(parent);
        }
    }

    let mut content = String::new();
    
    // Wstawiamy wygenerowane drzewo ścieżek
    content.push_str("```plaintext\n");
    content.push_str(tree_text);
    content.push_str("```\n\n");

    let mut counter = 1;

    for p_str in paths {
        if p_str.ends_with('/') {
            continue; // Pomijamy katalogi
        }

        let absolute_path = Path::new(base_dir).join(p_str);
        let ext = absolute_path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();
        
        let lang = get_file_type(&ext).md_lang;

        if is_blacklisted_extension(&ext) {
            content.push_str(&format!(
                "### {:03}: `{}`\n\n> *(Plik binarny/graficzny - pominięto)*\n\n",
                counter, p_str
            ));
            counter += 1;
            continue;
        }

        match fs::read_to_string(&absolute_path) {
            Ok(file_content) => {
                content.push_str(&format!(
                    "### {:03}: `{}`\n\n```{}\n{}\n```\n\n",
                    counter, p_str, lang, file_content
                ));
            }
            Err(_) => {
                content.push_str(&format!(
                    "### {:03}: `{}`\n\n> *(Błąd odczytu / plik nie jest UTF-8)*\n\n",
                    counter, p_str
                ));
            }
        }
        counter += 1;
    }

    match fs::write(path, content) {
        Ok(_) => println!("💾 Pomyślnie zapisano cache (kod) do pliku: {}", filepath),
        Err(e) => eprintln!("❌ Błąd zapisu kodu do pliku {}: {}", filepath, e),
    }
}