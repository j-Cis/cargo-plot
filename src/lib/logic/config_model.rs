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
    pub pattern: ConfigPattern,
    #[serde(default)]
    pub layout: ConfigLayout,
    pub trimming: Option<ConfigTrimming>,
    pub export: Option<ConfigExport>,
    #[serde(default)]
    pub render: ConfigRender,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigPattern {
    #[serde(default = "default_work_path")]
    pub work_path: String,
    #[serde(default)]
    pub ignore_case: bool,
    pub patterns: Vec<String>,
    pub mode: String,
}

fn default_work_path() -> String { ".".to_string() }

#[derive(Debug, Deserialize, Default, Clone)]
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

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ConfigRender {
    #[serde(default)]
    pub hide_stats: bool,
    #[serde(default)]
    pub hide_promo: bool,
    #[serde(default)]
    pub quiet_work: bool,
}