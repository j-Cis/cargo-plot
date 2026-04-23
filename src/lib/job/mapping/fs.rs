use crate::lib::job::{self};

impl job::schema::ValidPreparedJobParams {
	/// Gwarantuje, że pod wskazaną ścieżką istnieje plik. Jeśli nie – tworzy go z ustawień domyślnych.
	pub fn ensure_config_exists(target_path: &str) -> Result<(), String> {
		let p = std::path::Path::new(target_path);

		if !p.exists() {
			// Zapewniamy istnienie folderów nadrzędnych
			if let Some(parent) = p.parent() {
				std::fs::create_dir_all(parent)
					.map_err(|e| format!("Nie udało się utworzyć folderu dla {}: {}", target_path, e))?;
			}

			let default_cfg = Self::default();
			let toml_content = default_cfg.to_toml("default_job", None, None);

			std::fs::write(p, toml_content)
				.map_err(|e| format!("Błąd zapisu pliku konfiguracyjnego {}: {}", target_path, e))?;

			println!("🌱 Utworzono plik konfiguracji: {}", target_path);
		}
		Ok(())
	}

	/// Zmienia nazwę zepsutego pliku na backup (dodając tag czasu) i tworzy na jego miejsce nowy, domyślny.
	pub fn backup_and_reset_config(target_path: &str) -> Result<(), String> {
		let p = std::path::Path::new(target_path);

		if p.exists() {
			let time_tag = crate::lib::logic::tag_time().0;
			// Sprytne doklejenie tagu: np. "moj_config_backup-R2026...Q43.toml"
			let backup_path = format!("{}_backup-{}.toml", target_path.trim_end_matches(".toml"), time_tag);

			std::fs::rename(p, &backup_path)
				.map_err(|e| format!("Błąd tworzenia backupu dla {}: {}", target_path, e))?;

			println!("⚠️ Uszkodzony plik! Utworzono kopię zapasową: {}", backup_path);
		}

		// Skoro zrobiliśmy miejsce, wzywamy naszą główną metodę do utworzenia świeżego pliku
		Self::ensure_config_exists(target_path)
	}
}

impl job::schema::ValidSaveAsParams {
	/// Helper: Gwarantuje, że folder docelowy istnieje i zwraca do niego pełną ścieżkę
	pub fn ensure_out_dir(&self) -> std::io::Result<std::path::PathBuf> {
		if !self.out_dir.exists() {
			std::fs::create_dir_all(&self.out_dir)?;
		}
		Ok(self.out_dir.clone())
	}
}
