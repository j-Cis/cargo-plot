// [EN]: Path classification and icon mapping for tree visualization.
// [PL]: Klasyfikacja ścieżek i mapowanie ikon dla wizualizacji drzewa.

/// [EN]: Global icon used for directory nodes.
/// [PL]: Globalna ikona używana dla węzłów będących folderami.
pub const DIR_ICON: &str = "📂";

pub const FILE_ICON: &str = "📄";

/// [EN]: Defines visual and metadata properties for a file type.
/// [PL]: Definiuje wizualne i metadanowe właściwości dla typu pliku.
pub struct PathFileType {
    pub icon: &'static str,
    pub md_lang: &'static str,
}

/// [EN]: Returns file properties based on its extension.
/// [PL]: Zwraca właściwości pliku na podstawie jego rozszerzenia.
#[must_use]
pub fn get_file_type(ext: &str) -> PathFileType {
    match ext {
        "rs" => PathFileType {
            icon: "🦀",
            md_lang: "rust",
        },
        "toml" => PathFileType {
            icon: "⚙️",
            md_lang: "toml",
        },
        "slint" => PathFileType {
            icon: "🎨",
            md_lang: "slint",
        },
        "md" => PathFileType {
            icon: "📝",
            md_lang: "markdown",
        },
        "json" => PathFileType {
            icon: "🔣",
            md_lang: "json",
        },
        "yaml" | "yml" => PathFileType {
            icon: "🛠️",
            md_lang: "yaml",
        },
        "html" => PathFileType {
            icon: "📖",
            md_lang: "html",
        },
        "css" => PathFileType {
            icon: "🖌️",
            md_lang: "css",
        },
        "js" => PathFileType {
            icon: "📜",
            md_lang: "javascript",
        },
        "ts" => PathFileType {
            icon: "📘",
            md_lang: "typescript",
        },
        // [EN]: Default fallback for unknown file types.
        // [PL]: Domyślny fallback dla nieznanych typów plików.
        _ => PathFileType {
            icon: "📄",
            md_lang: "text",
        },
    }
}

/// [EN]: Character set used for drawing tree branches and indents.
/// [PL]: Zestaw znaków używanych do rysowania gałęzi drzewa i wcięć.
#[derive(Debug, Clone)]
pub struct TreeStyle {
    // [EN]: Directories (d)
    // [PL]: Foldery (d)
    pub dir_last_with_children: String, // └──┬
    pub dir_last_no_children: String,   // └───
    pub dir_mid_with_children: String,  // ├──┬
    pub dir_mid_no_children: String,    // ├───

    // [EN]: Files (f)
    // [PL]: Pliki (f)
    pub file_last: String, // └──•
    pub file_mid: String,  // ├──•

    // [EN]: Indentations for subsequent levels (i)
    // [PL]: Wcięcia dla kolejnych poziomów (i)
    pub indent_last: String, // "   "
    pub indent_mid: String,  // "│  "
}

impl Default for TreeStyle {
    fn default() -> Self {
        Self {
            dir_last_with_children: "└──┬".to_string(),
            dir_last_no_children: "└───".to_string(),
            dir_mid_with_children: "├──┬".to_string(),
            dir_mid_no_children: "├───".to_string(),

            file_last: "└──•".to_string(),
            file_mid: "├──•".to_string(),

            indent_last: "   ".to_string(),
            indent_mid: "│  ".to_string(),
        }
    }
}