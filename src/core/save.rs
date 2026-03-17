use std::fs;
use std::path::Path;
use crate::theme::for_path_tree::get_file_type;

pub struct SaveFile;

impl SaveFile {
    /// Wspólna logika zapisu do pliku (DRY): tworzenie folderów i zapis IO.
    fn write_to_disk(filepath: &str, content: &str, log_name: &str) {
        let path = Path::new(filepath);

        // Upewnienie się, że foldery nadrzędne istnieją
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("❌ Błąd: Nie można utworzyć katalogu {:?} ({})", parent, e);
                    return;
                }
            }
        }

        // Zapis pliku
        match fs::write(path, content) {
            Ok(_) => println!("💾 Pomyślnie zapisano {} do pliku: {}", log_name, filepath),
            Err(e) => eprintln!("❌ Błąd zapisu {} do pliku {}: {}", log_name, filepath, e),
        }
    }

    /// Formatowanie i zapis samego widoku struktury (ścieżek)
    pub fn paths(content: &str, filepath: &str, tag: &str) {
        let markdown_content = format!("```plaintext\n{}\n```\n\n{}", content, tag);
        Self::write_to_disk(filepath, &markdown_content, "ścieżki");
    }

    /// Formatowanie i zapis pełnego cache (drzewo + zawartość plików)
    pub fn codes(tree_text: &str, paths: &[String], base_dir: &str, filepath: &str, tag: &str) {
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

        // Znacznik na końcu
        content.push_str(&format!("\n\n{}", tag));

        Self::write_to_disk(filepath, &content, "kod (cache)");
    }
}

// [EN]: Security mechanisms to prevent processing non-text or binary files.
// [PL]: Mechanizmy bezpieczeństwa zapobiegające przetwarzaniu plików nietekstowych lub binarnych.

/// [EN]: Checks if a file extension is on the list of forbidden binary types.
/// [PL]: Sprawdza, czy rozszerzenie pliku znajduje się na liście zabronionych typów binarnych.
fn is_blacklisted_extension(ext: &str) -> bool {
    let e = ext.to_lowercase();

    matches!(
        e.as_str(),
        // --------------------------------------------------
        // GRAFIKA I DESIGN
        // --------------------------------------------------
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "svg" | "webp" | "tiff" | "tif" | "heic" | "psd" | 
        "ai" | 
        // --------------------------------------------------
        // BINARKI | BIBLIOTEKI I ARTEFAKTY KOMPILACJI
        // --------------------------------------------------
        "exe" | "dll" | "so" | "dylib" | "bin" | "wasm" | "pdb" | "rlib" | "rmeta" | "lib" | 
        "o" | "a" | "obj" | "pch" | "ilk" | "exp" | 
        "jar" | "class" | "war" | "ear" | 
        "pyc" | "pyd" | "pyo" | "whl" | 
        // --------------------------------------------------
        // ARCHIWA I PACZKI
        // --------------------------------------------------
        "zip" | "tar" | "gz" | "tgz" | "7z" | "rar" | "bz2" | "xz" | "iso" | "dmg" | "pkg" | "apk" | 
        // --------------------------------------------------
        // DOKUMENTY | BAZY DANYCH I FONTY
        // --------------------------------------------------
        "sqlite" | "sqlite3" | "db" | "db3" | "mdf" | "ldf" | "rdb" | 
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "odt" | "ods" | "odp" | 
        "woff" | "woff2" | "ttf" | "eot" | "otf" | 
        // --------------------------------------------------
        // MEDIA (AUDIO / WIDEO)
        // --------------------------------------------------
        "mp3" | "mp4" | "avi" | "mkv" | "wav" | "flac" | "ogg" | "m4a" | "mov" | "wmv" | "flv"
    )
}