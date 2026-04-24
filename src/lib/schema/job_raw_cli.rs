use std::collections::BTreeSet;

use clap::{Args, Parser};

use super::SharedJobAttributeToSelect;
// ============================================================================
// GŁÓWNY PUNKT WEJŚCIA DLA WTYCZKI CARGO
// ============================================================================

#[derive(Parser, Debug, Clone)]
#[command(name = "cargo", bin_name = "cargo", version, about, long_about = None)]
pub enum CargoCliRoot {
	#[command(name = "plot", about = "Szwajcarski scyzoryk do wizualizacji struktury projektu", version)]
	Plot(RawCliJob),
}

// ============================================================================
// STRUKTURA ARGUMENTÓW CLI
// ============================================================================

#[derive(Args, Debug, Clone)]
// ⚡ TWARDA GRUPA: Wymusza użycie dokładnie jednej z tych trzech flag na start.
#[command(group(
    clap::ArgGroup::new("exec_mode")
        .required(true)
        .args(["job_blank", "jobs_load", "help_for_pattern", "help_for_config", "jobs_load_only_show"])
))]
pub struct RawCliJob {
	// ========================================================================
	// INFORMACJE I POMOC (Exclusive - blokują inne flagi)
	// ========================================================================
	/// Wyświetla szczegółową pomoc dotyczącą składni wzorców (Patterns)
	#[arg(help_heading = "Information", short = 'P', long = "help-pattern")]
	pub help_for_pattern: bool,

	/// Wyświetla pomoc dotyczącą struktury pliku konfiguracyjnego TOML
	#[arg(help_heading = "Information", short = 'T', long = "help-tasks-make")]
	pub help_for_config: bool,
	
	/// Załadowywuje plik z konfiguracją zadania i tylko wyświetla listę zadań, nic nie robi więcej.
	#[arg(
        help_heading = "Information",
        short = 'L',
        long = "help-tasks-show",
        num_args = 0..=1,
        default_missing_value = ""
    )]
	pub jobs_load_only_show: Option<String>,

	
	// ========================================================================
	// RESZTA OPCJI (Działają tylko logicznie w trybie -b, zgodnie z Twoim kodem)
	// ========================================================================
	/// Uruchom w trybie "suchy bieg" (dry run)
	#[arg(help_heading = "Run Options",  short = 'U', long = "dry-run")]
	pub dry_run: Option<bool>,

	/// Nie zapisuj wyników na dysku
	#[arg(help_heading = "Run Options",  short = 'N',long = "no-export", visible_aliases = ["no-save", "only-view"])]
	pub export_nothing: Option<bool>,

	/// Całkowita cisza - nie drukuje absolutnie niczego na ekranie (nawet błędów)
	#[arg(help_heading = "Run Options", short = 'O', long = "clear")]
	pub print_nothing: Option<bool>,

	/// Pokaż tylko ostrzeżenia
	#[arg(help_heading = "Run Options", short = 'W', long = "only-warnings")]
	pub print_warnings: Option<bool>,

	/// Zapisuje dodatkowe informacjie
	#[arg(help_heading = "Run Options",  short = 'E', long = "inspection-in-export")]
	pub inspection_in_export: Option<bool>,

	/// Wyświetla dodatkowe informacje
	#[arg(help_heading = "Run Options",  short = 'I', long = "inspection-in-print")]
	pub inspection_in_print: Option<bool>,

	// ░░░░░░░░░░ PODSTAWY (Sterowanie potokiem wg diagramu) ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░


	/// Załadowywuje plik z konfiguracją zadania w formacie TOML. Wymaga podania ścieżki lub użyje domyślnej po samej fladze.
	#[arg(
        help_heading = "Main Options",
        short = 'l',
        long = "jobs-load",
        num_args = 0..=1,
        default_missing_value = ""
    )]
	pub jobs_load: Option<String>,

	/// Wybiera zadanie lub zadania do obsłużenia z pliku TOML. Opcjonalne, ma zastosowanie TYLKO przy użyciu flagi -l.
	#[arg(
        help_heading = "Main Options",
        short = 't',
        long = "job-ids",
        value_parser = parse_ids, // ⚡ Podpięto parser
        requires = "jobs_load" // ⚡ Twarde wymuszenie Clap: nie zadziała bez -l
    )]
	pub task_id: Option<BTreeSet<String>>,

	/// Pozwala utworzyć nowe czyste zapytanie z domyślnymi ustawieniami, zamiast wczytywać z configu lub pliku TOML.
	#[arg(help_heading = "Main Options", short = 'b', long = "job-blank")]
	pub job_blank: bool,

	/// Uruchamia zadanie w trybie przyspieszonym (tylko bazowe parametry, optymalna ścieżka wykonania)
	#[arg(help_heading = "Main Options", short = 's', long = "speed", requires = "job_blank")]
	pub speed: bool,

	// ░░░░░░░░░░ EXPLORER ░░░░░░░░░░
	/// Ścieżka katalogu do skanowania (domyślnie obecny katalog)
	#[arg(help_heading = "Scan Options", short = 'd', long = "dir-to-scan", default_value = ".")]
	pub dir_to_scan: String,

	/// Wzorce wyszukiwania (np. \"src/**/*.rs\", \"!target/**\")
	#[arg(help_heading = "Scan Options", short = 'p', long = "pattern", visible_aliases = ["pat", "patterns"], value_name = "GLOB")]
	pub patterns: Vec<String>,

	/// Części zapytania [MF, MD, XF, XD] (np. --parts md mf)
	/// lub pełna nazwa [matched-files, matched-dirs, mismatched-files, mismatched-dirs]
	#[arg(help_heading = "Scan Options", short = 'q', long = "query-parts", default_value = "mf, md")]
	pub query_parts: Vec<String>,

	/// Ignoruj wielkość liter przy dopasowywaniu wzorców
	#[arg(help_heading = "Scan Options", short = 'i', long = "ignore-case")]
	pub ignore_case: Option<bool>,

	// ░░░░░░░░░░ EXPORT ░░░░░░░░░░

	// wymagane jęsli save włączone
	#[arg(
		help_heading = "Save Options",
		short = 'o',
		long = "out-dir",
		default_value = "./target/.cargo-plot/out/"
	)]
	pub out_dir: String,

	#[arg(help_heading = "Save Options", short = 'u', long, help = "Tytuł raportu Markdown")]
	pub title: Option<String>,

	#[arg(help_heading = "Save Options", short = 'n', long, help = "Nazwa pliku wyjściowego")]
	pub name: Option<String>,

	#[arg(help_heading = "Save Options", short = 'X', long, help = "Czy nazwa ma być prefiksem zamiast sufiksu?")]
	pub name_is_first: Option<bool>,

	#[arg(help_heading = "Save Options", long, help = "Zapisz SOTC i COTS jako osobne pliki")]
	pub save_separately: Option<bool>,

	// ░░░░░░░░░░ THEME / LAYOUT ░░░░░░░░░░
	
	/// kolumna item jest obowiązkowa, jeśli brak ? - dodawan jest na końcu
	/// Kolumny do wyświetlenia (oddzielone przecinkami: date,time,size,path,item)
	#[arg(
        short = 'c',
        long = "columns",
        value_parser = parse_attribute, // ⚡ Podpięto parser
        value_delimiter = ',',
        default_value = "date,time,size,path,item",
        help_heading = "Layout Options"
    )]
	pub attributes: Option<Vec<SharedJobAttributeToSelect>>,

	#[arg(help_heading = "Theme Options", long = "show-colors")]
	pub show_colors: Option<bool>,

	#[arg(help_heading = "Theme Options", long = "tree-hide")]
	pub tree_hide: Option<bool>,

	#[arg(help_heading = "Layout Options", long = "icons-hide", conflicts_with = "icon_more")]
	pub icon_hide: Option<bool>,

	#[arg(help_heading = "Layout Options", long = "name-hide")]
	pub name_hide: Option<bool>,

	#[arg(help_heading = "Layout Options", long = "align-hide")]
	pub align_hide: Option<bool>,

	#[arg(help_heading = "Theme Options", long = "icons-more", conflicts_with = "icon_hide")]
	pub icon_more: Option<bool>,

	#[arg(help_heading = "Theme Options", long = "num-before")]
	pub num_before: Option<bool>,

	#[arg(help_heading = "Theme Options", short ='B', long = "size-binary")]
	pub size_binary: Option<bool>,

	#[arg(help_heading = "Theme Options", short ='Y', long = "fmt-date")]
	pub fmt_date: Option<String>,

	#[arg(help_heading = "Theme Options", short ='H', long = "fmt-time")]
	pub fmt_time: Option<String>,


	// ░░░░░░░░░░ PILE OPTIONS ░░░░░░░░░░
	/// Typ grupowania (Pile). Dostępne opcje: name, exte, none.
	#[arg(help_heading = "Pile Options", long = "pile")]
	pub pile: Option<String>,

	/// Jeśli włączono grupowanie, katalogi zawsze wyświetlają się jako pierwsze.
	#[arg(
        help_heading = "Pile Options",
        long = "dir-first",
		short = 'D',
        requires = "pile", // ⚡ Wymaga flagi --pile
    )]
	pub dir_first: Option<bool>,

	/// Jeśli włączono grupowanie (name), zbiera pliki o tej samej nazwie obok katalogu nadrzędnego.
	#[arg(
        help_heading = "Pile Options",
		short = 'G',
        long = "name-nearby",
        requires = "pile" // ⚡ Wymaga flagi --pile
    )]
	pub name_nearby: Option<bool>,

	// ░░░░░░░░░░ SORT OPTIONS ░░░░░░░░░░
	/// Kryterium sortowania: date, size, path, name, none.
	#[arg(help_heading = "Sort Options", long = "sort", value_parser = ["date", "size", "path", "name", "none"])]
	pub sort: Option<String>,

	/// Odwróć kolejność sortowania (Descending). Wymaga --sort.
	#[arg(help_heading = "Sort Options", short = 'R', long = "reverse", requires = "sort")]
	pub reverse: Option<bool>,

	/// Odbicie lustrzane sortowania tekstowego. Wymaga --sort (path/name).
	#[arg(help_heading = "Sort Options", short = 'M', long = "mirror", requires = "sort")]
	pub mirror: Option<bool>,

	/// Strategia sortowania (Spec, Num, AZaz). Wymaga --sort (path/name).
	#[arg(
        help_heading = "Sort Options",
        short = 'S',
        long = "strategy",
        requires = "sort",
        value_parser = parse_strategy // ⚡ Walidacja 24 strategii
    )]
	pub strategy: Option<String>,
}

/// Customowy parser dla kolumn atrybutów, wspierający przyjazne dla użytkownika nazwy.
fn parse_attribute(s: &str) -> Result<SharedJobAttributeToSelect, String> {
	match s.to_lowercase().trim() {
		"date" => Ok(SharedJobAttributeToSelect::Date),
		"time" => Ok(SharedJobAttributeToSelect::Time),
		"size" => Ok(SharedJobAttributeToSelect::Size),
		"path" => Ok(SharedJobAttributeToSelect::Path),
		"item" | "treelist" => Ok(SharedJobAttributeToSelect::Item), // Zgodnie ze starą notacją wszystko to jest de facto elementem Item
		_ => Err(format!("Nieznana kolumna: '{}'. Dostępne to: date, time, size, path, item", s)),
	}
}

/// Walidator strategii sortowania - dopuszcza tylko 24 predefiniowane kombinacje.
fn parse_strategy(s: &str) -> Result<String, String> {
	let valid_strategies = [
		"Spec,Num,AZaz",
		"Spec,Num,azAZ",
		"Spec,Num,aAzZ",
		"Spec,Num,AaZz",
		"Num,Spec,AZaz",
		"Num,Spec,azAZ",
		"Num,Spec,aAzZ",
		"Num,Spec,AaZz",
		"Spec,AZaz,Num",
		"Spec,azAZ,Num",
		"Spec,aAzZ,Num",
		"Spec,AaZz,Num",
		"Num,AZaz,Spec",
		"Num,azAZ,Spec",
		"Num,aAzZ,Spec",
		"Num,AaZz,Spec",
		"AZaz,Num,Spec",
		"azAZ,Num,Spec",
		"aAzZ,Num,Spec",
		"AaZz,Num,Spec",
		"AZaz,Spec,Num",
		"azAZ,Spec,Num",
		"aAzZ,Spec,Num",
		"AaZz,Spec,Num",
	];

	if valid_strategies.contains(&s) {
		Ok(s.to_string())
	} else {
		Err(format!("Nieprawidłowa strategia: '{}'. Przykład poprawnej: 'Spec,Num,AaZz'", s))
	}
}

/// Customowy parser dla identyfikatorów zadań (wspiera format "x1,q2")
fn parse_ids(s: &str) -> Result<BTreeSet<String>, String> {
	let ids: BTreeSet<String> = s.split(',').map(|id| id.trim().to_string()).filter(|id| !id.is_empty()).collect();

	if ids.is_empty() {
		Err("Nie podano prawidłowych identyfikatorów zadań. Użyj formatu np. 'x1,q2'".to_string())
	} else {
		Ok(ids)
	}
}
