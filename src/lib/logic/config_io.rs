use std::fs;

use super::ConfigManifest;

pub fn load_manifest(path: &str) -> Result<ConfigManifest, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Błąd odczytu pliku TOML '{}': {}", path, e))?;

    toml::from_str(&content).map_err(|e| format!("Błąd parsowania TOML: {}", e))
}

pub fn create_default_if_missing(path: &str) -> Result<(), String> {
    let p = std::path::Path::new(path);
    if !p.exists() {
        fs::write(path, CONFIG_TEMPLATE)
            .map_err(|e| format!("Błąd podczas zapisu domyślnej konfiguracji '{}': {}", path, e))?;
        println!("✅ Utworzono domyślny plik konfiguracyjny: {}", path);
    }
    Ok(())
}

const CONFIG_TEMPLATE: &str = r#"# ============================================================================
# 【ENG】 x-do Batch Configuration File
# 【POL】 Plik konfiguracji wsadowej x-do
# ============================================================================

[[job]]
id = "p1"
name = "snapshot-rust"
description = "【ENG】 Standard Rust project snapshot / 【POL】 Standardowy zrzut projektu Rust"

[job.pattern]
work_path = "."
ignore_case = false
patterns = [
    "./{.rustfmt,Cargo}.toml",
    "./{src,examples,tests}/**{/*.rs,/}",
    "./build.rs"
]
mode = "matched"

[job.layout]
list_instead_tree = false
sort = "kind"
reverse = false
columns = ["date", "time", "size", "treelist", "icon", "number", "path"]
more_icons = false

[job.trimming]
# show_page = 1
# page_size = 20

[job.export]
save_sotc_at = "./target/.cargo-plot/abc"
title_sotc = "SOTC - Rust Snapshot"
save_cots_at = "./target/.cargo-plot/def"
title_cots = "COTS - Rust Snapshot"

[job.render]
hide_stats = false
hide_promo = false
quiet_work = false
"#;
