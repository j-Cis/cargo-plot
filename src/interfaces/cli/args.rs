use cargo_plot::core::path_matcher::SortStrategy;
use clap::{Args, Parser, ValueEnum};

/// [POL]: Główny wrapper dla wtyczki Cargo.
/// Oszukuje clap'a, mówiąc mu: "Główny program nazywa się 'cargo', a 'plot' to jego subkomenda".
#[derive(Parser, Debug)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum CargoCli {
    /// [EN]: Cargo plot subcommand.
    /// [PL]: Podkomenda cargo plot.
    Plot(CliArgs),
}

/// [POL]: Nasze docelowe argumenty CLI. Zauważ, że teraz to jest `Args`, a nie `Parser`.
#[derive(Args, Debug)]
#[command(author, version, about = "Zaawansowany skaner struktury plików Rusta", long_about = None)]
pub struct CliArgs {
    /// [EN]: Input path to scan.
    /// [PL]: Ścieżka wejściowa do skanowania.
    #[arg(short = 'd', long = "dir", default_value = ".")]
    pub enter_path: String,

    /// [EN]: Match patterns.
    /// [PL]: Wzorce dopasowań.
    #[arg(short = 'p', long = "pat", required = true)]
    pub patterns: Vec<String>,

    /// [EN]: Display only matched paths.
    /// [PL]: Wyświetlaj tylko dopasowane ścieżki.
    #[arg(long)]
    pub include: bool,

    /// [EN]: Display only rejected paths.
    /// [PL]: Wyświetlaj tylko odrzucone ścieżki.
    #[arg(long)]
    pub exclude: bool,

    /// [EN]: Ignore case.
    /// [PL]: Ignoruj wielkość liter.
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// [EN]: Results sorting strategy.
    /// [PL]: Strategia sortowania wyników.
    #[arg(short = 's', long = "sort", value_enum, default_value_t = CliSortStrategy::AzFileMerge)]
    pub sort: CliSortStrategy,
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
