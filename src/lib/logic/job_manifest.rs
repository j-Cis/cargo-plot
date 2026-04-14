use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ConfigManifest {
    pub job: Vec<ConfigJob>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigJob {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub pattern: ConfigPattern,
    
    #[serde(default)]
    pub layout: ConfigLayout,
    pub trimming: Option<ConfigTrimming>,
    pub export: Option<ConfigExport>,
    
    #[serde(default)]
    pub render: ConfigRender,
}

#[derive(Debug, Deserialize)]
pub struct ConfigPattern {
    #[serde(default = "default_work_path")]
    pub work_path: String,
    #[serde(default)]
    pub ignore_case: bool,
    pub patterns: Vec<String>,
    pub mode: String,
}

fn default_work_path() -> String { ".".to_string() }

#[derive(Debug, Deserialize, Default)]
pub struct ConfigLayout {
    #[serde(default)]
    pub list_instead_tree: bool,
    #[serde(default = "default_sort")]
    pub sort: String,
    #[serde(default)]
    pub reverse: bool,
    #[serde(default)]
    pub columns: Vec<String>,
    #[serde(default)]
    pub more_icons: bool,
}

fn default_sort() -> String { "kind".to_string() }

#[derive(Debug, Deserialize)]
pub struct ConfigTrimming {
    pub show_page: Option<usize>,
    pub page_size: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigExport {
    pub save_sotc_at: Option<String>,
    pub title_sotc: Option<String>,
    pub save_cots_at: Option<String>,
    pub title_cots: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ConfigRender {
    #[serde(default)]
    pub hide_stats: bool,
    #[serde(default)]
    pub hide_promo: bool,
    #[serde(default)]
    pub quiet_work: bool,
}

impl ConfigManifest {
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("❌ Nie można odczytać pliku konfiguracyjnego '{}': {}", path, e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("❌ Błąd parsowania pliku TOML: {}", e))
    }

    pub fn list_jobs(&self, file_path: &str) {
        println!("======================================================================");
        println!("📜 DOSTĘPNE ZADANIA W: {}", file_path);
        println!("======================================================================");
        
        if self.job.is_empty() {
            println!("⚠️ Brak zdefiniowanych zadań (jobs).");
            return;
        }
        
        for job in &self.job {
            let name = job.name.as_deref().unwrap_or("Brak nazwy");
            let desc = job.description.as_deref().unwrap_or("Brak opisu");
            println!("🔹 [{}] - {} - {}", job.id, name, desc);
        }
    }

    pub fn get_job(&self, target_id: Option<&str>) -> Result<&ConfigJob, String> {
        if self.job.is_empty() {
            return Err("❌ Plik nie zawiera żadnych zadań.".to_string());
        }

        match target_id {
            Some(id) => self.job.iter()
                .find(|j| j.id == id)
                .ok_or_else(|| format!("❌ Nie znaleziono zadania o ID: '{}'", id)),
            None => Ok(self.job.first().unwrap()),
        }
    }
}