use std::{fs, path::Path};



/// Gwarantuje istnienie pliku: zapisuje go tylko wtedy, gdy nie istnieje.
pub fn file_ensure(path: &Path, content: &str) {
	if let Some(parent) = path.parent() {
		let _ = fs::create_dir_all(parent);
	}
	if !path.exists() {
		let _ = fs::write(path, content);
	}
}
/// Tworzy kopię zapasową pliku dodając rozszerzenie .bak, jeśli plik istnieje.
pub fn file_backup(path: &Path) {
	if path.exists() {
		let mut new_name = path.file_name().unwrap_or_default().to_os_string();
		new_name.push(".bak");
		let backup_path = path.with_file_name(new_name);
		let _ = fs::copy(path, backup_path);
	}
}
pub fn file_save_force(path: &Path, content: &str) {
	if let Some(parent) = path.parent() {
		let _ = fs::create_dir_all(parent);
	}
	fs::write(path, content).expect("Błąd krytyczny: Nie udało się zapisać pliku");
}

/// Bezpieczny zapis: robi backup istniejącego pliku przed jego nadpisaniem.
pub fn file_save_safe(path: &Path, content: &str) {
	file_backup(path);
	file_save_force(path, content);
}

pub fn file_remove(path: &Path) {
	if path.exists() {
		// Ignorujemy wynik, bo jeśli plik zniknął w międzyczasie,
		// to cel (brak pliku) i tak został osiągnięty.
		let _ = fs::remove_file(path);
	}
}

/// Bezpieczny zapis z weryfikacją: nadpisuje (i robi backup) TYLKO wtedy,
/// gdy nowa treść różni się od tej już zapisanej na dysku.
pub fn file_save_safe_if_changed(path: &Path, content: &str) {
    if path.exists() {
        // Jeśli plik istnieje, sprawdzamy jego zawartość
        if let Ok(existing_content) = fs::read_to_string(path) {
            if existing_content == content {
                // Treść jest identyczna - nic nie zmieniamy, pomijamy cały proces
                return;
            }
        }
    }

    // Wywołujemy standardową procedurę (backup + zapis), bo treść jest nowa 
    // lub plik jeszcze fizycznie nie istnieje.
    file_save_safe(path, content);
}