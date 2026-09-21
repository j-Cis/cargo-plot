use clap::{Args, Parser};

#[derive(Parser, Debug, Clone)]
#[command(name = "cargo", bin_name = "cargo", version, about, long_about = None)]
pub enum CargoCliRoot {
    #[command(name = "plot", about = "Szwajcarski scyzoryk do wizualizacji struktury projektu", version)]
    Plot(PlotArgs),
}

#[derive(Args, Debug, Clone)]
pub struct PlotArgs {
    // ==========================================
    // 🔍 OPCJE SKANOWANIA (querypath)
    // ==========================================
    /// Ścieżki katalogów do skanowania (domyślnie obecny katalog)
    #[arg(short = 'd', long = "dir", default_value = ".", num_args = 1..)]
    pub dirs_to_scan: Vec<String>,

    /// Wzorce wyszukiwania (np. "*.rs", "!target/**")
    #[arg(short = 'p', long = "pattern", visible_aliases = ["pat", "patterns"], num_args = 1..)]
    pub patterns: Vec<String>,

    /// Ignoruj wielkość liter przy dopasowywaniu wzorców
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    // ==========================================
    // 🎨 OPCJE WYGLĄDU (querypath-fmt) - KOLUMNY
    // ==========================================
    /// Domyślna szerokość kolumny z nazwą pliku
    #[arg(long = "name-width", default_value_t = 25)]
    pub name_width: usize,

    /// Domyślna szerokość kolumny ze ścieżką
    #[arg(long = "path-width", default_value_t = 35)]
    pub path_width: usize,

    /// Układ kolumn: H (Struktura/Drzewo), W (Wymiar/Rozmiar), D (Data/Czas), P (Ścieżka). 
    /// Kolumna 'H' stanowi oś. Przykład: "W H D P" (Wymiar po lewej, Data i Ścieżka po prawej).
    #[arg(short = 'c', long = "columns", default_value = "H W D P")]
    pub columns: String,

    /// Użyj systemu SI (KB, MB) zamiast binarnego IEC (KiB, MiB) dla rozmiarów plików
    #[arg(short = 'B', long = "size-si")]
    pub size_si: bool,

    /// Sposób wyświetlania wagi dla katalogów (both, matched, none)
    #[arg(long = "dir-weight", default_value = "both")]
    pub dir_weight: String,

    /// Wzorzec formatowania daty i czasu (używa tokenów temporal-fmt)
    #[arg(short = 'Y', long = "time-fmt", default_value = "YYYY-MM-MD hh:mm")]
    pub time_format: String,

    // ==========================================
    // 🎨 OPCJE WYGLĄDU (querypath-fmt) - WIERSZE
    // ==========================================
    /// Zwiń/Ukryj pliki w widoku (pokazuj tylko katalogi)
    #[arg(long = "dirs-only")]
    pub dirs_only: bool,

    /// Wyłącz domyślną numerację tekstowych plików w drzewie
    #[arg(long = "no-numeration")]
    pub no_numeration: bool,

    /// Włącz numerację plików binarnych
    #[arg(long = "numerate-binaries")]
    pub numerate_binaries: bool,

    /// Włącz numerację katalogów
    #[arg(long = "numerate-dirs")]
    pub numerate_dirs: bool,

    // ==========================================
    // 🔀 OPCJE SORTOWANIA (querypath-fmt)
    // ==========================================
    /// Strategia grupowania elementów: T (TextFile), D (Directory), B (BinaryFile). Przykład: "T D B"
    #[arg(long = "group-order", default_value = "T D B")]
    pub group_order: String,

    /// Kolejność klas znaków w nazwach: S (Special), D (Digit), L (Letter). Przykład: "S D L"
    #[arg(long = "char-order", default_value = "S D L")]
    pub char_order: String,

    /// Priorytet elementów o tej samej nazwie bazowej: (file, dir)
    #[arg(long = "same-name-priority", default_value = "file")]
    pub same_name_priority: String,

    /// Priorytet elementów bez rozszerzenia (np. "Makefile" względem "Makefile.toml"): (above, below)
    #[arg(long = "no-ext-priority", default_value = "above")]
    pub no_ext_priority: String,

    /// Ignoruj ukryte kropki (np. ".gitignore") podczas sortowania alfabetycznego
    #[arg(long = "ignore-leading-dot")]
    pub ignore_leading_dot: bool,

    /// Uwzględniaj wielkość liter (Case Sensitive) przy sortowaniu (domyślnie ignoruje)
    #[arg(long = "case-sensitive")]
    pub case_sensitive: bool,

    // ==========================================
    // 📦 OPCJE EKSPORTU (querypath-snapshot)
    // ==========================================
    /// Ścieżka do katalogu, w którym zostanie zapisany wygenerowany raport Markdown
    #[arg(short = 'o', long = "out-dir")]
    pub out_dir: Option<String>,

    /// Tytuł raportu Markdown
    #[arg(short = 'u', long = "title", default_value = "CODE SNAPSHOT")]
    pub title: String,

    /// Maksymalny rozmiar pojedynczego pliku tekstowego (w bajtach) dołączanego do raportu
    #[arg(long = "max-file-size", default_value_t = 524288)]
    pub max_file_size: u64,
}