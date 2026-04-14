use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigManifest {
	pub job: Vec<ConfigJob>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigJob {
	pub id: String,
	pub name: Option<String>,
	pub description: Option<String>,
	#[serde(default)]
    pub quiet_work: bool,
    pub scan: ConfigPattern,
    pub spec: ConfigSpec,
    pub table_sotc: ConfigLayout,
	pub export: Option<ConfigExport>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigPattern {
	#[serde(default = "default_work_path")]
	pub work_path: String,
    pub patterns: Vec<String>,
	#[serde(default)]
	pub ignore_case: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigSpec {
	pub mode: String,
	#[serde(default)]
	pub hide_stats: bool,
	#[serde(default)]
	pub hide_promo: bool,
    pub trimming: Option<ConfigTrimming>, // Zagnieżdżone w [job.spec.trimming]
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ConfigLayout {
    pub sort_by: String,    // Zgodnie z TOML
    pub sort_order: String, // Zgodnie z TOML
	pub reverse: bool,
    pub structure: String,
	pub columns: Vec<String>,
    pub trim_page: usize,
	pub more_icons: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigTrimming {
	pub show_page: Option<usize>,
	pub page_size: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigExport {
	pub save_sotc_at: Option<String>,
	pub title_sotc: Option<String>,
	pub save_cots_at: Option<String>,
	pub title_cots: Option<String>,
}

fn default_work_path() -> String { ".".to_string() }