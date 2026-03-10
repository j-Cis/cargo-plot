// src/lib/fn_fileslang.rs

/// Struktura przechowująca metadane dla danego typu pliku
pub struct PathFileType {
    pub icon: &'static str,
    pub md_lang: &'static str,
}
/// SSoT dla ikony folderu
pub const DIR_ICON: &str = "📂";

/// SSoT (Single Source of Truth) dla rozszerzeń plików.
/// Zwraca odpowiednią ikonę do drzewa ASCII oraz język formatowania Markdown.
pub fn get_file_type(ext: &str) -> PathFileType {
    match ext {
        "rs" => PathFileType { icon: "🦀", md_lang: "rust" },
        "toml" => PathFileType { icon: "⚙️", md_lang: "toml" },
        "slint" => PathFileType { icon: "🎨", md_lang: "slint" },
        "md" => PathFileType { icon: "📝", md_lang: "markdown" },
        "json" => PathFileType { icon: "🔣", md_lang: "json" },
        "yaml" | "yml" => PathFileType { icon: "🛠️", md_lang: "yaml" },
        "html" => PathFileType { icon: "🌐", md_lang: "html" },
        "css" => PathFileType { icon: "🖌️", md_lang: "css" },
        "js" => PathFileType { icon: "📜", md_lang: "javascript" },
        "ts" => PathFileType { icon: "📘", md_lang: "typescript" },
        _ => PathFileType { icon: "📄", md_lang: "text" }, // Domyślny fallback
    }
}