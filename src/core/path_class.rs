
/// Zwraca odpowiednią ikonę (emoji) dla podanej ścieżki,
/// rozpoznając foldery (końcówka '/') oraz elementy ukryte (kropka na początku nazwy).
pub fn get_icon_for_path(path: &str) -> &'static str {
    let is_dir = path.ends_with('/');
    
    // Wyciągamy samą nazwę pliku/folderu:
    // 1. Usuwamy ew. ukośnik z końca (żeby folder nie zwrócił pustego stringa)
    // 2. Dzielimy przez ukośniki i bierzemy ostatni element
    let nazwa = path.trim_end_matches('/').split('/').last().unwrap_or("");
    let is_hidden = nazwa.starts_with('.');

    // Dobieramy odpowiednią ikonę na podstawie dwóch cech
    match (is_dir, is_hidden) {
        (true, false) => "📁",  // Zwykły folder
        (true, true)  => "🗃️",  // Ukryty folder (z kropką)
        (false, false)=> "📄",  // Zwykły plik
        (false, true) => "⚙️ ", // Ukryty plik (konfiguracyjny z kropką)
    }
}