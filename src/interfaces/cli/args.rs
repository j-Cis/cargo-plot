use cargo_plot::core::path_matcher::SortStrategy;
use cargo_plot::core::path_view::ViewMode;
use clap::{Args, Parser, ValueEnum};

/// [POL]: Główny wrapper dla wtyczki Cargo.
/// Oszukuje clap'a, mówiąc mu: "Główny program nazywa się 'cargo', a 'plot' to jego subkomenda".
#[derive(Parser, Debug)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum CargoCli {
    /// [ENG]: Cargo plot subcommand.
    /// [POL]: Podkomenda cargo plot.
    Plot(CliArgs),
}

/// [POL]: Nasze docelowe argumenty CLI. Zauważ, że teraz to jest `Args`, a nie `Parser`.
#[derive(Args, Debug)]
#[command(author, version, about = "Zaawansowany skaner struktury plików", long_about = None)]
pub struct CliArgs {
    /// [ENG]: Input path to scan.
    /// [POL]: Ścieżka wejściowa do skanowania.
    #[arg(short = 'd', long = "dir", default_value = ".")]
    pub enter_path: String,

    /// [ENG]: Match patterns.
    /// [POL]: Wzorce dopasowań.
    #[arg(short = 'p', long = "pat", required = true)]
    pub patterns: Vec<String>,

    /// [ENG]: Results sorting strategy.
    /// [POL]: Strategia sortowania wyników.
    #[arg(short = 's', long = "sort", value_enum, default_value_t = CliSortStrategy::AzFileMerge)]
    pub sort: CliSortStrategy,

    /// [POL]: Wybiera format wyświetlania wyników (drzewo, lista, siatka).
    #[arg(short = 'v', long = "view", value_enum, default_value_t = CliViewMode::Tree)]
    pub view: CliViewMode,

    /// [ENG]: Display only matched paths.
    /// [POL]: Wyświetlaj tylko dopasowane ścieżki.
    #[arg(short = 'm', long = "on-match")]
    pub include: bool,

    /// [ENG]: Display only rejected paths.
    /// [POL]: Wyświetlaj tylko odrzucone ścieżki.
    #[arg(short = 'x', long = "on-mismatch")]
    pub exclude: bool,

    /// [ENG]: Ignore case.
    /// [POL]: Ignoruj wielkość liter.
    #[arg(long = "ignore-case")]
    pub ignore_case: bool,    

    /// [POL]: Ukrywa główny folder (root) w widoku drzewa.
    #[arg(long = "treeview-no-root", default_value_t = false)]
    pub no_root: bool,

    /// [POL]: Wyświetla dodatkowe informacje, statystyki i nagłówki (tryb gadatliwy).
    #[arg(short = 'i', long = "info", default_value_t = false)]
    pub info: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq)]
pub enum CliViewMode {
    Tree,
    List,
    Grid,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
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
