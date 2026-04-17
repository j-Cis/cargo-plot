# CargoPlot v0.2.0 (v:R2026W12D6H03M32S35T29Q09COTS [Content of the structure])

```plaintext
[KiB 174.4] └──┬ 📂 cargo-plot-2                   ./cargo-plot-2/
[KiB 1.380]    ├──• ⚙️ Cargo.toml                 ./Cargo.toml
[KiB 173.0]    └──┬ 📂 src                         ./src/
[  B 70.00]       ├──• 🦀 addon.rs                 ./src/addon.rs
[KiB 2.431]       ├──┬ 📂 addon                    ./src/addon/
[KiB 2.431]       │  └──• 🦀 time_tag.rs           ./src/addon/time_tag.rs
[  B 120.0]       ├──• 🦀 core.rs                  ./src/core.rs
[KiB 65.85]       ├──┬ 📂 core                     ./src/core/
[KiB 1.117]       │  ├──• 🦀 file_stats.rs         ./src/core/file_stats.rs
[KiB 3.177]       │  ├──┬ 📂 file_stats            ./src/core/file_stats/
[KiB 3.177]       │  │  └──• 🦀 weight.rs          ./src/core/file_stats/weight.rs
[  B 285.0]       │  ├──• 🦀 path_matcher.rs       ./src/core/path_matcher.rs
[KiB 23.00]       │  ├──┬ 📂 path_matcher          ./src/core/path_matcher/
[KiB 14.65]       │  │  ├──• 🦀 matcher.rs         ./src/core/path_matcher/matcher.rs
[KiB 4.501]       │  │  ├──• 🦀 sort.rs            ./src/core/path_matcher/sort.rs
[KiB 3.845]       │  │  └──• 🦀 stats.rs           ./src/core/path_matcher/stats.rs
[  B 101.0]       │  ├──• 🦀 path_store.rs         ./src/core/path_store.rs
[KiB 4.134]       │  ├──┬ 📂 path_store            ./src/core/path_store/
[KiB 2.069]       │  │  ├──• 🦀 context.rs         ./src/core/path_store/context.rs
[KiB 2.065]       │  │  └──• 🦀 store.rs           ./src/core/path_store/store.rs
[  B 329.0]       │  ├──• 🦀 path_view.rs          ./src/core/path_view.rs
[KiB 24.31]       │  ├──┬ 📂 path_view             ./src/core/path_view/
[KiB 11.03]       │  │  ├──• 🦀 grid.rs            ./src/core/path_view/grid.rs
[KiB 2.647]       │  │  ├──• 🦀 list.rs            ./src/core/path_view/list.rs
[KiB 2.528]       │  │  ├──• 🦀 node.rs            ./src/core/path_view/node.rs
[KiB 8.099]       │  │  └──• 🦀 tree.rs            ./src/core/path_view/tree.rs
[KiB 1.683]       │  ├──• 🦀 patterns_expand.rs    ./src/core/patterns_expand.rs
[KiB 7.727]       │  └──• 🦀 save.rs               ./src/core/save.rs
[KiB 5.772]       ├──• 🦀 execute.rs               ./src/execute.rs
[KiB 5.250]       ├──• 🦀 i18n.rs                  ./src/i18n.rs
[  B 161.0]       ├──• 🦀 interfaces.rs            ./src/interfaces.rs
[KiB 88.51]       ├──┬ 📂 interfaces               ./src/interfaces/
[KiB 1.306]       │  ├──• 🦀 cli.rs                ./src/interfaces/cli.rs
[KiB 16.90]       │  ├──┬ 📂 cli                   ./src/interfaces/cli/
[KiB 9.784]       │  │  ├──• 🦀 args.rs            ./src/interfaces/cli/args.rs
[KiB 7.119]       │  │  └──• 🦀 engine.rs          ./src/interfaces/cli/engine.rs
[KiB 4.423]       │  ├──• 🦀 gui.rs                ./src/interfaces/gui.rs
[KiB 40.22]       │  ├──┬ 📂 gui                   ./src/interfaces/gui/
[KiB 10.04]       │  │  ├──• 🦀 code.rs            ./src/interfaces/gui/code.rs
[KiB 3.955]       │  │  ├──• 🦀 i18n.rs            ./src/interfaces/gui/i18n.rs
[KiB 8.537]       │  │  ├──• 🦀 paths.rs           ./src/interfaces/gui/paths.rs
[KiB 12.95]       │  │  ├──• 🦀 settings.rs        ./src/interfaces/gui/settings.rs
[KiB 4.729]       │  │  └──• 🦀 shared.rs          ./src/interfaces/gui/shared.rs
[  B 390.0]       │  ├──• 🦀 tui.rs                ./src/interfaces/tui.rs
[KiB 25.27]       │  └──┬ 📂 tui                   ./src/interfaces/tui/
[KiB 10.62]       │     ├──• 🦀 i18n.rs            ./src/interfaces/tui/i18n.rs
[KiB 13.27]       │     ├──• 🦀 menu.rs            ./src/interfaces/tui/menu.rs
[KiB 1.375]       │     └──• 🦀 state.rs           ./src/interfaces/tui/state.rs
[  B 75.00]       ├──• 🦀 lib.rs                   ./src/lib.rs
[  B 664.0]       ├──• 🦀 main.rs                  ./src/main.rs
[  B 79.00]       ├──• 🦀 output.rs                ./src/output.rs
[  B 46.00]       ├──• 🦀 theme.rs                 ./src/theme.rs
[KiB 4.084]       └──┬ 📂 theme                    ./src/theme/
[  B 837.0]          ├──• 🦀 for_path_list.rs      ./src/theme/for_path_list.rs
[KiB 3.267]          └──• 🦀 for_path_tree.rs      ./src/theme/for_path_tree.rs

```

### 001: `./Cargo.toml`

```rust
[package]
name = "cargo-plot"
version = "0.2.0"
authors = ["Jan Roman Cisowski „j-Cis”"]
edition = "2024"
rust-version = "1.94.0"
description = "Szwajcarski scyzoryk do wizualizacji struktury projektu i generowania dokumentacji bezpośrednio z poziomu Cargo."
license = "MIT OR Apache-2.0"
readme = "README.md"
repository = "https://github.com/j-Cis/cargo-plot"

keywords = [ "cargo",  "tree",  "markdown",  "filesystem", "documentation"]
categories = [ "development-tools::cargo-plugins",  "command-line-utilities", "command-line-interface", "text-processing",]
resolver = "3"

[package.metadata.cargo]
edition = "2024"


[dependencies]
chrono = "0.4.44"
walkdir = "2.5.0"
regex = "1.12.3"
clap = { version = "4.5.60", features = ["derive"] }
cliclack = "0.5.0"
colored = "3.1.1"
console = "0.16.3"
ctrlc = "3.5.2"
shlex = "1.3.0"
eframe = "0.33.3"
rfd = "0.17.2"


# ==========================================
# Globalna konfiguracja lintów (Analiza kodu)
# ==========================================
[lints.rust]
# Kategorycznie zabraniamy używania bloków `unsafe` w całym projekcie
unsafe_code = "forbid"
# Ostrzegamy o nieużywanych importach, zmiennych i funkcjach
# unused = "warn"
#
[lints.clippy]
# Włączamy surowsze reguły, ale jako ostrzeżenia (nie zepsują kompilacji)
# pedantic = "warn"
# Możemy tu też wyciszyć globalnie to, co nas irytuje (opcjonalnie):
too_many_arguments = "allow"

```

### 002: `./src/addon.rs`

```rust
pub mod time_tag;

pub use time_tag::{NaiveDate, NaiveTime, TimeTag};

```

### 003: `./src/addon/time_tag.rs`

```rust
// [EN]: Functions for creating consistent date and time stamps.
// [PL]: Funkcje do tworzenia spójnych sygnatur daty i czasu.

use chrono::{Datelike, Local, Timelike, Weekday};
pub use chrono::{NaiveDate, NaiveTime};

/// [EN]: Utility struct for generating consistent time tags.
/// [PL]: Struktura narzędziowa do generowania spójnych sygnatur czasowych.
pub struct TimeTag;

impl TimeTag {
    /// [EN]: Generates a time_tag for the current local time.
    /// [PL]: Generuje time_tag dla obecnego, lokalnego czasu.
    #[must_use]
    pub fn now() -> String {
        let now = Local::now();
        Self::format(now.date_naive(), now.time())
    }

    /// [EN]: Generates a time_tag for a specific provided date and time.
    /// [PL]: Generuje time_tag dla konkretnej, podanej daty i czasu.
    #[must_use]
    pub fn custom(date: NaiveDate, time: NaiveTime) -> String {
        Self::format(date, time)
    }

    // [EN]: Private function that performs manual string construction (DRY principle).
    // [PL]: PRYWATNA funkcja, która wykonuje ręczne budowanie ciągu znaków (zasada DRY).
    fn format(date: NaiveDate, time: NaiveTime) -> String {
        let year = date.year();
        let quarter = ((date.month() - 1) / 3) + 1;

        let weekday = match date.weekday() {
            Weekday::Mon => "Mon",
            Weekday::Tue => "Tue",
            Weekday::Wed => "Wed",
            Weekday::Thu => "Thu",
            Weekday::Fri => "Fri",
            Weekday::Sat => "Sat",
            Weekday::Sun => "Sun",
        };

        let month = match date.month() {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => unreachable!(),
        };

        let millis = time.nanosecond() / 1_000_000;

        // [EN]: Format: YYYYQn Dnnn Wnn _ Day DD Mon _ HH MM SS mmm
        // [PL]: Format: RRRRQn Dnnn Wnn _ Dzień DD Miesiąc _ GG MM SS mmm
        format!(
            "{}Q{}D{:03}W{:02}_{}{:02}{}_{:02}{:02}{:02}{:03}",
            year,
            quarter,
            date.ordinal(),
            date.iso_week().week(),
            weekday,
            date.day(),
            month,
            time.hour(),
            time.minute(),
            time.second(),
            millis
        )
    }
}

```

### 004: `./src/core.rs`

```rust
pub mod file_stats;
pub mod path_matcher;
pub mod path_store;
pub mod path_view;
pub mod patterns_expand;
pub mod save;

```

### 005: `./src/core/file_stats.rs`

```rust
// use std::fs;
use std::path::{Path, PathBuf};
pub mod weight;

use self::weight::get_path_weight;

/// [POL]: Struktura przechowująca metadane (statystyki) pliku lub folderu.
#[derive(Debug, Clone)]
pub struct FileStats {
    pub path: String,      // Oryginalna ścieżka relatywna (np. "src/main.rs")
    pub absolute: PathBuf, // Pełna ścieżka absolutna na dysku
    pub weight_bytes: u64, // Rozmiar w bajtach

                           // ⚡ Miejsce na przyszłe parametry:
                           // pub created_at: Option<std::time::SystemTime>,
                           // pub modified_at: Option<std::time::SystemTime>,
}

impl FileStats {
    /// [POL]: Pobiera statystyki pliku bezpośrednio z dysku.
    pub fn fetch(path: &str, entry_absolute: &str) -> Self {
        let absolute = Path::new(entry_absolute).join(path);

        let weight_bytes = get_path_weight(&absolute, true);
        // let weight_bytes = fs::metadata(&absolute)
        //     .map(|m| m.len())
        //     .unwrap_or(0);

        Self {
            path: path.to_string(),
            absolute,
            weight_bytes,
        }
    }
}

```

### 006: `./src/core/file_stats/weight.rs`

```rust
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

    // ⚡ Jeśli sum_included_only jest false (flaga -a), liczymy rekurencyjnie fizyczny rozmiar
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

```

### 007: `./src/core/path_matcher.rs`

```rust
/// [POL]: Główny moduł logiki dopasowywania ścieżek.
/// [ENG]: Core module for path matching logic.
pub mod matcher;
pub mod sort;
pub mod stats;

pub use self::matcher::{PathMatcher, PathMatchers};
pub use self::sort::SortStrategy;
pub use self::stats::{MatchStats, ShowMode};

```

### 008: `./src/core/path_matcher/matcher.rs`

```rust
use super::sort::SortStrategy;
use super::stats::{MatchStats, ResultSet, ShowMode};
use regex::Regex;
use std::collections::HashSet;

// ==============================================================================
// ⚡ POJEDYNCZY WZORZEC (PathMatcher)
// ==============================================================================

/// [POL]: Struktura odpowiedzialna za dopasowanie pojedynczego wzorca z uwzględnieniem zależności strukturalnych.
/// [ENG]: Structure responsible for matching a single pattern considering structural dependencies.
pub struct PathMatcher {
    regex: Regex,
    targets_file: bool,
    // [POL]: Flaga @ (para plik-folder)
    // [ENG]: Flag @ (file-directory pair)
    requires_sibling: bool,
    // [POL]: Flaga $ (jednostronna relacja)
    // [ENG]: Flag $ (one-way relation)
    requires_orphan: bool,
    // [POL]: Flaga + (rekurencyjne zacienianie)
    // [ENG]: Flag + (recursive shadowing)
    is_deep: bool,
    // [POL]: Nazwa bazowa modułu do weryfikacji relacji
    // [ENG]: Base name of the module for relation verification
    base_name: String,
    // [POL]: Flaga negacji (!).
    // [ENG]: Negation flag (!).
    pub is_negated: bool,
}

impl PathMatcher {
    pub fn new(pattern: &str, case_sensitive: bool) -> Result<Self, regex::Error> {
        let is_negated = pattern.starts_with('!');
        let actual_pattern = if is_negated { &pattern[1..] } else { pattern };

        let is_deep = actual_pattern.ends_with('+');
        let requires_sibling = actual_pattern.contains('@');
        let requires_orphan = actual_pattern.contains('$');
        let clean_pattern_str = actual_pattern.replace(['@', '$', '+'], "");

        let base_name = clean_pattern_str
            .trim_end_matches('/')
            .trim_end_matches("**")
            .split('/')
            .next_back()
            .unwrap_or("")
            .split('.')
            .next()
            .unwrap_or("")
            .to_string();

        let mut re = String::new();

        if !case_sensitive {
            re.push_str("(?i)");
        }

        let mut is_anchored = false;
        let mut p = clean_pattern_str.as_str();

        let targets_file = !p.ends_with('/') && !p.ends_with("**");

        if p.starts_with("./") {
            is_anchored = true;
            p = &p[2..];
        } else if p.starts_with("**/") {
            is_anchored = true;
        }

        if is_anchored {
            re.push('^');
        } else {
            re.push_str("(?:^|/)");
        }

        let chars: Vec<char> = p.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '\\' => {
                    if i + 1 < chars.len() {
                        i += 1;
                        re.push_str(&regex::escape(&chars[i].to_string()));
                    }
                }
                '.' => re.push_str("\\."),
                '/' => {
                    if is_deep && i == chars.len() - 1 {
                        // [POL]: Pominięcie końcowego ukośnika dla flagi '+'.
                        // [ENG]: Omission of trailing slash for the '+' flag.
                    } else {
                        re.push('/');
                    }
                }
                '*' => {
                    if i + 1 < chars.len() && chars[i + 1] == '*' {
                        if i + 2 < chars.len() && chars[i + 2] == '/' {
                            re.push_str("(?:[^/]+/)*");
                            i += 2;
                        } else {
                            re.push_str(".+");
                            i += 1;
                        }
                    } else {
                        re.push_str("[^/]*");
                    }
                }
                '?' => re.push_str("[^/]"),
                '{' => {
                    let mut options = String::new();
                    i += 1;
                    while i < chars.len() && chars[i] != '}' {
                        options.push(chars[i]);
                        i += 1;
                    }
                    let escaped: Vec<String> = options.split(',').map(regex::escape).collect();
                    re.push_str(&format!("(?:{})", escaped.join("|")));
                }
                '[' => {
                    re.push('[');
                    if i + 1 < chars.len() && chars[i + 1] == '!' {
                        re.push('^');
                        i += 1;
                    }
                }
                ']' | '-' | '^' => re.push(chars[i]),
                c => re.push_str(&regex::escape(&c.to_string())),
            }
            i += 1;
        }

        if is_deep {
            re.push_str("(?:/.*)?$");
        } else {
            re.push('$');
        }

        Ok(Self {
            regex: Regex::new(&re)?,
            targets_file,
            requires_sibling,
            requires_orphan,
            is_deep,
            base_name,
            is_negated,
        })
    }

    /// [POL]: Sprawdza dopasowanie ścieżki, uwzględniając relacje rodzeństwa w strukturze plików.
    /// [ENG]: Validates path matching, considering sibling relations within the file structure.
    pub fn is_match(&self, path: &str, env: &HashSet<&str>) -> bool {
        if self.targets_file && path.ends_with('/') {
            return false;
        }

        let clean_path = path.strip_prefix("./").unwrap_or(path);

        if !self.regex.is_match(clean_path) {
            return false;
        }

        // [POL]: Relacja rodzeństwa (@) lub sieroty ($) dla plików.
        // [ENG]: Sibling relation (@) or orphan relation ($) for files.
        if (self.requires_sibling || self.requires_orphan) && !path.ends_with('/') {
            if self.is_deep && self.requires_sibling {
                if !self.check_authorized_root(path, env) {
                    return false;
                }
                return true;
            }
            let mut components: Vec<&str> = path.split('/').collect();
            if let Some(file_name) = components.pop() {
                let parent_dir = components.join("/");
                let core_name = file_name.split('.').next().unwrap_or("");
                let expected_folder = if parent_dir.is_empty() {
                    format!("{}/", core_name)
                } else {
                    format!("{}/{}/", parent_dir, core_name)
                };

                if !env.contains(expected_folder.as_str()) {
                    return false;
                }
            }
        }

        // [POL]: Dodatkowa weryfikacja rodzeństwa (@) dla katalogów.
        // [ENG]: Additional sibling verification (@) for directories.
        if self.requires_sibling && path.ends_with('/') {
            if self.is_deep {
                if !self.check_authorized_root(path, env) {
                    return false;
                }
            } else {
                let dir_no_slash = path.trim_end_matches('/');
                let has_file_sibling = env.iter().any(|&p| {
                    p.starts_with(dir_no_slash)
                        && p[dir_no_slash.len()..].starts_with('.')
                        && !p.ends_with('/')
                });

                if !has_file_sibling {
                    return false;
                }
            }
        }

        true
    }

    /// [POL]: Ewaluuje kolekcję ścieżek, sortuje wyniki i wywołuje odpowiednie akcje.
    /// [ENG]: Evaluates a path collection, sorts the results, and triggers respective actions.
    // #[allow(clippy::too_many_arguments)]
    pub fn evaluate<I, S, OnMatch, OnMismatch>(
        &self,
        paths: I,
        env: &HashSet<&str>,
        strategy: SortStrategy,
        show_mode: ShowMode,
        mut on_match: OnMatch,
        mut on_mismatch: OnMismatch,
    ) -> MatchStats
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        OnMatch: FnMut(&str),
        OnMismatch: FnMut(&str),
    {
        let mut matched = Vec::new();
        let mut mismatched = Vec::new();

        for path in paths {
            if self.is_match(path.as_ref(), env) {
                matched.push(path);
            } else {
                mismatched.push(path);
            }
        }

        strategy.apply(&mut matched);
        strategy.apply(&mut mismatched);

        let stats = MatchStats {
            m_size_matched: matched.len(),
            x_size_mismatched: mismatched.len(),
            total: matched.len() + mismatched.len(),
            m_matched: ResultSet {
                paths: matched.iter().map(|s| s.as_ref().to_string()).collect(),
                tree: None,
                list: None,
                grid: None,
            },
            x_mismatched: ResultSet {
                paths: mismatched.iter().map(|s| s.as_ref().to_string()).collect(),
                tree: None,
                list: None,
                grid: None,
            },
        };

        if show_mode == ShowMode::Include || show_mode == ShowMode::Context {
            for path in &matched {
                on_match(path.as_ref());
            }
        }

        if show_mode == ShowMode::Exclude || show_mode == ShowMode::Context {
            for path in &mismatched {
                on_mismatch(path.as_ref());
            }
        }

        stats
    }

    /// [POL]: Weryfikuje autoryzację korzenia modułu w relacji plik-folder dla trybu 'deep'.
    /// [ENG]: Verifies module root authorisation in the file-directory relation for 'deep' mode.
    fn check_authorized_root(&self, path: &str, env: &HashSet<&str>) -> bool {
        let clean = path.strip_prefix("./").unwrap_or(path);
        let components: Vec<&str> = clean.split('/').collect();

        for i in 0..components.len() {
            let comp_core = components[i].split('.').next().unwrap_or("");

            if comp_core == self.base_name {
                let base_dir = if i == 0 {
                    self.base_name.clone()
                } else {
                    format!("{}/{}", components[0..i].join("/"), self.base_name)
                };

                let full_base_dir = if path.starts_with("./") {
                    format!("./{}", base_dir)
                } else {
                    base_dir
                };
                let dir_path = format!("{}/", full_base_dir);

                let has_dir = env.contains(dir_path.as_str());
                let has_file = env.iter().any(|&p| {
                    p.starts_with(&full_base_dir)
                        && p[full_base_dir.len()..].starts_with('.')
                        && !p.ends_with('/')
                });

                if has_dir && has_file {
                    return true;
                }
            }
        }
        false
    }
}

// ==============================================================================
// ⚡ KONTENER WIELU WZORCÓW (PathMatchers)
// ==============================================================================

/// [POL]: Kontener przechowujący kolekcję silników dopasowujących ścieżki.
/// [ENG]: A container holding a collection of path matching engines.
pub struct PathMatchers {
    matchers: Vec<PathMatcher>,
}

impl PathMatchers {
    /// [POL]: Tworzy nową instancję, kompilując listę wzorców po uprzednim rozwinięciu klamer.
    /// [ENG]: Creates a new instance by compiling a list of patterns after performing brace expansion.
    pub fn new<I, S>(patterns: I, case_sensitive: bool) -> Result<Self, regex::Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut matchers = Vec::new();
        for pat in patterns {
            matchers.push(PathMatcher::new(pat.as_ref(), case_sensitive)?);
        }
        Ok(Self { matchers })
    }

    /// [POL]: Weryfikuje, czy ścieżka spełnia warunki narzucone przez zbiór wzorców (w tym negacje).
    /// [ENG]: Verifies if the path meets the conditions imposed by the pattern set (including negations).
    pub fn is_match(&self, path: &str, env: &HashSet<&str>) -> bool {
        if self.matchers.is_empty() {
            return false;
        }

        let mut has_positive = false;
        let mut matched_positive = false;

        for matcher in &self.matchers {
            if matcher.is_negated {
                // [POL]: Twarde WETO. Dopasowanie negatywne bezwzględnie odrzuca ścieżkę.
                // [ENG]: Hard VETO. A negative match unconditionally rejects the path.
                if matcher.is_match(path, env) {
                    return false;
                }
            } else {
                has_positive = true;
                if !matched_positive && matcher.is_match(path, env) {
                    matched_positive = true;
                }
            }
        }

        // [POL]: Ostateczna decyzja na podstawie zebranych danych.
        // [ENG]: Final decision based on collected data.
        if has_positive { matched_positive } else { true }
    }

    /// [POL]: Ewaluuje zbiór ścieżek, sortuje je i wykonuje odpowiednie domknięcia.
    /// [ENG]: Evaluates a set of paths, sorts them, and executes respective closures.
    pub fn evaluate<I, S, OnMatch, OnMismatch>(
        &self,
        paths: I,
        env: &HashSet<&str>,
        strategy: SortStrategy,
        show_mode: ShowMode,
        mut on_match: OnMatch,
        mut on_mismatch: OnMismatch,
    ) -> MatchStats
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        OnMatch: FnMut(&str),
        OnMismatch: FnMut(&str),
    {
        let mut matched = Vec::new();
        let mut mismatched = Vec::new();

        for path in paths {
            if self.is_match(path.as_ref(), env) {
                matched.push(path);
            } else {
                mismatched.push(path);
            }
        }

        strategy.apply(&mut matched);
        strategy.apply(&mut mismatched);

        let stats = MatchStats {
            m_size_matched: matched.len(),
            x_size_mismatched: mismatched.len(),
            total: matched.len() + mismatched.len(),
            m_matched: ResultSet {
                paths: matched.iter().map(|s| s.as_ref().to_string()).collect(),
                tree: None,
                list: None,
                grid: None,
            },
            x_mismatched: ResultSet {
                paths: mismatched.iter().map(|s| s.as_ref().to_string()).collect(),
                tree: None,
                list: None,
                grid: None,
            },
        };

        if show_mode == ShowMode::Include || show_mode == ShowMode::Context {
            for path in matched {
                on_match(path.as_ref());
            }
        }

        if show_mode == ShowMode::Exclude || show_mode == ShowMode::Context {
            for path in mismatched {
                on_mismatch(path.as_ref());
            }
        }

        stats
    }
}

```

### 009: `./src/core/path_matcher/sort.rs`

```rust
use std::cmp::Ordering;

/// [POL]: Definiuje dostępne strategie sortowania kolekcji ścieżek.
/// [ENG]: Defines available sorting strategies for path collections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortStrategy {
    /// [POL]: Brak stosowania algorytmu sortowania.
    /// [ENG]: No sorting algorithm applied.
    None,

    /// [POL]: Sortowanie alfanumeryczne w porządku rosnącym.
    /// [ENG]: Alphanumeric sorting in ascending order.
    Az,

    /// [POL]: Sortowanie alfanumeryczne w porządku malejącym.
    /// [ENG]: Alphanumeric sorting in descending order.
    Za,

    /// [POL]: Priorytet dla plików, następnie sortowanie alfanumeryczne rosnąco.
    /// [ENG]: Priority for files, followed by alphanumeric ascending sort.
    AzFileFirst,

    /// [POL]: Priorytet dla plików, następnie sortowanie alfanumeryczne malejąco.
    /// [ENG]: Priority for files, followed by alphanumeric descending sort.
    ZaFileFirst,

    /// [POL]: Priorytet dla katalogów, następnie sortowanie alfanumeryczne rosnąco.
    /// [ENG]: Priority for directories, followed by alphanumeric ascending sort.
    AzDirFirst,

    /// [POL]: Priorytet dla katalogów, następnie sortowanie alfanumeryczne malejąco.
    /// [ENG]: Priority for directories, followed by alphanumeric descending sort.
    ZaDirFirst,

    /// [POL]: Sortowanie alfanumeryczne rosnąco, grupujące logiczne pary plik-katalog (np. moduły) z priorytetem dla plików.
    /// [ENG]: Alphanumeric ascending sort grouping logical file-directory pairs (e.g. modules), prioritising files.
    AzFileFirstMerge,

    /// [POL]: Sortowanie alfanumeryczne malejąco, grupujące logiczne pary plik-katalog z priorytetem dla plików.
    /// [ENG]: Alphanumeric descending sort grouping logical file-directory pairs, prioritising files.
    ZaFileFirstMerge,

    /// [POL]: Sortowanie alfanumeryczne rosnąco, grupujące logiczne pary plik-katalog z priorytetem dla katalogów.
    /// [ENG]: Alphanumeric ascending sort grouping logical file-directory pairs, prioritising directories.
    AzDirFirstMerge,

    /// [POL]: Sortowanie alfanumeryczne malejąco, grupujące logiczne pary plik-katalog z priorytetem dla katalogów.
    /// [ENG]: Alphanumeric descending sort grouping logical file-directory pairs, prioritising directories.
    ZaDirFirstMerge,
}

impl SortStrategy {
    /// [POL]: Sortuje kolekcję ścieżek w miejscu (in-place) na podstawie wybranej strategii.
    /// [ENG]: Sorts a collection of paths in-place based on the selected strategy.
    pub fn apply<S: AsRef<str>>(&self, paths: &mut [S]) {
        if *self == SortStrategy::None {
            return;
        }

        paths.sort_by(|a_s, b_s| {
            let a = a_s.as_ref();
            let b = b_s.as_ref();

            let a_is_dir = a.ends_with('/');
            let b_is_dir = b.ends_with('/');

            // Wywołujemy naszą prywatną, hermetyczną metodę
            let a_merge = Self::get_merge_key(a);
            let b_merge = Self::get_merge_key(b);

            match self {
                SortStrategy::None => Ordering::Equal,
                SortStrategy::Az => a.cmp(b),
                SortStrategy::Za => b.cmp(a),
                SortStrategy::AzFileFirst => (a_is_dir, a).cmp(&(b_is_dir, b)),
                SortStrategy::ZaFileFirst => (a_is_dir, b).cmp(&(b_is_dir, a)),
                SortStrategy::AzDirFirst => (!a_is_dir, a).cmp(&(!b_is_dir, b)),
                SortStrategy::ZaDirFirst => (!a_is_dir, b).cmp(&(!b_is_dir, a)),
                SortStrategy::AzFileFirstMerge => {
                    (a_merge, a_is_dir, a).cmp(&(b_merge, b_is_dir, b))
                }
                SortStrategy::ZaFileFirstMerge => {
                    (b_merge, a_is_dir, b).cmp(&(a_merge, b_is_dir, a))
                }
                SortStrategy::AzDirFirstMerge => {
                    (a_merge, !a_is_dir, a).cmp(&(b_merge, !b_is_dir, b))
                }
                SortStrategy::ZaDirFirstMerge => {
                    (b_merge, !a_is_dir, b).cmp(&(a_merge, !b_is_dir, a))
                }
            }
        });
    }

    /// [POL]: Prywatna metoda. Ekstrahuje rdzenną nazwę ścieżki dla strategii Merge.
    /// [ENG]: Private method. Extracts the core path name for Merge strategies.
    fn get_merge_key(path: &str) -> &str {
        let trimmed = path.trim_end_matches('/');
        if let Some(idx) = trimmed.rfind('.')
            && idx > 0
            && trimmed.as_bytes()[idx - 1] != b'/'
        {
            return &trimmed[..idx];
        }
        trimmed
    }
}

```

### 010: `./src/core/path_matcher/stats.rs`

```rust
use crate::core::path_view::{PathGrid, PathList, PathTree, ViewMode};

/// [POL]: Podzbiór wyników zawierający surowe ścieżki i wygenerowane widoki.
#[derive(Default)]
pub struct ResultSet {
    pub paths: Vec<String>,
    pub tree: Option<PathTree>,
    pub list: Option<PathList>,
    pub grid: Option<PathGrid>,
}

// [ENG]: Simple stats object to avoid manual counting in the Engine.
// [POL]: Prosty obiekt statystyk, aby uniknąć ręcznego liczenia w Engine.
#[derive(Default)]
pub struct MatchStats {
    pub m_size_matched: usize,
    pub x_size_mismatched: usize,
    pub total: usize,
    pub m_matched: ResultSet,
    pub x_mismatched: ResultSet,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShowMode {
    Include,
    Exclude,
    Context,
}

impl MatchStats {
    /// : Hermetyzacja renderowania po stronie rdzenia.
    /// Zwraca gotowy, złożony ciąg znaków, gotowy do wrzucenia w konsolę lub plik.
    #[must_use]
    pub fn render_output(
        &self,
        view_mode: ViewMode,
        show_mode: ShowMode,
        print_info: bool,
        use_color: bool,
    ) -> String {
        let mut out = String::new();
        let do_include = show_mode == ShowMode::Include || show_mode == ShowMode::Context;
        let do_exclude = show_mode == ShowMode::Exclude || show_mode == ShowMode::Context;

        match view_mode {
            ViewMode::Grid => {
                if do_include && let Some(grid) = &self.m_matched.grid {
                    if print_info {
                        out.push_str("✅\n");
                    }
                    if use_color {
                        out.push_str(&grid.render_cli());
                    } else {
                        out.push_str(&grid.render_txt());
                    }
                }
                if do_exclude && let Some(grid) = &self.x_mismatched.grid {
                    if print_info {
                        out.push_str("❌\n");
                    }
                    if use_color {
                        out.push_str(&grid.render_cli());
                    } else {
                        out.push_str(&grid.render_txt());
                    }
                }
            }
            ViewMode::Tree => {
                if do_include && let Some(tree) = &self.m_matched.tree {
                    if print_info {
                        out.push_str("✅\n");
                    }
                    if use_color {
                        out.push_str(&tree.render_cli());
                    } else {
                        out.push_str(&tree.render_txt());
                    }
                }
                if do_exclude && let Some(tree) = &self.x_mismatched.tree {
                    if print_info {
                        out.push_str("❌\n");
                    }
                    if use_color {
                        out.push_str(&tree.render_cli());
                    } else {
                        out.push_str(&tree.render_txt());
                    }
                }
            }
            ViewMode::List => {
                if do_include && let Some(list) = &self.m_matched.list {
                    if print_info {
                        out.push_str("✅\n");
                    }
                    if use_color {
                        out.push_str(&list.render_cli(true));
                    } else {
                        out.push_str(&list.render_txt());
                    }
                }
                if do_exclude && let Some(list) = &self.x_mismatched.list {
                    if print_info {
                        out.push_str("❌\n");
                    }
                    if use_color {
                        out.push_str(&list.render_cli(false));
                    } else {
                        out.push_str(&list.render_txt());
                    }
                }
            }
        }

        out
    }
}

```

### 011: `./src/core/path_store.rs`

```rust
pub mod context;
pub mod store;

pub use self::context::PathContext;
pub use self::store::PathStore;

```

### 012: `./src/core/path_store/context.rs`

```rust
use std::env;
use std::fs;
use std::path::Path;

/// [POL]: Kontekst ścieżki roboczej - oblicza relacje między terminalem a celem skanowania.
/// [ENG]: Working path context - calculates relations between terminal and scan target.
#[derive(Debug)]
pub struct PathContext {
    pub base_absolute: String,
    pub entry_absolute: String,
    pub entry_relative: String,
}

impl PathContext {
    pub fn resolve<P: AsRef<Path>>(entered_path: P) -> Result<Self, String> {
        let path_ref = entered_path.as_ref();

        // 1. BASE ABSOLUTE: Gdzie fizycznie odpalono program?
        let cwd = env::current_dir().map_err(|e| format!("Błąd odczytu CWD: {}", e))?;
        let base_abs = cwd
            .to_string_lossy()
            .trim_start_matches(r"\\?\")
            .replace('\\', "/");

        // 2. ENTRY ABSOLUTE: Pełna ścieżka do folderu, który skanujemy
        let abs_path = fs::canonicalize(path_ref)
            .map_err(|e| format!("Nie można ustalić ścieżki '{:?}': {}", path_ref, e))?;
        let entry_abs = abs_path
            .to_string_lossy()
            .trim_start_matches(r"\\?\")
            .replace('\\', "/");

        // 3. ENTRY RELATIVE: Ścieżka od terminala do skanowanego folderu
        let entry_rel = match abs_path.strip_prefix(&cwd) {
            Ok(rel) => {
                let rel_str = rel.to_string_lossy().replace('\\', "/");
                if rel_str.is_empty() {
                    "./".to_string() // Cel to ten sam folder co terminal
                } else {
                    format!("./{}/", rel_str)
                }
            }
            Err(_) => {
                // Jeśli cel jest na innym dysku (np. C:\ a terminal na D:\)
                // lub całkiem poza strukturą CWD, relatywna nie istnieje.
                // Wracamy wtedy do tego, co wpisał użytkownik, lub dajemy absolutną.
                path_ref.to_string_lossy().replace('\\', "/")
            }
        };

        Ok(Self {
            base_absolute: base_abs,
            entry_absolute: entry_abs,
            entry_relative: entry_rel,
        })
    }
}

```

### 013: `./src/core/path_store/store.rs`

```rust
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

// use std::fs;
// use std::path::Path;

// [ENG]: Container for scanned paths and their searchable pool.
// [POL]: Kontener na zeskanowane ścieżki i ich przeszukiwalną pulę.
#[derive(Debug)]
pub struct PathStore {
    pub list: Vec<String>,
}
impl PathStore {
    /// [POL]: Skanuje katalog rekurencyjnie i zwraca znormalizowane ścieżki (prefix "./", separator "/", suffix "/" dla folderów).
    /// [ENG]: Scans the directory recursively and returns normalised paths (prefix "./", separator "/", suffix "/" for directories).
    pub fn scan<P: AsRef<Path>>(dir_path: P) -> Self {
        let mut list = Vec::new();
        let entry_path = dir_path.as_ref();

        for entry in WalkDir::new(entry_path).into_iter().filter_map(|e| e.ok()) {
            // [POL]: Pominięcie katalogu głównego (głębokość 0).
            // [ENG]: Skip the root directory (depth 0).
            if entry.depth() == 0 {
                continue;
            }

            // [POL]: Pominięcie dowiązań symbolicznych i punktów reparse.
            // [ENG]: Skip symbolic links and reparse points.
            if entry.path_is_symlink() {
                continue;
            }

            if let Ok(rel_path) = entry.path().strip_prefix(entry_path) {
                // [POL]: Normalizacja separatorów systemowych do formatu uniwersalnego.
                // [ENG]: Normalisation of system separators to a universal format.
                let relative_str = rel_path.to_string_lossy().replace('\\', "/");
                let mut final_path = format!("./{}", relative_str);

                if entry.file_type().is_dir() {
                    final_path.push('/');
                }

                list.push(final_path);
            }
        }

        Self { list }
    }

    // [ENG]: Creates a temporary pool of references for the matcher.
    // [POL]: Tworzy tymczasową pulę referencji (paths_pool) dla matchera.
    pub fn get_index(&self) -> HashSet<&str> {
        self.list.iter().map(|s| s.as_str()).collect()
    }
}

```

### 014: `./src/core/path_view.rs`

```rust
pub mod grid;
pub mod list;
pub mod node;
pub mod tree;

// Re-eksportujemy dla wygody, aby w engine.rs używać PathTree i FileNode bezpośrednio
pub use grid::PathGrid;
pub use list::PathList;
pub use tree::PathTree;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Tree,
    List,
    Grid,
}

```

### 015: `./src/core/path_view/grid.rs`

```rust
use colored::Colorize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::node::FileNode;
use crate::core::file_stats::weight::{self, WeightConfig};
use crate::core::path_matcher::SortStrategy;
use crate::theme::for_path_tree::{DIR_ICON, TreeStyle, get_file_type};

pub struct PathGrid {
    roots: Vec<FileNode>,
    style: TreeStyle,
}

impl PathGrid {
    #[must_use]
    pub fn build(
        paths_strings: &[String],
        base_dir: &str,
        sort_strategy: SortStrategy,
        weight_cfg: &WeightConfig,
        root_name: Option<&str>,
        no_emoji: bool,
    ) -> Self {
        // Dokładnie taka sama logika budowania struktury węzłów jak w PathTree::build
        let base_path_obj = Path::new(base_dir);
        let paths: Vec<PathBuf> = paths_strings.iter().map(PathBuf::from).collect();
        let mut tree_map: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();

        for p in &paths {
            let parent = p
                .parent()
                .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
            tree_map.entry(parent).or_default().push(p.clone());
        }

        fn build_node(
            path: &PathBuf,
            paths_map: &BTreeMap<PathBuf, Vec<PathBuf>>,
            base_path: &Path,
            sort_strategy: SortStrategy,
            weight_cfg: &WeightConfig,
            no_emoji: bool,
        ) -> FileNode {
            let name = path
                .file_name()
                .map_or_else(|| "/".to_string(), |n| n.to_string_lossy().to_string());
            let is_dir = path.is_dir() || path.to_string_lossy().ends_with('/');
            let icon = if no_emoji {
                String::new()
            } else if is_dir {
                DIR_ICON.to_string()
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                get_file_type(ext).icon.to_string()
            } else {
                "📄".to_string()
            };

            let absolute_path = base_path.join(path);
            let mut weight_bytes =
                weight::get_path_weight(&absolute_path, weight_cfg.dir_sum_included);
            let mut children = vec![];

            if let Some(child_paths) = paths_map.get(path) {
                let mut child_nodes: Vec<FileNode> = child_paths
                    .iter()
                    .map(|c| {
                        build_node(c, paths_map, base_path, sort_strategy, weight_cfg, no_emoji)
                    })
                    .collect();

                FileNode::sort_slice(&mut child_nodes, sort_strategy);

                if is_dir && weight_cfg.dir_sum_included {
                    weight_bytes = child_nodes.iter().map(|n| n.weight_bytes).sum();
                }
                children = child_nodes;
            }

            let weight_str = weight::format_weight(weight_bytes, is_dir, weight_cfg);
            FileNode {
                name,
                path: path.clone(),
                is_dir,
                icon,
                weight_str,
                weight_bytes,
                children,
            }
        }

        let roots_paths: Vec<PathBuf> = paths
            .iter()
            .filter(|p| {
                let parent = p.parent();
                parent.is_none()
                    || parent.unwrap() == Path::new("")
                    || !paths.contains(&parent.unwrap().to_path_buf())
            })
            .cloned()
            .collect();

        let mut top_nodes: Vec<FileNode> = roots_paths
            .into_iter()
            .map(|r| {
                build_node(
                    &r,
                    &tree_map,
                    base_path_obj,
                    sort_strategy,
                    weight_cfg,
                    no_emoji,
                )
            })
            .collect();

        FileNode::sort_slice(&mut top_nodes, sort_strategy);

        // [ENG]: Logic for creating the final root node with proper weight calculation.
        // [POL]: Logika tworzenia końcowego węzła głównego z poprawnym obliczeniem wagi.
        let final_roots = if let Some(r_name) = root_name {
            // [ENG]: Calculate total weight for the root node.
            // [POL]: Obliczenie całkowitej wagi dla węzła głównego.
            let root_bytes = if weight_cfg.dir_sum_included {
                // [POL]: Suma wag bezpośrednich dzieci (dopasowanych elementów).
                top_nodes.iter().map(|n| n.weight_bytes).sum()
            } else {
                // [POL]: Fizyczna waga folderu wejściowego z dysku.
                weight::get_path_weight(base_path_obj, false)
            };

            let root_weight_str = weight::format_weight(root_bytes, true, weight_cfg);

            vec![FileNode {
                name: r_name.to_string(),
                path: PathBuf::from(r_name),
                is_dir: true,
                icon: if no_emoji {
                    String::new()
                } else {
                    DIR_ICON.to_string()
                },
                weight_str: root_weight_str,
                weight_bytes: root_bytes,
                children: top_nodes,
            }]
        } else {
            top_nodes
        };

        Self {
            roots: final_roots,
            style: TreeStyle::default(),
        }
    }

    #[must_use]
    pub fn render_cli(&self) -> String {
        let max_width = self.calc_max_width(&self.roots, 0);
        self.plot(&self.roots, "", true, max_width)
    }

    #[must_use]
    pub fn render_txt(&self) -> String {
        let max_width = self.calc_max_width(&self.roots, 0);
        self.plot(&self.roots, "", false, max_width)
    }

    fn calc_max_width(&self, nodes: &[FileNode], indent_len: usize) -> usize {
        let mut max = 0;
        for (i, node) in nodes.iter().enumerate() {
            let is_last = i == nodes.len() - 1;
            let has_children = !node.children.is_empty();
            let branch = if node.is_dir {
                match (is_last, has_children) {
                    (true, true) => &self.style.dir_last_with_children,
                    (false, true) => &self.style.dir_mid_with_children,
                    (true, false) => &self.style.dir_last_no_children,
                    (false, false) => &self.style.dir_mid_no_children,
                }
            } else if is_last {
                &self.style.file_last
            } else {
                &self.style.file_mid
            };

            let current_len = node.weight_str.chars().count()
                + indent_len
                + branch.chars().count()
                + 1
                + node.icon.chars().count()
                + 1
                + node.name.chars().count();
            if current_len > max {
                max = current_len;
            }

            if has_children {
                let next_indent = indent_len
                    + if is_last {
                        self.style.indent_last.chars().count()
                    } else {
                        self.style.indent_mid.chars().count()
                    };
                let child_max = self.calc_max_width(&node.children, next_indent);
                if child_max > max {
                    max = child_max;
                }
            }
        }
        max
    }

    fn plot(&self, nodes: &[FileNode], indent: &str, use_color: bool, max_width: usize) -> String {
        let mut result = String::new();
        for (i, node) in nodes.iter().enumerate() {
            let is_last = i == nodes.len() - 1;
            let has_children = !node.children.is_empty();
            let branch = if node.is_dir {
                match (is_last, has_children) {
                    (true, true) => &self.style.dir_last_with_children,
                    (false, true) => &self.style.dir_mid_with_children,
                    (true, false) => &self.style.dir_last_no_children,
                    (false, false) => &self.style.dir_mid_no_children,
                }
            } else if is_last {
                &self.style.file_last
            } else {
                &self.style.file_mid
            };

            let weight_prefix = if node.weight_str.is_empty() {
                String::new()
            } else if use_color {
                node.weight_str.truecolor(120, 120, 120).to_string()
            } else {
                node.weight_str.clone()
            };

            let raw_left_len = node.weight_str.chars().count()
                + indent.chars().count()
                + branch.chars().count()
                + 1
                + node.icon.chars().count()
                + 1
                + node.name.chars().count();
            let pad_len = max_width.saturating_sub(raw_left_len) + 4;
            let padding = " ".repeat(pad_len);

            let rel_path_str = node.path.to_string_lossy().replace('\\', "/");
            let display_path = if node.is_dir && !rel_path_str.ends_with('/') {
                format!("./{}/", rel_path_str)
            } else if !rel_path_str.starts_with("./") && !rel_path_str.starts_with('.') {
                format!("./{}", rel_path_str)
            } else {
                rel_path_str
            };

            let right_colored = if use_color {
                if node.is_dir {
                    display_path.truecolor(200, 200, 50).to_string()
                } else {
                    display_path.white().to_string()
                }
            } else {
                display_path
            };

            let left_colored = if use_color {
                if node.is_dir {
                    format!(
                        "{}{}{} {}{}",
                        weight_prefix,
                        indent.green(),
                        branch.green(),
                        node.icon,
                        node.name.truecolor(200, 200, 50)
                    )
                } else {
                    format!(
                        "{}{}{} {}{}",
                        weight_prefix,
                        indent.green(),
                        branch.green(),
                        node.icon,
                        node.name.white()
                    )
                }
            } else {
                format!(
                    "{}{}{} {} {}",
                    weight_prefix, indent, branch, node.icon, node.name
                )
            };

            result.push_str(&format!("{}{}{}\n", left_colored, padding, right_colored));

            if has_children {
                let new_indent = if is_last {
                    format!("{}{}", indent, self.style.indent_last)
                } else {
                    format!("{}{}", indent, self.style.indent_mid)
                };
                result.push_str(&self.plot(&node.children, &new_indent, use_color, max_width));
            }
        }
        result
    }
}

```

### 016: `./src/core/path_view/list.rs`

```rust
use super::node::FileNode;
use crate::core::file_stats::weight::{self, WeightConfig};
use crate::core::path_matcher::SortStrategy;
use crate::theme::for_path_list::get_icon_for_path;
use colored::Colorize;
/// [POL]: Zarządca wyświetlania wyników w formie płaskiej listy.
pub struct PathList {
    items: Vec<FileNode>,
}

impl PathList {
    /// [POL]: Buduje listę na podstawie zbioru ścieżek i statystyk.
    pub fn build(
        paths_strings: &[String],
        base_dir: &str,
        sort_strategy: SortStrategy,
        weight_cfg: &WeightConfig,
        no_emoji: bool,
    ) -> Self {
        // Wykorzystujemy istniejącą logikę węzłów, ale bez rekurencji (płaska lista)
        let mut items: Vec<FileNode> = paths_strings
            .iter()
            .map(|p_str| {
                let absolute = std::path::Path::new(base_dir).join(p_str);
                let is_dir = p_str.ends_with('/');
                let weight_bytes =
                    crate::core::file_stats::weight::get_path_weight(&absolute, true);
                let weight_str = weight::format_weight(weight_bytes, is_dir, weight_cfg);

                FileNode {
                    name: p_str.clone(),
                    path: absolute,
                    is_dir,
                    icon: if no_emoji {
                        String::new()
                    } else {
                        get_icon_for_path(p_str).to_string()
                    },
                    weight_str,
                    weight_bytes,
                    children: vec![], // Lista nie ma dzieci
                }
            })
            .collect();

        FileNode::sort_slice(&mut items, sort_strategy);

        Self { items }
    }

    /// [POL]: Renderuje listę dla terminala (z kolorami i ikonami).
    pub fn render_cli(&self, _is_match: bool) -> String {
        let mut out = String::new();
        // let tag = if is_match { "✅ MATCH: ".green() } else { "❌ REJECT:".red() };

        for item in &self.items {
            let line = format!(
                "{} {} {}\n",
                item.weight_str.truecolor(120, 120, 120),
                item.icon,
                if item.is_dir {
                    item.name.yellow()
                } else {
                    item.name.white()
                }
            );
            out.push_str(&line);
        }
        out
    }

    #[must_use]
    pub fn render_txt(&self) -> String {
        let mut out = String::new();
        for item in &self.items {
            // Brak formatowania ANSI
            let line = format!("{} {} {}\n", item.weight_str, item.icon, item.name);
            out.push_str(&line);
        }
        out
    }
}

```

### 017: `./src/core/path_view/node.rs`

```rust
use crate::core::path_matcher::SortStrategy;
use std::path::PathBuf;

/// [POL]: Reprezentuje pojedynczy węzeł w drzewie systemu plików.
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub icon: String,
    pub weight_str: String,
    pub weight_bytes: u64,
    pub children: Vec<FileNode>,
}

impl FileNode {
    /// [POL]: Sortuje listę węzłów w miejscu zgodnie z wybraną strategią.
    pub fn sort_slice(nodes: &mut [FileNode], strategy: SortStrategy) {
        if strategy == SortStrategy::None {
            return;
        }

        nodes.sort_by(|a, b| {
            let a_is_dir = a.is_dir;
            let b_is_dir = b.is_dir;

            // Klucz Merge: "interfaces.rs" -> "interfaces", "interfaces/" -> "interfaces"
            let a_merge = Self::get_merge_key(&a.name);
            let b_merge = Self::get_merge_key(&b.name);

            match strategy {
                // 1. CZYSTE ALFANUMERYCZNE
                SortStrategy::Az => a.name.cmp(&b.name),
                SortStrategy::Za => b.name.cmp(&a.name),

                // 2. PLIKI PIERWSZE (Globalnie)
                SortStrategy::AzFileFirst => (a_is_dir, &a.name).cmp(&(b_is_dir, &b.name)),
                SortStrategy::ZaFileFirst => (a_is_dir, &b.name).cmp(&(b_is_dir, &a.name)),

                // 3. KATALOGI PIERWSZE (Globalnie)
                SortStrategy::AzDirFirst => (!a_is_dir, &a.name).cmp(&(!b_is_dir, &b.name)),
                SortStrategy::ZaDirFirst => (!a_is_dir, &b.name).cmp(&(!b_is_dir, &a.name)),

                // 4. PLIKI PIERWSZE + MERGE (Grupowanie modułów)
                SortStrategy::AzFileFirstMerge => {
                    (a_merge, a_is_dir, &a.name).cmp(&(b_merge, b_is_dir, &b.name))
                }
                SortStrategy::ZaFileFirstMerge => {
                    (b_merge, a_is_dir, &b.name).cmp(&(a_merge, b_is_dir, &a.name))
                }

                // 5. KATALOGI PIERWSZE + MERGE (Zgodnie z Twoją notatką: fallback do DirFirst)
                SortStrategy::AzDirFirstMerge => (!a_is_dir, &a.name).cmp(&(!b_is_dir, &b.name)),
                SortStrategy::ZaDirFirstMerge => (!a_is_dir, &b.name).cmp(&(!b_is_dir, &a.name)),

                _ => a.name.cmp(&b.name),
            }
        });
    }

    /// [POL]: Wyciąga rdzeń nazwy do grupowania (np. "main.rs" -> "main").
    fn get_merge_key(name: &str) -> &str {
        if let Some(idx) = name.rfind('.')
            && idx > 0
        {
            return &name[..idx];
        }
        name
    }
}

```

### 018: `./src/core/path_view/tree.rs`

```rust
use colored::Colorize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

// Importy z rodzeństwa i innych modułów core
use super::node::FileNode;
use crate::core::file_stats::weight::{self, WeightConfig};
use crate::core::path_matcher::SortStrategy;
use crate::theme::for_path_tree::{DIR_ICON, FILE_ICON, TreeStyle, get_file_type};
pub struct PathTree {
    roots: Vec<FileNode>,
    style: TreeStyle,
}

impl PathTree {
    #[must_use]
    pub fn build(
        paths_strings: &[String],
        base_dir: &str,
        sort_strategy: SortStrategy,
        weight_cfg: &WeightConfig,
        root_name: Option<&str>,
        no_emoji: bool,
    ) -> Self {
        let base_path_obj = Path::new(base_dir);
        let paths: Vec<PathBuf> = paths_strings.iter().map(PathBuf::from).collect();
        let mut tree_map: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();

        for p in &paths {
            let parent = p
                .parent()
                .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
            tree_map.entry(parent).or_default().push(p.clone());
        }

        fn build_node(
            path: &PathBuf,
            paths_map: &BTreeMap<PathBuf, Vec<PathBuf>>,
            base_path: &Path,
            sort_strategy: SortStrategy,
            weight_cfg: &WeightConfig,
            no_emoji: bool,
        ) -> FileNode {
            let name = path
                .file_name()
                .map_or_else(|| "/".to_string(), |n| n.to_string_lossy().to_string());

            let is_dir = path.is_dir() || path.to_string_lossy().ends_with('/');
            let icon = if no_emoji {
                String::new()
            } else if is_dir {
                DIR_ICON.to_string()
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                get_file_type(ext).icon.to_string()
            } else {
                FILE_ICON.to_string()
            };

            let absolute_path = base_path.join(path);
            let mut weight_bytes =
                weight::get_path_weight(&absolute_path, weight_cfg.dir_sum_included);
            let mut children = vec![];

            if let Some(child_paths) = paths_map.get(path) {
                let mut child_nodes: Vec<FileNode> = child_paths
                    .iter()
                    .map(|c| {
                        build_node(c, paths_map, base_path, sort_strategy, weight_cfg, no_emoji)
                    })
                    .collect();

                FileNode::sort_slice(&mut child_nodes, sort_strategy);

                if is_dir && weight_cfg.dir_sum_included {
                    weight_bytes = child_nodes.iter().map(|n| n.weight_bytes).sum();
                }
                children = child_nodes;
            }

            let weight_str = weight::format_weight(weight_bytes, is_dir, weight_cfg);

            FileNode {
                name,
                path: path.clone(),
                is_dir,
                icon,
                weight_str,
                weight_bytes,
                children,
            }
        }

        let roots_paths: Vec<PathBuf> = paths
            .iter()
            .filter(|p| {
                let parent = p.parent();
                parent.is_none()
                    || parent.unwrap() == Path::new("")
                    || !paths.contains(&parent.unwrap().to_path_buf())
            })
            .cloned()
            .collect();

        let mut top_nodes: Vec<FileNode> = roots_paths
            .into_iter()
            .map(|r| {
                build_node(
                    &r,
                    &tree_map,
                    base_path_obj,
                    sort_strategy,
                    weight_cfg,
                    no_emoji,
                )
            })
            .collect();

        FileNode::sort_slice(&mut top_nodes, sort_strategy);

        // [ENG]: Logic for creating the final root node with proper weight calculation.
        // [POL]: Logika tworzenia końcowego węzła głównego z poprawnym obliczeniem wagi.
        let final_roots = if let Some(r_name) = root_name {
            // [ENG]: Calculate total weight for the root node.
            // [POL]: Obliczenie całkowitej wagi dla węzła głównego.
            let root_bytes = if weight_cfg.dir_sum_included {
                // [POL]: Suma wag bezpośrednich dzieci (dopasowanych elementów).
                top_nodes.iter().map(|n| n.weight_bytes).sum()
            } else {
                // [POL]: Fizyczna waga folderu wejściowego z dysku.
                weight::get_path_weight(base_path_obj, false)
            };

            let root_weight_str = weight::format_weight(root_bytes, true, weight_cfg);

            vec![FileNode {
                name: r_name.to_string(),
                path: PathBuf::from(r_name),
                is_dir: true,
                icon: if no_emoji {
                    String::new()
                } else {
                    DIR_ICON.to_string()
                },
                weight_str: root_weight_str,
                weight_bytes: root_bytes,
                children: top_nodes,
            }]
        } else {
            top_nodes
        };

        Self {
            roots: final_roots,
            style: TreeStyle::default(),
        }
    }

    #[must_use]
    pub fn render_cli(&self) -> String {
        self.plot(&self.roots, "", true)
    }

    #[must_use]
    pub fn render_txt(&self) -> String {
        self.plot(&self.roots, "", false)
    }

    fn plot(&self, nodes: &[FileNode], indent: &str, use_color: bool) -> String {
        let mut result = String::new();
        for (i, node) in nodes.iter().enumerate() {
            let is_last = i == nodes.len() - 1;
            let has_children = !node.children.is_empty();

            let branch = if node.is_dir {
                match (is_last, has_children) {
                    (true, true) => &self.style.dir_last_with_children,
                    (false, true) => &self.style.dir_mid_with_children,
                    (true, false) => &self.style.dir_last_no_children,
                    (false, false) => &self.style.dir_mid_no_children,
                }
            } else if is_last {
                &self.style.file_last
            } else {
                &self.style.file_mid
            };

            let weight_prefix = if node.weight_str.is_empty() {
                String::new()
            } else if use_color {
                node.weight_str.truecolor(120, 120, 120).to_string()
            } else {
                node.weight_str.clone()
            };

            let line = if use_color {
                if node.is_dir {
                    format!(
                        "{weight_prefix}{}{branch_color} {icon} {name}\n",
                        indent.green(),
                        branch_color = branch.green(),
                        icon = node.icon,
                        name = node.name.truecolor(200, 200, 50)
                    )
                } else {
                    format!(
                        "{weight_prefix}{}{branch_color} {icon} {name}\n",
                        indent.green(),
                        branch_color = branch.green(),
                        icon = node.icon,
                        name = node.name.white()
                    )
                }
            } else {
                format!(
                    "{weight_prefix}{indent}{branch} {icon} {name}\n",
                    icon = node.icon,
                    name = node.name
                )
            };

            result.push_str(&line);

            if has_children {
                let new_indent = if is_last {
                    format!("{indent}{}", self.style.indent_last)
                } else {
                    format!("{indent}{}", self.style.indent_mid)
                };
                result.push_str(&self.plot(&node.children, &new_indent, use_color));
            }
        }
        result
    }
}

```

### 019: `./src/core/patterns_expand.rs`

```rust
/// [POL]: Kontekst wzorców - przechowuje oryginalne wzorce użytkownika oraz ich rozwiniętą formę.
/// [ENG]: Pattern context - stores original user patterns and their tok form.
#[derive(Debug, Clone)]
pub struct PatternContext {
    pub raw: Vec<String>,
    pub tok: Vec<String>,
}

impl PatternContext {
    /// [POL]: Tworzy nowy kontekst, automatycznie rozwijając klamry w podanych wzorcach.
    /// [ENG]: Creates a new context, automatically expanding braces in the provided patterns.
    pub fn new<I, S>(patterns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut raw = Vec::new();
        let mut tok = Vec::new();

        for pat in patterns {
            let pat_str = pat.as_ref();
            raw.push(pat_str.to_string());
            tok.extend(Self::expand_braces(pat_str));
        }

        Self { raw, tok }
    }

    /// [POL]: Prywatna metoda: rozwija klamry we wzorcu (np. {a,b} -> [a, b]). Obsługuje rekurencję.
    /// [ENG]: Private method: expands braces in a pattern (e.g. {a,b} -> [a, b]). Supports recursion.
    fn expand_braces(pattern: &str) -> Vec<String> {
        if let (Some(start), Some(end)) = (pattern.find('{'), pattern.find('}'))
            && start < end
        {
            let prefix = &pattern[..start];
            let suffix = &pattern[end + 1..];
            let options = &pattern[start + 1..end];

            let mut tok = Vec::new();
            for opt in options.split(',') {
                let new_pattern = format!("{}{}{}", prefix, opt, suffix);
                tok.extend(Self::expand_braces(&new_pattern));
            }
            return tok;
        }
        vec![pattern.to_string()]
    }
}

```

### 020: `./src/core/save.rs`

```rust
use super::super::i18n::I18n;
use crate::theme::for_path_tree::get_file_type;
use std::fs;
use std::path::Path;

pub struct SaveFile;

impl SaveFile {
    // ⚡ Nowa funkcja tabelarycznej stopki
    pub fn generate_by_section(tag: &str, enter_path: &str, i18n: &I18n, cmd: &str) -> String {
        let mut f = String::new();
        f.push_str("\n\n---\n\n");
        f.push_str("> | Property | Value |\n");
        f.push_str("> | ---: | :--- |\n");
        f.push_str(&format!(
            "> | **{}** | `cargo-plot v0.2.0` |\n",
            i18n.footer_tool()
        ));
        f.push_str(&format!(
            "> | **{}** | `{}` |\n",
            i18n.footer_input(),
            enter_path
        ));
        f.push_str(&format!("> | **{}** | `{}` |\n", i18n.footer_cmd(), cmd));
        f.push_str(&format!("> | **{}** | `{}` |\n", i18n.footer_tag(), tag));

        let links = "[Crates.io](https://crates.io/crates/cargo-plot) \\| [GitHub](https://github.com/j-Cis/cargo-plot/releases)";
        f.push_str(&format!("> | **{}** | {} |\n", i18n.footer_links(), links));
        f.push_str(&format!(
            "> | **{}** | `cargo install cargo-plot` |\n",
            i18n.footer_links()
        ));
        f.push_str(&format!(
            "> | **{}** | `cargo plot --help` |\n",
            i18n.footer_help()
        ));
        f.push_str("\n---\n");
        f
    }

    /// Wspólna logika zapisu do pliku (DRY): tworzenie folderów i zapis IO.
    fn write_to_disk(filepath: &str, content: &str, log_name: &str, i18n: &I18n) {
        let path = Path::new(filepath);

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
            && !parent.exists()
            && let Err(e) = fs::create_dir_all(parent)
        {
            eprintln!(
                "{}",
                i18n.dir_create_err(&parent.to_string_lossy(), &e.to_string())
            );
            return;
        }

        match fs::write(path, content) {
            Ok(_) => println!("{}", i18n.save_success(log_name, filepath)),
            Err(e) => eprintln!("{}", i18n.save_err(log_name, filepath, &e.to_string())),
        }
    }

    /// Formatowanie i zapis samego widoku struktury (ścieżek)
    pub fn paths(
        content: &str,
        filepath: &str,
        tag: &str,
        add_by: bool,
        i18n: &I18n,
        cmd: &str,
        enter_path: &str,
    ) {
        let by_section = if add_by {
            Self::generate_by_section(tag, enter_path, i18n, cmd)
        } else {
            String::new()
        };
        let internal_tag = if add_by { "" } else { tag };
        let file_name = Path::new(filepath)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        let markdown_content = format!(
            "# {}\n\n```plaintext\n{}\n```\n\n{}{}",
            file_name, content, internal_tag, by_section
        );

        Self::write_to_disk(
            filepath,
            &markdown_content,
            if i18n.lang == crate::i18n::Lang::Pl {
                "ścieżki"
            } else {
                "paths"
            },
            i18n,
        );
    }

    /// Formatowanie i zapis pełnego cache (drzewo + zawartość plików)
    pub fn codes(
        tree_text: &str,
        paths: &[String],
        base_dir: &str,
        filepath: &str,
        tag: &str,
        add_by: bool,
        i18n: &I18n,
        cmd: &str,
        enter_path: &str,
    ) {
        let by_section = if add_by {
            Self::generate_by_section(tag, enter_path, i18n, cmd)
        } else {
            String::new()
        };
        let internal_tag = if add_by { "" } else { tag };
        let file_name = Path::new(filepath)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        let mut content = String::new();
        content.push_str(&format!("# {}\n\n", file_name));

        // Wstawiamy wygenerowane drzewo ścieżek
        content.push_str("```plaintext\n");
        content.push_str(tree_text);
        content.push_str("```\n\n");

        let mut counter = 1;

        for p_str in paths {
            if p_str.ends_with('/') {
                continue; // Pomijamy katalogi
            }

            let absolute_path = Path::new(base_dir).join(p_str);
            let ext = absolute_path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();

            let lang = get_file_type(&ext).md_lang;

            if is_blacklisted_extension(&ext) {
                content.push_str(&format!(
                    "### {:03}: `{}`\n\n{}\n\n",
                    counter,
                    p_str,
                    i18n.skip_binary()
                ));
                counter += 1;
                continue;
            }

            match fs::read_to_string(&absolute_path) {
                Ok(file_content) => {
                    content.push_str(&format!(
                        "### {:03}: `{}`\n\n```{}\n{}\n```\n\n",
                        counter, p_str, lang, file_content
                    ));
                }
                Err(_) => {
                    content.push_str(&format!(
                        "### {:03}: `{}`\n\n{}\n\n",
                        counter,
                        p_str,
                        i18n.read_err()
                    ));
                }
            }
            counter += 1;
        }

        content.push_str(&format!("\n\n{}{}", internal_tag, by_section));
        Self::write_to_disk(
            filepath,
            &content,
            if i18n.lang == crate::i18n::Lang::Pl {
                "kod (cache)"
            } else {
                "code (cache)"
            },
            i18n,
        );
    }
}

// [EN]: Security mechanisms to prevent processing non-text or binary files.
// [PL]: Mechanizmy bezpieczeństwa zapobiegające przetwarzaniu plików nietekstowych lub binarnych.

/// [EN]: Checks if a file extension is on the list of forbidden binary types.
/// [PL]: Sprawdza, czy rozszerzenie pliku znajduje się na liście zabronionych typów binarnych.
pub fn is_blacklisted_extension(ext: &str) -> bool {
    let e = ext.to_lowercase();

    matches!(
        e.as_str(),
        // --------------------------------------------------
        // GRAFIKA I DESIGN
        // --------------------------------------------------
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "svg" | "webp" | "tiff" | "tif" | "heic" | "psd" | 
        "ai" | 
        // --------------------------------------------------
        // BINARKI | BIBLIOTEKI I ARTEFAKTY KOMPILACJI
        // --------------------------------------------------
        "exe" | "dll" | "so" | "dylib" | "bin" | "wasm" | "pdb" | "rlib" | "rmeta" | "lib" | 
        "o" | "a" | "obj" | "pch" | "ilk" | "exp" | 
        "jar" | "class" | "war" | "ear" | 
        "pyc" | "pyd" | "pyo" | "whl" | 
        // --------------------------------------------------
        // ARCHIWA I PACZKI
        // --------------------------------------------------
        "zip" | "tar" | "gz" | "tgz" | "7z" | "rar" | "bz2" | "xz" | "iso" | "dmg" | "pkg" | "apk" | 
        // --------------------------------------------------
        // DOKUMENTY | BAZY DANYCH I FONTY
        // --------------------------------------------------
        "sqlite" | "sqlite3" | "db" | "db3" | "mdf" | "ldf" | "rdb" | 
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "odt" | "ods" | "odp" | 
        "woff" | "woff2" | "ttf" | "eot" | "otf" | 
        // --------------------------------------------------
        // MEDIA (AUDIO / WIDEO)
        // --------------------------------------------------
        "mp3" | "mp4" | "avi" | "mkv" | "wav" | "flac" | "ogg" | "m4a" | "mov" | "wmv" | "flv"
    )
}

```

### 021: `./src/execute.rs`

```rust
use crate::core::file_stats::FileStats;
use crate::core::file_stats::weight::WeightConfig;
pub use crate::core::path_matcher::SortStrategy;
use crate::core::path_matcher::{MatchStats, PathMatchers, ShowMode};
use crate::core::path_store::{PathContext, PathStore};
use crate::core::path_view::{PathGrid, PathList, PathTree, ViewMode};
use crate::core::patterns_expand::PatternContext;
use std::path::Path;

/// [ENG]: Primary execution function that coordinates scanning, matching, and view building.
/// [POL]: Główna funkcja wykonawcza koordynująca skanowanie, dopasowywanie i budowanie widoków.
pub fn execute<OnMatch, OnMismatch>(
    enter_path: &str,
    patterns: &[String],
    is_case_sensitive: bool,
    sort_strategy: SortStrategy,
    show_mode: ShowMode,
    view_mode: ViewMode,
    weight_cfg: WeightConfig, // ⚡ Używamy konfiguracji przekazanej z CLI/GUI
    no_root: bool,
    print_info: bool,
    no_emoji: bool,
    i18n: &crate::i18n::I18n,
    mut on_match: OnMatch,
    mut on_mismatch: OnMismatch,
) -> MatchStats
where
    OnMatch: FnMut(&FileStats),
    OnMismatch: FnMut(&FileStats),
{
    // [ENG]: 1. Initialize contexts.
    // [POL]: 1. Inicjalizacja kontekstów.
    let pattern_ctx = PatternContext::new(patterns);
    let path_ctx = PathContext::resolve(enter_path).unwrap_or_else(|e| {
        eprintln!("❌ {}", e);
        std::process::exit(1);
    });

    // [ENG]: 2. Initial state logging (Restored full verbosity).
    // [POL]: 2. Logowanie stanu początkowego (Przywrócono pełną szczegółowość).
    if print_info {
        println!("{}", i18n.cli_base_abs(&path_ctx.base_absolute));
        println!("{}", i18n.cli_target_abs(&path_ctx.entry_absolute));
        println!("{}", i18n.cli_target_rel(&path_ctx.entry_relative));
        println!("---------------------------------------");
        println!("{}", i18n.cli_case_sensitive(is_case_sensitive));
        println!(
            "{}",
            i18n.cli_patterns_raw(&format!("{:?}", pattern_ctx.raw))
        );
        println!(
            "{}",
            i18n.cli_patterns_tok(&format!("{:?}", pattern_ctx.tok))
        );
        println!("---------------------------------------");
    } else {
        println!("---------------------------------------");
    }

    // [ENG]: 3. Build matchers.
    // [POL]: 3. Budowa silników dopasowujących.
    let matchers =
        PathMatchers::new(&pattern_ctx.tok, is_case_sensitive).expect("Błąd kompilacji wzorców");

    // [ENG]: 4. Scan disk.
    // [POL]: 4. Skanowanie dysku.
    let paths_store = PathStore::scan(&path_ctx.entry_absolute);
    let paths_set = paths_store.get_index();
    let entry_abs = path_ctx.entry_absolute.clone();

    // [ENG]: 6. Evaluate paths and fetch stats via callbacks.
    // [POL]: 6. Ewaluacja ścieżek i pobieranie statystyk przez callbacki.
    let mut stats = matchers.evaluate(
        &paths_store.list,
        &paths_set,
        sort_strategy,
        show_mode,
        |raw_path| {
            let stats = FileStats::fetch(raw_path, &entry_abs);
            on_match(&stats);
        },
        |raw_path| {
            let stats = FileStats::fetch(raw_path, &entry_abs);
            on_mismatch(&stats);
        },
    );

    // [ENG]: 7. Build views using the provided weight configuration.
    // [POL]: 7. Budowa widoków przy użyciu dostarczonej konfiguracji wagi.
    let root_name = if no_root {
        None
    } else {
        Path::new(&path_ctx.entry_absolute)
            .file_name()
            .and_then(|n| n.to_str())
    };

    let do_include = show_mode == ShowMode::Include || show_mode == ShowMode::Context;
    let do_exclude = show_mode == ShowMode::Exclude || show_mode == ShowMode::Context;

    match view_mode {
        ViewMode::Grid => {
            if do_include {
                stats.m_matched.grid = Some(PathGrid::build(
                    &stats.m_matched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    root_name,
                    no_emoji,
                ));
            }
            if do_exclude {
                stats.x_mismatched.grid = Some(PathGrid::build(
                    &stats.x_mismatched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    root_name,
                    no_emoji,
                ));
            }
        }
        ViewMode::Tree => {
            if do_include {
                stats.m_matched.tree = Some(PathTree::build(
                    &stats.m_matched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    root_name,
                    no_emoji,
                ));
            }
            if do_exclude {
                stats.x_mismatched.tree = Some(PathTree::build(
                    &stats.x_mismatched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    root_name,
                    no_emoji,
                ));
            }
        }
        ViewMode::List => {
            if do_include {
                stats.m_matched.list = Some(PathList::build(
                    &stats.m_matched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    no_emoji,
                ));
            }
            if do_exclude {
                stats.x_mismatched.list = Some(PathList::build(
                    &stats.x_mismatched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    no_emoji,
                ));
            }
        }
    }

    stats
}

```

### 022: `./src/i18n.rs`

```rust
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Lang {
    Pl,
    En,
}

impl Lang {
    pub fn detect() -> Self {
        if env::var("LANG")
            .unwrap_or_default()
            .to_lowercase()
            .starts_with("pl")
        {
            Self::Pl
        } else {
            Self::En
        }
    }
}

pub struct I18n {
    pub lang: Lang,
}

impl I18n {
    pub fn new(lang: Option<Lang>) -> Self {
        Self {
            lang: lang.unwrap_or_else(Lang::detect),
        }
    }

    // =====================================================================
    // 1. TOP - TEKST OGÓLNY
    // =====================================================================
    pub fn save_success(&self, name: &str, path: &str) -> String {
        match self.lang {
            Lang::Pl => format!("💾 Pomyślnie zapisano {} do pliku: {}", name, path),
            Lang::En => format!("💾 Successfully saved {} to file: {}", name, path),
        }
    }
    pub fn save_err(&self, name: &str, path: &str, err: &str) -> String {
        match self.lang {
            Lang::Pl => format!("❌ Błąd zapisu {} do pliku {}: {}", name, path, err),
            Lang::En => format!("❌ Error saving {} to file {}: {}", name, path, err),
        }
    }
    pub fn dir_create_err(&self, dir: &str, err: &str) -> String {
        match self.lang {
            Lang::Pl => format!("❌ Błąd: Nie można utworzyć katalogu {} ({})", dir, err),
            Lang::En => format!("❌ Error: Cannot create directory {} ({})", dir, err),
        }
    }

    // =====================================================================
    // 2. LIB / CORE - LOGIKA BAZOWA
    // =====================================================================
    pub fn skip_binary(&self) -> &'static str {
        match self.lang {
            Lang::Pl => "> *(Plik binarny/graficzny - pominięto zawartość)*",
            Lang::En => "> *(Binary/graphic file - content skipped)*",
        }
    }
    pub fn read_err(&self) -> &'static str {
        match self.lang {
            Lang::Pl => "> *(Błąd odczytu / plik nie jest UTF-8)*",
            Lang::En => "> *(Read error / file is not UTF-8)*",
        }
    }

    pub fn footer_tool(&self) -> &str {
        match self.lang {
            Lang::Pl => "Narzędzie",
            _ => "Tool",
        }
    }
    pub fn footer_input(&self) -> &str {
        match self.lang {
            Lang::Pl => "Folder",
            _ => "Input",
        }
    }
    pub fn footer_cmd(&self) -> &str {
        match self.lang {
            Lang::Pl => "Komenda",
            _ => "Command",
        }
    }
    pub fn footer_tag(&self) -> &str {
        match self.lang {
            Lang::Pl => "Tag",
            _ => "TimeTag",
        }
    }
    pub fn footer_links(&self) -> &str {
        match self.lang {
            Lang::Pl => "Linki",
            _ => "Links",
        }
    }
    pub fn footer_help(&self) -> &str {
        match self.lang {
            Lang::Pl => "Pomoc",
            _ => "Help",
        }
    }

    // =====================================================================
    // 3. CLI - INTERFEJS TERMINALOWY
    // =====================================================================
    pub fn cli_base_abs(&self, path: &str) -> String {
        match self.lang {
            Lang::Pl => format!("📂 Baza terminala (Absolutna): {}", path),
            Lang::En => format!("📂 Terminal base (Absolute): {}", path),
        }
    }
    pub fn cli_target_abs(&self, path: &str) -> String {
        match self.lang {
            Lang::Pl => format!("📂 Cel skanowania (Absolutna): {}", path),
            Lang::En => format!("📂 Scan target (Absolute): {}", path),
        }
    }
    pub fn cli_target_rel(&self, path: &str) -> String {
        match self.lang {
            Lang::Pl => format!("📂 Cel skanowania (Relatywna): {}", path),
            Lang::En => format!("📂 Scan target (Relative): {}", path),
        }
    }
    pub fn cli_case_sensitive(&self, val: bool) -> String {
        match self.lang {
            Lang::Pl => format!("🔠 Wrażliwość na litery: {}", val),
            Lang::En => format!("🔠 Case sensitive: {}", val),
        }
    }
    pub fn cli_patterns_raw(&self, pat: &str) -> String {
        match self.lang {
            Lang::Pl => format!("🔍 Wzorce (RAW): {}", pat),
            Lang::En => format!("🔍 Patterns (RAW): {}", pat),
        }
    }
    pub fn cli_patterns_tok(&self, pat: &str) -> String {
        match self.lang {
            Lang::Pl => format!("⚙️ Wzorce (TOK): {}", pat),
            Lang::En => format!("⚙️ Patterns (TOK): {}", pat),
        }
    }
    pub fn cli_summary_matched(&self, count: usize, total: usize) -> String {
        match self.lang {
            Lang::Pl => format!("📊 Podsumowanie: Dopasowano {} z {} ścieżek.", count, total),
            Lang::En => format!("📊 Summary: Matched {} of {} paths.", count, total),
        }
    }
    pub fn cli_summary_rejected(&self, count: usize, total: usize) -> String {
        match self.lang {
            Lang::Pl => format!("📊 Podsumowanie: Odrzucono {} z {} ścieżek.", count, total),
            Lang::En => format!("📊 Summary: Rejected {} of {} paths.", count, total),
        }
    }
}

```

### 023: `./src/interfaces.rs`

```rust
// [ENG]: User interaction layer (Ports and Adapters).
// [POL]: Warstwa interakcji z użytkownikiem (Porty i Adaptery).

pub mod cli;
pub mod gui;
pub mod tui;

```

### 024: `./src/interfaces/cli.rs`

```rust
pub mod args;
pub mod engine;

use self::args::CargoCli;
use clap::Parser;

// [ENG]: Main entry point for the CLI interface and global router.
// [POL]: Główny punkt wejścia dla interfejsu CLI i globalny router.
pub fn run_cli() {
    let args_os = std::env::args();
    let mut args: Vec<String> = args_os.collect();

    // ⚡ NOWOŚĆ: Jeśli wywołano bez żadnych argumentów (samo `cargo plot`),
    // wstrzykujemy domyślnie flagę `-g` (GUI).
    let is_empty = args.len() == 1 || (args.len() == 2 && args[1] == "plot");
    if is_empty {
        args.push("-g".to_string());
    }

    // [ENG]: Injection trick: If run via 'cargo run -- -d...', 'plot' is missing.
    // [POL]: Trik z wstrzyknięciem: Jeśli uruchomiono przez 'cargo run -- -d...', brakuje 'plot'.
    if args.len() > 1 && args[1] != "plot" {
        args.insert(1, "plot".to_string());
    }

    // [ENG]: Parse from the modified list.
    // [POL]: Parsowanie ze zmodyfikowanej listy.
    let CargoCli::Plot(flags) = CargoCli::parse_from(args);

    // [ENG]: Transfer control based on parsed flags.
    // [POL]: Przekazanie kontroli na podstawie sparsowanych flag.
    if flags.gui {
        crate::interfaces::gui::run_gui(flags);
    } else if flags.tui {
        crate::interfaces::tui::run_tui();
    } else {
        engine::run(flags);
    }
}

```

### 025: `./src/interfaces/cli/args.rs`

```rust
use cargo_plot::core::path_matcher::SortStrategy;
use cargo_plot::core::path_view::ViewMode;
use cargo_plot::i18n::Lang;
use clap::{Args, Parser, ValueEnum};

/// [ENG]: Main wrapper for the Cargo plugin.
/// [POL]: Główny wrapper dla wtyczki Cargo.
#[derive(Parser, Debug)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum CargoCli {
    /// [ENG]: Cargo plot subcommand.
    /// [POL]: Podkomenda cargo plot.
    Plot(CliArgs),
}

/// [ENG]: Command line arguments for cargo-plot.
/// [POL]: Argumenty wiersza poleceń dla cargo-plot.
#[derive(Args, Debug, Clone)]
#[command(author, version, about = "Skaner struktury plików / File structure scanner", long_about = None)]
pub struct CliArgs {
    /// [ENG]: 📂 Input path to scan.
    /// [POL]: 📂 Ścieżka wejściowa do skanowania.
    #[arg(short = 'd', long = "dir", default_value = ".")]
    pub enter_path: String,

    /// [ENG]: 💾 Output directory path for saved results.
    /// [POL]: 💾 Ścieżka do katalogu wyjściowego na rezultaty.
    #[arg(short = 'o', long = "dir-out", num_args = 0..=1, default_missing_value = "AUTO")]
    pub dir_out: Option<String>,

    /// [ENG]: 🔍 Match patterns.
    /// [POL]: 🔍 Wzorce dopasowań.
    #[arg(short = 'p', long = "pat", required_unless_present_any = ["gui", "tui"])]
    pub patterns: Vec<String>,

    /// [ENG]: ✔️ Treat patterns as match (include) rules.
    /// [POL]: ✔️ Traktuj wzorce jako zasady dopasowania (włącz).
    #[arg(short = 'm', long = "pat-match", required_unless_present_any = ["exclude", "gui", "tui"])]
    pub include: bool,

    /// [ENG]: ❌ Treat patterns as mismatch (exclude) rules.
    /// [POL]: ❌ Traktuj wzorce jako zasady odrzucenia (wyklucz).
    #[arg(short = 'x', long = "pat-mismatch", required_unless_present_any = ["include", "gui", "tui"])]
    pub exclude: bool,

    /// [ENG]: 🔠 Ignore case sensitivity in patterns.
    /// [POL]: 🔠 Ignoruj wielkość liter we wzorcach.
    #[arg(short = 'c', long = "pat-ignore-case")]
    pub ignore_case: bool,

    /// [ENG]: 🗂️ Results sorting strategy.
    /// [POL]: 🗂️ Strategia sortowania wyników.
    #[arg(short = 's', long = "sort", value_enum, default_value_t = CliSortStrategy::AzFileMerge)]
    pub sort: CliSortStrategy,

    /// [ENG]: 👁️ Selects the display format (tree, list, grid).
    /// [POL]: 👁️ Wybiera format wyświetlania wyników (drzewo, lista, siatka).
    #[arg(short = 'v', long = "view", value_enum, default_value_t = CliViewMode::Tree)]
    pub view: CliViewMode,

    /// [ENG]: 📝 Save the paths structure to a file.
    /// [POL]: 📝 Zapisuje strukturę ścieżek do pliku.
    #[arg(long = "save-address")]
    pub save_address: bool,

    /// [ENG]: 📦 Save the file contents archive to a file.
    /// [POL]: 📦 Zapisuje archiwum z zawartością plików.
    #[arg(long = "save-archive")]
    pub save_archive: bool,

    /// [ENG]: 🏷️ Add a footer with command information to saved files.
    /// [POL]: 🏷️ Dodaje stopkę z informacją o komendzie do zapisanych plików.
    #[arg(short = 'b', long = "by")]
    pub by: bool,

    /// [ENG]: 🌳 Hide the root directory in the tree view.
    /// [POL]: 🌳 Ukrywa główny folder (korzeń) w widoku drzewa.
    #[arg(long = "treeview-no-root")]
    pub no_root: bool,

    /// [ENG]: ℹ️ Display summary statistics and headers.
    /// [POL]: ℹ️ Wyświetla statystyki podsumowujące i nagłówki.
    #[arg(short = 'i', long = "info")]
    pub info: bool,

    /// [ENG]: 🚫 Disable emoji rendering in the output.
    /// [POL]: 🚫 Wyłącza renderowanie ikon/emoji w wynikach.
    #[arg(long = "no-emoji")]
    pub no_emoji: bool,

    /// [ENG]: 🖥️ Launch the application in Graphical User Interface (GUI) mode.
    /// [POL]: 🖥️ Uruchamia aplikację w trybie graficznym (GUI).
    #[arg(short = 'g', long = "gui")]
    pub gui: bool,

    /// [ENG]: ⌨️ Launch the application in Terminal User Interface (TUI) mode.
    /// [POL]: ⌨️ Uruchamia aplikację w interaktywnym trybie terminalowym (TUI).
    #[arg(short = 't', long = "tui")]
    pub tui: bool,

    /// [ENG]: 🌍 Force a specific interface language.
    /// [POL]: 🌍 Wymusza określony język interfejsu.
    #[arg(long, value_enum)]
    pub lang: Option<Lang>,

    /// [ENG]: ⚖️ Weight unit system (dec for SI, bin for IEC).
    /// [POL]: ⚖️ System jednostek wagi (dec dla SI, bin dla IEC).
    #[arg(short = 'u', long = "unit", value_enum, default_value_t = CliUnitSystem::Bin)]
    pub unit: CliUnitSystem,

    /// [ENG]: 🧮 Calculate actual folder weight including unmatched files.
    /// [POL]: 🧮 Oblicza rzeczywistą wagę folderu wliczając wszystkie pliki.
    #[arg(short = 'a', long = "all")]
    pub all: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum CliViewMode {
    Tree,
    List,
    Grid,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum CliSortStrategy {
    None,
    Az,
    Za,
    AzFile,
    ZaFile,
    AzDir,
    ZaDir,
    AzFileMerge,
    ZaFileMerge,
    AzDirMerge,
    ZaDirMerge,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum CliUnitSystem {
    Dec,
    Bin,
}

impl From<CliSortStrategy> for SortStrategy {
    fn from(val: CliSortStrategy) -> Self {
        match val {
            CliSortStrategy::None => SortStrategy::None,
            CliSortStrategy::Az => SortStrategy::Az,
            CliSortStrategy::Za => SortStrategy::Za,
            CliSortStrategy::AzFile => SortStrategy::AzFileFirst,
            CliSortStrategy::ZaFile => SortStrategy::ZaFileFirst,
            CliSortStrategy::AzDir => SortStrategy::AzDirFirst,
            CliSortStrategy::ZaDir => SortStrategy::ZaDirFirst,
            CliSortStrategy::AzFileMerge => SortStrategy::AzFileFirstMerge,
            CliSortStrategy::ZaFileMerge => SortStrategy::ZaFileFirstMerge,
            CliSortStrategy::AzDirMerge => SortStrategy::AzDirFirstMerge,
            CliSortStrategy::ZaDirMerge => SortStrategy::ZaDirFirstMerge,
        }
    }
}

impl From<CliViewMode> for ViewMode {
    fn from(val: CliViewMode) -> Self {
        match val {
            CliViewMode::Tree => ViewMode::Tree,
            CliViewMode::List => ViewMode::List,
            CliViewMode::Grid => ViewMode::Grid,
        }
    }
}

impl CliArgs {
    /// [ENG]: Reconstructs a clean terminal command string.
    /// [POL]: Odtwarza czystą komendę terminalową.
    pub fn to_command_string(
        &self,
        is_m: bool,
        is_x: bool,
        is_address: bool,
        is_archive: bool,
    ) -> String {
        let mut cmd = vec!["cargo".to_string(), "plot".to_string()];

        if self.enter_path != "." && !self.enter_path.is_empty() {
            cmd.push("-d".to_string());
            cmd.push(format!("\"{}\"", self.enter_path));
        }

        if let Some(dir) = &self.dir_out {
            cmd.push("-o".to_string());
            if dir != "AUTO" {
                cmd.push(format!("\"{}\"", dir));
            } else {
                cmd.push("AUTO".to_string());
            }
        }

        // ⚡ POPRAWKA 1: Wzorce -p są teraz iterowane i dodawane osobno
        if !self.patterns.is_empty() {
            for pattern in &self.patterns {
                cmd.push("-p".to_string());
                cmd.push(format!("\"{}\"", pattern));
            }
        }

        // ⚡ GWARANCJA POPRAWNOŚCI: Komenda idealnie dopasowana do zapisywanego pliku
        if is_m {
            cmd.push("-m".to_string());
        }
        if is_x {
            cmd.push("-x".to_string());
        }

        if self.ignore_case {
            cmd.push("-c".to_string());
        }

        if self.sort != CliSortStrategy::AzFileMerge {
            let sort_str = match self.sort {
                CliSortStrategy::None => "none",
                CliSortStrategy::Az => "az",
                CliSortStrategy::Za => "za",
                CliSortStrategy::AzFile => "az-file",
                CliSortStrategy::ZaFile => "za-file",
                CliSortStrategy::AzDir => "az-dir",
                CliSortStrategy::ZaDir => "za-dir",
                CliSortStrategy::AzFileMerge => "az-file-merge",
                CliSortStrategy::ZaFileMerge => "za-file-merge",
                CliSortStrategy::AzDirMerge => "az-dir-merge",
                CliSortStrategy::ZaDirMerge => "za-dir-merge",
            };
            cmd.push("-s".to_string());
            cmd.push(sort_str.to_string());
        }

        if self.view != CliViewMode::Tree {
            let view_str = match self.view {
                CliViewMode::Tree => "tree",
                CliViewMode::List => "list",
                CliViewMode::Grid => "grid",
            };
            cmd.push("-v".to_string());
            cmd.push(view_str.to_string());
        }

        // ⚡ GWARANCJA POPRAWNOŚCI: Wymuszamy flagi zapisu zależnie od tego, z jakiego miejsca generujemy raport
        if self.save_address || is_address {
            cmd.push("--save-address".to_string());
        }
        if self.save_archive || is_archive {
            cmd.push("--save-archive".to_string());
        }
        if self.by {
            cmd.push("-b".to_string());
        }
        if self.no_root {
            cmd.push("--treeview-no-root".to_string());
        }
        if self.info {
            cmd.push("-i".to_string());
        }
        if self.no_emoji {
            cmd.push("--no-emoji".to_string());
        }
        if self.all {
            cmd.push("-a".to_string());
        }

        if self.unit != CliUnitSystem::Bin {
            cmd.push("-u".to_string());
            cmd.push("dec".to_string());
        }

        if let Some(l) = &self.lang {
            cmd.push("--lang".to_string());
            match l {
                Lang::Pl => cmd.push("pl".to_string()),
                Lang::En => cmd.push("en".to_string()),
            }
        }

        cmd.join(" ")
    }
}

```

### 026: `./src/interfaces/cli/engine.rs`

```rust
use crate::interfaces::cli::args::{CliArgs, CliUnitSystem};
use cargo_plot::addon::TimeTag;
use cargo_plot::core::file_stats::weight::{UnitSystem, WeightConfig};
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::core::path_store::PathContext;
use cargo_plot::core::path_view::ViewMode;
use cargo_plot::core::save::SaveFile;
use cargo_plot::execute::{self, SortStrategy};
use cargo_plot::i18n::I18n;

/// [ENG]: ⚙️ Main execution engine coordinating the scanning and rendering process.
/// [POL]: ⚙️ Główny silnik wykonawczy koordynujący proces skanowania i renderowania.
pub fn run(args: CliArgs) {
    // [ENG]: 📝 Initialize i18n and resolve basic flags.
    // [POL]: 📝 Inicjalizacja i18n i rozwiązanie podstawowych flag.
    let i18n = I18n::new(args.lang);
    let is_case_sensitive = !args.ignore_case;
    let sort_strategy: SortStrategy = args.sort.into();
    let view_mode: ViewMode = args.view.into();

    // [ENG]: ⚖️ Define weight calculation rules based on unit and 'all' flags.
    // [POL]: ⚖️ Definicja reguł obliczania wagi na podstawie flag jednostki oraz 'all'.
    let weight_cfg = WeightConfig {
        system: match args.unit {
            CliUnitSystem::Bin => UnitSystem::Binary,
            CliUnitSystem::Dec => UnitSystem::Decimal,
        },
        // [POL]: Jeśli 'all' (-a) jest true, liczymy fizyczną wagę z dysku dla folderów.
        dir_sum_included: !args.all,
        ..WeightConfig::default()
    };

    // [ENG]: 🎚️ Determines the display mode based on include (-m) and exclude (-x) flags.
    // [POL]: 🎚️ Ustala tryb wyświetlania na podstawie flag włączania (-m) i wykluczania (-x).
    let show_mode = match (args.include, args.exclude) {
        (true, false) => ShowMode::Include,
        (false, true) => ShowMode::Exclude,
        _ => ShowMode::Context,
    };

    // [ENG]: 🚀 Executes the core matching logic with prepared weight configuration.
    // [POL]: 🚀 Wykonuje główną logikę dopasowywania z przygotowaną konfiguracją wagi.
    let stats = execute::execute(
        &args.enter_path,
        &args.patterns,
        is_case_sensitive,
        sort_strategy,
        show_mode,
        view_mode,
        weight_cfg, // ⚡ WSTRZYKNIĘTE
        args.no_root,
        args.info,
        args.no_emoji,
        &i18n,
        |_| {},
        |_| {},
    );

    // [ENG]: 🖥️ Renders the output to the terminal with ANSI colors.
    // [POL]: 🖥️ Renderuje wynik do terminala z użyciem kolorów ANSI.
    let output_str_cli = stats.render_output(view_mode, show_mode, args.info, true);
    print!("{}", output_str_cli);

    // [ENG]: 💾 Handles file saving if address or archive flags are active.
    // [POL]: 💾 Obsługuje zapis do plików, jeśli aktywne są flagi adresu lub archiwum.
    if args.save_address || args.save_archive {
        let tag = TimeTag::now();

        // [ENG]: 📄 Renders plain text for Markdown output.
        // [POL]: 📄 Renderuje czysty tekst dla wyjścia w formacie Markdown.
        let output_str_txt_m = stats.render_output(view_mode, ShowMode::Include, args.info, false);
        let output_str_txt_x = stats.render_output(view_mode, ShowMode::Exclude, args.info, false);

        // [ENG]: 📂 Resolves the output directory path to .cargo-plot/ by default.
        // [POL]: 📂 Rozwiązuje ścieżkę katalogu wyjściowego (domyślnie na .cargo-plot/).
        let resolve_dir = |val: &Option<String>, base_path: &str| -> String {
            let is_auto = val
                .as_ref()
                .is_none_or(|v| v.trim().is_empty() || v == "AUTO");
            if is_auto {
                let mut b = base_path.replace('\\', "/");
                if !b.ends_with('/') {
                    b.push('/');
                }
                format!("{}.cargo-plot/", b)
            } else {
                let mut p = val.as_ref().unwrap().replace('\\', "/");
                if !p.ends_with('/') {
                    p.push('/');
                }
                p
            }
        };

        let output_dir = resolve_dir(&args.dir_out, &args.enter_path);

        // [ENG]: 📝 Saves the path structure (address).
        // [POL]: 📝 Zapisuje strukturę ścieżek (adres).
        if args.save_address {
            if args.include || !args.exclude {
                let filepath = format!("{}plot-address_{}_M.md", output_dir, tag);
                let cmd_m = args.to_command_string(true, false, true, false);
                SaveFile::paths(
                    &output_str_txt_m,
                    &filepath,
                    &tag,
                    args.by,
                    &i18n,
                    &cmd_m,
                    &args.enter_path,
                );
            }
            if args.exclude || !args.include {
                let filepath = format!("{}plot-address_{}_X.md", output_dir, tag);
                let cmd_x = args.to_command_string(false, true, true, false);
                SaveFile::paths(
                    &output_str_txt_x,
                    &filepath,
                    &tag,
                    args.by,
                    &i18n,
                    &cmd_x,
                    &args.enter_path,
                );
            }
        }

        // [ENG]: 📦 Saves the full file contents (archive).
        // [POL]: 📦 Zapisuje pełną zawartość plików (archiwum).
        if args.save_archive
            && let Ok(ctx) = PathContext::resolve(&args.enter_path)
        {
            if args.include || !args.exclude {
                let filepath = format!("{}plot-archive_{}_M.md", output_dir, tag);
                let cmd_m = args.to_command_string(true, false, false, true);
                SaveFile::codes(
                    &output_str_txt_m,
                    &stats.m_matched.paths,
                    &ctx.entry_absolute,
                    &filepath,
                    &tag,
                    args.by,
                    &i18n,
                    &cmd_m,
                    &args.enter_path,
                );
            }
            if args.exclude || !args.include {
                let filepath = format!("{}plot-archive_{}_X.md", output_dir, tag);
                let cmd_x = args.to_command_string(false, true, false, true);
                SaveFile::codes(
                    &output_str_txt_x,
                    &stats.x_mismatched.paths,
                    &ctx.entry_absolute,
                    &filepath,
                    &tag,
                    args.by,
                    &i18n,
                    &cmd_x,
                    &args.enter_path,
                );
            }
        }
    }

    // [ENG]: 📊 Prints summary statistics if info flag is active.
    // [POL]: 📊 Wyświetla statystyki podsumowujące, jeśli aktywna jest flaga info.
    if args.info {
        println!("---------------------------------------");
        println!(
            "{}",
            i18n.cli_summary_matched(stats.m_size_matched, stats.total)
        );
        println!(
            "{}",
            i18n.cli_summary_rejected(stats.x_size_mismatched, stats.total)
        );
    } else {
        println!("---------------------------------------");
    }
}

```

### 027: `./src/interfaces/gui.rs`

```rust
pub mod code;
pub mod i18n;
pub mod paths;
pub mod settings;
pub mod shared;

use crate::interfaces::cli::args::CliArgs;
use eframe::egui;

#[derive(PartialEq)]
pub enum Tab {
    Settings,
    Paths,
    Code,
}

#[derive(PartialEq)]
pub enum PathsTab {
    Match,
    Mismatch,
}

// ⚡ Dodana zakładka dla karty "Kod"
#[derive(PartialEq)]
pub enum CodeTab {
    Match,
    Mismatch,
}

#[derive(Default, Clone)]
pub struct TreeStats {
    pub txt_count: usize,
    pub txt_weight: u64,
    pub bin_count: usize,
    pub bin_weight: u64,
    pub err_count: usize,
    pub err_weight: u64,
    pub empty_count: usize,
    pub matched_count: usize,
    pub total_count: usize,
}

pub struct CargoPlotApp {
    pub args: CliArgs,
    pub active_tab: Tab,
    pub active_paths_tab: PathsTab,
    pub active_code_tab: CodeTab,
    pub new_pattern_input: String,
    pub out_path_input: String,
    pub generated_paths_m: String,
    pub generated_paths_x: String,
    pub generated_code_m: String,
    pub generated_code_x: String,
    pub stats_m: TreeStats,
    pub stats_x: TreeStats,
    pub ui_scale: f32,
}

impl CargoPlotApp {
    pub fn new(args: CliArgs) -> Self {
        let default_out = args.dir_out.clone().unwrap_or_default();
        Self {
            args,
            active_tab: Tab::Settings,
            active_paths_tab: PathsTab::Match,
            active_code_tab: CodeTab::Match, // ⚡ Domyślnie ładujemy zakładkę MATCH
            new_pattern_input: String::new(),
            out_path_input: default_out, // Inicjalizacja ścieżki
            generated_paths_m: String::new(),
            generated_paths_x: String::new(),
            generated_code_m: String::new(), // ⚡ Pusty na start
            generated_code_x: String::new(), // ⚡ Pusty na start
            stats_m: TreeStats::default(),
            stats_x: TreeStats::default(),
            ui_scale: 1.0,
        }
    }
}

impl eframe::App for CargoPlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(self.ui_scale);

        // GÓRNY PANEL (Teraz tylko 3 karty)
        egui::TopBottomPanel::top("top_tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, Tab::Settings, "Setting\nUstawienia");
                ui.selectable_value(&mut self.active_tab, Tab::Paths, "Paths\nŚcieżki");
                ui.selectable_value(&mut self.active_tab, Tab::Code, "Code\nKod");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);

                    if ui
                        .button("➕")
                        .on_hover_text("Powiększ (Zoom in)")
                        .clicked()
                    {
                        self.ui_scale += 0.1; // Powiększa o 10%
                    }

                    if ui
                        .button("🔄")
                        .on_hover_text("Resetuj skalę (100%)")
                        .clicked()
                    {
                        self.ui_scale = 1.0; // Wraca do standardu
                    }

                    if ui
                        .button("➖")
                        .on_hover_text("Pomniejsz (Zoom out)")
                        .clicked()
                        && self.ui_scale > 0.6
                    {
                        // Zabezpieczenie, żeby nie zmniejszyć za bardzo
                        self.ui_scale -= 0.1;
                    }

                    // Wyświetla aktualny procent powiększenia (np. "120%")
                    ui.label(
                        egui::RichText::new(format!("🔍 Skala: {:.0}%", self.ui_scale * 100.0))
                            .weak(),
                    );
                });
            });
        });

        // ŚRODEK OKNA
        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            Tab::Settings => settings::show(ui, self),
            Tab::Paths => paths::show(ui, self),
            Tab::Code => code::show(ui, self),
        });
    }
}

pub fn run_gui(args: CliArgs) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("cargo-plot"),
        ..Default::default()
    };
    eframe::run_native(
        "cargo-plot",
        options,
        Box::new(|_cc| Ok(Box::new(CargoPlotApp::new(args)))),
    )
    .unwrap();
}

```

### 028: `./src/interfaces/gui/code.rs`

```rust
use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use crate::interfaces::gui::shared::{draw_editor, draw_footer, draw_tabs, resolve_dir};
use crate::interfaces::gui::{CargoPlotApp, CodeTab, TreeStats};
use cargo_plot::addon::TimeTag;
use cargo_plot::core::file_stats::FileStats;
use cargo_plot::core::file_stats::weight::{UnitSystem, WeightConfig};
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::core::save::is_blacklisted_extension;
use cargo_plot::execute;
use eframe::egui;

/// [ENG]: View function for the Code tab, managing source extraction and statistics.
/// [POL]: Funkcja widoku dla karty Kod, zarządzająca ekstrakcją źródeł i statystykami.
pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    let gt = GuiI18n::new(app.args.lang);

    // [ENG]: 1. TOP TABS - Navigation between matched and mismatched code buffers.
    // [POL]: 1. GÓRNE ZAKŁADKI - Nawigacja między buforami kodu dopasowanego i odrzuconego.
    let mut is_match = app.active_code_tab == CodeTab::Match;
    draw_tabs(ui, &gt, &mut is_match);
    app.active_code_tab = if is_match {
        CodeTab::Match
    } else {
        CodeTab::Mismatch
    };

    ui.separator();

    // [ENG]: 2. ACTION BAR - Controls for code generation and archival save.
    // [POL]: 2. PASEK AKCJI - Sterowanie generowaniem kodu i zapisem archiwalnym.
    ui.horizontal(|ui| {
        if ui.button(gt.t(GT::BtnGenerateCode)).clicked() {
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            let show_mode = if is_match {
                ShowMode::Include
            } else {
                ShowMode::Exclude
            };

            // [ENG]: Weight configuration remains fixed for code extraction to ensure consistency.
            // [POL]: Konfiguracja wagi pozostaje stała dla ekstrakcji kodu, aby zapewnić spójność.
            let weight_cfg = WeightConfig {
                system: if app.args.unit == crate::interfaces::cli::args::CliUnitSystem::Bin {
                    UnitSystem::Binary
                } else {
                    UnitSystem::Decimal
                },
                dir_sum_included: !app.args.all,
                ..WeightConfig::default()
            };

            let mut st_m = TreeStats::default();
            let mut st_x = TreeStats::default();

            // [ENG]: Execute main engine with closures for statistics and file classification.
            // [POL]: Wykonanie głównego silnika z domknięciami dla statystyk i klasyfikacji plików.
            let stats = execute::execute(
                &app.args.enter_path,
                &app.args.patterns,
                !app.args.ignore_case,
                app.args.sort.into(),
                show_mode,
                app.args.view.into(),
                weight_cfg,
                app.args.no_root,
                false,
                app.args.no_emoji,
                &i18n,
                |f: &FileStats| {
                    if f.weight_bytes == 0 {
                        st_m.empty_count += 1;
                    }
                    if !f.path.ends_with('/') {
                        let ext = f
                            .absolute
                            .extension()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_lowercase();
                        if is_blacklisted_extension(&ext) {
                            st_m.bin_count += 1;
                            st_m.bin_weight += f.weight_bytes;
                        } else {
                            st_m.txt_count += 1;
                            st_m.txt_weight += f.weight_bytes;
                        }
                    }
                },
                |f: &FileStats| {
                    if f.weight_bytes == 0 {
                        st_x.empty_count += 1;
                    }
                    if !f.path.ends_with('/') {
                        let ext = f
                            .absolute
                            .extension()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_lowercase();
                        if is_blacklisted_extension(&ext) {
                            st_x.bin_count += 1;
                            st_x.bin_weight += f.weight_bytes;
                        } else {
                            st_x.txt_count += 1;
                            st_x.txt_weight += f.weight_bytes;
                        }
                    }
                },
            );

            st_m.matched_count = stats.m_size_matched;
            st_m.total_count = stats.total;
            st_x.matched_count = stats.x_size_mismatched;
            st_x.total_count = stats.total;

            app.stats_m = st_m;
            app.stats_x = st_x;

            let base_dir = std::path::Path::new(&app.args.enter_path);

            // [ENG]: Process code extraction for the selected result set.
            // [POL]: Przetwarzanie ekstrakcji kodu dla wybranego zestawu wyników.
            if is_match {
                let tree_m =
                    stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
                let mut content_m = format!("```plaintext\n{}\n```\n\n", tree_m);
                let mut counter_m = 1;
                for p_str in &stats.m_matched.paths {
                    if p_str.ends_with('/') {
                        continue;
                    }
                    let absolute_path = base_dir.join(p_str);
                    match std::fs::read_to_string(&absolute_path) {
                        Ok(txt) => content_m.push_str(&format!(
                            "### {:03}: `{}`\n\n```rust\n{}\n```\n\n",
                            counter_m, p_str, txt
                        )),
                        Err(_) => content_m.push_str(&format!(
                            "### {:03}: `{}`\n\n{}\n\n",
                            counter_m,
                            p_str,
                            gt.t(GT::LabelSkipBinary)
                        )),
                    }
                    counter_m += 1;
                }
                app.generated_code_m = content_m;
            } else {
                let tree_x =
                    stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
                let mut content_x = format!("```plaintext\n{}\n```\n\n", tree_x);
                let mut counter_x = 1;
                for p_str in &stats.x_mismatched.paths {
                    if p_str.ends_with('/') {
                        continue;
                    }
                    let absolute_path = base_dir.join(p_str);
                    match std::fs::read_to_string(&absolute_path) {
                        Ok(txt) => content_x.push_str(&format!(
                            "### {:03}: `{}`\n\n```rust\n{}\n```\n\n",
                            counter_x, p_str, txt
                        )),
                        Err(_) => content_x.push_str(&format!(
                            "### {:03}: `{}`\n\n{}\n\n",
                            counter_x,
                            p_str,
                            gt.t(GT::LabelSkipBinary)
                        )),
                    }
                    counter_x += 1;
                }
                app.generated_code_x = content_x;
            }
        }

        ui.add_space(15.0);
        ui.checkbox(&mut app.args.by, gt.t(GT::LabelAddFooter));
        ui.add_space(15.0);

        // [ENG]: Archival saving with metadata table.
        // [POL]: Zapis archiwalny z tabelą metadanych.
        if is_match {
            if ui.button(gt.t(GT::BtnSaveMatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!(
                    "{}plot-archive_{}_M.md",
                    resolve_dir(&app.args.dir_out, &app.args.enter_path),
                    tag
                );
                let mut final_text = app.generated_code_m.clone();
                if app.args.by {
                    let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                    let cmd_string = app.args.to_command_string(true, false, false, true);
                    final_text.push_str(&cargo_plot::core::save::SaveFile::generate_by_section(
                        &tag,
                        &app.args.enter_path,
                        &i18n,
                        &cmd_string,
                    ));
                }
                let _ = std::fs::write(&filepath, final_text);
            }
        } else {
            if ui.button(gt.t(GT::BtnSaveMismatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!(
                    "{}plot-archive_{}_X.md",
                    resolve_dir(&app.args.dir_out, &app.args.enter_path),
                    tag
                );
                let mut final_text = app.generated_code_x.clone();
                if app.args.by {
                    let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                    let cmd_string = app.args.to_command_string(false, true, false, true);
                    final_text.push_str(&cargo_plot::core::save::SaveFile::generate_by_section(
                        &tag,
                        &app.args.enter_path,
                        &i18n,
                        &cmd_string,
                    ));
                }
                let _ = std::fs::write(&filepath, final_text);
            }
        }
    });

    ui.separator();

    // [ENG]: 3. FOOTER - Update statistics pinned to the bottom.
    // [POL]: 3. STOPKA - Aktualizacja statystyk przypiętych do dołu.
    let current_stats = if is_match { &app.stats_m } else { &app.stats_x };
    draw_footer(ui, "code_stats_footer", current_stats);

    // [ENG]: 4. MAIN EDITOR - Display extracted file contents.
    // [POL]: 4. GŁÓWNY EDYTOR - Widok wyekstrahowanej zawartości plików.
    let text_buffer = match app.active_code_tab {
        CodeTab::Match => &mut app.generated_code_m,
        CodeTab::Mismatch => &mut app.generated_code_x,
    };
    draw_editor(ui, text_buffer);
}

```

### 029: `./src/interfaces/gui/i18n.rs`

```rust
// [ENG]: GUI Internationalization module.
// [POL]: Moduł internacjonalizacji interfejsu graficznego.

use cargo_plot::i18n::Lang;

pub struct GuiI18n {
    pub lang: Lang,
}

pub enum GuiText {
    LabelLang,
    LabelScanPath,
    LabelOutFolder,
    LabelSorting,
    LabelViewMode,
    LabelNoRoot,
    HeadingPatterns,
    LabelIgnoreCase,
    LabelNewPattern,
    BtnAddPattern,
    BtnClearAll,
    BtnBrowse,
    MsgNoPatterns,
    FooterDownload,
    FooterInstall,
    FooterUninstall,
    BtnGenerate,
    LabelAddFooter,
    BtnSaveMatch,
    BtnSaveMismatch,
    TabMatch,
    TabMismatch,
    BtnGenerateCode,
    LabelSkipBinary,
}

impl GuiI18n {
    pub fn new(lang: Option<Lang>) -> Self {
        Self {
            lang: lang.unwrap_or(Lang::En),
        }
    }

    pub fn t(&self, text: GuiText) -> &'static str {
        match self.lang {
            Lang::Pl => match text {
                GuiText::LabelLang => "🌍 Język:",
                GuiText::LabelScanPath => "📂 Ścieżka skanowania:",
                GuiText::LabelOutFolder => "💾 Folder zapisu (Output):",
                GuiText::LabelSorting => "Sortowanie",
                GuiText::LabelViewMode => "Tryb widoku",
                GuiText::LabelNoRoot => "Ukryj ROOT w drzewie",
                GuiText::HeadingPatterns => "🔍 Wzorce dopasowań (Patterns)",
                GuiText::LabelIgnoreCase => "🔠 Ignoruj wielkość liter",
                GuiText::LabelNewPattern => "Nowy:",
                GuiText::BtnAddPattern => "➕ Dodaj wzorzec",
                GuiText::BtnClearAll => "💣 Usuń wszystkie",
                GuiText::BtnBrowse => "Wybierz...",
                GuiText::MsgNoPatterns => "Brak wzorców. Dodaj przynajmniej jeden!",
                GuiText::FooterDownload => "Pobierz binarkę (GitHub)",
                GuiText::FooterInstall => "Instalacja:",
                GuiText::FooterUninstall => "Usuwanie:",
                GuiText::BtnGenerate => "🔄 Generuj / Regeneruj",
                GuiText::LabelAddFooter => "Dodaj stopkę (--by)",
                GuiText::BtnSaveMatch => "💾 Zapisz (-m)",
                GuiText::BtnSaveMismatch => "💾 Zapisz (-x)",
                GuiText::TabMatch => "✔ (-m) MATCH",
                GuiText::TabMismatch => "✖ (-x) MISMATCH",
                GuiText::BtnGenerateCode => "🔄 Generuj kod (Cache)",
                GuiText::LabelSkipBinary => "> *(Pominięto plik binarny/graficzny)*",
            },
            Lang::En => match text {
                GuiText::LabelLang => "🌍 Language:",
                GuiText::LabelScanPath => "📂 Scan path:",
                GuiText::LabelOutFolder => "💾 Output folder:",
                GuiText::LabelSorting => "Sorting",
                GuiText::LabelViewMode => "View mode",
                GuiText::LabelNoRoot => "Hide ROOT in tree",
                GuiText::HeadingPatterns => "🔍 Match Patterns",
                GuiText::LabelIgnoreCase => "🔠 Ignore case",
                GuiText::LabelNewPattern => "New:",
                GuiText::BtnAddPattern => "➕ Add pattern",
                GuiText::BtnClearAll => "💣 Clear all",
                GuiText::BtnBrowse => "Browse...",
                GuiText::MsgNoPatterns => "No patterns. Add at least one!",
                GuiText::FooterDownload => "Download binary (GitHub)",
                GuiText::FooterInstall => "Install:",
                GuiText::FooterUninstall => "Uninstall:",
                GuiText::BtnGenerate => "🔄 Generate / Regenerate",
                GuiText::LabelAddFooter => "Add footer (--by)",
                GuiText::BtnSaveMatch => "💾 Save (-m)",
                GuiText::BtnSaveMismatch => "💾 Save (-x)",
                GuiText::TabMatch => "✔ (-m) MATCH",
                GuiText::TabMismatch => "✖ (-x) MISMATCH",
                GuiText::BtnGenerateCode => "🔄 Generate code (Cache)",
                GuiText::LabelSkipBinary => "> *(Binary/graphic file skipped)*",
            },
        }
    }
}

```

### 030: `./src/interfaces/gui/paths.rs`

```rust
use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use crate::interfaces::gui::shared::{draw_editor, draw_footer, draw_tabs, resolve_dir};
use crate::interfaces::gui::{CargoPlotApp, PathsTab, TreeStats};
use cargo_plot::addon::TimeTag;
use cargo_plot::core::file_stats::FileStats;
use cargo_plot::core::file_stats::weight::{UnitSystem, WeightConfig};
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::core::save::is_blacklisted_extension;
use cargo_plot::execute;
use eframe::egui;

/// [ENG]: View function for the Paths tab, managing structure generation and unit toggling.
/// [POL]: Funkcja widoku dla karty Ścieżki, zarządzająca generowaniem struktury i przełączaniem jednostek.
pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    let gt = GuiI18n::new(app.args.lang);

    // [ENG]: 1. TOP TABS - Sub-navigation for Match/Mismatch results.
    // [POL]: 1. GÓRNE ZAKŁADKI - Podnawigacja dla wyników Match/Mismatch.
    let mut is_match = app.active_paths_tab == PathsTab::Match;
    draw_tabs(ui, &gt, &mut is_match);
    app.active_paths_tab = if is_match {
        PathsTab::Match
    } else {
        PathsTab::Mismatch
    };

    ui.separator();

    // [ENG]: 2. ACTION BAR - Controls for generation, unit systems, and file saving.
    // [POL]: 2. PASEK AKCJI - Sterowanie generowaniem, systemami jednostek i zapisem plików.
    ui.horizontal(|ui| {
        // [ENG]: Logic for triggering data generation.
        // [POL]: Logika wyzwalająca generowanie danych.
        if ui.button(gt.t(GT::BtnGenerate)).clicked() {
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            let show_mode = if is_match {
                ShowMode::Include
            } else {
                ShowMode::Exclude
            };

            // [ENG]: Construct WeightConfig based on current application settings (-u and -a flags).
            // [POL]: Konstrukcja WeightConfig na podstawie bieżących ustawień aplikacji (flagi -u oraz -a).
            let weight_cfg = WeightConfig {
                system: if app.args.unit == crate::interfaces::cli::args::CliUnitSystem::Bin {
                    UnitSystem::Binary
                } else {
                    UnitSystem::Decimal
                },
                dir_sum_included: !app.args.all,
                ..WeightConfig::default()
            };

            let mut st_m = TreeStats::default();
            let mut st_x = TreeStats::default();

            // [ENG]: Execute scan with statistics collectors via closures.
            // [POL]: Wykonanie skanowania z kolektorami statystyk przez domknięcia.
            let stats = execute::execute(
                &app.args.enter_path,
                &app.args.patterns,
                !app.args.ignore_case,
                app.args.sort.into(),
                show_mode,
                app.args.view.into(),
                weight_cfg,
                app.args.no_root,
                false,
                app.args.no_emoji,
                &i18n,
                |f: &FileStats| {
                    if f.weight_bytes == 0 {
                        st_m.empty_count += 1;
                    }
                    if !f.path.ends_with('/') {
                        let ext = f
                            .absolute
                            .extension()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_lowercase();
                        if is_blacklisted_extension(&ext) {
                            st_m.bin_count += 1;
                            st_m.bin_weight += f.weight_bytes;
                        } else {
                            st_m.txt_count += 1;
                            st_m.txt_weight += f.weight_bytes;
                        }
                    }
                },
                |f: &FileStats| {
                    if f.weight_bytes == 0 {
                        st_x.empty_count += 1;
                    }
                    if !f.path.ends_with('/') {
                        let ext = f
                            .absolute
                            .extension()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_lowercase();
                        if is_blacklisted_extension(&ext) {
                            st_x.bin_count += 1;
                            st_x.bin_weight += f.weight_bytes;
                        } else {
                            st_x.txt_count += 1;
                            st_x.txt_weight += f.weight_bytes;
                        }
                    }
                },
            );

            // [ENG]: Update application state with results and calculated statistics.
            // [POL]: Aktualizacja stanu aplikacji o wyniki i obliczone statystyki.
            st_m.matched_count = stats.m_size_matched;
            st_m.total_count = stats.total;
            st_x.matched_count = stats.x_size_mismatched;
            st_x.total_count = stats.total;

            app.stats_m = st_m;
            app.stats_x = st_x;

            if is_match {
                app.generated_paths_m =
                    stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
            } else {
                app.generated_paths_x =
                    stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
            }
        }

        ui.add_space(10.0);
        ui.checkbox(&mut app.args.by, gt.t(GT::LabelAddFooter));

        ui.add_space(15.0);

        // [ENG]: Live unit system toggle. Label is pre-calculated to avoid borrow-checker conflicts.
        // [POL]: Przełącznik systemu jednostek na żywo. Etykieta obliczona wcześniej, by uniknąć konfliktów borrow-checkera.
        let mut is_bin = app.args.unit == crate::interfaces::cli::args::CliUnitSystem::Bin;
        let unit_label = if is_bin { "IEC (Bin)" } else { "SI (Dec)" };

        if ui
            .checkbox(&mut is_bin, unit_label)
            .on_hover_text("B/KB vs B/KiB")
            .changed()
        {
            app.args.unit = if is_bin {
                crate::interfaces::cli::args::CliUnitSystem::Bin
            } else {
                crate::interfaces::cli::args::CliUnitSystem::Dec
            };
        }

        ui.add_space(15.0);

        // [ENG]: Handle contextual save actions.
        // [POL]: Obsługa kontekstowych akcji zapisu.
        if is_match {
            if ui.button(gt.t(GT::BtnSaveMatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!(
                    "{}plot-address_{}_M.md",
                    resolve_dir(&app.args.dir_out, &app.args.enter_path),
                    tag
                );
                let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                let cmd_string = app.args.to_command_string(true, false, true, false);
                cargo_plot::core::save::SaveFile::paths(
                    &app.generated_paths_m,
                    &filepath,
                    &tag,
                    app.args.by,
                    &i18n,
                    &cmd_string,
                    &app.args.enter_path,
                );
            }
        } else {
            if ui.button(gt.t(GT::BtnSaveMismatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!(
                    "{}plot-address_{}_X.md",
                    resolve_dir(&app.args.dir_out, &app.args.enter_path),
                    tag
                );
                let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                let cmd_string = app.args.to_command_string(false, true, true, false);
                cargo_plot::core::save::SaveFile::paths(
                    &app.generated_paths_x,
                    &filepath,
                    &tag,
                    app.args.by,
                    &i18n,
                    &cmd_string,
                    &app.args.enter_path,
                );
            }
        }
    });

    ui.separator();

    // [ENG]: 3. FOOTER - Statistics display.
    // [POL]: 3. STOPKA - Wyświetlanie statystyk.
    let current_stats = if is_match { &app.stats_m } else { &app.stats_x };
    draw_footer(ui, "paths_stats_footer", current_stats);

    // [ENG]: 4. MAIN EDITOR - Generated content area.
    // [POL]: 4. GŁÓWNY EDYTOR - Obszar wygenerowanej treści.
    let text_buffer = match app.active_paths_tab {
        PathsTab::Match => &mut app.generated_paths_m,
        PathsTab::Mismatch => &mut app.generated_paths_x,
    };
    draw_editor(ui, text_buffer);
}

```

### 031: `./src/interfaces/gui/settings.rs`

```rust
use crate::interfaces::cli::args::{CliSortStrategy, CliViewMode};
use crate::interfaces::gui::CargoPlotApp;
use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use cargo_plot::i18n::Lang;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    // [ENG]: Initialize the GUI translation engine.
    // [POL]: Inicjalizacja silnika tłumaczeń GUI.
    let gt = GuiI18n::new(app.args.lang);

    // [ENG]: 1. Pinned Footer - Attached to the bottom of the screen.
    // [POL]: 1. Przyklejona stopka - Przypięta do dołu ekranu.
    egui::TopBottomPanel::bottom("settings_footer_panel")
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📦 cargo-plot v0.2.0").strong());
                ui.separator();
                ui.hyperlink_to("Crates.io", "https://crates.io/crates/cargo-plot");
                ui.separator();
                ui.hyperlink_to(
                    gt.t(GT::FooterDownload),
                    "https://github.com/j-Cis/cargo-plot/releases",
                );
            });

            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(gt.t(GT::FooterInstall)).weak());
                ui.code("cargo install cargo-plot");
                ui.separator();
                ui.label(egui::RichText::new(gt.t(GT::FooterUninstall)).weak());
                ui.code("cargo uninstall cargo-plot");
            });
            ui.add_space(10.0);
        });

    // [ENG]: 2. Main Content Area - Scrollable settings.
    // [POL]: 2. Główny obszar treści - Przewijalne ustawienia.
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // ⚡ Ustawiamy globalny limit szerokości (bez ucinania krawędzi)
            ui.set_max_width(600.0);
            ui.add_space(10.0);

            // [ENG]: Language selection.
            // [POL]: Wybór języka.
            ui.horizontal(|ui| {
                ui.label(gt.t(GT::LabelLang));
                ui.radio_value(&mut app.args.lang, Some(Lang::Pl), "Polski");
                ui.radio_value(&mut app.args.lang, Some(Lang::En), "English");
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // [ENG]: Path Selection Grid - Perfectly aligns labels and inputs to the right edge.
            // [POL]: Siatka wyboru ścieżek - Idealnie wyrównuje etykiety i pola do prawej krawędzi.
            egui::Grid::new("path_settings_grid")
                .num_columns(2)
                .spacing([10.0, 10.0])
                .min_col_width(120.0)
                .show(ui, |ui| {
                    // Row 1: Scan path
                    ui.label(gt.t(GT::LabelScanPath));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(gt.t(GT::BtnBrowse)).clicked()
                            && let Some(folder) = rfd::FileDialog::new().pick_folder()
                        {
                            app.args.enter_path = folder.to_string_lossy().replace('\\', "/");
                        }
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut app.args.enter_path),
                        );
                    });
                    ui.end_row();

                    // Row 2: Output folder
                    ui.label(gt.t(GT::LabelOutFolder));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(gt.t(GT::BtnBrowse)).clicked()
                            && let Some(folder) = rfd::FileDialog::new().pick_folder()
                        {
                            let mut path = folder.to_string_lossy().replace('\\', "/");
                            if !path.ends_with('/') {
                                path.push('/');
                            }
                            app.out_path_input = path.clone();
                            app.args.dir_out = Some(path);
                        }

                        let txt_response = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut app.out_path_input),
                        );
                        if txt_response.changed() {
                            let trimmed = app.out_path_input.trim();
                            app.args.dir_out = if trimmed.is_empty() {
                                None
                            } else {
                                Some(trimmed.to_string())
                            };
                        }
                    });
                    ui.end_row();
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // [ENG]: View and Sorting.
            // [POL]: Widok i sortowanie.
            ui.horizontal(|ui| {
                egui::ComboBox::from_label(gt.t(GT::LabelSorting))
                    .selected_text(format!("{:?}", app.args.sort))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.args.sort,
                            CliSortStrategy::AzFileMerge,
                            "AzFileMerge",
                        );
                        ui.selectable_value(
                            &mut app.args.sort,
                            CliSortStrategy::ZaFileMerge,
                            "ZaFileMerge",
                        );
                        ui.selectable_value(
                            &mut app.args.sort,
                            CliSortStrategy::AzDirMerge,
                            "AzDirMerge",
                        );
                        ui.selectable_value(
                            &mut app.args.sort,
                            CliSortStrategy::ZaDirMerge,
                            "ZaDirMerge",
                        );
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::AzFile, "AzFile");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::ZaFile, "ZaFile");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::AzDir, "AzDir");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::ZaDir, "ZaDir");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::Az, "Az");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::Za, "Za");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::None, "None");
                    });

                ui.add_space(15.0);

                egui::ComboBox::from_label(gt.t(GT::LabelViewMode))
                    .selected_text(format!("{:?}", app.args.view))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.args.view, CliViewMode::Tree, "Tree");
                        ui.selectable_value(&mut app.args.view, CliViewMode::List, "List");
                        ui.selectable_value(&mut app.args.view, CliViewMode::Grid, "Grid");
                    });

                ui.add_space(15.0);
                ui.checkbox(&mut app.args.no_root, gt.t(GT::LabelNoRoot));
                ui.add_space(15.0);
                ui.checkbox(&mut app.args.all, "Fizyczna waga folderów (-a)");
            });

            ui.add_space(20.0);

            // [ENG]: Match Patterns Section.
            // [POL]: Sekcja wzorców dopasowań.
            ui.heading(gt.t(GT::HeadingPatterns));
            ui.add_space(15.0);

            ui.horizontal(|ui| {
                ui.checkbox(&mut app.args.ignore_case, gt.t(GT::LabelIgnoreCase));
                ui.label(gt.t(GT::LabelNewPattern));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let btn_clicked = ui.button(gt.t(GT::BtnAddPattern)).clicked();
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut app.new_pattern_input),
                    );

                    if (btn_clicked
                        || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                        && !app.new_pattern_input.trim().is_empty()
                    {
                        let input = app.new_pattern_input.trim();

                        // ⚡ FAST-TRACK: Automatyczne parsowanie ciągów z CLI
                        if input.contains("-p ") || input.contains("--pat ") {
                            // Ujednolicamy znacznik flagi
                            let normalized = input.replace("--pat ", "-p ");

                            for part in normalized.split("-p ") {
                                let mut trimmed = part.trim();

                                // Ignorujemy śmieci takie jak komenda bazowa na początku
                                if trimmed.starts_with("cargo") || trimmed.is_empty() {
                                    continue;
                                }

                                // Zdejmujemy cudzysłowy i odcinamy ewentualne inne flagi na końcu ciągu
                                if (trimmed.starts_with('"') && trimmed.ends_with('"'))
                                    || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
                                {
                                    trimmed = &trimmed[1..trimmed.len() - 1]; // Idealne cudzysłowy po obu stronach
                                } else if trimmed.starts_with('"') || trimmed.starts_with('\'') {
                                    // Zaczyna się od cudzysłowu, ale ma śmieci po nim (np. inne flagi -i)
                                    let quote = trimmed.chars().next().unwrap();
                                    if let Some(end_idx) = trimmed[1..].find(quote) {
                                        trimmed = &trimmed[1..=end_idx];
                                    }
                                } else if let Some(space_idx) = trimmed.find(' ') {
                                    // Brak cudzysłowów, ucinamy do pierwszej spacji (inne flagi)
                                    trimmed = &trimmed[..space_idx];
                                }

                                if !trimmed.is_empty() {
                                    app.args.patterns.push(trimmed.to_string());
                                }
                            }
                        } else {
                            // Zwykłe dodanie pojedynczego wzorca wpisanego ręcznie
                            app.args.patterns.push(input.to_string());
                        }

                        app.new_pattern_input.clear();
                        response.request_focus();
                    }
                });
            });

            ui.add_space(5.0);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_height(200.0);
                // ⚡ Naprawa krawędzi: Wypełnia idealnie dostępną przestrzeń (z uwzględnieniem paddingu ramki)
                ui.set_min_width(ui.available_width());

                let mut move_up = None;
                let mut move_down = None;
                let mut remove = None;

                for (i, pat) in app.args.patterns.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.button("🗑").clicked() {
                            remove = Some(i);
                        }
                        if ui.button("⬆").clicked() {
                            move_up = Some(i);
                        }
                        if ui.button("⬇").clicked() {
                            move_down = Some(i);
                        }
                        ui.label(pat);
                    });
                }

                if let Some(i) = remove {
                    app.args.patterns.remove(i);
                }
                if let Some(i) = move_up
                    && i > 0
                {
                    app.args.patterns.swap(i, i - 1);
                }
                if let Some(i) = move_down
                    && i + 1 < app.args.patterns.len()
                {
                    app.args.patterns.swap(i, i + 1);
                }

                if !app.args.patterns.is_empty() {
                    ui.separator();
                    if ui.button(gt.t(GT::BtnClearAll)).clicked() {
                        app.args.patterns.clear();
                    }
                } else {
                    ui.label(
                        egui::RichText::new(gt.t(GT::MsgNoPatterns))
                            .italics()
                            .weak(),
                    );
                }
            });

            ui.add_space(20.0);
        });
    });
}

```

### 032: `./src/interfaces/gui/shared.rs`

```rust
use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use eframe::egui;

// [ENG]: Helper to resolve output directory from app arguments.
// [POL]: Pomocnik do wyznaczania folderu zapisu z argumentów aplikacji.
pub fn resolve_dir(val: &Option<String>, base_path: &str) -> String {
    let is_auto = val
        .as_ref()
        .is_none_or(|v| v.trim().is_empty() || v == "AUTO");
    if is_auto {
        let mut b = base_path.replace('\\', "/");
        if !b.ends_with('/') {
            b.push('/');
        }
        format!("{}.cargo-plot/", b)
    } else {
        let mut p = val.as_ref().unwrap().replace('\\', "/");
        if !p.ends_with('/') {
            p.push('/');
        }
        p
    }
}

// [ENG]: UI component: 50/50 Match & Mismatch tabs stretching across the top.
// [POL]: Komponent UI: Zakładki Match i Mismatch 50/50 rozciągnięte na górze.
pub fn draw_tabs(ui: &mut egui::Ui, gt: &GuiI18n, is_match: &mut bool) {
    ui.horizontal(|ui| {
        let item_width = (ui.available_width() - 8.0) / 2.0;

        // --- MATCH (-m) ---
        let mut m_color = egui::Color32::from_rgb(150, 150, 150);
        let mut m_bg = egui::Color32::TRANSPARENT;
        if *is_match {
            m_color = egui::Color32::from_rgb(138, 90, 255);
            m_bg = egui::Color32::from_rgb(40, 40, 40);
        }

        let m_btn = ui.add_sized(
            [item_width, 40.0],
            egui::Button::new(
                egui::RichText::new(gt.t(GT::TabMatch))
                    .size(16.0)
                    .strong()
                    .color(m_color),
            )
            .fill(m_bg),
        );
        if m_btn.clicked() {
            *is_match = true;
        }

        ui.add_space(8.0);

        // --- MISMATCH (-x) ---
        let mut x_color = egui::Color32::from_rgb(150, 150, 150);
        let mut x_bg = egui::Color32::TRANSPARENT;
        if !*is_match {
            x_color = egui::Color32::from_rgb(255, 80, 100);
            x_bg = egui::Color32::from_rgb(40, 40, 40);
        }

        let x_btn = ui.add_sized(
            [item_width, 40.0],
            egui::Button::new(
                egui::RichText::new(gt.t(GT::TabMismatch))
                    .size(16.0)
                    .strong()
                    .color(x_color),
            )
            .fill(x_bg),
        );
        if x_btn.clicked() {
            *is_match = false;
        }
    });
}

// [ENG]: UI component: Statistics footer placeholder.
// [POL]: Komponent UI: Stopka ze statystykami.
pub fn draw_footer(
    ui: &mut egui::Ui,
    panel_id: &'static str,
    stats: &crate::interfaces::gui::TreeStats,
) {
    let fmt_bytes = |b: u64| -> String {
        let kb = b as f64 / 1024.0;
        if kb < 1.0 {
            format!("{} B", b)
        } else if kb < 1024.0 {
            format!("{:.1} KB", kb)
        } else {
            format!("{:.2} MB", kb / 1024.0)
        }
    };

    egui::TopBottomPanel::bottom(panel_id).show_inside(ui, |ui| {
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label(format!(
                "📝 Txt: {} ({})",
                stats.txt_count,
                fmt_bytes(stats.txt_weight)
            ));
            ui.separator();
            ui.label(format!(
                "📦 Bin: {} ({})",
                stats.bin_count,
                fmt_bytes(stats.bin_weight)
            ));
            ui.separator();

            if stats.err_count > 0 {
                // ⚡ Zaznacza się na czerwono, jeśli są błędy
                ui.label(
                    egui::RichText::new(format!(
                        "🚫 Err: {} ({})",
                        stats.err_count,
                        fmt_bytes(stats.err_weight)
                    ))
                    .color(egui::Color32::RED),
                );
                ui.separator();
            } else {
                ui.label("🚫 Err: 0 (0 B)");
                ui.separator();
            }

            ui.label(format!("🕳️ Empty: {}", stats.empty_count));
            ui.separator();
            ui.label(format!(
                "🎯 Matched: {} / {}",
                stats.matched_count, stats.total_count
            ));
        });
        ui.add_space(5.0);
    });
}

// [ENG]: UI component: Central scrollable editor.
// [POL]: Komponent UI: Centralny przewijalny edytor.
pub fn draw_editor(ui: &mut egui::Ui, text_buffer: &mut String) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            ui.add(
                egui::TextEdit::multiline(text_buffer)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY),
            );
        });
    });
}

```

### 033: `./src/interfaces/tui.rs`

```rust
// [ENG]: Interactive Terminal User Interface (TUI) module registry.
// [POL]: Rejestr modułu interaktywnego interfejsu tekstowego (TUI).

pub mod i18n;
pub mod menu;
pub mod state;

pub fn run_tui() {
    let mut s = state::StateTui::new();
    cliclack::clear_screen().unwrap();
    //cliclack::intro(" 📖 https://crates.io/crates/cargo-plot").unwrap();
    menu::menu_main(&mut s);
}

```

### 034: `./src/interfaces/tui/i18n.rs`

```rust
use cargo_plot::i18n::Lang;
use console::style;

fn pad(text: &str) -> String {
    format!("  {}  ", text)
}

pub struct Txt {
    pub pol: &'static str,
    pub eng: &'static str,
}

pub trait Translatable {
    fn trans(&self) -> Txt;
    fn theme(&self, text: String) -> String {
        text
    }
}

pub enum Prompt {
    HeaderMain,
    BtnLang,
    BtnQuickStart,
    BtnPaths,
    BtnView,
    BtnOutput,
    BtnFilters,
    BtnRun,
    BtnExit,
    InputPatterns,
    ExitBye,
    WarnNoPatterns,
    // Prompty dla podmenu (wersje dwujęzyczne w jednej linii dla szybkości)
    SubBasePath,
    SubIgnoreCase,
    SubSelectView,
    SubSelectSort,
    SubNoRoot,
    SubDirOut,
    SubSaveAddress,
    SubSaveArchive,
    SubBy,
    SubOnMatch,
    SubOnMismatch,
    SubInfo,
    BtnCliMode,
    InputCliCommand,
    SuccessCliParse,
    BtnHelp,
    HelpPause,
    SubHelpHeader,
    HelpPatternsBtn,
    HelpFlagsBtn,
    HelpTextPatterns,
    HelpTextFlags,
    BtnGui,
}

impl Translatable for Prompt {
    fn trans(&self) -> Txt {
        match self {
            Prompt::HeaderMain => Txt {
                pol: "📦 cargo-plot [POL] - Interaktywny Kreator",
                eng: "📦 cargo-plot [ENG] - Interactive Builder",
            },
            Prompt::BtnLang => Txt {
                pol: "🌍 Zmień język / Change language",
                eng: "🌍 Zmień język / Change language",
            },
            Prompt::BtnQuickStart => Txt {
                pol: "🚀 SZYBKI START (Podaj wzorce i uruchom)",
                eng: "🚀 QUICK START (Enter patterns and run)",
            },
            Prompt::BtnPaths => Txt {
                pol: "🛠️  Ścieżki i Wzorce",
                eng: "🛠️  Paths and Patterns",
            },
            Prompt::BtnView => Txt {
                pol: "👁️  Widok i Sortowanie",
                eng: "👁️  View and Sorting",
            },
            Prompt::BtnOutput => Txt {
                pol: "💾 Zapis plików",
                eng: "💾 Output and Saving",
            },
            Prompt::BtnFilters => Txt {
                pol: "⚙️  Filtry i Opcje",
                eng: "⚙️  Filters and Options",
            },
            Prompt::BtnRun => Txt {
                pol: "▶️  URUCHOM SKANOWANIE",
                eng: "▶️  RUN SCANNER",
            },
            Prompt::BtnExit => Txt {
                pol: "❌ WYJŚCIE",
                eng: "❌ EXIT",
            },
            Prompt::InputPatterns => Txt {
                pol: "Podaj wzorce (oddzielone przecinkiem, np. *.rs, Cargo.toml):",
                eng: "Enter patterns (comma separated, e.g. *.rs, Cargo.toml):",
            },
            Prompt::ExitBye => Txt {
                pol: "Do widzenia!",
                eng: "Goodbye!",
            },
            Prompt::WarnNoPatterns => Txt {
                pol: "Brak wzorców! Podaj przynajmniej jeden.",
                eng: "Missing patterns! Provide at least one.",
            },

            // Podmenu
            Prompt::SubBasePath => Txt {
                pol: "Ścieżka bazowa / Base path:",
                eng: "Ścieżka bazowa / Base path:",
            },
            Prompt::SubIgnoreCase => Txt {
                pol: "Ignorować wielkość liter? / Ignore case?",
                eng: "Ignorować wielkość liter? / Ignore case?",
            },
            Prompt::SubSelectView => Txt {
                pol: "Wybierz widok / Select view:",
                eng: "Wybierz widok / Select view:",
            },
            Prompt::SubSelectSort => Txt {
                pol: "Wybierz sortowanie / Select sorting:",
                eng: "Wybierz sortowanie / Select sorting:",
            },
            Prompt::SubNoRoot => Txt {
                pol: "Ukryć główny folder? / Hide root dir?",
                eng: "Ukryć główny folder? / Hide root dir?",
            },
            Prompt::SubDirOut => Txt {
                pol: "Folder zapisu (--dir-out) [puste=CWD, AUTO=./other/]:",
                eng: "Output folder (--dir-out) [empty=CWD, AUTO=./other/]:",
            },
            Prompt::SubSaveAddress => Txt {
                pol: "Zapisywać listę ścieżek (--save-address)?",
                eng: "Save paths list (--save-address)?",
            },
            Prompt::SubSaveArchive => Txt {
                pol: "Zapisywać kody źródłowe (--save-archive)?",
                eng: "Save source codes (--save-archive)?",
            },
            Prompt::SubBy => Txt {
                pol: "Dodać stopkę na dole pliku? / Add info footer?",
                eng: "Dodać stopkę na dole pliku? / Add info footer?",
            },
            Prompt::SubOnMatch => Txt {
                pol: "Pokaż dopasowane? / Show matched?",
                eng: "Pokaż dopasowane? / Show matched?",
            },
            Prompt::SubOnMismatch => Txt {
                pol: "Pokaż odrzucone? / Show rejected?",
                eng: "Pokaż odrzucone? / Show rejected?",
            },
            Prompt::SubInfo => Txt {
                pol: "Pokaż statystyki? / Show info stats?",
                eng: "Pokaż statystyki? / Show info stats?",
            },
            Prompt::BtnCliMode => Txt {
                pol: "⌨️  Wklej komendę (Raw CLI)",
                eng: "⌨️  Paste command (Raw CLI)",
            },
            Prompt::InputCliCommand => Txt {
                pol: "Wklej flagi lub całą komendę (np. -d ./ -m):",
                eng: "Paste flags or full command (e.g. -d ./ -m):",
            },
            Prompt::SuccessCliParse => Txt {
                pol: "Wczytano konfigurację!",
                eng: "Configuration loaded!",
            },
            Prompt::BtnHelp => Txt {
                pol: "❓ Pomoc (Wzorce i Flagi)",
                eng: "❓ Help (Patterns & Flags)",
            },
            Prompt::SubHelpHeader => Txt {
                pol: "Wybierz temat pomocy:",
                eng: "Choose help topic:",
            },
            Prompt::HelpPatternsBtn => Txt {
                pol: "Składnia Wzorców",
                eng: "Patterns Syntax",
            },
            Prompt::HelpFlagsBtn => Txt {
                pol: "Opis Flag i Opcji",
                eng: "Flags & Options Description",
            },
            Prompt::HelpTextPatterns => Txt {
                pol: "=== WZORCE DOPASOWAŃ ===
* - Dowolne znaki (np. *.rs)
** - Dowolne zagnieżdżenie (np. src/**/*.rs)
{a,b}   - Rozwinięcie klamrowe (np. {src,tests}/*.rs)
!       - Negacja / Odrzucenie (np. !*test*)
+       - Tryb głęboki: cała zawartość folderu (np. src/+)
@       - Rodzeństwo: wymaga pary plik + folder o tej samej nazwie
$       - Sierota: dopasowuje TYLKO, gdy brakuje pary plik/folder

=== PRZYKŁADY ===
*.rs               -> Pokaż wszystkie pliki .rs
!@tui{.rs,/}+      -> Wyklucz plik tui.rs oraz folder tui/ z całą zawartością (+)",

                eng: "=== PATTERN SYNTAX ===
* - Any characters (e.g. *.rs)
** - Any dir depth (e.g. src/**/*.rs)
{a,b}   - Brace expansion (e.g. {src,tests}/*.rs)
!       - Negation / Reject (e.g. !*test*)
+       - Deep mode: all contents of a directory (e.g. src/+)
@       - Sibling: requires file + dir pair with the same name
$       - Orphan: matches ONLY when file/dir pair is missing

=== EXAMPLES ===
*.rs               -> Show all .rs files
!@tui{.rs,/}+      -> Exclude tui.rs file and tui/ dir with all its contents (+)",
            },
            Prompt::HelpTextFlags => Txt {
                pol: "=== FLAGI I OPCJE (W TUI JAKO PRZEŁĄCZNIKI) ===
-d, --dir          : Ścieżka bazowa skanowania (Domyślnie: ./)
-p, --pat          : Wzorce dopasowań (wymagane, oddzielane przecinkiem)
-s, --sort         : Strategia sortowania wyników (np. AzFileMerge)
-v, --view         : Widok wyników (Tree, List, Grid)
-m, --on-match     : Pokaż tylko dopasowane ścieżki
-x, --on-mismatch  : Pokaż tylko odrzucone ścieżki
-o, --out-paths    : Zapisz wynik jako listę ścieżek (Markdown)
-c, --out-cache    : Zapisz wynik wraz z kodem plików (Markdown Cache)
-b, --by           : Dodaj stopkę informacyjną z komendą na końcu pliku
-i, --info         : Pokaż statystyki skanowania (Dopasowano/Odrzucono)
--ignore-case      : Ignoruj wielkość liter we wzorcach
--treeview-no-root : Ukryj główny folder roboczy w widoku drzewa",

                eng: "=== FLAGS & OPTIONS (TOGGLES IN TUI) ===
-d, --dir          : Base input path to scan (Default: ./)
-p, --pat          : Match patterns (required, comma separated)
-s, --sort         : Sorting strategy (e.g. AzFileMerge)
-v, --view         : Results view (Tree, List, Grid)
-m, --on-match     : Show only matched paths
-x, --on-mismatch  : Show only rejected paths
-o, --out-paths    : Save result as paths list (Markdown)
-c, --out-cache    : Save result with file codes (Markdown Cache)
-b, --by           : Add info footer with command at the end of file
-i, --info         : Show scan statistics (Matched/Rejected)
--ignore-case      : Ignore case in patterns
--treeview-no-root : Hide main working directory in tree view",
            },
            Prompt::HelpPause => Txt {
                pol: "Naciśnij [Enter], aby wrócić do menu...",
                eng: "Press [Enter] to return to menu...",
            },
            Prompt::BtnGui => Txt {
                pol: "🖥️  Otwórz w oknie (GUI)",
                eng: "🖥️  Open in window (GUI)",
            },
        }
    }

    fn theme(&self, text: String) -> String {
        match self {
            Prompt::BtnQuickStart => style(text).on_blue().white().bold().to_string(),
            Prompt::BtnRun => style(text).on_green().black().bold().to_string(),
            Prompt::BtnLang => style(text).cyan().to_string(),
            Prompt::BtnExit => style(text).red().to_string(),
            Prompt::HeaderMain => style(text).on_white().black().bold().to_string(),
            Prompt::BtnCliMode => style(text).on_black().yellow().bold().to_string(),
            Prompt::BtnHelp => style(text).magenta().bold().to_string(),
            Prompt::BtnGui => style(text).on_magenta().white().bold().to_string(),
            _ => text,
        }
    }
}

pub struct T {
    lang: Lang,
}

impl T {
    pub fn new(lang: Lang) -> Self {
        Self { lang }
    }
    pub fn fmt<I: Translatable>(&self, item: I) -> String {
        let txt = item.trans();
        let text = match self.lang {
            Lang::Pl => txt.pol,
            Lang::En => txt.eng,
        };
        item.theme(pad(text))
    }
    pub fn raw<I: Translatable>(&self, item: I) -> String {
        let txt = item.trans();
        match self.lang {
            Lang::Pl => txt.pol.to_string(),
            Lang::En => txt.eng.to_string(),
        }
    }
}

```

### 035: `./src/interfaces/tui/menu.rs`

```rust
use super::i18n::{Prompt, T};
use super::state::StateTui;
use crate::interfaces::cli::args::CargoCli;
use crate::interfaces::cli::args::{CliSortStrategy, CliViewMode};
use crate::interfaces::cli::engine;
use clap::Parser;
use console::style;

#[derive(Clone, PartialEq, Eq)]
enum Action {
    Lang,
    QuickStart,
    CliMode,
    Paths,
    View,
    Output,
    Filters,
    Help,
    Run,
    Gui,
    Exit,
}

pub fn menu_main(s: &mut StateTui) {
    let mut last_action = Action::Paths;

    loop {
        let t = T::new(s.lang);
        let header = t.fmt(Prompt::HeaderMain);

        // ⚡ DYNAMICZNE ETYKIETY KOKPITU
        let pat_str = if s.args.patterns.is_empty() {
            "[]".to_string()
        } else {
            format!("[{}...]", s.args.patterns[0])
        };
        let lbl_paths = format!(
            "{} (dir: '{}', pat: {})",
            t.fmt(Prompt::BtnPaths),
            s.args.enter_path,
            pat_str
        );
        let lbl_view = format!(
            "{} (view: {:?}, sort: {:?}, root: {})",
            t.fmt(Prompt::BtnView),
            s.args.view,
            s.args.sort,
            !s.args.no_root
        );

        let out_p = s.args.dir_out.as_deref().unwrap_or("AUTO");
        let lbl_out = format!(
            "{} (dir-out: {}, address: {}, archive: {}, by: {})",
            t.fmt(Prompt::BtnOutput),
            out_p,
            s.args.save_address,
            s.args.save_archive,
            s.args.by
        );

        let lbl_filt = format!(
            "{} (match: {}, mismatch: {}, info: {})",
            t.fmt(Prompt::BtnFilters),
            s.args.include,
            s.args.exclude,
            s.args.info
        );

        // ⚡ BUDOWA MENU
        let links_hint = style("crates.io/crates/cargo-plot  |  github.com/j-Cis/cargo-plot")
            .dim()
            .to_string();
        let action_result = cliclack::select(header)
            .initial_value(last_action.clone())
            .item(Action::Lang, t.fmt(Prompt::BtnLang), "")
            .item(Action::QuickStart, t.fmt(Prompt::BtnQuickStart), "")
            .item(Action::CliMode, t.fmt(Prompt::BtnCliMode), "")
            .item(Action::Paths, lbl_paths, "")
            .item(Action::View, lbl_view, "")
            .item(Action::Output, lbl_out, "")
            .item(Action::Filters, lbl_filt, "")
            .item(Action::Help, t.fmt(Prompt::BtnHelp), "")
            .item(Action::Run, t.fmt(Prompt::BtnRun), "")
            .item(Action::Gui, t.fmt(Prompt::BtnGui), "")
            .item(Action::Exit, t.fmt(Prompt::BtnExit), links_hint)
            .interact();

        // ⚡ OBSŁUGA AKCJI
        match action_result {
            Ok(Action::Lang) => s.toggle_lang(),
            Ok(Action::QuickStart) => {
                let raw_pat: String = cliclack::input(t.raw(Prompt::InputPatterns))
                    .interact()
                    .unwrap_or_default();
                if !raw_pat.trim().is_empty() {
                    s.args.patterns = split_patterns(&raw_pat);
                    cliclack::outro("🚀 ...").unwrap();
                    engine::run(s.args.clone());
                    return;
                }
            }
            Ok(Action::CliMode) => {
                let cmd: String = cliclack::input(t.raw(Prompt::InputCliCommand))
                    .interact()
                    .unwrap_or_default();

                if !cmd.trim().is_empty() {
                    // ⚡ Shlex idealnie tnie stringa jak bash, a jeśli ktoś zgubi cudzysłów, wyłapie błąd
                    if let Some(mut parsed_split) = shlex::split(&cmd) {
                        // Czyścimy początek (wywalamy "cargo", "run", "--", "plot")
                        while !parsed_split.is_empty() {
                            let first = parsed_split[0].to_lowercase();
                            if first == "cargo"
                                || first == "run"
                                || first == "--"
                                || first == "plot"
                                || first.contains("cargo-plot")
                            {
                                parsed_split.remove(0);
                            } else {
                                break;
                            }
                        }

                        // Podajemy do parsera Clap
                        let mut cli_args = vec!["cargo".to_string(), "plot".to_string()];
                        cli_args.extend(parsed_split);

                        match CargoCli::try_parse_from(cli_args) {
                            Ok(CargoCli::Plot(parsed_args)) => {
                                s.args = parsed_args;
                                cliclack::log::success(t.raw(Prompt::SuccessCliParse)).unwrap();
                            }
                            Err(e) => {
                                cliclack::log::error(format!("{}", e)).unwrap();
                            }
                        }
                    } else {
                        // Obsługa błędu ze strony shlex
                        cliclack::log::error(
                            "Błąd parsowania komendy! Prawdopodobnie nie domknięto cudzysłowu.",
                        )
                        .unwrap();
                    }
                }
            }
            Ok(Action::Paths) => {
                last_action = Action::Paths;
                handle_paths(s, &t);
            }
            Ok(Action::View) => {
                last_action = Action::View;
                handle_view(s, &t);
            }
            Ok(Action::Output) => {
                last_action = Action::Output;
                handle_output(s, &t);
            }
            Ok(Action::Filters) => {
                last_action = Action::Filters;
                handle_filters(s, &t);
            }
            Ok(Action::Help) => {
                let help_choice = cliclack::select(t.raw(Prompt::SubHelpHeader))
                    .item(1, t.raw(Prompt::HelpPatternsBtn), "")
                    .item(2, t.raw(Prompt::HelpFlagsBtn), "")
                    .item(0, t.raw(Prompt::BtnExit), "")
                    .interact()
                    .unwrap_or(0);

                if help_choice == 1 {
                    cliclack::note("📖 WZORCE / PATTERNS", t.raw(Prompt::HelpTextPatterns))
                        .unwrap();
                    let _: String = cliclack::input(t.raw(Prompt::HelpPause))
                        .required(false) // ⚡ TO POZWALA NA PUSTY ENTER
                        .interact()
                        .unwrap_or_default();
                } else if help_choice == 2 {
                    cliclack::note("⚙️ FLAGI / FLAGS", t.raw(Prompt::HelpTextFlags)).unwrap();
                    let _: String = cliclack::input(t.raw(Prompt::HelpPause))
                        .required(false) // ⚡ TO POZWALA NA PUSTY ENTER
                        .interact()
                        .unwrap_or_default();
                }
            }
            Ok(Action::Run) => {
                if s.args.patterns.is_empty() {
                    cliclack::log::warning(t.raw(Prompt::WarnNoPatterns)).unwrap();
                    continue;
                }
                cliclack::outro("🚀 ...").unwrap();
                engine::run(s.args.clone());
                return;
            }
            Ok(Action::Gui) => {
                // Wyświetlamy komunikat na pożegnanie z terminalem
                cliclack::outro(t.fmt(Prompt::BtnGui)).unwrap();

                // Odpalamy nasze nowe okienko, przekazując mu całą zebraną konfigurację
                crate::interfaces::gui::run_gui(s.args.clone());

                // Zamykamy pętlę TUI - pałeczkę przejmuje egui!
                return;
            }
            Ok(Action::Exit) | Err(_) => {
                cliclack::outro(t.raw(Prompt::ExitBye)).unwrap();
                return;
            }
        }
        cliclack::clear_screen().unwrap();
    }
}

// =====================================================================
// SZYBKIE PODMENU (Helpery modyfikujące stan)
// =====================================================================

fn handle_paths(s: &mut StateTui, t: &T) {
    s.args.enter_path = cliclack::input(t.raw(Prompt::SubBasePath))
        .default_input(&s.args.enter_path)
        .interact()
        .unwrap_or(s.args.enter_path.clone());
    let current_pat = s.args.patterns.join(", ");
    let new_pat: String = cliclack::input(t.raw(Prompt::InputPatterns))
        .default_input(&current_pat)
        .interact()
        .unwrap_or(current_pat);
    s.args.patterns = split_patterns(&new_pat);
    s.args.ignore_case = cliclack::confirm(t.raw(Prompt::SubIgnoreCase))
        .initial_value(s.args.ignore_case)
        .interact()
        .unwrap_or(s.args.ignore_case);
}

fn handle_view(s: &mut StateTui, t: &T) {
    s.args.view = cliclack::select(t.raw(Prompt::SubSelectView))
        .initial_value(s.args.view)
        .item(CliViewMode::Tree, "Tree", "")
        .item(CliViewMode::List, "List", "")
        .item(CliViewMode::Grid, "Grid", "")
        .interact()
        .unwrap_or(s.args.view);

    s.args.sort = cliclack::select(t.raw(Prompt::SubSelectSort))
        .initial_value(s.args.sort)
        .item(
            CliSortStrategy::AzFileMerge,
            "AzFileMerge (Domyślne/Default)",
            "",
        )
        .item(CliSortStrategy::ZaFileMerge, "ZaFileMerge", "")
        .item(CliSortStrategy::AzDirMerge, "AzDirMerge", "")
        .item(CliSortStrategy::ZaDirMerge, "ZaDirMerge", "")
        .item(CliSortStrategy::AzFile, "AzFile (Najpierw pliki)", "")
        .item(CliSortStrategy::ZaFile, "ZaFile", "")
        .item(CliSortStrategy::AzDir, "AzDir (Najpierw foldery)", "")
        .item(CliSortStrategy::ZaDir, "ZaDir", "")
        .item(CliSortStrategy::Az, "Az (Alfanumerycznie)", "")
        .item(CliSortStrategy::Za, "Za (Odwrócone)", "")
        .item(CliSortStrategy::None, "None (Brak sortowania)", "")
        .interact()
        .unwrap_or(s.args.sort);

    s.args.no_root = cliclack::confirm(t.raw(Prompt::SubNoRoot))
        .initial_value(s.args.no_root)
        .interact()
        .unwrap_or(s.args.no_root);
}

fn handle_output(s: &mut StateTui, t: &T) {
    let out_p: String = cliclack::input(t.raw(Prompt::SubDirOut))
        .default_input(s.args.dir_out.as_deref().unwrap_or(""))
        .interact()
        .unwrap_or_default();
    s.args.dir_out = if out_p.trim().is_empty() {
        None
    } else {
        Some(out_p.trim().to_string())
    };

    s.args.save_address = cliclack::confirm(t.raw(Prompt::SubSaveAddress))
        .initial_value(s.args.save_address)
        .interact()
        .unwrap_or(s.args.save_address);

    s.args.save_archive = cliclack::confirm(t.raw(Prompt::SubSaveArchive))
        .initial_value(s.args.save_archive)
        .interact()
        .unwrap_or(s.args.save_archive);

    s.args.by = cliclack::confirm(t.raw(Prompt::SubBy))
        .initial_value(s.args.by)
        .interact()
        .unwrap_or(s.args.by);
}

fn handle_filters(s: &mut StateTui, t: &T) {
    s.args.include = cliclack::confirm(t.raw(Prompt::SubOnMatch))
        .initial_value(s.args.include)
        .interact()
        .unwrap_or(s.args.include);
    s.args.exclude = cliclack::confirm(t.raw(Prompt::SubOnMismatch))
        .initial_value(s.args.exclude)
        .interact()
        .unwrap_or(s.args.exclude);
    s.args.info = cliclack::confirm(t.raw(Prompt::SubInfo))
        .initial_value(s.args.info)
        .interact()
        .unwrap_or(s.args.info);
}

// =====================================================================
// POMOCNICZY PARSER WZORCÓW
// =====================================================================
fn split_patterns(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_braces = 0;

    for c in input.chars() {
        match c {
            '{' => {
                in_braces += 1;
                current.push(c);
            }
            '}' => {
                if in_braces > 0 {
                    in_braces -= 1;
                }
                current.push(c);
            }
            ',' if in_braces == 0 => {
                if !current.trim().is_empty() {
                    result.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.trim().is_empty() {
        result.push(current.trim().to_string());
    }
    result
}

/*/
fn split_cli_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';

    for c in input.chars() {
        if in_quotes {
            if c == quote_char {
                in_quotes = false;
            } else {
                current.push(c);
            }
        } else {
            match c {
                '"' | '\'' => {
                    in_quotes = true;
                    quote_char = c;
                }
                ' ' => {
                    if !current.is_empty() {
                        args.push(current.clone());
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }
    }
    if !current.is_empty() {
        args.push(current);
    }
    args
}
    */

```

### 036: `./src/interfaces/tui/state.rs`

```rust
use crate::interfaces::cli::args::{CliArgs, CliSortStrategy, CliViewMode};
use cargo_plot::i18n::Lang;

pub struct StateTui {
    pub lang: Lang,
    pub args: CliArgs,
}

impl StateTui {
    pub fn new() -> Self {
        let lang = Lang::detect();
        Self {
            lang,
            args: CliArgs {
                // Domyślne wartości, dokładnie takie jak w CLI
                enter_path: ".".to_string(),
                patterns: vec![],
                sort: CliSortStrategy::AzFileMerge,
                view: CliViewMode::Tree,
                include: true,
                exclude: false,
                dir_out: None,
                save_address: false,
                save_archive: false,
                by: false,
                tui: true,
                unit: crate::interfaces::cli::args::CliUnitSystem::Bin,
                all: false,
                ignore_case: false,
                no_root: false,
                info: true, // Domyślnie włączamy statystyki (-i)
                gui: false,
                no_emoji: false,
                lang: Some(lang),
            },
        }
    }

    /// Aktualizuje język w interfejsie i w argumentach dla silnika
    pub fn toggle_lang(&mut self) {
        self.lang = match self.lang {
            Lang::Pl => Lang::En,
            Lang::En => Lang::Pl,
        };
        self.args.lang = Some(self.lang);
    }
}

```

### 037: `./src/lib.rs`

```rust
pub mod addon;
pub mod core;
pub mod execute;
pub mod i18n;
pub mod theme;

```

### 038: `./src/main.rs`

```rust
// [ENG]: Main entry point switching between interfaces.
// [POL]: Główny punkt wejścia przełączający między interfejsami.

#![allow(clippy::pedantic, clippy::struct_excessive_bools)]

mod interfaces;

fn main() {
    // [ENG]: Register an empty Ctrl+C handler to prevent abrupt termination.
    // [POL]: Rejestrujemy pusty handler Ctrl+C, zapobiegając natychmiastowemu zabiciu programu.
    ctrlc::set_handler(move || {}).expect("Błąd podczas ustawiania handlera Ctrl+C");

    // [ENG]: Pass execution directly to the CLI parser and router.
    // [POL]: Przekazanie wykonania bezpośrednio do parsera i routera CLI.
    interfaces::cli::run_cli();
}

```

### 039: `./src/output.rs`

```rust
pub mod save_path;
pub mod save_code;
pub mod generator;
//pub use save_path
```

### 040: `./src/theme.rs`

```rust
pub mod for_path_list;
pub mod for_path_tree;

```

### 041: `./src/theme/for_path_list.rs`

```rust
/// [POL]: Przypisuje ikonę (emoji) do ścieżki na podstawie atrybutów: katalog oraz status elementu ukrytego.
/// [ENG]: Assigns an icon (emoji) to a path based on attributes: directory status and hidden element status.
pub fn get_icon_for_path(path: &str) -> &'static str {
    let is_dir = path.ends_with('/');

    let nazwa = path
        .trim_end_matches('/')
        .split('/')
        .next_back()
        .unwrap_or("");
    let is_hidden = nazwa.starts_with('.');

    match (is_dir, is_hidden) {
        (true, false) => "📁",  // [POL]: Folder        | [ENG]: Directory
        (true, true) => "🗃️",   // [POL]: Ukryty folder | [ENG]: Hidden directory
        (false, false) => "📄", // [POL]: Plik          | [ENG]: File
        (false, true) => "⚙️ ", // [POL]: Ukryty plik   | [ENG]: Hidden file
    }
}

```

### 042: `./src/theme/for_path_tree.rs`

```rust
// [ENG]: Path classification and icon mapping for tree visualization.
// [POL]: Klasyfikacja ścieżek i mapowanie ikon dla wizualizacji drzewa.

/// [ENG]: Global icon used for directory nodes.
/// [POL]: Globalna ikona używana dla węzłów będących folderami.
pub const DIR_ICON: &str = "📂";

pub const FILE_ICON: &str = "📄";

/// [ENG]: Defines visual and metadata properties for a file type.
/// [POL]: Definiuje wizualne i metadanowe właściwości dla typu pliku.
pub struct PathFileType {
    pub icon: &'static str,
    pub md_lang: &'static str,
}

/// [ENG]: Returns file properties based on its extension.
/// [POL]: Zwraca właściwości pliku na podstawie jego rozszerzenia.
#[must_use]
pub fn get_file_type(ext: &str) -> PathFileType {
    match ext {
        "rs" => PathFileType {
            icon: "🦀",
            md_lang: "rust",
        },
        "toml" => PathFileType {
            icon: "⚙️",
            md_lang: "toml",
        },
        "slint" => PathFileType {
            icon: "🎨",
            md_lang: "slint",
        },
        "md" => PathFileType {
            icon: "📝",
            md_lang: "markdown",
        },
        "json" => PathFileType {
            icon: "🔣",
            md_lang: "json",
        },
        "yaml" | "yml" => PathFileType {
            icon: "🛠️",
            md_lang: "yaml",
        },
        "html" => PathFileType {
            icon: "📖",
            md_lang: "html",
        },
        "css" => PathFileType {
            icon: "🖌️",
            md_lang: "css",
        },
        "js" => PathFileType {
            icon: "📜",
            md_lang: "javascript",
        },
        "ts" => PathFileType {
            icon: "📘",
            md_lang: "typescript",
        },
        // [ENG]: Default fallback for unknown file types.
        // [POL]: Domyślny fallback dla nieznanych typów plików.
        _ => PathFileType {
            icon: "📄",
            md_lang: "text",
        },
    }
}

/// [ENG]: Character set used for drawing tree branches and indents.
/// [POL]: Zestaw znaków używanych do rysowania gałęzi drzewa i wcięć.
#[derive(Debug, Clone)]
pub struct TreeStyle {
    // [ENG]: Directories (d)
    // [POL]: Foldery (d)
    pub dir_last_with_children: String, // └──┬
    pub dir_last_no_children: String,   // └───
    pub dir_mid_with_children: String,  // ├──┬
    pub dir_mid_no_children: String,    // ├───

    // [ENG]: Files (f)
    // [POL]: Pliki (f)
    pub file_last: String, // └──•
    pub file_mid: String,  // ├──•

    // [ENG]: Indentations for subsequent levels (i)
    // [POL]: Wcięcia dla kolejnych poziomów (i)
    pub indent_last: String, // "   "
    pub indent_mid: String,  // "│  "
}

impl Default for TreeStyle {
    fn default() -> Self {
        Self {
            dir_last_with_children: "└──┬".to_string(),
            dir_last_no_children: "└───".to_string(),
            dir_mid_with_children: "├──┬".to_string(),
            dir_mid_no_children: "├───".to_string(),

            file_last: "└──•".to_string(),
            file_mid: "├──•".to_string(),

            indent_last: "   ".to_string(),
            indent_mid: "│  ".to_string(),
        }
    }
}

```



---

> | Property | Value |
> | ---: | :--- |
> | **Tool** | `cargo-plot v0.2.0` |
> | **Input** | `A:/A-JAN/git-rust/j-Cis/libs-util/cargo-plot-2` |
> | **Command** | `cargo plot -d "A:/A-JAN/git-rust/j-Cis/libs-util/cargo-plot-2" -o "A:/A-JAN/git-rust/j-Cis/libs-util/cargo-plot-2/" -p "./src/+" -p "Cargo.toml" -m -v grid --save-archive -b --lang en` |
> | **TimeTag** | `2026Q1D080W12_Sat21Mar_033235486` |
> | **Links** | [Crates.io](https://crates.io/crates/cargo-plot) \| [GitHub](https://github.com/j-Cis/cargo-plot/releases) |
> | **Links** | `cargo install cargo-plot` |
> | **Help** | `cargo plot --help` |

---
