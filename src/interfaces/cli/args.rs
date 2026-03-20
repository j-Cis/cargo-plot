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
    #[arg(short = 'm', long = "pat-match")]
    pub include: bool,

    /// [ENG]: ❌ Treat patterns as mismatch (exclude) rules.
    /// [POL]: ❌ Traktuj wzorce jako zasady odrzucenia (wyklucz).
    #[arg(short = 'x', long = "pat-mismatch")]
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
    pub fn to_command_string(&self) -> String {
        let mut cmd = vec!["cargo".to_string(), "plot".to_string()];

        if self.enter_path != "." && !self.enter_path.is_empty() {
            cmd.push("-d".to_string());
            cmd.push(format!("\"{}\"", self.enter_path));
        }

        if let Some(dir) = &self.dir_out {
            cmd.push("-o".to_string());
            if dir != "AUTO" {
                cmd.push(format!("\"{}\"", dir));
            }
        }

        if !self.patterns.is_empty() {
            cmd.push("-p".to_string());
            cmd.push(format!("\"{}\"", self.patterns.join(",")));
        }

        if self.include { cmd.push("-m".to_string()); }
        if self.exclude { cmd.push("-x".to_string()); }
        if self.ignore_case { cmd.push("-c".to_string()); }

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

        if self.save_address { cmd.push("--save-address".to_string()); }
        if self.save_archive { cmd.push("--save-archive".to_string()); }
        if self.by { cmd.push("-b".to_string()); }
        if self.no_root { cmd.push("--treeview-no-root".to_string()); }
        if self.info { cmd.push("-i".to_string()); }
        if self.no_emoji { cmd.push("--no-emoji".to_string()); }
        if self.all { cmd.push("-a".to_string()); }
        
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