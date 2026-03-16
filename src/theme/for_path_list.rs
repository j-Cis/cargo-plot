/// [POL]: Przypisuje ikonę (emoji) do ścieżki na podstawie atrybutów: katalog oraz status elementu ukrytego.
/// [ENG]: Assigns an icon (emoji) to a path based on attributes: directory status and hidden element status.
pub fn get_icon_for_path(path: &str) -> &'static str {
    let is_dir = path.ends_with('/');

    let nazwa = path.trim_end_matches('/').split('/').last().unwrap_or("");
    let is_hidden = nazwa.starts_with('.');

    match (is_dir, is_hidden) {
        (true, false) => "📁",  // [POL]: Folder        | [ENG]: Directory
        (true, true) => "🗃️",   // [POL]: Ukryty folder | [ENG]: Hidden directory
        (false, false) => "📄", // [POL]: Plik          | [ENG]: File
        (false, true) => "⚙️ ", // [POL]: Ukryty plik   | [ENG]: Hidden file
    }
}
