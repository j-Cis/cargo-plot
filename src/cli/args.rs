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
