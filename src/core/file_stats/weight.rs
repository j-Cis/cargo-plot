// [ENG]: Logic for calculating and formatting file and directory weights.
// [POL]: Logika obliczania i formatowania wag plików oraz folderów.

use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitSystem {
    Decimal,
    Binary,
    Both,
    None,
}

#[derive(Debug, Clone)]
pub struct WeightConfig {
    pub system: UnitSystem,
    pub precision: usize,
    pub show_for_files: bool,
    pub show_for_dirs: bool,
    pub dir_sum_included: bool,
}

impl Default for WeightConfig {
    fn default() -> Self {
        Self {
            system: UnitSystem::Decimal,
            precision: 5,
            show_for_files: true,
            show_for_dirs: true,
            dir_sum_included: true,
        }
    }
}

/// [POL]: Pobiera wagę ścieżki (plik lub folder rekurencyjnie).
pub fn get_path_weight(path: &Path, sum_included_only: bool) -> u64 {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return 0,
    };

    if metadata.is_file() {
        return metadata.len();
    }

    if metadata.is_dir() && !sum_included_only {
        return get_dir_size(path);
    }

    0
}

/// [POL]: Prywatny pomocnik do liczenia rozmiaru folderu na dysku.
fn get_dir_size(path: &Path) -> u64 {
    fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|e| {
                    let p = e.path();
                    if p.is_dir() {
                        get_dir_size(&p)
                    } else {
                        e.metadata().map(|m| m.len()).unwrap_or(0)
                    }
                })
                .sum()
        })
        .unwrap_or(0)
}

/// [POL]: Formatuje bajty na czytelny ciąg znaków (np. [kB 12.34]).
pub fn format_weight(bytes: u64, is_dir: bool, config: &WeightConfig) -> String {
    if config.system == UnitSystem::None {
        return String::new();
    }

    let should_show = (is_dir && config.show_for_dirs) || (!is_dir && config.show_for_files);
    if !should_show {
        let empty_width = 7 + config.precision;
        return format!("{:width$}", "", width = empty_width);
    }

    let (base, units) = match config.system {
        UnitSystem::Binary => (1024.0_f64, vec!["B", "KiB", "MiB", "GiB", "TiB", "PiB"]),
        _ => (1000.0_f64, vec!["B", "kB", "MB", "GB", "TB", "PB"]),
    };

    if bytes == 0 {
        return format!(
            "[{:>3} {:>width$}] ",
            units[0],
            "0",
            width = config.precision
        );
    }

    let bytes_f = bytes as f64;
    let exp = (bytes_f.ln() / base.ln()).floor() as usize;
    let exp = exp.min(units.len() - 1);
    let value = bytes_f / base.powi(exp as i32);
    let unit = units[exp];

    let mut formatted_value = format!("{value:.10}");
    if formatted_value.len() > config.precision {
        formatted_value = formatted_value[..config.precision]
            .trim_end_matches('.')
            .to_string();
    } else {
        formatted_value = format!("{formatted_value:>width$}", width = config.precision);
    }

    format!("[{unit:>3} {formatted_value}] ")
}
