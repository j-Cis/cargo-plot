use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitSystem {
    Decimal, // 1000^n (kB, MB...)
    Binary,  // 1024^n (KiB, MiB...)
    Both,
    None,
}

#[derive(Debug, Clone)]
pub struct WeightConfig {
    pub system: UnitSystem,
    pub precision: usize, // Całkowita szerokość pola "xxxxx" (min 3)
    pub show_for_files: bool,
    pub show_for_dirs: bool,
    pub dir_sum_included: bool, // true = tylko uwzględnione, false = rzeczywista waga folderu
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

/// Główna funkcja formatująca wagę do postaci [qq xxxxx]
pub fn format_weight(bytes: u64, config: &WeightConfig) -> String {
    if config.system == UnitSystem::None {
        return String::new();
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

    // Formatowanie liczby do stałej szerokości "xxxxx"
    let formatted_value = format_value_with_precision(value, config.precision);

    format!("[{:>3} {}] ", unit, formatted_value)
}

fn format_value_with_precision(value: f64, width: usize) -> String {
    // Sprawdzamy ile cyfr ma część całkowita
    let integer_part = value.floor() as u64;
    let integer_str = integer_part.to_string();
    let int_len = integer_str.len();

    if int_len >= width {
        // Jeśli sama liczba całkowita zajmuje całe miejsce lub więcej
        return integer_str[..width].to_string();
    }

    // Obliczamy ile miejsc po przecinku nam zostało (width - int_len - 1 dla kropki)
    let available_precision = if width > int_len + 1 {
        width - int_len - 1
    } else {
        0
    };

    let formatted = format!("{:.1$}", value, available_precision);

    // Na wypadek zaokrągleń (np. 99.99 -> 100.0), przycinamy do width
    if formatted.len() > width {
        formatted[..width].trim_end_matches('.').to_string()
    } else {
        format!("{:>width$}", formatted, width = width)
    }
}

/// Pobiera wagę pliku lub folderu (rekurencyjnie)
pub fn get_path_weight(path: &Path, sum_included_only: bool) -> u64 {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return 0,
    };

    if metadata.is_file() {
        return metadata.len();
    }

    if metadata.is_dir() && !sum_included_only {
        // Rzeczywista waga folderu na dysku
        return get_dir_size(path);
    }

    0 // Jeśli liczymy tylko sumę plików, bazowo folder ma 0 (sumowanie nastąpi w drzewie)
}

fn get_dir_size(path: &Path) -> u64 {
    fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
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
