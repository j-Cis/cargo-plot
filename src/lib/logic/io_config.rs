use std::fs;
use std::path::Path;

// Importujemy z Twojej biblioteki:
use super::{tag_time, JobSpec};

// Zakładam, że masz ConfigManifest, który np. posiada pole `job: Vec<JobSpec>`
// Jeśli korzystasz z Serde, upewnij się, że struktury mają #[derive(Serialize, Deserialize)]
use super::ConfigManifest;

/// Mechanizm odpowiedzialny za zarządzanie wejściem/wyjściem dla plików konfiguracyjnych TOML.
pub struct IoConfig;

impl IoConfig {
    /// Domyślna ścieżka do pliku konfiguracyjnego w projekcie
    pub const DEFAULT_PATH: &'static str = "./.x-do.toml";

    // ============================================================================
    // GENEROWANIE ZAWARTOŚCI
    // ============================================================================

    /// Zwraca domyślną zawartość konfiguracji wygenerowaną na podstawie `JobSpec::default()`
    fn generate_default_content() -> String {
        // Opcja A (Zalecana, jeśli Twoje struktury używają #[derive(Serialize)]):
        // let default_manifest = ConfigManifest { job: vec![JobSpec::default()] };
        // toml::to_string_pretty(&default_manifest).unwrap_or_default()
        
        // Opcja B (Jeśli jeszcze nie używasz Serde do serializacji, 
        // to ręczny zrzut "na podstawie" JobSpec::default):
        r#"[[job]]
id = "default_job"
mode = "m"

[job.scan]
work_path = "."
patterns = [
    "./{.rustfmt,Cargo,rust-toolchain,Makefile}.toml&/",
    "./**/*.rs&/",
    "!./target/**",
    "!./.git/**",
    "./.{gitattributes,gitignore}",
    "./.github/workflows/*.yml&/",
    "./.vscode/settings.json&/",
    "./{API,ARCHITECTURE,AUTHORS,CHANGELOG,README,ROADMAP,TODO}.md"
]
ignore_case = false

[job.table]
sort_by = "name"
sort_order = "asc"
structure = "tree"
columns = ["date", "time", "size", "tree", "icon", "number", "path"]
trim_page = 1
more_icons = false

save_sotc_at = "./target/.cargo-plot/File"
title_sotc = "Rust Project Snapshot [Structure of the content]"
save_cots_at = "./target/.cargo-plot/Docs"
title_cots = "Rust Project Snapshot [Content of the structure]"
"#
        .to_string()
    }

    // ============================================================================
    // ZARZĄDZANIE PLIKIEM (ZAPIS / BACKUP)
    // ============================================================================

    /// Tworzy plik konfiguracyjny, tylko jeśli ten jeszcze nie istnieje.
    pub fn default_config_init_if_missing(path: &str) -> Result<(), String> {
        if !Path::new(path).exists() {
            Self::default_config_reset_force(path)?;
            println!("🌱 Utworzono domyślny plik konfiguracyjny: {}", path);
        }
        Ok(())
    }

    /// Tworzy (lub nadpisuje) plik konfiguracyjny twardym resetem do ustawień fabrycznych.
    pub fn default_config_reset_force(path: &str) -> Result<(), String> {
        let content = Self::generate_default_content();
        fs::write(path, content)
            .map_err(|e| format!("❌ Błąd zapisu pliku konfiguracyjnego {}: {}", path, e))?;
        Ok(())
    }

    /// Bezpieczny reset: zmienia nazwę starego pliku dodając do niego tag czasowy, 
    /// a następnie tworzy nowy ze świeżą konfiguracją.
    pub fn default_config_reset_safe(path: &str) -> Result<(), String> {
        if Path::new(path).exists() {
            let tag = tag_time().0; // Pobiera czas wygenerowany przez Twoją funkcję
            let backup_path = format!("{}.{}.bak", path, tag);
            
            fs::rename(path, &backup_path)
                .map_err(|e| format!("❌ Błąd podczas tworzenia kopii zapasowej: {}", e))?;
                
            println!("📦 Utworzono kopię zapasową starej konfiguracji: {}", backup_path);
        }
        
        Self::default_config_reset_force(path)?;
        println!("🔄 Zresetowano plik konfiguracyjny (utworzono nowy: {})", path);
        
        Ok(())
    }

    // ============================================================================
    // ODCZYT PLIKU (LOADER)
    // ============================================================================

    /// Ładuje konfigurację z domyślnego miejsca: `./.x-do.toml`
    pub fn default_config_loader() -> Result<ConfigManifest, String> {
        Self::config_loader_from(Self::DEFAULT_PATH)
    }

    /// Ładuje i parsuje konfigurację ze wskazanej ścieżki.
    pub fn config_loader_from(path: &str) -> Result<ConfigManifest, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("❌ Nie można odczytać pliku {}: {}", path, e))?;
        
        // Wymaga crate 'toml' i serde::Deserialize zaimplementowanego na ConfigManifest
        toml::from_str(&content)
            .map_err(|e| format!("❌ Błąd parsowania struktury TOML w pliku {}: {}", path, e))
    }
}