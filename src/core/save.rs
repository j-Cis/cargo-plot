use super::super::i18n::I18n;
use crate::theme::for_path_tree::get_file_type;
use std::fs;
use std::path::Path;

pub struct SaveFile;

impl SaveFile {
    // ⚡ Upubliczniamy funkcję, żeby kod w `code.rs` mógł wygenerować stopkę
    pub fn generate_by_section(tag: &str, typ: &str, i18n: &I18n, cmd: &str) -> String {
        format!(
            "\n\n---\n---\n\n{}\n\n{}\n\n```bash\n{}\n```\n\n{}\n\n{}\n\n{}\n\n---\n",
            i18n.by_title(typ),
            i18n.by_cmd(),
            cmd, // ⚡ Używa czystej, przetworzonej komendy!
            i18n.by_instructions(),
            i18n.by_link(),
            i18n.by_version(tag)
        )
    }
    /// Wspólna logika zapisu do pliku (DRY): tworzenie folderów i zapis IO.
    fn write_to_disk(filepath: &str, content: &str, log_name: &str, i18n: &I18n) {
        let path = Path::new(filepath);

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
            && !parent.exists()
            && let Err(e) = fs::create_dir_all(parent)
        {
            eprintln!(
                "{}",
                i18n.dir_create_err(&parent.to_string_lossy(), &e.to_string())
            );
            return;
        }

        match fs::write(path, content) {
            Ok(_) => println!("{}", i18n.save_success(log_name, filepath)),
            Err(e) => eprintln!("{}", i18n.save_err(log_name, filepath, &e.to_string())),
        }
    }
    /// Formatowanie i zapis samego widoku struktury (ścieżek)
    pub fn paths(content: &str, filepath: &str, tag: &str, add_by: bool, i18n: &I18n, cmd: &str) {
        let by_section = if add_by {
            Self::generate_by_section(tag, "paths", i18n, cmd)
        } else {
            String::new()
        };
        let internal_tag = if add_by { "" } else { tag }; // Zapobiega dublowaniu tagu
        let file_name = Path::new(filepath)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        // ⚡ DODAJE NAGŁÓWEK H1 NA POCZĄTKU
        let markdown_content = format!(
            "# {}\n\n```plaintext\n{}\n```\n\n{}{}",
            file_name, content, internal_tag, by_section
        );

        Self::write_to_disk(
            filepath,
            &markdown_content,
            if i18n.lang == crate::i18n::Lang::Pl {
                "ścieżki"
            } else {
                "paths"
            },
            i18n,
        );
    }

    /// Formatowanie i zapis pełnego cache (drzewo + zawartość plików)
    pub fn codes(
        tree_text: &str, paths: &[String], base_dir: &str, filepath: &str, tag: &str, add_by: bool, i18n: &I18n, cmd: &str,
    ) {
        let by_section = if add_by {
            Self::generate_by_section(tag, "codes", i18n, cmd)
        } else {
            String::new()
        };
        let internal_tag = if add_by { "" } else { tag }; // Zapobiega dublowaniu tagu
        let file_name = Path::new(filepath)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        let mut content = String::new();
        content.push_str(&format!("# {}\n\n", file_name));

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
                    "### {:03}: `{}`\n\n{}\n\n",
                    counter,
                    p_str,
                    i18n.skip_binary()
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
                        "### {:03}: `{}`\n\n{}\n\n",
                        counter,
                        p_str,
                        i18n.read_err()
                    ));
                }
            }
            counter += 1;
        }

        content.push_str(&format!("\n\n{}{}", internal_tag, by_section));
        Self::write_to_disk(
            filepath,
            &content,
            if i18n.lang == crate::i18n::Lang::Pl {
                "kod (cache)"
            } else {
                "code (cache)"
            },
            i18n,
        );
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
