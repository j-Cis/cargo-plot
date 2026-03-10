# Dokumentacja Projektu 2026Q1D070W11_Wed11Mar_001738202

**Wywołana komenda:**
```bash
target\debug\cargo-plot.exe plot doc --out-dir . --out d -I num -T files-first --no-default-excludes -e ./f.md -e ./d.md -e ./target/ -e ./.git/ -e ./test/ -e ./.gitignore -e ./u.md -e ./Cargo.lock -e ./LICENSE-APACHE -e ./LICENSE-MIT -e ./.github/ -e ./.cargo/ -e ./doc/ -e ./README.md -w binary --weight-precision 5 --no-dir-weight --watermark last --print-command --title-file Dokumentacja Projektu
```

```text
[KiB 1.689] ├──• ⚙️ Cargo.toml
            └──┬ 📂 src
[  B 671.0]    ├──• 🦀 main.rs
               ├──┬ 📂 cli
[KiB 7.231]    │  ├──• 🦀 args.rs
[  B 724.0]    │  ├──• 🦀 dist.rs
[KiB 1.791]    │  ├──• 🦀 doc.rs
[  B 408.0]    │  ├──• 🦀 mod.rs
[  B 577.0]    │  ├──• 🦀 stamp.rs
[KiB 3.690]    │  ├──• 🦀 tree.rs
[KiB 2.486]    │  └──• 🦀 utils.rs
               ├──┬ 📂 lib
[KiB 6.758]    │  ├──• 🦀 fn_copy_dist.rs
[KiB 1.702]    │  ├──• 🦀 fn_datestamp.rs
[KiB 1.913]    │  ├──• 🦀 fn_doc_gen.rs
[KiB 2.703]    │  ├──• 🦀 fn_doc_id.rs
[  B 570.0]    │  ├──• 🦀 fn_doc_models.rs
[KiB 4.593]    │  ├──• 🦀 fn_doc_write.rs
[KiB 1.964]    │  ├──• 🦀 fn_files_blacklist.rs
[KiB 8.222]    │  ├──• 🦀 fn_filespath.rs
[KiB 4.604]    │  ├──• 🦀 fn_filestree.rs
[  B 724.0]    │  ├──• 🦀 fn_path_utils.rs
[KiB 1.546]    │  ├──• 🦀 fn_pathtype.rs
[KiB 4.278]    │  ├──• 🦀 fn_plotfiles.rs
[KiB 3.602]    │  ├──• 🦀 fn_weight.rs
[  B 288.0]    │  └──• 🦀 mod.rs
               └──┬ 📂 tui
[KiB 1.393]       ├──• 🦀 dist.rs
[KiB 4.244]       ├──• 🦀 doc.rs
[KiB 1.487]       ├──• 🦀 mod.rs
[KiB 1.023]       ├──• 🦀 stamp.rs
[KiB 2.616]       ├──• 🦀 tree.rs
[KiB 6.317]       └──• 🦀 utils.rs
```

## Plik-001: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_filestree.rs`

```rust
// Zaktualizowany Plik-004: src/lib/fn_filestree.rs
use crate::fn_pathtype::{DIR_ICON, get_file_type};
use crate::fn_weight::{WeightConfig, format_weight, get_path_weight};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Struktura węzła drzewa
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub icon: String,
    pub weight_str: String, // Nowe pole na sformatowaną wagę [qq xxxxx]
    pub weight_bytes: u64,  // Surowa waga do obliczeń sumarycznych
    pub children: Vec<FileNode>,
}

/// Helper do sortowania węzłów zgodnie z wybraną metodą
fn sort_nodes(nodes: &mut [FileNode], sort_method: &str) {
    match sort_method {
        "files-first" => nodes.sort_by(|a, b| {
            if a.is_dir == b.is_dir {
                a.name.cmp(&b.name)
            } else if !a.is_dir {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }),
        "dirs-first" => nodes.sort_by(|a, b| {
            if a.is_dir == b.is_dir {
                a.name.cmp(&b.name)
            } else if a.is_dir {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }),
        _ => nodes.sort_by(|a, b| a.name.cmp(&b.name)),
    }
}

/// Funkcja formatująca - buduje drzewo i przypisuje ikony oraz wagi
pub fn filestree(
    paths: Vec<PathBuf>,
    sort_method: &str,
    weight_cfg: &WeightConfig, // NOWY ARGUMENT
) -> Vec<FileNode> {
    let mut tree_map: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    for p in &paths {
        let parent = p
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("/"));
        tree_map.entry(parent).or_default().push(p.clone());
    }

    fn build_node(
        path: &PathBuf,
        paths: &BTreeMap<PathBuf, Vec<PathBuf>>,
        sort_method: &str,
        weight_cfg: &WeightConfig, // NOWY ARGUMENT
    ) -> FileNode {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        let is_dir = path.is_dir();

        let icon = if is_dir {
            DIR_ICON.to_string()
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            get_file_type(ext).icon.to_string()
        } else {
            "📄".to_string()
        };

        // KROK A: Pobieramy bazową wagę (0 dla folderów w trybie sumy uwzględnionych)
        let mut weight_bytes = get_path_weight(path, weight_cfg.dir_sum_included);

        let mut children = vec![];
        if let Some(child_paths) = paths.get(path) {
            let mut child_nodes: Vec<FileNode> = child_paths
                .iter()
                .map(|c| build_node(c, paths, sort_method, weight_cfg))
                .collect();

            crate::fn_filestree::sort_nodes(&mut child_nodes, sort_method);

            // KROK B: Jeśli to folder i sumujemy tylko ujęte pliki, zsumuj wagi dzieci
            if is_dir && weight_cfg.dir_sum_included {
                weight_bytes = child_nodes.iter().map(|n| n.weight_bytes).sum();
            }

            children = child_nodes;
        }

        // KROK C: Formatowanie wagi do ciągu "[qq xxxxx]"
        let mut weight_str = String::new();

        // Sprawdzamy czy system wag jest w ogóle włączony
        if weight_cfg.system != crate::fn_weight::UnitSystem::None {
            let should_show =
                (is_dir && weight_cfg.show_for_dirs) || (!is_dir && weight_cfg.show_for_files);

            if should_show {
                weight_str = format_weight(weight_bytes, weight_cfg);
            } else {
                // Jeśli ukrywamy wagę dla tego węzła, wstawiamy puste spacje
                // szerokość = 7 (nawiasy, jednostka, spacje) + precyzja
                let empty_width = 7 + weight_cfg.precision;
                weight_str = format!("{:width$}", "", width = empty_width);
            }
        }

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

    let roots: Vec<PathBuf> = paths
        .iter()
        .filter(|p| p.parent().is_none() || !paths.contains(&p.parent().unwrap().to_path_buf()))
        .cloned()
        .collect();

    let mut top_nodes: Vec<FileNode> = roots
        .into_iter()
        .map(|r| build_node(&r, &tree_map, sort_method, weight_cfg))
        .collect();

    crate::fn_filestree::sort_nodes(&mut top_nodes, sort_method);

    top_nodes
}

```

## Plik-002: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/cli/dist.rs`

```rust
// Plik: src/cli/dist.rs
use crate::cli::args::DistCopyArgs;
use lib::fn_copy_dist::{DistConfig, copy_dist};

pub fn handle_dist_copy(args: DistCopyArgs) {
    let bin_refs: Vec<&str> = args.bin.iter().map(|s| s.as_str()).collect();
    let config = DistConfig {
        target_dir: &args.target_dir,
        dist_dir: &args.dist_dir,
        binaries: bin_refs,
        clear_dist: args.clear,
        overwrite: !args.no_overwrite,
        dry_run: args.dry_run,
    };

    match copy_dist(&config) {
        Ok(files) => {
            for (s, d) in files {
                println!(" [+] {} -> {}", s.display(), d.display());
            }
        }
        Err(e) => eprintln!("[-] Błąd dystrybucji: {}", e),
    }
}

```

## Plik-003: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_datestamp.rs`

```rust
// ./lib/fn_datestamp.rs
use chrono::{Datelike, Local, Timelike, Weekday};
pub use chrono::{NaiveDate, NaiveTime};

/// Generuje datestamp dla obecnego, lokalnego czasu.
/// Wywołanie: `datestamp_now()`
pub fn datestamp_now() -> String {
    let now = Local::now();
    format_datestamp(now.date_naive(), now.time())
}

/// Generuje datestamp dla konkretnej, podanej daty i czasu.
/// Wywołanie: `datestamp(date, time)`
pub fn datestamp(date: NaiveDate, time: NaiveTime) -> String {
    format_datestamp(date, time)
}

/// PRYWATNA funkcja, która odwala całą brudną robotę (zasada DRY).
/// Nie ma modyfikatora `pub`, więc jest niewidoczna poza tym plikiem.
fn format_datestamp(date: NaiveDate, time: NaiveTime) -> String {
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

```

## Plik-004: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/cli/doc.rs`

```rust
// Plik: src/cli/doc.rs
use crate::cli::args::{DocArgs, IdStyle, InsertTreeMethod};
use crate::cli::utils::{build_weight_config, collect_tasks};
use lib::fn_doc_gen::generate_docs;
use lib::fn_doc_models::DocTask;
use lib::fn_filespath::filespath;

pub fn handle_doc(args: DocArgs) {
    let tasks = collect_tasks(&args.shared);
    let w_cfg = build_weight_config(&args.shared);

    // Klonujemy wywołanie z konsoli, aby umieścić je w pliku
    let cmd_str = if args.print_command {
        Some(std::env::args().collect::<Vec<_>>().join(" "))
    } else {
        None
    };

    let watermark_str = match args.watermark {
        crate::cli::args::WatermarkPosition::First => "first",
        crate::cli::args::WatermarkPosition::Last => "last",
        crate::cli::args::WatermarkPosition::None => "none",
    };

    let doc_task = DocTask {
        output_filename: &args.out,
        insert_tree: match args.insert_tree {
            InsertTreeMethod::DirsFirst => "dirs-first",
            InsertTreeMethod::None => "with-out",
            _ => "files-first",
        },
        id_style: match args.id_style {
            IdStyle::Num => "id-num",
            IdStyle::None => "id-non",
            _ => "id-tag",
        },
        tasks,
        weight_config: w_cfg,
        watermark: watermark_str,
        command_str: cmd_str,
        suffix_stamp: args.suffix_stamp,
        title_file: &args.title_file,
        title_file_with_path: args.title_file_with_path,
    };

    if args.dry_run {
        println!(
            "[!] SYMULACJA: Wykryto {} plików do przetworzenia.",
            filespath(&doc_task.tasks).len()
        );
        return;
    }

    if let Err(e) = generate_docs(vec![doc_task], &args.out_dir) {
        eprintln!("[-] Błąd generowania raportu w '{}': {}", args.out_dir, e);
    }
}

```

## Plik-005: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_path_utils.rs`

```rust
// src/lib/fn_path_utils.rs
use std::path::Path;

/// Standaryzuje ścieżkę: zamienia ukośniki na uniksowe i usuwa windowsowy prefiks rozszerzony.
pub fn standardize_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .trim_start_matches("//?/")
        .to_string()
}

/// Formatuje ścieżkę względem podanego katalogu bazowego (np. obecnego katalogu roboczego).
/// Jeśli ścieżka zawiera się w bazowej, zwraca ładny format `./relatywna/sciezka`.
pub fn to_display_path(path: &Path, base_dir: &Path) -> String {
    match path.strip_prefix(base_dir) {
        Ok(rel_path) => format!("./{}", standardize_path(rel_path)),
        Err(_) => standardize_path(path),
    }
}

```

## Plik-006: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_weight.rs`

```rust
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
            "[{:>2} {:>width$}] ",
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

```

## Plik-007: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_plotfiles.rs`

```rust
// Zaktualizowany Plik-021: src/lib/fn_plotfiles.rs
use crate::fn_filestree::FileNode;
use colored::*;

/// Zestaw znaków używanych do rysowania gałęzi drzewa.
#[derive(Debug, Clone)]
pub struct TreeStyle {
    // Foldery (d)
    pub dir_last_with_children: String, // └──┬
    pub dir_last_no_children: String,   // └───
    pub dir_mid_with_children: String,  // ├──┬
    pub dir_mid_no_children: String,    // ├───

    // Pliki (f)
    pub file_last: String, // └──
    pub file_mid: String,  // ├──

    // Wcięcia dla kolejnych poziomów (i)
    pub indent_last: String, // "   " (3 spacje)
    pub indent_mid: String,  // "│  " (kreska + 2 spacje)
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

/// Prywatna funkcja pomocnicza, która odwala całą powtarzalną robotę.
fn plot(nodes: &[FileNode], indent: &str, s: &TreeStyle, use_color: bool) -> String {
    let mut result = String::new();

    for (i, node) in nodes.iter().enumerate() {
        let is_last = i == nodes.len() - 1;
        let has_children = !node.children.is_empty();

        // 1. Wybór odpowiedniego znaku gałęzi
        let branch = if node.is_dir {
            match (is_last, has_children) {
                (true, true) => &s.dir_last_with_children,
                (false, true) => &s.dir_mid_with_children,
                (true, false) => &s.dir_last_no_children,
                (false, false) => &s.dir_mid_no_children,
            }
        } else if is_last {
            &s.file_last
        } else {
            &s.file_mid
        };

        // KROK NOWY: Przygotowanie kolorowanej (lub nie) ramki z wagą
        let weight_prefix = if node.weight_str.is_empty() {
            String::new()
        } else if use_color {
            // W CLI waga będzie szara, by nie odciągać uwagi od struktury plików
            node.weight_str.truecolor(120, 120, 120).to_string()
        } else {
            node.weight_str.clone()
        };

        // 2. Formatowanie konkretnej linii (z kolorami lub bez)
        let line = if use_color {
            if node.is_dir {
                format!(
                    "{}{}{} {}{}/\n",
                    weight_prefix, // ZMIANA TUTAJ
                    indent.green(),
                    branch.green(),
                    node.icon,
                    node.name.truecolor(200, 200, 50)
                )
            } else {
                format!(
                    "{}{}{} {}{}\n",
                    weight_prefix, // ZMIANA TUTAJ
                    indent.green(),
                    branch.green(),
                    node.icon,
                    node.name.white()
                )
            }
        } else {
            // ZMIANA TUTAJ: Doklejenie prefixu dla zwykłego tekstu
            format!(
                "{}{}{} {} {}\n",
                weight_prefix, indent, branch, node.icon, node.name
            )
        };

        result.push_str(&line);

        // 3. Rekurencja dla dzieci z wyliczonym nowym wcięciem
        if has_children {
            let new_indent = if is_last {
                format!("{}{}", indent, s.indent_last)
            } else {
                format!("{}{}", indent, s.indent_mid)
            };
            result.push_str(&plot(&node.children, &new_indent, s, use_color));
        }
    }

    result
}

/// GENEROWANIE PLAIN TEXT / MARKDOWN
pub fn plotfiles_txt(nodes: &[FileNode], indent: &str, style: Option<&TreeStyle>) -> String {
    let default_style = TreeStyle::default();
    let s = style.unwrap_or(&default_style);

    plot(nodes, indent, s, false)
}

/// GENEROWANIE KOLOROWANEGO ASCII DO CLI
pub fn plotfiles_cli(nodes: &[FileNode], indent: &str, style: Option<&TreeStyle>) -> String {
    let default_style = TreeStyle::default();
    let s = style.unwrap_or(&default_style);

    plot(nodes, indent, s, true)
}

```

## Plik-008: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/mod.rs`

```rust
pub mod fn_datestamp;
pub mod fn_filespath;
pub mod fn_filestree;
pub mod fn_plotfiles;
pub mod fn_weight;

pub mod fn_doc_gen;
pub mod fn_doc_id;
pub mod fn_doc_models;
pub mod fn_doc_write;

pub mod fn_files_blacklist;
pub mod fn_path_utils;
pub mod fn_pathtype;

pub mod fn_copy_dist;

```

## Plik-009: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_doc_models.rs`

```rust
use crate::fn_filespath::Task;
use crate::fn_weight::WeightConfig;

/// Struktura definiująca jedno zadanie generowania pliku Markdown
pub struct DocTask<'a> {
    pub output_filename: &'a str,
    pub insert_tree: &'a str, // "dirs-first", "files-first", "with-out"
    pub id_style: &'a str,    // "id-tag", "id-num", "id-non"
    pub tasks: Vec<Task<'a>>,
    pub weight_config: WeightConfig, // Nowe pole
    pub watermark: &'a str,
    pub command_str: Option<String>,
    pub suffix_stamp: bool,
    pub title_file: &'a str,
    pub title_file_with_path: bool,
}

```

## Plik-010: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_doc_write.rs`

```rust
use crate::fn_files_blacklist::is_blacklisted_extension;
use crate::fn_path_utils::to_display_path;
use crate::fn_pathtype::get_file_type;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub fn write_md(
    out_path: &str,
    files: &[PathBuf],
    id_map: &HashMap<PathBuf, String>,
    tree_text: Option<String>,
    id_style: &str,
    watermark: &str,
    command_str: &Option<String>,
    stamp: &str,
    suffix_stamp: bool,
    title_file: &str,
    title_file_with_path: bool,
) -> io::Result<()> {
    let mut content = String::new();

    // ==========================================
    // LOGIKA TYTUŁU
    // ==========================================
    let mut title_line = format!("# {}", title_file);

    if !suffix_stamp {
        title_line.push_str(&format!(" {}", stamp));
    }

    if title_file_with_path {
        title_line.push_str(&format!(" ({})", out_path));
    }

    content.push_str(&title_line);
    content.push_str("\n\n");
    // ==========================================

    let watermark_text = "> 🚀 Raport wygenerowany przy użyciu [cargo-plot](https://crates.io/crates/cargo-plot) | Źródło: [GitHub](https://github.com/j-Cis/cargo-plot)\n\n";

    // 1. Znak wodny na początku
    if watermark == "first" {
        content.push_str(watermark_text);
    }

    // 2. Reprodukcja komendy
    if let Some(cmd) = command_str {
        content.push_str(&format!("**Wywołana komenda:**\n```bash\n{}\n```\n\n", cmd));
    }

    if let Some(tree) = tree_text {
        content.push_str("```text\n");
        content.push_str(&tree);
        content.push_str("```\n\n");
    }

    let current_dir = std::env::current_dir().unwrap_or_default();
    let mut file_counter = 1;

    for path in files {
        if path.is_dir() {
            continue;
        }

        let display_path = to_display_path(path, &current_dir);

        if path.exists() {
            let original_id = id_map
                .get(path)
                .cloned()
                .unwrap_or_else(|| "BrakID".to_string());

            // <-- POPRAWIONE: używamy id_style bezpośrednio
            let header_name = match id_style {
                "id-num" => format!("Plik-{:03}", file_counter),
                "id-non" => "Plik".to_string(),
                _ => format!("Plik-{}", original_id),
            };
            file_counter += 1;

            let ext = path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();
            let lang = get_file_type(&ext).md_lang;

            // KROK 1: Sprawdzenie czarnej listy rozszerzeń
            if is_blacklisted_extension(&ext) {
                content.push_str(&format!(
                    "## {}: `{}`\n\n> *(Plik binarny/graficzny - pominięto zawartość)*\n\n",
                    header_name, display_path
                ));
                continue;
            }

            // KROK 2: Bezpieczna próba odczytu zawartości
            match fs::read_to_string(path) {
                Ok(file_content) => {
                    if lang == "markdown" {
                        content.push_str(&format!("## {}: `{}`\n\n", header_name, display_path));
                        for line in file_content.lines() {
                            if line.trim().is_empty() {
                                content.push_str(">\n");
                            } else {
                                content.push_str(&format!("> {}\n", line));
                            }
                        }
                        content.push_str("\n\n");
                    } else {
                        content.push_str(&format!(
                            "## {}: `{}`\n\n```{}\n{}\n```\n\n",
                            header_name, display_path, lang, file_content
                        ));
                    }
                }
                Err(_) => {
                    // Fallback: Plik nie ma rozszerzenia binarnego, ale jego zawartość to nie jest czysty tekst UTF-8
                    content.push_str(&format!("## {}: `{}`\n\n> *(Nie można odczytać pliku jako tekst UTF-8 - pominięto)*\n\n", header_name, display_path));
                }
            }
        } else {
            content.push_str(&format!(
                "## BŁĄD: `{}` (Plik nie istnieje)\n\n",
                display_path
            ));
        }
    }

    // 3. Znak wodny na końcu (Domyślnie)
    if watermark == "last" {
        content.push_str("---\n");
        content.push_str(watermark_text);
    }

    fs::write(out_path, &content)?;
    Ok(())
}

```

## Plik-011: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/tui/mod.rs`

```rust
use cliclack::{confirm, intro, outro, outro_cancel, select};
use std::process::exit;

mod dist;
mod doc;
mod stamp;
mod tree;
mod utils;

pub fn run_tui() {
    intro(" 📦 cargo-plot - Profesjonalny Panel Sterowania ").unwrap();

    loop {
        let action = select("Wybierz moduł API:")
            .item(
                "tree",
                "🌲 Tree Explorer",
                "Wizualizacja struktur (Multi-Task)",
            )
            .item(
                "doc",
                "📄 Doc Orchestrator",
                "Generowanie raportów Markdown",
            )
            .item("dist", "📦 Dist Manager", "Zarządzanie paczkami binarnymi")
            .item("stamp", "🕒 Stamp Tool", "Generator sygnatur czasowych")
            .item("quit", "❌ Wyjdź", "")
            .interact();

        match action {
            Ok("tree") => tree::run_tree_flow(),
            Ok("doc") => doc::run_doc_flow(),
            Ok("dist") => dist::run_dist_flow(),
            Ok("stamp") => stamp::run_stamp_flow(),
            Ok("quit") => {
                outro("Zamykanie panelu...").unwrap();
                exit(0);
            }
            _ => {
                outro_cancel("Przerwano.").unwrap();
                exit(0);
            }
        }

        if !confirm("Czy chcesz wykonać inną operację?")
            .initial_value(true)
            .interact()
            .unwrap_or(false)
        {
            outro("Do zobaczenia!").unwrap();
            break;
        }
    }
}

```

## Plik-012: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/tui/utils.rs`

```rust
// Plik: src/tui/utils.rs
use cliclack::{input, select};
use lib::fn_weight::{UnitSystem, WeightConfig};

pub struct TaskData {
    pub loc: String,
    pub inc: Vec<String>,
    pub exc: Vec<String>,
    pub fil: Vec<String>,
    pub out_type: &'static str,
}

impl TaskData {
    // FIX: Dodaliśmy <'_>, aby uciszyć ostrzeżenie o elidowanych lifetime'ach
    pub fn to_api_task(&self) -> lib::fn_filespath::Task<'_> {
        lib::fn_filespath::Task {
            path_location: &self.loc,
            path_include_only: self.inc.iter().map(|s| s.as_str()).collect(),
            path_exclude: self.exc.iter().map(|s| s.as_str()).collect(),
            filter_files: self.fil.iter().map(|s| s.as_str()).collect(),
            output_type: self.out_type,
            // FIX: Usunięto ..Default::default(), bo wypełniamy wszystkie pola
        }
    }
}

pub fn ask_for_task_data(idx: usize) -> TaskData {
    println!("\n--- Konfiguracja zadania #{} ---", idx);
    let loc: String = input("  Lokalizacja (loc):")
        .default_input(".")
        .interact()
        .unwrap();

    let use_defaults = cliclack::confirm(
        "Czy użyć domyślnej listy ignorowanych (pomiń .git, target, node_modules itp.)?",
    )
    .initial_value(true)
    .interact()
    .unwrap();

    let inc;
    let exc;
    let fil;

    if use_defaults {
        inc = vec![];
        exc = vec![
            ".git/".to_string(),
            "target/".to_string(),
            "node_modules/".to_string(),
            ".vs/".to_string(),
            ".idea/".to_string(),
            ".vscode/".to_string(),
            ".cargo/".to_string(),
            ".github/".to_string(),
        ];
        fil = vec![];
    } else {
        let inc_raw: String = cliclack::input("  Whitelist (inc) [oddzielaj przecinkiem]:")
            .placeholder("np. ./src/, Cargo.toml, ./lib/")
            .required(false)
            .interact()
            .unwrap_or_default();

        let exc_raw: String = cliclack::input("  Blacklist (exc) [oddzielaj przecinkiem]:")
            .placeholder("np. ./target/, .git/, node_modules/, Cargo.lock")
            .required(false)
            .interact()
            .unwrap_or_default();

        let fil_raw: String = cliclack::input("  Filtry plików (fil) [oddzielaj przecinkiem]:")
            .placeholder("np. *.rs, *.md, build.rs")
            .required(false)
            .interact()
            .unwrap_or_default();

        inc = process_inc(split_and_trim(&inc_raw));
        exc = split_and_trim(&exc_raw);
        fil = split_and_trim(&fil_raw);
    }

    let out_type = select_type();

    TaskData {
        loc,
        inc,
        exc,
        fil,
        out_type,
    }
}

fn process_inc(list: Vec<String>) -> Vec<String> {
    list.into_iter()
        .map(|s| {
            // FIX na "Brak Wyniku": Usuwamy ./ z początku, bo Glob tego nie lubi
            let cleaned = s.trim_start_matches("./");

            if cleaned.ends_with('/') || !cleaned.contains('.') {
                let base = cleaned.trim_end_matches('/');
                if base.is_empty() {
                    "**/*".to_string()
                } else {
                    format!("{}/**/*", base)
                }
            } else {
                cleaned.to_string()
            }
        })
        .collect()
}

pub fn split_and_trim(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn select_sort() -> &'static str {
    select("Sortowanie:")
        .item("alpha", "Alfabetyczne", "")
        .item("dirs-first", "Katalogi najpierw", "")
        .item("files-first", "Pliki najpierw", "")
        .interact()
        .unwrap()
}

pub fn select_type() -> &'static str {
    select("Co wyświetlić?")
        .item("dirs_and_files", "Wszystko", "")
        .item("files", "Tylko pliki", "")
        .item("dirs", "Tylko foldery", "")
        .interact()
        .unwrap()
}

pub fn select_id_style() -> &'static str {
    select("Styl nagłówków (ID):")
        .item("id-tag", "Opisowy (tag)", "")
        .item("id-num", "Numerowany (num)", "")
        .item("id-non", "Tylko ścieżka", "")
        .interact()
        .unwrap()
}

pub fn select_tree_style() -> &'static str {
    select("Spis treści (drzewo):")
        .item("files-first", "Pliki na górze", "")
        .item("dirs-first", "Foldery na górze", "")
        .item("with-out", "Brak drzewa", "")
        .interact()
        .unwrap()
}

pub fn ask_for_weight_config() -> WeightConfig {
    let system_str = select("Czy wyświetlać wagę (rozmiar) plików i folderów?")
        .item("none", "❌ Nie (wyłączone)", "")
        .item("binary", "💾 System binarny (KiB, MiB)", "IEC: 1024^n")
        .item("decimal", "💽 System dziesiętny (kB, MB)", "SI: 1000^n")
        .interact()
        .unwrap();

    let system = match system_str {
        "binary" => UnitSystem::Binary,
        "decimal" => UnitSystem::Decimal,
        _ => {
            return WeightConfig {
                system: UnitSystem::None,
                ..Default::default()
            };
        }
    };

    // Jeśli wybrano system, zadajemy pytania szczegółowe
    let precision_str: String = input("Precyzja (szerokość ramki liczbowej):")
        .default_input("5")
        .interact()
        .unwrap();

    let precision = precision_str.parse::<usize>().unwrap_or(5).max(3);

    let show_for_files = cliclack::confirm("Czy pokazywać rozmiar przy plikach?")
        .initial_value(true)
        .interact()
        .unwrap();

    let show_for_dirs = cliclack::confirm("Czy pokazywać zsumowany rozmiar przy folderach?")
        .initial_value(true)
        .interact()
        .unwrap();

    let mut dir_sum_included = true;
    if show_for_dirs {
        let sum_mode = select("Jak liczyć pojemność folderów?")
            .item(
                "filtered",
                "Suma widocznych plików",
                "Tylko pliki ujęte na liście",
            )
            .item(
                "real",
                "Rzeczywisty rozmiar",
                "Bezpośrednio z dysku twardego",
            )
            .interact()
            .unwrap();
        dir_sum_included = sum_mode == "filtered";
    }

    WeightConfig {
        system,
        precision,
        show_for_files,
        show_for_dirs,
        dir_sum_included,
    }
}

```

## Plik-013: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/tui/doc.rs`

```rust
use cliclack::{confirm, input, intro, spinner};
use lib::fn_doc_gen::generate_docs;
use lib::fn_doc_models::DocTask;
use lib::fn_filespath::Task;

// Importujemy niezbędne narzędzia z modułu utils
use super::utils::{TaskData, ask_for_task_data};

pub fn run_doc_flow() {
    let output_dir: String = input("Katalog wyjściowy dla raportów:")
        .default_input("doc")
        .interact()
        .unwrap();

    let mut reports_configs = Vec::new();

    loop {
        intro(format!(
            " 📄 Konfiguracja raportu nr {} ",
            reports_configs.len() + 1
        ))
        .unwrap();

        let name: String = input("Nazwa pliku (prefix):")
            .default_input("code")
            .interact()
            .unwrap();

        let id_s = super::utils::select_id_style();
        let tree_s = super::utils::select_tree_style();

        // -- NOWY BLOK WAG --
        let w_cfg = if tree_s != "with-out" {
            super::utils::ask_for_weight_config()
        } else {
            lib::fn_weight::WeightConfig {
                system: lib::fn_weight::UnitSystem::None,
                ..Default::default()
            }
        };

        let mut tasks_for_this_report = Vec::new();

        let wm = cliclack::select("Gdzie umieścić podpis (watermark) cargo-plot?")
            .item("last", "Na końcu pliku (Domyślnie)", "")
            .item("first", "Na początku pliku", "")
            .item("none", "Nie dodawaj podpisu", "")
            .interact()
            .unwrap();

        let print_cmd =
            confirm("Czy wygenerować na górze raportu komendę odtwarzającą to zadanie?")
                .initial_value(true)
                .interact()
                .unwrap();

        loop {
            // Teraz funkcja jest zaimportowana, więc zadziała bezpośrednio
            tasks_for_this_report.push(ask_for_task_data(tasks_for_this_report.len() + 1));

            if !confirm("Czy dodać kolejne zadanie skanowania (Task) DO TEGO raportu?")
                .initial_value(false)
                .interact()
                .unwrap()
            {
                break;
            }
        }

        reports_configs.push((
            name,
            id_s,
            tree_s,
            tasks_for_this_report,
            w_cfg,
            wm,
            print_cmd,
        ));

        if !confirm("Czy chcesz zdefiniować KOLEJNY, osobny raport (DocTask)?")
            .initial_value(false)
            .interact()
            .unwrap()
        {
            break;
        }
    }

    let is_dry = confirm("Czy uruchomić tryb symulacji (Dry-Run)?")
        .initial_value(false)
        .interact()
        .unwrap();

    let spin = spinner();
    spin.start("Generowanie wszystkich raportów...");

    let mut final_doc_tasks = Vec::new();

    for r in &reports_configs {
        let api_tasks: Vec<Task> = r.3.iter().map(|t: &TaskData| t.to_api_task()).collect();

        // TUI generuje "zastępczą" komendę CLI, którą można skopiować!
        let cmd_str = if r.6 {
            let mut mock_cmd = format!(
                "cargo plot doc --out-dir \"{}\" --out \"{}\" -I {} -T {}",
                output_dir, r.0, r.1, r.2
            );
            for t in &r.3 {
                mock_cmd.push_str(&format!(" --task \"loc={},out={}\"", t.loc, t.out_type));
            }
            Some(mock_cmd)
        } else {
            None
        };

        final_doc_tasks.push(DocTask {
            output_filename: &r.0,
            insert_tree: r.2,
            id_style: r.1,
            tasks: api_tasks,
            weight_config: r.4.clone(),
            watermark: r.5,
            command_str: cmd_str,
            // W TUI domyślnie zachowujemy się jak wcześniej (możemy to w przyszłości rozbudować)
            suffix_stamp: true,
            title_file: "RAPORT",
            title_file_with_path: true,
        });
    }

    if is_dry {
        spin.stop(format!(
            "Symulacja zakończona. Wygenerowano by {} raportów.",
            final_doc_tasks.len()
        ));
    } else {
        match generate_docs(final_doc_tasks, &output_dir) {
            Ok(_) => spin.stop(format!("Wszystkie raporty zapisano w /{}/", output_dir)),
            Err(e) => spin.error(format!("Błąd krytyczny: {}", e)),
        }
    }
}

```

## Plik-014: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/main.rs`

```rust
// Plik: src/main.rs
use clap::Parser;
use std::env;

mod cli;
mod tui;

fn main() {
    // [QoL Fix]: Jeśli uruchomiono binarkę bez żadnych argumentów (np. czyste `cargo run`
    // lub podwójne kliknięcie na cargo-plot.exe), pomijamy walidację Clapa i odpalamy TUI.
    if env::args().len() <= 1 {
        tui::run_tui();
        return;
    }

    // Jeśli są argumenty, pozwalamy Clapowi je sparsować (wymaga słowa 'plot')
    let cli::args::CargoCli::Plot(plot_args) = cli::args::CargoCli::parse();

    match plot_args.command {
        Some(cmd) => cli::run_command(cmd),
        None => tui::run_tui(), // Zadziała np. dla `cargo run -- plot`
    }
}

```

## Plik-015: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_pathtype.rs`

```rust
// src/lib/fn_fileslang.rs

/// Struktura przechowująca metadane dla danego typu pliku
pub struct PathFileType {
    pub icon: &'static str,
    pub md_lang: &'static str,
}
/// SSoT dla ikony folderu
pub const DIR_ICON: &str = "📂";

/// SSoT (Single Source of Truth) dla rozszerzeń plików.
/// Zwraca odpowiednią ikonę do drzewa ASCII oraz język formatowania Markdown.
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
            icon: "🌐",
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
        _ => PathFileType {
            icon: "📄",
            md_lang: "text",
        }, // Domyślny fallback
    }
}

```

## Plik-016: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/cli/stamp.rs`

```rust
// Plik: src/cli/stamp.rs
use crate::cli::args::StampArgs;
use lib::fn_datestamp::{NaiveDate, NaiveTime, datestamp, datestamp_now};

pub fn handle_stamp(args: StampArgs) {
    if let (Some(d_str), Some(t_str)) = (args.date, args.time) {
        let d = NaiveDate::parse_from_str(&d_str, "%Y-%m-%d").expect("Błędny format daty");
        let t = NaiveTime::parse_from_str(&format!("{}.{}", t_str, args.millis), "%H:%M:%S%.3f")
            .expect("Błędny format czasu");
        println!("{}", datestamp(d, t));
    } else {
        println!("{}", datestamp_now());
    }
}

```

## Plik-017: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_doc_gen.rs`

```rust
use crate::fn_datestamp::datestamp_now;
use crate::fn_doc_id::generate_ids;
use crate::fn_doc_models::DocTask;
use crate::fn_doc_write::write_md;
use crate::fn_filespath::filespath;
use crate::fn_filestree::filestree;
use crate::fn_plotfiles::plotfiles_txt;
use std::fs;
use std::io;

pub fn generate_docs(doc_tasks: Vec<DocTask>, output_dir: &str) -> io::Result<()> {
    fs::create_dir_all(output_dir)?;

    for doc_task in doc_tasks {
        // Generujemy jeden wspólny znacznik czasu dla zadania
        let stamp = datestamp_now();

        // LOGIKA NAZWY PLIKU
        let out_file = if doc_task.suffix_stamp {
            format!("{}__{}.md", doc_task.output_filename, stamp)
        } else {
            format!("{}.md", doc_task.output_filename)
        };

        let out_path = format!("{}/{}", output_dir, out_file);

        // 1. Zbieramy ścieżki
        let paths = filespath(&doc_task.tasks);

        // 2. Generowanie tekstu drzewa
        let tree_text = if doc_task.insert_tree != "with-out" {
            //  używamy konfiguracji wbudowanej w zadanie!
            let tree_nodes =
                filestree(paths.clone(), doc_task.insert_tree, &doc_task.weight_config);
            let txt = plotfiles_txt(&tree_nodes, "", None);
            Some(txt)
        } else {
            None
        };

        // 3. Nadajemy identyfikatory
        let id_map = generate_ids(&paths);

        // 4. Przekazujemy styl ID do funkcji zapisu
        write_md(
            &out_path,
            &paths,
            &id_map,
            tree_text,
            doc_task.id_style,
            doc_task.watermark,
            &doc_task.command_str,
            &stamp,
            doc_task.suffix_stamp,
            doc_task.title_file,
            doc_task.title_file_with_path,
        )?;

        // Możemy wydrukować info o POJEDYNCZYM wygenerowanym pliku
        println!(" [+] Wygenerowano raport: {}", out_path);
    }

    Ok(())
}

```

## Plik-018: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_doc_id.rs`

```rust
use std::collections::HashMap;
use std::path::PathBuf;

pub fn generate_ids(paths: &[PathBuf]) -> HashMap<PathBuf, String> {
    let mut map = HashMap::new();
    let mut counters: HashMap<String, usize> = HashMap::new();

    // Klonujemy i sortujemy ścieżki, żeby ID były nadawane powtarzalnie
    let mut sorted_paths = paths.to_vec();
    sorted_paths.sort();

    for path in sorted_paths {
        // Ignorujemy foldery, przypisujemy ID tylko plikom
        if path.is_dir() {
            continue;
        }
        // DODAJEMY .to_string() NA KOŃCU, ABY ZROBIĆ NIEZALEŻNĄ KOPIĘ
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Tutaj .replace() i tak zwraca już własnego Stringa, więc jest bezpiecznie
        let path_str = path.to_string_lossy().replace('\\', "/");

        // 1. Twarde reguły dla znanych plików
        if file_name == "Cargo.toml" {
            map.insert(path.clone(), "TomlCargo".to_string());
            continue;
        }
        if file_name == "Makefile.toml" {
            map.insert(path.clone(), "TomlMakefile".to_string());
            continue;
        }
        if file_name == "build.rs" {
            map.insert(path.clone(), "RustBuild".to_string());
            continue;
        }
        if path_str.contains("src/ui/index.slint") {
            map.insert(path.clone(), "SlintIndex".to_string());
            continue;
        }

        // 2. Dynamiczne ID na podstawie ścieżki
        let prefix = if path_str.contains("src/lib") {
            if file_name == "mod.rs" {
                "RustLibMod".to_string()
            } else {
                "RustLibPub".to_string()
            }
        } else if path_str.contains("src/bin") || path_str.contains("src/main.rs") {
            "RustBin".to_string()
        } else if path_str.contains("src/ui") {
            "Slint".to_string()
        } else {
            let ext = path.extension().unwrap_or_default().to_string_lossy();
            format!("File{}", capitalize(&ext))
        };

        // Licznik dla danej kategorii
        let count = counters.entry(prefix.clone()).or_insert(1);

        let id = if file_name == "mod.rs" && prefix == "RustLibMod" {
            format!("{}_00", prefix) // mod.rs zawsze jako 00
        } else {
            format!("{}_{:02}", prefix, count)
        };

        map.insert(path, id);
        if !(file_name == "mod.rs" && prefix == "RustLibMod") {
            *count += 1;
        }
    }

    map
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

```

## Plik-019: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/tui/dist.rs`

```rust
use cliclack::{confirm, input, intro, spinner};
use lib::fn_copy_dist::{DistConfig, copy_dist};

pub fn run_dist_flow() {
    intro(" 📦 Zarządzanie Dystrybucją ").unwrap();

    let target: String = input("Katalog kompilacji (target):")
        .default_input("./target")
        .interact()
        .unwrap();
    let dist: String = input("Katalog docelowy (dist):")
        .default_input("./dist")
        .interact()
        .unwrap();
    let bins: String = input("Binarki (przecinek) [Enter = wszystkie]:")
        .required(false)
        .interact()
        .unwrap_or_default();

    let clear = confirm("Wyczyścić katalog docelowy?")
        .initial_value(true)
        .interact()
        .unwrap();
    let dry = confirm("Tryb symulacji (Dry Run)?")
        .initial_value(false)
        .interact()
        .unwrap();

    let spin = spinner();
    spin.start("Kopiowanie artefaktów...");

    let owned_bins = super::utils::split_and_trim(&bins);
    let bin_refs: Vec<&str> = owned_bins.iter().map(|s| s.as_str()).collect();

    let config = DistConfig {
        target_dir: &target,
        dist_dir: &dist,
        binaries: bin_refs,
        clear_dist: clear,
        overwrite: true,
        dry_run: dry,
    };

    match copy_dist(&config) {
        Ok(f) => spin.stop(format!("Zakończono. Przetworzono {} plików.", f.len())),
        Err(e) => spin.error(format!("Błąd: {}", e)),
    }
}

```

## Plik-020: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/tui/tree.rs`

```rust
use super::utils::TaskData;
use cliclack::{confirm, intro, spinner}; // Usunięto outro i select
use lib::fn_filespath::{Task, filespath};
use lib::fn_filestree::filestree;
use lib::fn_plotfiles::plotfiles_cli; // Usunięto ask_for_task_data (jeśli nieużywane bezpośrednio)

pub fn run_tree_flow() {
    intro(" 🌲 Eksplorator Drzewa (Multi-Task) ").unwrap();

    let mut tasks_data: Vec<TaskData> = Vec::new();

    loop {
        tasks_data.push(super::utils::ask_for_task_data(tasks_data.len() + 1));
        if !confirm("Czy dodać kolejną lokalizację (Task)?")
            .initial_value(false)
            .interact()
            .unwrap()
        {
            break;
        }
    }

    let sort = super::utils::select_sort();

    // -- ZMIANA: Wywołujemy nowy konfigurator wag --
    let w_cfg = super::utils::ask_for_weight_config();

    // Prefix '_' mówi Rustowi: "Wiem, że tego nie używam (jeszcze), nie krzycz"
    let _use_custom_style = confirm("Czy użyć niestandardowego stylu gałęzi?")
        .initial_value(false)
        .interact()
        .unwrap();

    let save_to_file =
        confirm("Czy zapisać wynikowe drzewo do pliku .md (zamiast pokazywać w konsoli)?")
            .initial_value(false)
            .interact()
            .unwrap();

    let md_path = if save_to_file {
        // Wymuszamy typowanie bezpośrednio na zmiennej wejściowej 'path', tak jak to robiliśmy w innych miejscach
        let path: String = cliclack::input("Podaj nazwę pliku (np. drzewo.md):")
            .default_input("drzewo.md")
            .interact()
            .unwrap();
        Some(path)
    } else {
        None
    };

    let spin = spinner();
    spin.start("Budowanie złożonej struktury...");

    let tasks: Vec<Task> = tasks_data
        .iter()
        .map(|t: &super::utils::TaskData| t.to_api_task())
        .collect();

    let nodes = filestree(filespath(&tasks), sort, &w_cfg);

    spin.stop("Skanowanie zakończone:");

    // Generujemy tekst drzewa
    if let Some(path) = md_path {
        let txt = lib::fn_plotfiles::plotfiles_txt(&nodes, "", None);
        std::fs::write(&path, format!("```text\n{}\n```\n", txt)).unwrap();
        cliclack::outro(format!("Sukces! Drzewo zapisano do pliku: {}", path)).unwrap();
    } else {
        let tree_output = plotfiles_cli(&nodes, "", None);
        if tree_output.trim().is_empty() {
            cliclack::outro_cancel("Brak wyników: Żaden plik nie pasuje do podanych filtrów.")
                .unwrap();
        } else {
            println!("\n{}\n", tree_output);
            cliclack::outro("Drzewo wyrenderowane pomyślnie!").unwrap();
        }
    }
}

```

## Plik-021: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/cli/tree.rs`

```rust
// Plik: src/cli/tree.rs
use crate::cli::args::{SortMethod, TreeArgs};
use crate::cli::utils::{build_weight_config, collect_tasks};
use lib::fn_filespath::filespath;
use lib::fn_filestree::filestree;
use lib::fn_plotfiles::plotfiles_cli;

pub fn handle_tree(args: TreeArgs) {
    let tasks = collect_tasks(&args.shared);
    let paths = filespath(&tasks);

    let sort_str = match args.sort {
        SortMethod::DirsFirst => "dirs-first",
        SortMethod::FilesFirst => "files-first",
        _ => "alpha",
    };

    // POBIERAMY KONFIGURACJĘ WAG NA PODSTAWIE FLAG CLI
    let w_cfg = build_weight_config(&args.shared);

    let nodes = filestree(paths, sort_str, &w_cfg);

    // ==========================================
    // NOWA LOGIKA WYDRUKU / ZAPISU DO PLIKU
    // ==========================================

    // 1. Zawsze drukuj do konsoli, chyba że użytkownik podał plik i NIE poprosił o konsolę
    let print_to_console = args.out_file.is_none() || args.print_console;

    if print_to_console {
        println!("{}", plotfiles_cli(&nodes, "", None));
    }

    // 2. Zapisz do pliku, jeśli podano argument --out-file
    // 2. Zapisz do pliku, jeśli podano argument --out-file
    if let Some(out_file) = args.out_file {
        let stamp = lib::fn_datestamp::datestamp_now();

        // Magia ucinania rozszerzenia (np. z "plik.md" robimy "plik__STAMP.md")
        let final_out_file = if args.suffix_stamp {
            let path = std::path::Path::new(&out_file);
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let ext = path.extension().unwrap_or_default().to_string_lossy();
            let parent = path.parent().unwrap_or_else(|| std::path::Path::new(""));

            let new_name = if ext.is_empty() {
                format!("{}__{}", stem, stamp)
            } else {
                format!("{}__{}.{}", stem, stamp, ext)
            };

            let pb = parent.join(new_name);
            if parent.as_os_str().is_empty() {
                pb.to_string_lossy().into_owned()
            } else {
                pb.to_string_lossy().replace('\\', "/")
            }
        } else {
            out_file.clone()
        };

        let mut content = String::new();

        // ==========================================
        // LOGIKA TYTUŁU DLA TREE
        // ==========================================
        let mut title_line = format!("# {}", args.title_file);
        if !args.suffix_stamp {
            title_line.push_str(&format!(" {}", stamp));
        }
        if args.title_file_with_path {
            title_line.push_str(&format!(" ({})", final_out_file));
        }
        content.push_str(&title_line);
        content.push_str("\n\n");
        // ==========================================

        let watermark_text = "> 🚀 Wygenerowano przy użyciu [cargo-plot](https://crates.io/crates/cargo-plot) | Źródło: [GitHub](https://github.com/j-Cis/cargo-plot)\n\n";

        if args.watermark == crate::cli::args::WatermarkPosition::First {
            content.push_str(watermark_text);
        }

        if args.print_command {
            let cmd = std::env::args().collect::<Vec<_>>().join(" ");
            content.push_str(&format!("**Wywołana komenda:**\n```bash\n{}\n```\n\n", cmd));
        }

        let txt = lib::fn_plotfiles::plotfiles_txt(&nodes, "", None);
        content.push_str(&format!("```text\n{}\n```\n", txt));

        if args.watermark == crate::cli::args::WatermarkPosition::Last {
            content.push_str("\n---\n");
            content.push_str(watermark_text);
        }

        std::fs::write(&final_out_file, content).unwrap();
        println!(" [+] Sukces! Drzewo zapisano do pliku: {}", final_out_file);
    }
}

```

## Plik-022: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/cli/mod.rs`

```rust
// Plik: src/cli/mod.rs
pub mod args;
mod dist;
mod doc;
mod stamp;
mod tree;
mod utils;

use args::Commands;

pub fn run_command(cmd: Commands) {
    match cmd {
        Commands::Tree(args) => tree::handle_tree(args),
        Commands::Doc(args) => doc::handle_doc(args),
        Commands::Stamp(args) => stamp::handle_stamp(args),
        Commands::DistCopy(args) => dist::handle_dist_copy(args),
    }
}

```

## Plik-023: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_copy_dist.rs`

```rust
// src/lib/fn_copy_dist.rs
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Struktura konfiguracyjna do zarządzania dystrybucją (Wzorzec: Parameter Object).
pub struct DistConfig<'a> {
    pub target_dir: &'a str,
    pub dist_dir: &'a str,
    /// Lista nazw binarek (bez rozszerzeń). Jeśli pusta - kopiuje wszystkie odnalezione binarki.
    pub binaries: Vec<&'a str>,
    pub clear_dist: bool,
    pub overwrite: bool,
    pub dry_run: bool,
}

impl<'a> Default for DistConfig<'a> {
    fn default() -> Self {
        Self {
            target_dir: "./target",
            dist_dir: "./dist",
            binaries: vec![],
            clear_dist: false,
            overwrite: true,
            dry_run: false,
        }
    }
}

/// Helper: Mapuje architekturę na przyjazne nazwy systemów.
fn parse_os_from_triple(triple: &str) -> String {
    let t = triple.to_lowercase();
    if t.contains("windows") {
        "windows".to_string()
    } else if t.contains("linux") {
        "linux".to_string()
    } else if t.contains("darwin") || t.contains("apple") {
        "macos".to_string()
    } else if t.contains("android") {
        "android".to_string()
    } else if t.contains("wasm") {
        "wasm".to_string()
    } else {
        "unknown".to_string()
    }
}

/// Helper: Prosta heurystyka odróżniająca prawdziwą binarkę od śmieci po kompilacji w systemach Unix/Windows.
fn is_likely_binary(path: &Path, os_name: &str) -> bool {
    if !path.is_file() {
        return false;
    }

    // Ignorujemy ukryte pliki (na wszelki wypadek)
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
    if file_name.starts_with('.') {
        return false;
    }

    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        // Odrzucamy techniczne pliki Rusta
        if ["d", "rlib", "rmeta", "pdb", "lib", "dll", "so", "dylib"].contains(&ext_str.as_str()) {
            return false;
        }
        if os_name == "windows" {
            return ext_str == "exe";
        }
        if os_name == "wasm" {
            return ext_str == "wasm";
        }
    } else {
        // Brak rozszerzenia to standard dla plików wykonywalnych na Linux/macOS
        if os_name == "windows" {
            return false;
        }
    }

    true
}

/// Przeszukuje katalog kompilacji i kopiuje pliki według konfiguracji `DistConfig`.
/// Zwraca listę przetworzonych plików: Vec<(Źródło, Cel)>
pub fn copy_dist(config: &DistConfig) -> io::Result<Vec<(PathBuf, PathBuf)>> {
    let target_path = Path::new(config.target_dir);
    let dist_path = Path::new(config.dist_dir);

    // Fail Fast
    if !target_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Katalog '{}' nie istnieje. Uruchom najpierw `cargo build`.",
                config.target_dir
            ),
        ));
    }

    // Opcja: Czyszczenie folderu dystrybucyjnego przed kopiowaniem
    if config.clear_dist && dist_path.exists() && !config.dry_run {
        // Używamy `let _` bo jeśli folder nie istnieje lub jest zablokowany, chcemy po prostu iść dalej
        let _ = fs::remove_dir_all(dist_path);
    }

    let mut found_files = Vec::new(); // Lista krotek (źródło, docelowy_folder, docelowy_plik)
    let profiles = ["debug", "release"];

    // Funkcja wewnętrzna: Przeszukuje folder (np. target/release) i dopasowuje reguły
    let mut scan_directory = |search_dir: &Path, os_name: &str, dest_base_dir: &Path| {
        if config.binaries.is_empty() {
            // TRYB 1: Kopiuj WSZYSTKIE odnalezione binarki
            if let Ok(entries) = fs::read_dir(search_dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if is_likely_binary(&path, os_name) {
                        let dest_file = dest_base_dir.join(path.file_name().unwrap());
                        found_files.push((path, dest_base_dir.to_path_buf(), dest_file));
                    }
                }
            }
        } else {
            // TRYB 2: Kopiuj KONKRETNE binarki
            for bin in &config.binaries {
                let suffix = if os_name == "windows" {
                    ".exe"
                } else if os_name == "wasm" {
                    ".wasm"
                } else {
                    ""
                };
                let full_name = format!("{}{}", bin, suffix);
                let path = search_dir.join(&full_name);
                if path.exists() {
                    let dest_file = dest_base_dir.join(&full_name);
                    found_files.push((path, dest_base_dir.to_path_buf(), dest_file));
                }
            }
        }
    };

    // =========================================================
    // KROK 1: Skanowanie kompilacji natywnej (Hosta)
    // =========================================================
    let host_os = std::env::consts::OS;
    for profile in &profiles {
        let search_dir = target_path.join(profile);
        let dest_base = dist_path.join(host_os).join(profile);
        if search_dir.exists() {
            scan_directory(&search_dir, host_os, &dest_base);
        }
    }

    // =========================================================
    // KROK 2: Skanowanie cross-kompilacji (Target Triples)
    // =========================================================
    if let Ok(entries) = fs::read_dir(target_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                if dir_name.contains('-') {
                    let os_name = parse_os_from_triple(&dir_name);
                    for profile in &profiles {
                        let search_dir = path.join(profile);
                        let dest_base = dist_path.join(&os_name).join(profile);
                        if search_dir.exists() {
                            scan_directory(&search_dir, &os_name, &dest_base);
                        }
                    }
                }
            }
        }
    }

    // =========================================================
    // KROK 3: Fizyczne operacje (z uwzględnieniem overwrite i dry_run)
    // =========================================================
    let mut processed_files = Vec::new();

    for (src, dest_dir, dest_file) in found_files {
        // Obsługa nadpisywania
        if dest_file.exists() && !config.overwrite {
            continue; // Pomijamy ten plik
        }

        if !config.dry_run {
            fs::create_dir_all(&dest_dir)?;
            fs::copy(&src, &dest_file)?;
        }

        processed_files.push((src, dest_file));
    }

    Ok(processed_files)
}

```

## Plik-024: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_filespath.rs`

```rust
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Rule {
    regex: Regex,
    only_dir: bool,
    // is_generic: bool,
    // raw_clean: String,
}

fn glob_to_regex(pattern: &str) -> Rule {
    let raw = pattern.trim();
    let mut p = raw.replace('\\', "/");

    // NAPRAWA: Jeśli użytkownik podał "./folder", ucinamy "./",
    // ponieważ relatywna ścieżka (rel_path) nigdy tego nie zawiera.
    if p.starts_with("./") {
        p = p[2..].to_string();
    }

    // let is_generic = p == "*" || p == "**/*";
    let only_dir = p.ends_with('/');
    if only_dir {
        p.pop();
    }
    // let raw_clean = p.clone();

    let mut regex_str = regex::escape(&p);
    regex_str = regex_str.replace(r"\*\*", ".*");
    regex_str = regex_str.replace(r"\*", "[^/]*");
    regex_str = regex_str.replace(r"\?", "[^/]");
    regex_str = regex_str.replace(r"\[!", "[^");

    if regex_str.starts_with('/') {
        regex_str = format!("^{}", &regex_str[1..]);
    } else if regex_str.starts_with(".*") {
        regex_str = format!("^{}", regex_str);
    } else {
        regex_str = format!("(?:^|/){}", regex_str);
    }

    if only_dir {
        regex_str.push_str("(?:/.*)?$");
    } else {
        regex_str.push('$');
    }

    let final_regex = format!("(?i){}", regex_str);

    Rule {
        regex: Regex::new(&final_regex).unwrap_or_else(|_| Regex::new("(?i)$.^").unwrap()),
        only_dir,
        // is_generic,
        // raw_clean,
    }
}

/// Element tablicy wejściowej
/// Element tablicy wejściowej
pub struct Task<'a> {
    pub path_location: &'a str,
    pub path_exclude: Vec<&'a str>,
    pub path_include_only: Vec<&'a str>,
    pub filter_files: Vec<&'a str>,
    pub output_type: &'a str, // "dirs", "files", "dirs_and_files"
}

// Implementujemy wartości domyślne, co pozwoli nam pomijać nieużywane pola
impl<'a> Default for Task<'a> {
    fn default() -> Self {
        Self {
            path_location: ".",
            path_exclude: vec![],
            path_include_only: vec![],
            filter_files: vec![],
            output_type: "dirs_and_files",
        }
    }
}

pub fn filespath(tasks: &[Task]) -> Vec<PathBuf> {
    let mut all_results = HashSet::new();

    for task in tasks {
        let root_path = Path::new(task.path_location);
        let canonical_root =
            fs::canonicalize(root_path).unwrap_or_else(|_| root_path.to_path_buf());

        // Przygotowanie reguł
        let mut exclude_rules = Vec::new();
        for p in &task.path_exclude {
            if !p.trim().is_empty() {
                exclude_rules.push(glob_to_regex(p));
            }
        }

        let mut include_only_rules = Vec::new();
        for p in &task.path_include_only {
            if !p.trim().is_empty() {
                include_only_rules.push(glob_to_regex(p));
            }
        }

        let mut filter_files_rules = Vec::new();
        for p in &task.filter_files {
            if !p.trim().is_empty() {
                filter_files_rules.push(glob_to_regex(p));
            }
        }

        // =========================================================
        // KROK 1: PEŁNY SKAN Z ODRZUCENIEM CAŁYCH GAŁĘZI EXCLUDE
        // =========================================================
        let mut scanned_paths = Vec::new();
        scan_step1(
            &canonical_root,
            &canonical_root,
            &exclude_rules,
            &mut scanned_paths,
        );

        // =========================================================
        // KROK 2: ZACHOWANIE FOLDERÓW I FILTROWANIE PLIKÓW INCLUDE
        // =========================================================
        for path in scanned_paths {
            let rel_path = path
                .strip_prefix(&canonical_root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            let path_slash = format!("{}/", rel_path);

            if !include_only_rules.is_empty() {
                let mut matches = false;
                for rule in &include_only_rules {
                    if rule.only_dir {
                        // Jeśli reguła dotyczy TYLKO folderów
                        if path.is_dir() && rule.regex.is_match(&path_slash) {
                            matches = true;
                            break;
                        }
                    } else {
                        // Jeśli reguła jest uniwersalna (pliki i foldery)
                        if rule.regex.is_match(&rel_path) || rule.regex.is_match(&path_slash) {
                            matches = true;
                            break;
                        }
                    }
                }
                if !matches {
                    continue;
                }
            }

            if path.is_dir() {
                // Jeśli tryb to NIE "files" (czyli "dirs" lub "dirs_and_files")
                // to dodajemy folder normalnie.
                if task.output_type != "files" {
                    all_results.insert(path);
                }
            } else {
                // Jeśli tryb to "dirs", całkowicie ignorujemy pliki
                if task.output_type == "dirs" {
                    continue;
                }

                // Pliki sprawdzamy pod kątem filter_files
                let mut is_file_matched = false;
                if filter_files_rules.is_empty() {
                    is_file_matched = true;
                } else {
                    for rule in &filter_files_rules {
                        if rule.only_dir {
                            continue;
                        }
                        if rule.regex.is_match(&rel_path) {
                            is_file_matched = true;
                            break;
                        }
                    }
                }

                if is_file_matched {
                    all_results.insert(path.clone());

                    // MAGIA DLA "files":
                    // Aby drzewo nie spłaszczyło się do zwykłej listy, musimy dodać foldery nadrzędne
                    // tylko dla TEGO KONKRETNEGO dopasowanego pliku. (Ukrywa to puste foldery!)
                    if task.output_type == "files" {
                        let mut current_parent = path.parent();
                        while let Some(p) = current_parent {
                            all_results.insert(p.to_path_buf());
                            if p == canonical_root {
                                break;
                            }
                            current_parent = p.parent();
                        }
                    }
                }
            }
        }
    }

    let result: Vec<PathBuf> = all_results.into_iter().collect();
    result
}

// Prywatna funkcja pomocnicza do wykonania Kroku 1
fn scan_step1(
    root_path: &Path,
    current_path: &Path,
    exclude_rules: &[Rule],
    scanned_paths: &mut Vec<PathBuf>,
) {
    let read_dir = match fs::read_dir(current_path) {
        Ok(rd) => rd,
        Err(_) => return,
    };

    for entry in read_dir.filter_map(|e| e.ok()) {
        let path = entry.path();
        let is_dir = path.is_dir();

        let rel_path = match path.strip_prefix(root_path) {
            Ok(p) => p.to_string_lossy().replace('\\', "/"),
            Err(_) => continue,
        };

        if rel_path.is_empty() {
            continue;
        }

        let path_slash = format!("{}/", rel_path);

        // KROK 1.1: Czy wykluczone przez EXCLUDE?
        let mut is_excluded = false;
        for rule in exclude_rules {
            if rule.only_dir && !is_dir {
                continue;
            }
            if rule.regex.is_match(&rel_path) || (is_dir && rule.regex.is_match(&path_slash)) {
                is_excluded = true;
                break;
            }
        }

        // Jeśli folder/plik jest wykluczony - URYWAMY GAŁĄŹ I NIE WCHODZIMY GŁĘBIEJ
        if is_excluded {
            continue;
        }

        // KROK 1.2: Dodajemy do tymczasowych wyników KROKU 1
        scanned_paths.push(path.clone());

        // KROK 1.3: Jeśli to bezpieczny folder, skanujemy jego zawartość
        if is_dir {
            scan_step1(root_path, &path, exclude_rules, scanned_paths);
        }
    }
}

```

## Plik-025: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/tui/stamp.rs`

```rust
use cliclack::{confirm, input, intro, outro};
use lib::fn_datestamp::{NaiveDate, NaiveTime, datestamp, datestamp_now};

pub fn run_stamp_flow() {
    intro(" 🕒 Generator Sygnatur Czasowych ").unwrap();

    let custom = confirm("Czy chcesz podać własną datę i czas?")
        .initial_value(false)
        .interact()
        .unwrap();

    if custom {
        let d_str: String = input("Data (RRRR-MM-DD):")
            .placeholder("2026-03-10")
            .interact()
            .unwrap();

        let t_str: String = input("Czas (GG:MM:SS):")
            .placeholder("14:30:00")
            .interact()
            .unwrap();

        let d = NaiveDate::parse_from_str(&d_str, "%Y-%m-%d").expect("Błędny format daty");
        let t = NaiveTime::parse_from_str(&t_str, "%H:%M:%S").expect("Błędny format czasu");

        let s = datestamp(d, t);
        outro(format!("Wygenerowana sygnatura: {}", s)).unwrap();
    } else {
        let s = datestamp_now();
        outro(format!("Aktualna sygnatura: {}", s)).unwrap();
    }
}

```

## Plik-026: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/cli/utils.rs`

```rust
// Plik: src/cli/utils.rs
use crate::cli::args::{CliUnitSystem, OutputType, SharedTaskArgs};
use lib::fn_filespath::Task;
use lib::fn_weight::{UnitSystem, WeightConfig};

pub fn collect_tasks(args: &SharedTaskArgs) -> Vec<Task<'_>> {
    let mut tasks = Vec::new();

    for t_str in &args.task {
        tasks.push(parse_inline_task(t_str));
    }

    if tasks.is_empty() && args.tasks.is_none() {
        let mut excludes: Vec<&str> = args.exclude.iter().map(|s| s.as_str()).collect();
        if !args.no_default_excludes {
            excludes.extend(vec![
                ".git/",
                "target/",
                "node_modules/",
                ".vs/",
                ".idea/",
                ".vscode/",
            ]);
        }

        tasks.push(Task {
            path_location: &args.path,
            path_exclude: excludes,
            path_include_only: args.include_only.iter().map(|s| s.as_str()).collect(),
            filter_files: args.filter_files.iter().map(|s| s.as_str()).collect(),
            output_type: match args.r#type {
                OutputType::Dirs => "dirs",
                OutputType::Files => "files",
                _ => "dirs_and_files",
            },
        });
    }

    tasks
}

fn parse_inline_task(input: &str) -> Task<'_> {
    let mut task = Task::default();
    let parts = input.split(',');
    for part in parts {
        let kv: Vec<&str> = part.split('=').collect();
        if kv.len() == 2 {
            match kv[0] {
                "loc" => task.path_location = kv[1],
                "inc" => task.path_include_only.push(kv[1]),
                "exc" => task.path_exclude.push(kv[1]),
                "fil" => task.filter_files.push(kv[1]),
                "out" => task.output_type = kv[1],
                _ => {}
            }
        }
    }
    task
}

/// Konwertuje parametry z linii poleceń na strukturę konfiguracyjną API
pub fn build_weight_config(args: &SharedTaskArgs) -> WeightConfig {
    let system = match args.weight_system {
        CliUnitSystem::Decimal => UnitSystem::Decimal,
        CliUnitSystem::Binary => UnitSystem::Binary,
        CliUnitSystem::Both => UnitSystem::Both,
        CliUnitSystem::None => UnitSystem::None,
    };

    WeightConfig {
        system,
        precision: args.weight_precision.max(3), // Minimum 3 znaki na liczbę
        show_for_dirs: !args.no_dir_weight,
        show_for_files: !args.no_file_weight,
        dir_sum_included: !args.real_dir_weight, // Domyślnie sumujemy tylko ujęte w filtrach
    }
}

```

## Plik-027: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/cli/args.rs`

```rust
// Plik: src/cli/args.rs
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum CargoCli {
    /// Narzędzie do wizualizacji struktury projektu i generowania dokumentacji Markdown
    Plot(PlotArgs),
}

#[derive(Args, Debug)]
#[command(
    author,
    version,
    about = "cargo-plot - Twój szwajcarski scyzoryk do dokumentacji w Rust"
)]
pub struct PlotArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Rysuje kolorowe drzewo plików i folderów w terminalu
    Tree(TreeArgs),
    /// Generuje kompletny raport Markdown ze struktury i zawartości plików
    Doc(DocArgs),
    /// Generuje unikalny, ujednolicony znacznik czasu
    Stamp(StampArgs),
    /// Kopiuje skompilowane binarki Rusta do folderu dystrybucyjnego (dist/)
    DistCopy(DistCopyArgs),
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum CliUnitSystem {
    Decimal,
    Binary,
    Both, // Jeśli zdecydujemy się obsłużyć ten tryb później
    None,
}

#[derive(Args, Debug, Clone)]
pub struct SharedTaskArgs {
    /// Ścieżka bazowa do rozpoczęcia skanowania
    #[arg(short, long, default_value = ".")]
    pub path: String,

    /// Wyłącza domyślne ignorowanie folderów technicznych (.git, target, node_modules, itp.)
    #[arg(long)]
    pub no_default_excludes: bool,

    /// Wzorce Glob ignorujące ścieżki i foldery (np. "./target/")
    #[arg(short, long)]
    pub exclude: Vec<String>,

    /// Rygorystyczna biała lista - ignoruje wszystko, co do niej nie pasuje
    #[arg(short, long)]
    pub include_only: Vec<String>,

    /// Filtr wyświetlający wyłącznie wybrane pliki (np. "*.rs")
    #[arg(short, long)]
    pub filter_files: Vec<String>,

    /// Tryb wyświetlania węzłów
    #[arg(short, long, value_enum, default_value_t = OutputType::All)]
    pub r#type: OutputType,

    /// Tryb Inline Multi-Task (np. loc=.,inc=Cargo.toml,out=files)
    #[arg(long)]
    pub task: Vec<String>,

    /// Ścieżka do zewnętrznego pliku konfiguracyjnego (.toml)
    #[arg(long)]
    pub tasks: Option<String>,

    /// System jednostek wagi plików
    #[arg(short = 'w', long = "weight", value_enum, default_value_t = CliUnitSystem::None)]
    pub weight_system: CliUnitSystem,

    /// Szerokość całkowita formatowania liczby wagi (domyślnie 5)
    #[arg(long = "weight-precision", default_value = "5")]
    pub weight_precision: usize,

    /// Czy ukryć wagi dla folderów
    #[arg(long = "no-dir-weight")]
    pub no_dir_weight: bool,

    /// Czy ukryć wagi dla plików
    #[arg(long = "no-file-weight")]
    pub no_file_weight: bool,

    /// Jeśli użyto, waga folderu to jego prawdziwy rozmiar na dysku, a nie tylko suma wyszukanych plików
    #[arg(long = "real-dir-weight")]
    pub real_dir_weight: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum OutputType {
    Dirs,
    Files,
    All,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum SortMethod {
    DirsFirst,
    FilesFirst,
    Alpha,
}

#[derive(Args, Debug)]
pub struct TreeArgs {
    #[command(flatten)]
    pub shared: SharedTaskArgs,

    /// Sposób sortowania węzłów drzewa
    #[arg(short, long, value_enum, default_value_t = SortMethod::Alpha)]
    pub sort: SortMethod,

    /// Zapisuje wynikowe drzewo do pliku Markdown (np. drzewo.md)
    #[arg(long = "out-file")]
    pub out_file: Option<String>,

    /// Wymusza wydruk drzewa w konsoli, nawet jeśli podano --out-file (zapisz i wyświetl)
    #[arg(long = "print-console")]
    pub print_console: bool,

    /// Pozycja znaku wodnego z informacją o cargo-plot (tylko w zapisanym pliku)
    #[arg(long, value_enum, default_value_t = WatermarkPosition::Last)]
    pub watermark: WatermarkPosition,

    /// Wyświetla użytą komendę CLI na początku pliku
    #[arg(long = "print-command")]
    pub print_command: bool,

    /// Dodaje unikalny znacznik czasu do nazwy pliku wyjściowego
    #[arg(long, visible_alias = "sufix-stamp")]
    pub suffix_stamp: bool,

    /// Główny tytuł dokumentu w zapisanym pliku
    #[arg(long, default_value = "RAPORT")]
    pub title_file: String,

    /// Dodaje ścieżkę pliku do głównego tytułu dokumentu
    #[arg(long)]
    pub title_file_with_path: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum IdStyle {
    Tag,
    Num,
    None,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum InsertTreeMethod {
    DirsFirst,
    FilesFirst,
    None,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum WatermarkPosition {
    First,
    Last,
    None,
}

#[derive(Args, Debug)]
pub struct DocArgs {
    #[command(flatten)]
    pub shared: SharedTaskArgs,

    /// Ścieżka do katalogu wyjściowego, w którym zostaną zapisane raporty
    #[arg(long, default_value = "doc")]
    pub out_dir: String,

    /// Bazowa nazwa pliku wyjściowego
    #[arg(short, long, default_value = "code")]
    pub out: String,

    /// Tryb symulacji (nie modyfikuje plików na dysku)
    #[arg(long, visible_alias = "simulate")]
    pub dry_run: bool,

    /// Formatowanie identyfikatorów plików w raporcie
    #[arg(short = 'I', long, value_enum, default_value_t = IdStyle::Tag)]
    pub id_style: IdStyle,

    /// Sposób rzutowania drzewa struktury na początku raportu
    #[arg(short = 'T', long, value_enum, default_value_t = InsertTreeMethod::FilesFirst)]
    pub insert_tree: InsertTreeMethod,

    /// Pozycja znaku wodnego z informacją o cargo-plot
    #[arg(long, value_enum, default_value_t = WatermarkPosition::Last)]
    pub watermark: WatermarkPosition,

    /// Wyświetla użytą komendę CLI na początku pliku
    #[arg(long = "print-command")]
    pub print_command: bool,

    /// Dodaje unikalny znacznik czasu do nazwy pliku wyjściowego
    #[arg(long, visible_alias = "sufix-stamp")]
    pub suffix_stamp: bool,

    /// Główny tytuł dokumentu w zapisanym pliku
    #[arg(long, default_value = "RAPORT")]
    pub title_file: String,

    /// Dodaje ścieżkę pliku do głównego tytułu dokumentu
    #[arg(long)]
    pub title_file_with_path: bool,
}

#[derive(Args, Debug)]
pub struct StampArgs {
    /// Data w formacie RRRR-MM-DD
    #[arg(short, long)]
    pub date: Option<String>,

    /// Czas w formacie GG:MM:SS (wymaga również flagi --date)
    #[arg(short, long)]
    pub time: Option<String>,

    /// Milisekundy. Używane tylko w połączeniu z flagą --time
    #[arg(short, long, default_value = "000")]
    pub millis: String,
}

#[derive(Args, Debug)]
pub struct DistCopyArgs {
    /// Nazwy plików do skopiowania (domyślnie: automatycznie kopiuje WSZYSTKIE binarki)
    #[arg(short, long)]
    pub bin: Vec<String>,

    /// Ścieżka do technicznego folderu kompilacji
    #[arg(long, default_value = "./target")]
    pub target_dir: String,

    /// Ścieżka do docelowego folderu dystrybucyjnego
    #[arg(long, default_value = "./dist")]
    pub dist_dir: String,

    /// Bezpiecznie czyści stary folder dystrybucyjny przed rozpoczęciem kopiowania
    #[arg(long)]
    pub clear: bool,

    /// Zabezpiecza przed nadpisaniem istniejących plików
    #[arg(long)]
    pub no_overwrite: bool,

    /// Tryb symulacji (nic nie tworzy i nic nie usuwa na dysku)
    #[arg(long, visible_alias = "simulate")]
    pub dry_run: bool,
}

```

## Plik-028: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/src/lib/fn_files_blacklist.rs`

```rust
// src/lib/fn_files_blacklist.rs

/// Sprawdza, czy podane rozszerzenie pliku należy do czarnej listy (pliki binarne, graficzne, media, archiwa).
/// Zwraca `true`, jeśli plik powinien zostać pominięty podczas wczytywania zawartości tekstowej.
pub fn is_blacklisted_extension(ext: &str) -> bool {
    let binary_extensions = [
        // --------------------------------------------------
        // GRAFIKA I DESIGN
        // --------------------------------------------------
        "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "webp", "tiff", "tif", "heic", "psd",
        "ai",
        // --------------------------------------------------
        // BINARKI, BIBLIOTEKI I ARTEFAKTY KOMPILACJI
        // --------------------------------------------------
        // Rust / Windows / Linux / Mac
        "exe", "dll", "so", "dylib", "bin", "wasm", "pdb", "rlib", "rmeta", "lib",
        // C / C++
        "o", "a", "obj", "pch", "ilk", "exp", // Java / JVM
        "jar", "class", "war", "ear", // Python
        "pyc", "pyd", "pyo", "whl",
        // --------------------------------------------------
        // ARCHIWA I PACZKI
        // --------------------------------------------------
        "zip", "tar", "gz", "tgz", "7z", "rar", "bz2", "xz", "iso", "dmg", "pkg", "apk",
        // --------------------------------------------------
        // DOKUMENTY, BAZY DANYCH I FONTY
        // --------------------------------------------------
        // Bazy danych
        "sqlite", "sqlite3", "db", "db3", "mdf", "ldf", "rdb", // Dokumenty Office / PDF
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp",
        // Fonty
        "woff", "woff2", "ttf", "eot", "otf",
        // --------------------------------------------------
        // MEDIA (AUDIO / WIDEO)
        // --------------------------------------------------
        "mp3", "mp4", "avi", "mkv", "wav", "flac", "ogg", "m4a", "mov", "wmv", "flv",
    ];

    binary_extensions.contains(&ext)
}

```

## Plik-029: `A:/A-JAN/git-rust/j-Cis/libs-utl/cargo-plot/Cargo.toml`

```toml
[package]
name = "cargo-plot"
version = "0.1.4"
authors = ["Jan Roman Cisowski „j-Cis”"]
edition = "2024"
rust-version = "1.94.0"
description = "Szwajcarski scyzoryk do wizualizacji struktury projektu i generowania raportów Markdown bezpośrednio z poziomu Cargo."
license = "MIT OR Apache-2.0"
readme = "README.md"
repository = "https://github.com/j-Cis/cargo-plot"

# Maksymalnie 5 słów kluczowych (limit crates.io) - zoptymalizowane pod SEO
keywords = [
    "cargo", 
    "tree", 
    "markdown", 
    "filesystem",
    "documentation"
]

# Rozszerzone kategorie (tutaj również jest limit max 5, my mamy 4 mocne)
categories = [
    "development-tools::cargo-plugins", 
    "command-line-utilities",
    "command-line-interface",
    "text-processing",
]
resolver = "3"

[package.metadata.cargo]
edition = "2024"


[dependencies]
# Kluczowe dla logiki
chrono = "0.4.44"
walkdir = "2.5.0"
regex = "1.12.3"

# Kluczowe dla interfejsu (CLI/TUI)
# Wykorzystanie formatowania TOML v1.1.0 (wieloliniowe tabele z trailing comma)
clap = { 
    version = "4.5.60", 
    features = ["derive"], 
}
cliclack = "0.4.1"
colored = "3.1.1"

[lib]
name = "lib"
path = "src/lib/mod.rs"

# ==========================================
# Globalna konfiguracja lintów (Analiza kodu)
# ==========================================
# [lints.rust]
# Kategorycznie zabraniamy używania bloków `unsafe` w całym projekcie
# unsafe_code = "forbid"
# Ostrzegamy o nieużywanych importach, zmiennych i funkcjach
# unused = "warn"
#
# [lints.clippy]
# Włączamy surowsze reguły, ale jako ostrzeżenia (nie zepsują kompilacji)
# pedantic = "warn"
# Możemy tu też wyciszyć globalnie to, co nas irytuje (opcjonalnie):
# too_many_arguments = "allow"
```

---
> 🚀 Raport wygenerowany przy użyciu [cargo-plot](https://crates.io/crates/cargo-plot) | Źródło: [GitHub](https://github.com/j-Cis/cargo-plot)

