use std::fs;
use std::path::Path;

/// [POL]: Zapisuje wygenerowany ciąg znaków do pliku tekstowego.
/// Tworzy brakujące katalogi po drodze, jeśli to konieczne.
pub fn save(content: &str, filepath: &str) {
    let path = Path::new(filepath);

    // Upewnij się, że foldery nadrzędne istnieją
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("❌ Błąd: Nie można utworzyć katalogu {:?} ({})", parent, e);
                return;
            }
        }
    }

    let markdown_content = format!("```plaintext\n{}\n```\n", content);

    // Właściwy zapis do pliku
    match fs::write(path, markdown_content) {
        Ok(_) => println!("💾 Pomyślnie zapisano wynik do pliku: {}", filepath),
        Err(e) => eprintln!("❌ Błąd zapisu do pliku {}: {}", filepath, e),
    }
}