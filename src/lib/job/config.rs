use std::path::PathBuf;

use chrono;

use crate::lib::{
	job::{self},
	logic::{self},
};

pub const IS_DEBUG: bool = true; // ⚡ GLOBALNA FLAGA DEBUGOWANIA
// ============================================================================
// OSTATECZNY WIERSZ DANYCH (Zhydratowany)
// ============================================================================

/// Reprezentuje finalny wiersz gotowy do tabeli, zawierający wszystkie
/// zgromadzone metadane z dysku i z etapu skanowania.
#[derive(Debug, Clone)]
pub struct ValidResultMainRow {
	pub dt_modified: chrono::DateTime<chrono::Local>,
	pub name_with_ext: String,
	pub size_real: u64,
	pub node: logic::ScannedNode,
}

#[derive(Debug, Clone)]
pub struct ValidResultMainTab {
	pub rows: Vec<ValidResultMainRow>,
	pub tier_max: usize,
	pub name_len_max: usize,
	pub path_len_max: usize,
}
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
/*
 * 🧠 KAŻDY `Valid<NAZWA>Parse` posiada parę `Valid<NAZWA>Config` i odwrotnie.
 *
 * 🧠 KAŻDY `Valid<NAZWA>Parse` ma metodę `parse_vec_as_config` - automatyczne wywołanie bliźniaka
 *
 * 🧠 KAŻDY `Valid<NAZWA>Config` ma metodę `get` - pobiera znormalizowaną konfiguracje
 *
 * 🧠 DODATKI DLA `Valid<NAZWA><Config|Parse>` mają nazwę `<ZASTOSOWANIE>ForValid<NAZWA>`
 *
 * 🧠 CELEM NINIESZJEGO PLIKU JEST ZAPEWNIANIE PRAWIDŁOWEJ KONFIGURACJI,
 *          CIĘŻKA LOGIKA POWINNA BYĆ W `Crate::lib::logic::*`;
 * 	        W ETAPACH KOLEJNYCH W `Crate::lib::job::engine_step*` W GŁÓWNEJ FUNKCJI - ARGUMENTY SĄ
 *          ZAWSZE TYPU `&job::Valid<NAZWA>Config`
 *
 *
 * ░░░░░░░░░░░░░░░░░░░░░░░░░░░ INDEX ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
 *
 * pub struct ValidResultMainRow
 * pub struct ValidResultMainTab
 * //0
 * pub struct PreparedJob
 * //1
 * pub enum ValidExecutionParse
 * pub struct ValidExecutionConfig
 * //2
 * pub enum ValidSaveAsParse
 * pub struct ValidSaveAsConfig
 * //3
 * pub enum ValidTablePartParse
 * pub struct ValidTablePartConfig
 * //4
 * pub enum ValidColumnItemParse
 * pub struct ValidColumnItemConfig
 * pub enum ModeListForValidColumnItem
 * pub enum ModeIconsForValidColumnItem
 * //5
 * pub enum ValidTableColumnsParse
 * pub struct ValidTableColumnsConfig
 * //6
 * pub enum ValidColumnSizeParse
 * pub struct ValidColumnSizeConfig
 * //7 i 8
 * pub enum ValidColumnDateParse
 * pub enum ValidColumnTimeParse
 * pub struct ValidColumnDateConfig
 * pub struct ValidColumnTimeConfig
 * //9
 * pub struct ValidPatternConfig
 * pub struct ValidPatternParse
 * //10
 * pub struct ValidWorkspaceConfig
 * pub struct ValidWorkspaceParse
 * //11
 * pub struct ValidSortByParse //brakuje enumu
 * pub struct ValidSortByConfig
 * pub enum ModeFileGroupForValidSortBy
 * pub enum StrategyForValidSortBy
 *
 *  */

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// MODEL ZBIORCZY
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone)]
pub struct ValidPreparedJobConfig {
	pub exec: ValidExecutionConfig,
	pub save: ValidSaveAsConfig,
	pub part: ValidTablePartConfig,
	pub item: ValidColumnItemConfig,
	pub cols: ValidTableColumnsConfig,
	pub size: ValidColumnSizeConfig,
	pub date: ValidColumnDateConfig,
	pub time: ValidColumnTimeConfig,
	pub patt: ValidPatternConfig,
	pub work: ValidWorkspaceConfig,
	pub sort: ValidSortByConfig,
}
impl Default for ValidPreparedJobConfig {
	fn default() -> Self {
		let blank: Vec<&str> = Vec::new();

		Self {
			// 0. Execution Config (Domyślnie: Debug: true, SaveAs: true, CliColor: true)
			exec: ValidExecutionParse::parse_vec_as_config(&blank),
			// 1. Workspace (Domyślnie ".")
			work: ValidWorkspaceParse::parse_vec_as_config(["."]),
			// 2. Patterns (Domyślnie zbiór filtrów zdefiniowany w config.rs)
			patt: ValidPatternParse::parse_vec_as_config(&blank),
			// 3. Table Parts (Domyślnie md + mf)
			part: ValidTablePartParse::parse_vec_as_config(&blank),
			// 4. Column Item (Domyślnie Tree, IconsLite, Name: false, Align: false)
			item: ValidColumnItemParse::parse_vec_as_config(["list-tree", "name-show", "ws-show", "icons-lite"]),
			// 5. Table Columns (Domyślnie Date, Time, Size, Item, Path)
			cols: ValidTableColumnsParse::parse_vec_as_config(&blank),
			// 6. Column Date (Domyślnie "%Y W%V %u-%a")
			date: ValidColumnDateParse::parse_vec_as_config(&blank),
			// 7. Column Time (Domyślnie "%H:%M:%S.%3f")
			time: ValidColumnTimeParse::parse_vec_as_config(&blank),
			// 8. Column Size (Domyślnie Decimal / SI)
			size: ValidColumnSizeParse::parse_vec_as_config(&blank),
			// 9. Sort By (Domyślnie Name, [Spec][Num][AZaz], Reverse: false)
			sort: ValidSortByParse::parse_vec_as_config(&blank),
			// 10. Save As (Domyślnie "./target/.cargo-plot/", "Project Snapshot", SOTC/COTS names)
			save: ValidSaveAsParse::parse_vec_as_config(&blank),
		}
	}
}
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// MODELE PODSTAWOWE
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidExecutionParse {
	Quiet(bool),
	Mute(bool),
	OnlyDry(bool),
	Debug(bool),
	SaveAs(bool),
	CliColor(bool),
}
#[derive(Debug, Clone)]
pub struct ValidExecutionConfig {
	pub quiet: bool,
	pub mute: bool,
	pub only_dry: bool,
	pub debug: bool,
	pub save_as: bool,
	pub cli_color: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidSaveAsParse {
	OutDir(String),
	Title(String),
	Name(String),
	NameIsPrefix(bool),
}
#[derive(Debug, Clone)]
pub struct ValidSaveAsConfig {
	pub raw: KeepRawForValiSaveAs,
	pub out_dir: PathBuf,
	pub title_index_sotc: String,
	pub name_index_sotc: String,
	pub title_files_cots: String,
	pub name_files_cots: String,
}
#[derive(Debug, Clone)]
pub enum ValidTablePartParse {
	MD,
	MF,
	XD,
	XF,
}
#[derive(Debug, Clone)]
pub struct ValidTablePartConfig {
	pub md: bool,
	pub mf: bool,
	pub xd: bool,
	pub xf: bool,
}
#[derive(Debug, Clone)]
pub enum ValidColumnItemParse {
	ListNone,
	ListTree,
	ListFlat,
	IconsNone,
	IconsLite,
	IconsMore,

	NumPrefix,
	NumSuffix,
	NameNone,
	NameShow,
	WhitespaceTrailNone,
	WhitespaceTrailShow,
}
#[derive(Debug, Clone)]
pub struct ValidColumnItemConfig {
	pub list: ModeListForValidColumnItem,
	pub icons: ModeIconsForValidColumnItem,
	pub name: bool,
	pub align_end: bool,
	pub num_is_first: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidTableColumnsParse {
	Date,
	Time,
	Size,
	Item,
	Path,
}
#[derive(Debug, Clone)]
pub struct ValidTableColumnsConfig {
	pub columns: Vec<ValidTableColumnsParse>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnSizeParse {
	Decimal, // System SI (podstawa 1000)
	Binary,  // System IEC (podstawa 1024)
}
#[derive(Debug, Clone)]
pub struct ValidColumnSizeConfig {
	pub mode: ValidColumnSizeParse,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnDateParse {
	Default,
}
#[derive(Debug, Clone)]
pub struct ValidColumnDateConfig {
	pub format: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnTimeParse {
	Default,
}
#[derive(Debug, Clone)]
pub struct ValidColumnTimeConfig {
	pub format: String,
}
#[derive(Debug, Clone)]
pub struct ValidPatternParse;
#[derive(Debug, Clone)]
pub struct ValidPatternConfig {
	pub patterns: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct ValidWorkspaceParse;
#[derive(Debug, Clone)]
pub struct ValidWorkspaceConfig {
	pub workspace_raw: String,
	pub workspace_dir: PathBuf,
	pub execution_dir: PathBuf,
	pub ignore_case: bool,
}
#[derive(Debug, Clone)]
pub struct ValidSortByParse;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeFileGroupForValidSortBy {
	Name,
	Exte,
	None,
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// MODELE POMOCNICZE
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeListForValidColumnItem {
	None,
	Flat,
	Tree,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeIconsForValidColumnItem {
	Lite,
	More,
	None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeepRawForValiSaveAs {
	pub path: String,
	pub title: String,
	pub name: String,
	pub prefix: bool,
}
#[derive(Debug, Clone)]
pub enum StrategyForValidSortBy {
	Date { reverse: bool },
	Size { reverse: bool },
	Name { mode: String, reverse: bool, file_group: ModeFileGroupForValidSortBy },
	Path { mode: String, reverse: bool, dir_split: bool, file_group: ModeFileGroupForValidSortBy },
}
#[derive(Debug, Clone)]
pub struct ValidSortByConfig {
	pub strategy: StrategyForValidSortBy,
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ODCZYTYWANIE USTAWIEŃ Z FLAG
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

pub trait ParseFromFlags: Sized {
	/// Typ konfiguracji, który zostanie zwrócony
	type Config;

	/// Przetwarza wektor stringów (flag) na zwalidowaną strukturę konfiguracyjną
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>;
	/// Parsuje pojedynczego stringa (flagę CLI) na wariant Enuma.
	/// Zwraca Result, aby umożliwić obsługę lub zignorowanie nieznanych flag.
	fn parse(s: &str) -> Result<Self, String>;
}
impl ParseFromFlags for ValidExecutionParse {
	type Config = ValidExecutionConfig;

	fn parse(s: &str) -> Result<Self, String> {
		let lower = s.trim().to_lowercase();
		match lower.as_str() {
			"quiet" | "q" => Ok(ValidExecutionParse::Quiet(true)),
			"mute" | "cli-off" | "silent" => Ok(ValidExecutionParse::Mute(true)),
			"dry" | "dry-run" => Ok(ValidExecutionParse::OnlyDry(true)),
			"debug" | "verbose" => Ok(ValidExecutionParse::Debug(true)),
			"save" | "save-as" => Ok(ValidExecutionParse::SaveAs(true)),
			"color" | "colors" => Ok(ValidExecutionParse::CliColor(true)),
			_ => Err(format!("Nieznany ValidExecutionParse: '{}'", s)),
		}
	}

	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		// ⚡ USTAWIENIA DOMYŚLNE ZGODNIE Z TWOIM POLECENIEM
		let mut quiet = false;
		let mut mute = false;
		let mut only_dry = false;
		let mut debug = true;
		let mut save_as = true;
		let mut cli_color = true;

		for item in inputs {
			if let Ok(parsed) = Self::parse(item.as_ref()) {
				match parsed {
					ValidExecutionParse::Quiet(v) => quiet = v,
					ValidExecutionParse::Mute(v) => mute = v,
					ValidExecutionParse::OnlyDry(v) => only_dry = v,
					ValidExecutionParse::Debug(v) => debug = v,
					ValidExecutionParse::SaveAs(v) => save_as = v,
					ValidExecutionParse::CliColor(v) => cli_color = v,
				}
			}
		}
		if mute {
			quiet = true;
		}
		ValidExecutionConfig { quiet, mute, only_dry, debug, save_as, cli_color }
	}
}
impl ParseFromFlags for ValidSaveAsParse {
	type Config = ValidSaveAsConfig;
	fn parse(s: &str) -> Result<Self, String> {
		let lower = s.trim().to_lowercase();

		if lower.starts_with("outdir=") || lower.starts_with("out_dir=") {
			let val = s.split_once('=').unwrap().1.trim();
			Ok(ValidSaveAsParse::OutDir(val.to_string()))
		} else if lower.starts_with("title=") {
			let val = s.split_once('=').unwrap().1.trim();
			Ok(ValidSaveAsParse::Title(val.to_string()))
		} else if lower.starts_with("name=") {
			let val = s.split_once('=').unwrap().1.trim();
			Ok(ValidSaveAsParse::Name(val.to_string()))
		} else if lower == "name-prefix" || lower == "name_prefix" {
			Ok(ValidSaveAsParse::NameIsPrefix(true))
		} else if lower == "name-suffix" || lower == "name_suffix" {
			Ok(ValidSaveAsParse::NameIsPrefix(false))
		} else {
			Err(format!("Nieznany ValidSaveAsParse: '{}'", s.trim()))
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		// 1. Zbieranie surowych wartości
		let mut out_dir_str = "./target/.cargo-plot/".to_string();
		let mut title = "Project Snapshot".to_string();
		let mut name = "".to_string();
		let mut name_is_prefix = false;

		for item in inputs {
			if let Ok(parsed) = Self::parse(item.as_ref()) {
				match parsed {
					ValidSaveAsParse::OutDir(v) => out_dir_str = v,
					ValidSaveAsParse::Title(v) => title = v,
					ValidSaveAsParse::Name(v) => name = v,
					ValidSaveAsParse::NameIsPrefix(v) => name_is_prefix = v,
				}
			}
		}

		// ⚡ Pakujemy surowe wartości do nowej struktury
		let raw = KeepRawForValiSaveAs {
			path: out_dir_str.clone(),
			title: title.clone(),
			name: name.clone(),
			prefix: name_is_prefix,
		};

		// ⚡ Tworzymy PathBuf od razu
		let out_dir = std::path::PathBuf::from(&out_dir_str);

		// 2. Pobieramy znacznik czasu JEDEN RAZ, żeby pliki były w pełni zsynchronizowane
		let time_tag = crate::lib::logic::tag_time().0;

		// 3. Generowanie tytułów
		let title_index_sotc =
			format!("# {}: {} (v:{}{}OT{} [{} of the {}])", "INDEX", title, time_tag, "S", "C", "Structure", "content");
		let title_files_cots =
			format!("# {}: {} (v:{}{}OT{} [{} of the {}])", "FILES", title, time_tag, "C", "S", "Content", "structure");

		// 4. Generowanie nazw plików (z zabezpieczeniem przed "wiszącym" podkreślnikiem gdy name to "")
		let name_index_sotc = if name.is_empty() {
			format!("{}{}.md", time_tag, "SOTC-INDEX")
		} else if name_is_prefix {
			format!("{}_{}{}.md", name, time_tag, "SOTC-INDEX")
		} else {
			format!("{}{}_{}.md", time_tag, "SOTC-INDEX", name)
		};

		let name_files_cots = if name.is_empty() {
			format!("{}{}.md", time_tag, "COTS-FILES")
		} else if name_is_prefix {
			format!("{}_{}{}.md", name, time_tag, "COTS-FILES")
		} else {
			format!("{}{}_{}.md", time_tag, "COTS-FILES", name)
		};

		// 5. Zwrócenie gotowej struktury
		ValidSaveAsConfig { raw, out_dir, title_index_sotc, title_files_cots, name_index_sotc, name_files_cots }
	}
}
impl ParseFromFlags for ValidTablePartParse {
	type Config = ValidTablePartConfig;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"MD" | "md" | "match-dirs" => Ok(ValidTablePartParse::MD),
			"MF" | "mf" | "match-files" => Ok(ValidTablePartParse::MF),
			"XD" | "xd" | "mismatch-dirs" => Ok(ValidTablePartParse::XD),
			"XF" | "xf" | "mismatch-files" => Ok(ValidTablePartParse::XF),
			_ => Err(format!("Nieprawidłowa nazwa: '{}'. Dostępne: md, mf, xd, xf", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let mut md = false;
		let mut mf = false;
		let mut xd = false;
		let mut xf = false;

		let mut has_any_valid = false;

		for item in inputs {
			// Cicho ignorujemy błędy parsowania (zgodnie z założeniem o niepoprawnych elementach)
			if let Ok(parsed) = Self::parse(item.as_ref()) {
				has_any_valid = true;
				match parsed {
					ValidTablePartParse::MD => md = true,
					ValidTablePartParse::MF => mf = true,
					ValidTablePartParse::XD => xd = true,
					ValidTablePartParse::XF => xf = true,
				}
			}
		}

		// Jeśli tablica była pusta lub wszystko było śmieciami, ładujemy domyślne ustawienia
		if !has_any_valid {
			md = true;
			mf = true;
		}

		ValidTablePartConfig { md, mf, xd, xf }
	}
}
impl ParseFromFlags for ValidColumnItemParse {
	type Config = ValidColumnItemConfig;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"list-none" => Ok(ValidColumnItemParse::ListNone),
			"list-flat" => Ok(ValidColumnItemParse::ListFlat),
			"list-tree" => Ok(ValidColumnItemParse::ListTree),

			"icons-lite" => Ok(ValidColumnItemParse::IconsLite),
			"icons-more" => Ok(ValidColumnItemParse::IconsMore),
			"icons-none" => Ok(ValidColumnItemParse::IconsNone),

			"num-prefix" => Ok(ValidColumnItemParse::NumPrefix),
			"num-suffix" => Ok(ValidColumnItemParse::NumSuffix),

			"name-none" => Ok(ValidColumnItemParse::NameNone),
			"name-show" => Ok(ValidColumnItemParse::NameShow),

			"ws-none" | "whitespace-none" => Ok(ValidColumnItemParse::WhitespaceTrailNone),
			"ws-show" | "whitespace-show" => Ok(ValidColumnItemParse::WhitespaceTrailShow),

			_ => Err(format!("Nieznany ValidColumnItemParse: '{}'", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let items: Vec<ValidColumnItemParse> =
			inputs.into_iter().filter_map(|s| Self::parse(s.as_ref()).ok()).collect();

		ValidColumnItemConfig::get(items)
	}
}
impl ParseFromFlags for ValidTableColumnsParse {
	type Config = ValidTableColumnsConfig;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"date" => Ok(ValidTableColumnsParse::Date),
			"time" => Ok(ValidTableColumnsParse::Time),
			"size" => Ok(ValidTableColumnsParse::Size),
			"item" => Ok(ValidTableColumnsParse::Item),
			"path" => Ok(ValidTableColumnsParse::Path),
			_ => Err(format!("Nieprawidłowa nazwa kolumny: '{}'. Dostępne: date, time, size, item, path", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let mut columns = Vec::new();

		let mut has_date = false;
		let mut has_time = false;
		let mut has_size = false;
		let mut has_item = false;
		let mut has_path = false;

		let mut has_any_valid = false;

		for item in inputs {
			if let Ok(parsed) = Self::parse(item.as_ref()) {
				has_any_valid = true;
				match parsed {
					ValidTableColumnsParse::Date => {
						if !has_date {
							columns.push(ValidTableColumnsParse::Date);
							has_date = true;
						}
					}
					ValidTableColumnsParse::Time => {
						if !has_time {
							columns.push(ValidTableColumnsParse::Time);
							has_time = true;
						}
					}
					ValidTableColumnsParse::Size => {
						if !has_size {
							columns.push(ValidTableColumnsParse::Size);
							has_size = true;
						}
					}
					ValidTableColumnsParse::Item => {
						if !has_item {
							columns.push(ValidTableColumnsParse::Item);
							has_item = true;
						}
					}
					ValidTableColumnsParse::Path => {
						if !has_path {
							columns.push(ValidTableColumnsParse::Path);
							has_path = true;
						}
					}
				}
			}
		}

		// Jeśli wejście było puste lub same błędy -> wczytujemy konfigurację domyślną
		if !has_any_valid {
			return ValidTableColumnsConfig {
				columns: vec![
					ValidTableColumnsParse::Date,
					ValidTableColumnsParse::Time,
					ValidTableColumnsParse::Size,
					ValidTableColumnsParse::Item,
					ValidTableColumnsParse::Path,
				],
			};
		}

		// Jeśli podano cokolwiek poprawnego, ale brakuje 'Item', doklejamy na koniec
		if !has_item {
			columns.push(ValidTableColumnsParse::Item);
		}

		ValidTableColumnsConfig { columns }
	}
}
impl ParseFromFlags for ValidColumnSizeParse {
	type Config = ValidColumnSizeConfig;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"dec" | "decimal" => Ok(ValidColumnSizeParse::Decimal),
			"bin" | "binary" => Ok(ValidColumnSizeParse::Binary),
			_ => Err(format!("Nieznany system miar: '{}'. Dostępne: dec, bin", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let mut mode = ValidColumnSizeParse::Decimal; // Domyślnie Dec

		for item in inputs {
			if let Ok(parsed) = Self::parse(item.as_ref()) {
				mode = parsed;
				break; // Przyjmujemy pierwszy poprawny wpis
			}
		}

		ValidColumnSizeConfig { mode }
	}
}
impl ParseFromFlags for ValidColumnDateParse {
	type Config = ValidColumnDateConfig;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"date-default" | "default" => Ok(ValidColumnDateParse::Default),
			_ => Err(format!("Nieznany format daty: '{}'", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let format = "%Y W%V %u-%a".to_string(); // Domyślny format: Rok Tydzień Dzień(1-7) SkrótDnia

		for item in inputs {
			if let Ok(_) = Self::parse(item.as_ref()) {
				// Tutaj można dodać logikę wyboru formatu, jeśli pojawi się więcej wariantów w enum
				break;
			}
		}

		ValidColumnDateConfig { format }
	}
}
impl ParseFromFlags for ValidColumnTimeParse {
	type Config = ValidColumnTimeConfig;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"time-default" | "default" => Ok(ValidColumnTimeParse::Default),
			_ => Err(format!("Nieznany format czasu: '{}'", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let format = "%H:%M:%S.%3f".to_string(); // Domyślny format: Godz:Min:Sek.Milisekundy

		for item in inputs {
			if let Ok(_) = Self::parse(item.as_ref()) {
				break;
			}
		}

		ValidColumnTimeConfig { format }
	}
}
impl ParseFromFlags for ValidPatternParse {
	type Config = ValidPatternConfig;
	fn parse(s: &str) -> Result<Self, String> { Err(format!("Typ nie wspiera parsowania pojedynczej flagi: '{}'", s)) }
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let patterns: Vec<String> = inputs.into_iter().map(|s| s.as_ref().to_string()).collect();

		// Jeśli lista jest pusta po zebraniu, ładujemy domyślne
		if patterns.is_empty() {
			return ValidPatternConfig {
				patterns: vec![
					"./{.rustfmt,Cargo,rust-toolchain,Makefile}.toml&/".to_string(),
					"./**/*.rs&/".to_string(),
					"!./target/**".to_string(),
					"!./.git/**".to_string(),
					"./.{gitattributes,gitignore}".to_string(),
					"./.github/workflows/*.yml&/".to_string(),
					"./.vscode/settings.json&/".to_string(),
					"./{API,ARCHITECTURE,AUTHORS,CHANGELOG,README,ROADMAP,TODO}.md".to_string(),
					"./dist/{**/*,*}.{bat,exe}&/".to_string(),
				],
				//ignore_case: ignore_case_sensitive.unwrap_or(false),
			};
		}

		ValidPatternConfig { patterns }
	}
}
impl ParseFromFlags for ValidWorkspaceParse {
	type Config = ValidWorkspaceConfig;
	fn parse(s: &str) -> Result<Self, String> { Err(format!("Typ nie wspiera parsowania pojedynczej flagi: '{}'", s)) }
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let mut workspace_raw = ".".to_string();
		let mut ignore_case = false; // ⚡ Domyślnie false
		let mut dir_set = false;

		for item in inputs {
			let s = item.as_ref().trim();
			if s.to_lowercase() == "ignore-case" {
				ignore_case = true;
			} else if !s.is_empty() && !dir_set {
				// Pierwszy niepusty string niebędący flagą uznajemy za ścieżkę
				workspace_raw = s.to_string();
				dir_set = true;
			}
		}
		let w = std::path::Path::new(&workspace_raw);
		let execution_dir: PathBuf = std::env::current_dir().expect("Nie można odczytać katalogu roboczego (CWD)");

		let workspace_dir: PathBuf = std::fs::canonicalize(w).unwrap_or_else(|x| {
			eprintln!("❌ Błąd lokalizacji workspace: {}", x);
			std::process::exit(1);
			//panic!("Nie można ustalić ścieżki '{}'", w.to_string_lossy());
		});

		ValidWorkspaceConfig { workspace_raw, workspace_dir, execution_dir, ignore_case }
	}
}
impl ParseFromFlags for ValidSortByParse {
	type Config = ValidSortByConfig;
	fn parse(s: &str) -> Result<Self, String> { Err(format!("Typ nie wspiera parsowania pojedynczej flagi: '{}'", s)) }
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		// 1. Ustawienia domyślne
		let mut strategy_name = "name";
		let mut reverse = false;
		let mut dir_split = true;
		let mut file_group = ModeFileGroupForValidSortBy::None;
		let mut mode_sort_name = "[Spec][Num][AZaz]".to_string();

		// 2. Płaskie parsowanie CLI/TOML (bez wchodzenia w detale algorytmów)
		for item in inputs {
			let s = item.as_ref().trim();
			let lower = s.to_lowercase();

			match lower.as_str() {
				"date" => strategy_name = "date",
				"size" => strategy_name = "size",
				"name" => strategy_name = "name",
				"path" => strategy_name = "path",

				"rev" | "reverse" => reverse = true,

				"group-name" => file_group = ModeFileGroupForValidSortBy::Name,
				"group-exte" => file_group = ModeFileGroupForValidSortBy::Exte,
				"group-none" => file_group = ModeFileGroupForValidSortBy::None,

				"dir-split-true" => dir_split = true,
				"dir-split-false" => dir_split = false,

				_ => {
					// Czyste przekazanie definicji maski (np. "[Num][Spec][azAZ]") do logiki
					if s.starts_with('[') && s.ends_with(']') {
						mode_sort_name = s.to_string();
					}
				}
			}
		}

		// 3. Budowa wariantu konfiguracji
		let strategy = match strategy_name {
			"date" => StrategyForValidSortBy::Date { reverse },
			"size" => StrategyForValidSortBy::Size { reverse },
			"path" => StrategyForValidSortBy::Path { mode: mode_sort_name, reverse, dir_split, file_group },
			_ => StrategyForValidSortBy::Name { mode: mode_sort_name, reverse, file_group },
		};

		ValidSortByConfig { strategy }
	}
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ZAPISYWANIE USTAWIEŃ DO FLAG
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

pub trait ConfigToFlags {
	/// Przekształca zwalidowaną konfigurację z powrotem na listę flag CLI
	fn to_flags(&self) -> Vec<String>;
}
impl ConfigToFlags for ValidExecutionConfig {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![];
		if self.quiet {
			f.push("quiet".to_string());
		}
		if self.mute {
			f.push("mute".to_string());
		}
		if self.only_dry {
			f.push("dry".to_string());
		}
		if self.debug {
			f.push("verbose".to_string());
		}
		if self.save_as {
			f.push("save".to_string());
		}
		if self.cli_color {
			f.push("color".to_string());
		}
		f
	}
}
impl ConfigToFlags for ValidSaveAsConfig {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![];
		f.push(format!("outdir={}", self.raw.path));
		f.push(format!("title={}", self.raw.title));

		if !self.raw.name.is_empty() {
			f.push(format!("name={}", self.raw.name));
		}
		if self.raw.prefix {
			f.push("name-prefix".to_string());
		} else {
			f.push("name-suffix".to_string());
		}
		f
	}
}
impl ConfigToFlags for ValidTablePartConfig {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![];
		if self.md {
			f.push("MD".to_string());
		}
		if self.mf {
			f.push("MF".to_string());
		}
		if self.xd {
			f.push("XD".to_string());
		}
		if self.xf {
			f.push("XF".to_string());
		}
		f
	}
}
impl ConfigToFlags for ValidColumnItemConfig {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![];
		match self.list {
			ModeListForValidColumnItem::Tree => f.push("list-tree".to_string()),
			ModeListForValidColumnItem::Flat => f.push("list-flat".to_string()),
			ModeListForValidColumnItem::None => f.push("list-none".to_string()),
		}
		match self.icons {
			ModeIconsForValidColumnItem::Lite => f.push("icons-lite".to_string()),
			ModeIconsForValidColumnItem::More => f.push("icons-more".to_string()),
			ModeIconsForValidColumnItem::None => f.push("icons-none".to_string()),
		}
		if self.name {
			f.push("name-show".to_string());
		} else {
			f.push("name-none".to_string());
		}
		if self.align_end {
			f.push("ws-show".to_string());
		} else {
			f.push("ws-none".to_string());
		}
		if self.num_is_first {
			f.push("num-prefix".to_string());
		} else {
			f.push("num-suffix".to_string());
		}
		f
	}
}
impl ConfigToFlags for ValidTableColumnsConfig {
	fn to_flags(&self) -> Vec<String> {
		self.columns
			.iter()
			.map(|c| {
				match c {
					ValidTableColumnsParse::Date => "date",
					ValidTableColumnsParse::Time => "time",
					ValidTableColumnsParse::Size => "size",
					ValidTableColumnsParse::Item => "item",
					ValidTableColumnsParse::Path => "path",
				}
				.to_string()
			})
			.collect()
	}
}
impl ConfigToFlags for ValidColumnSizeConfig {
	fn to_flags(&self) -> Vec<String> {
		match self.mode {
			ValidColumnSizeParse::Decimal => vec!["dec".to_string()],
			ValidColumnSizeParse::Binary => vec!["bin".to_string()],
		}
	}
}
impl ConfigToFlags for ValidColumnDateConfig {
	fn to_flags(&self) -> Vec<String> {
		// Ponieważ aktualnie wspieramy tylko tryb "default", zawsze to zwracamy.
		// Jeśli w przyszłości dodasz inne formaty, trzeba będzie tu dać `match`.
		vec!["default".to_string()]
	}
}
impl ConfigToFlags for ValidColumnTimeConfig {
	fn to_flags(&self) -> Vec<String> { vec!["default".to_string()] }
}
impl ConfigToFlags for ValidPatternConfig {
	fn to_flags(&self) -> Vec<String> { self.patterns.clone() }
}
impl ConfigToFlags for ValidWorkspaceConfig {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![self.workspace_raw.clone()];
		if self.ignore_case {
			f.push("ignore-case".to_string());
		}
		f
	}
}
impl ConfigToFlags for ValidSortByConfig {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![];
		match &self.strategy {
			StrategyForValidSortBy::Date { reverse } => {
				f.push("date".to_string());
				if *reverse {
					f.push("rev".to_string());
				}
			}
			StrategyForValidSortBy::Size { reverse } => {
				f.push("size".to_string());
				if *reverse {
					f.push("rev".to_string());
				}
			}
			StrategyForValidSortBy::Name { mode, reverse, file_group } => {
				f.push("name".to_string());
				f.push(mode.clone());
				if *reverse {
					f.push("rev".to_string());
				}
				match file_group {
					ModeFileGroupForValidSortBy::Name => f.push("group-name".to_string()),
					ModeFileGroupForValidSortBy::Exte => f.push("group-exte".to_string()),
					ModeFileGroupForValidSortBy::None => f.push("group-none".to_string()),
				}
			}
			StrategyForValidSortBy::Path { mode, reverse, dir_split, file_group } => {
				f.push("path".to_string());
				f.push(mode.clone());
				if *reverse {
					f.push("rev".to_string());
				}
				if *dir_split {
					f.push("dir-split-true".to_string());
				} else {
					f.push("dir-split-false".to_string());
				}
				match file_group {
					ModeFileGroupForValidSortBy::Name => f.push("group-name".to_string()),
					ModeFileGroupForValidSortBy::Exte => f.push("group-exte".to_string()),
					ModeFileGroupForValidSortBy::None => f.push("group-none".to_string()),
				}
			}
		}
		f
	}
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// POBIERANIE GOTOWYCH USTAWIEŃ
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
pub trait ConfigGetSetup {
	/// Typ zwracany przez metodę get (różny dla każdej struktury)
	type Output;
	/// Przekształca konfigurację w gotowy obiekt warstwy logicznej
	fn get(&self) -> Self::Output;
}
// impl ConfigGetSetup for ValidExecutionConfig {}
// impl ConfigGetSetup for ValidSaveAsConfig {}
// impl ConfigGetSetup for ValidTablePartConfig {}
// impl ConfigGetSetup for ValidColumnItemConfig {}
impl ValidColumnItemConfig {
	fn get(items: Vec<ValidColumnItemParse>) -> ValidColumnItemConfig {
		let mut list: Option<ModeListForValidColumnItem> = None;
		let mut icons: Option<ModeIconsForValidColumnItem> = None;
		let mut num_is_first: Option<bool> = None;
		let mut name: Option<bool> = None;
		let mut align_end: Option<bool> = None;

		let mut list_count = 0usize;
		let mut icons_count = 0usize;
		let mut num_count = 0usize;
		let mut name_count = 0usize;
		let mut ws_count = 0usize;

		for item in items {
			match item {
				// LIST
				ValidColumnItemParse::ListNone => {
					list = Some(ModeListForValidColumnItem::None);
					list_count += 1;
				}
				ValidColumnItemParse::ListFlat => {
					list = Some(ModeListForValidColumnItem::Flat);
					list_count += 1;
				}
				ValidColumnItemParse::ListTree => {
					list = Some(ModeListForValidColumnItem::Tree);
					list_count += 1;
				}

				// NUM
				ValidColumnItemParse::NumPrefix => {
					num_is_first = Some(true);
					num_count += 1;
				}
				ValidColumnItemParse::NumSuffix => {
					num_is_first = Some(false);
					num_count += 1;
				}

				// ICONS
				ValidColumnItemParse::IconsLite => {
					icons = Some(ModeIconsForValidColumnItem::Lite);
					icons_count += 1;
				}
				ValidColumnItemParse::IconsMore => {
					icons = Some(ModeIconsForValidColumnItem::More);
					icons_count += 1;
				}
				ValidColumnItemParse::IconsNone => {
					icons = Some(ModeIconsForValidColumnItem::None);
					icons_count += 1;
				}

				// NAME
				ValidColumnItemParse::NameNone => {
					name = Some(false);
					name_count += 1;
				}
				ValidColumnItemParse::NameShow => {
					name = Some(true);
					name_count += 1;
				}

				// WHITESPACE (ALIGN_END)
				ValidColumnItemParse::WhitespaceTrailNone => {
					align_end = Some(false);
					ws_count += 1;
				}
				ValidColumnItemParse::WhitespaceTrailShow => {
					align_end = Some(true);
					ws_count += 1;
				}
			}
		}

		Self {
			list: if list_count == 1 { list.unwrap() } else { ModeListForValidColumnItem::Tree },

			icons: if icons_count == 1 { icons.unwrap() } else { ModeIconsForValidColumnItem::Lite },

			num_is_first: if num_count == 1 { num_is_first.unwrap() } else { false },

			name: if name_count == 1 { name.unwrap() } else { false },

			align_end: if ws_count == 1 { align_end.unwrap() } else { false },
		}
	}
}
// impl ConfigGetSetup for ValidTableColumnsConfig {}
// impl ConfigGetSetup for ValidColumnSizeConfig {}
// impl ConfigGetSetup for ValidColumnDateConfig {}
// impl ConfigGetSetup for ValidColumnTimeConfig {}
// impl ConfigGetSetup for ValidPatternConfig {}
impl ValidPatternConfig {
	/// Zwraca gotowy obiekt logic::PatternsQueries używany przez skaner.
	/// ⚡ UWAGA: Przyjmuje teraz ignore_case z zewnątrz (z Workspace)!
	pub fn get(&self, ignore_case: bool) -> logic::PatternsQueries {
		let patterns_ref: Vec<&str> = self.patterns.iter().map(|s| s.as_str()).collect();
		logic::PatternsQueries::new(patterns_ref, ignore_case)
	}
}
// impl ConfigGetSetup for ValidWorkspaceConfig { }
impl ConfigGetSetup for ValidSortByConfig {
	type Output = logic::SortQueries;
	/// Zwraca gotowy obiekt z warstwy logic:: (np. logic::SortQueries),
	/// gdzie dopiero tam te proste stringi i bool'e zostaną przetworzone
	/// w potężny algorytm (wspomniane 24 warianty).
	fn get(&self) -> Self::Output { logic::SortQueries::new(self.clone()) }
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// PREPARED JOB (KONTENER NA WSZYSTKIE USTAWIENIA - STEP 0)
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

impl ValidPreparedJobConfig {
	/// Generuje plik TOML będący odwrotnością parsowania (eksportuje flagi)
	pub fn to_toml(&self, _id: &str, _name: Option<&str>, _description: Option<&str>) -> String {
		use std::fmt::Write; // Wymagane, aby writeln! działało na Stringu

		let i = "job";
		let mut out = String::from("# Cargo-Plot Configuration File\n\n");

		// TWOJE ZMIENNE TESTOWE (Przywrócone)
		let only_test_id = "default_job";
		let only_test_name = "snapshot-project";
		let only_test_description = "【ENG】 Standard Rust project snapshot / 【POL】 Standardowy zrzut projektu Rust";

		// Nagłówek zadania - {} dla braku cudzysłowów, {:?} dla wartości
		let _ = writeln!(out, "[[{}]]", i);
		let _ = writeln!(out, "id = {:?}", only_test_id);
		let _ = writeln!(out, "name = {:?}", only_test_name);
		let _ = writeln!(out, "description = {:?}", only_test_description);
		let _ = writeln!(out, "# [quiet, mute, dry, verbose, save, color]");
		let _ = writeln!(out, "run_mode = {:?}\n", self.exec.to_flags());

		// Sekcja SCAN
		let _ = writeln!(out, "[{}.explorer]", i);
		let _ = writeln!(out, "workspace = {:?}", self.work.workspace_raw.as_str());
		let _ = writeln!(out, "ignore_case = {}", self.work.ignore_case);
		let _ = writeln!(out, "patterns = {:?}", self.patt.patterns);
		let _ = writeln!(out, "# [MD,MF,XD,XF]");
		let _ = writeln!(out, "parts = {:?}\n", self.part.to_flags());

		// Sekcja TABLE
		let _ = writeln!(out, "[{}.attributes]", i);
		let _ = writeln!(out, "select = {:?}\n", self.cols.to_flags());

		// Sekcja COLUMNś
		let _ = writeln!(out, "[{}.attributes.config]", i);
		let _ = writeln!(out, "item = {:?}", self.item.to_flags());
		let _ = writeln!(out, "date = {:?}", "default");
		let _ = writeln!(out, "time = {:?}", "default");
		let _ = writeln!(
			out,
			"size = {:?}\n",
			if self.size.mode == ValidColumnSizeParse::Decimal { "decimal" } else { "binary" }
		);

		let _ = writeln!(out, "[{}.tuples]", i);
		let _ = writeln!(out, "sort = {:?}\n", self.sort.to_flags());

		// Sekcja SAVE (Odkomentowana i sformatowana)
		let _ = writeln!(out, "[{}.tuples.save]", i);
		let _ = writeln!(out, "out_dir = {:?}", self.save.raw.path);
		let _ = writeln!(out, "title = {:?}", self.save.raw.title);
		let _ = writeln!(out, "name = {:?}", self.save.raw.name);
		let _ = writeln!(out, "name_is_prefix = {}\n", self.save.raw.prefix);

		out
	}

	/// Wczytuje konfigurację z pliku TOML.
	/// Sztuczka: `path: None` użyje domyślnej lokalizacji, `Some("...")` użyje własnej.
	/// Główna metoda ładująca. Używa jednej ścieżki docelowej jako źródła prawdy.
	pub fn from_toml(path: Option<&str>) -> Result<Self, String> {
		// 1. Definiujemy jedną docelową ścieżkę (domyślną lub podaną przez użytkownika)
		let target_path = path.unwrap_or("./target/.cargo-plot/task.toml");

		// 2. Krok I: Jeśli pliku nie ma - zostanie natychmiast utworzony
		Self::ensure_config_exists(target_path)?;

		// 3. Krok II: Odczyt i ratunek
		let content = std::fs::read_to_string(target_path)
			.map_err(|e| format!("Nie można odczytać pliku {}: {}", target_path, e))?;

		let toml_val: toml::Value = match toml::from_str(&content) {
			Ok(val) => val,
			Err(_) => {
				// Składnia jest nieczytelna/zepsuta -> Odpalamy walec naprawczy!
				Self::backup_and_reset_config(target_path)?;

				// Po resecie wczytujemy na bezczelnego - to nasz świeży plik, więc na 100% zadziała
				let fresh_content = std::fs::read_to_string(target_path).unwrap();
				toml::from_str(&fresh_content).unwrap()
			}
		};

		// 4. Krok III: Wyciąganie wartości (logika z poprzednich etapów)
		let job = toml_val
			.get("job")
			.and_then(|j| j.as_array())
			.and_then(|arr| arr.get(0))
			.ok_or("Brak sekcji [[job]] w pliku TOML")?;

		// =====================================================================
		// ZWINNE HELPERY DO WYCIĄGANIA DANYCH Z TOML
		// =====================================================================
		let get_str = |parent: Option<&toml::Value>, key: &str, default: &str| -> String {
			parent.and_then(|p| p.get(key)).and_then(|v| v.as_str()).unwrap_or(default).to_string()
		};

		let get_bool = |parent: Option<&toml::Value>, key: &str, default: bool| -> bool {
			parent.and_then(|p| p.get(key)).and_then(|v| v.as_bool()).unwrap_or(default)
		};

		let get_arr = |parent: Option<&toml::Value>, key: &str| -> Vec<String> {
			parent
				.and_then(|p| p.get(key))
				.and_then(|v| v.as_array())
				.map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
				.unwrap_or_default()
		};

		// =====================================================================
		// NAWIGACJA PO SEKCJACH
		// =====================================================================
		let explorer = job.get("explorer");
		let attributes = job.get("attributes");
		let config = attributes.and_then(|a| a.get("config"));
		let tuples = job.get("tuples");
		let save_cfg = tuples.and_then(|t| t.get("save"));

		// =====================================================================
		// MAPOWANIE NA FLAGI CLI (Przygotowanie wsadu dla ParseFromFlags)
		// =====================================================================

		let run_mode = get_arr(Some(job), "run_mode");

		// Work: Budujemy z pojedynczych wartości naszą flagową strukturę wejściową
		let workspace = get_str(explorer, "workspace", ".");
		let ignore_case = get_bool(explorer, "ignore_case", false);
		let mut work_flags = vec![workspace];
		if ignore_case {
			work_flags.push("ignore-case".to_string());
		}

		let patt_flags = get_arr(explorer, "patterns");
		let part_flags = get_arr(explorer, "parts");

		let cols_flags = get_arr(attributes, "select");

		let item_flags = get_arr(config, "item");
		let date_flags = vec![get_str(config, "date", "default")];
		let time_flags = vec![get_str(config, "time", "default")];
		let size_flags = vec![get_str(config, "size", "decimal")];

		let sort_flags = get_arr(tuples, "sort");

		// Save: Budujemy flagi typu `klucz=wartosc`
		let out_dir = get_str(save_cfg, "out_dir", "./target/.cargo-plot/");
		let title = get_str(save_cfg, "title", "Project Snapshot");
		let name = get_str(save_cfg, "name", "");
		let name_is_prefix = get_bool(save_cfg, "name_is_prefix", false);

		let mut save_flags = Vec::new();
		if !out_dir.is_empty() {
			save_flags.push(format!("outdir={}", out_dir));
		}
		if !title.is_empty() {
			save_flags.push(format!("title={}", title));
		}
		if !name.is_empty() {
			save_flags.push(format!("name={}", name));
		}
		if name_is_prefix {
			save_flags.push("name-prefix".to_string());
		} else {
			save_flags.push("name-suffix".to_string());
		}

		// =====================================================================
		// BUDOWANIE OSTATECZNEJ STRUKTURY (Recykling logiki!)
		// =====================================================================
		Ok(ValidPreparedJobConfig {
			exec: ValidExecutionParse::parse_vec_as_config(&run_mode),
			work: ValidWorkspaceParse::parse_vec_as_config(&work_flags),
			patt: ValidPatternParse::parse_vec_as_config(&patt_flags),
			part: ValidTablePartParse::parse_vec_as_config(&part_flags),
			cols: ValidTableColumnsParse::parse_vec_as_config(&cols_flags),
			item: ValidColumnItemParse::parse_vec_as_config(&item_flags),
			date: ValidColumnDateParse::parse_vec_as_config(&date_flags),
			time: ValidColumnTimeParse::parse_vec_as_config(&time_flags),
			size: ValidColumnSizeParse::parse_vec_as_config(&size_flags),
			sort: ValidSortByParse::parse_vec_as_config(&sort_flags),
			save: ValidSaveAsParse::parse_vec_as_config(&save_flags),
		})
	}

	/// Gwarantuje, że pod wskazaną ścieżką istnieje plik. Jeśli nie – tworzy go z ustawień domyślnych.
	pub fn ensure_config_exists(target_path: &str) -> Result<(), String> {
		let p = std::path::Path::new(target_path);

		if !p.exists() {
			// Zapewniamy istnienie folderów nadrzędnych
			if let Some(parent) = p.parent() {
				std::fs::create_dir_all(parent)
					.map_err(|e| format!("Nie udało się utworzyć folderu dla {}: {}", target_path, e))?;
			}

			let default_cfg = Self::default();
			let toml_content = default_cfg.to_toml("default_job", None, None);

			std::fs::write(p, toml_content)
				.map_err(|e| format!("Błąd zapisu pliku konfiguracyjnego {}: {}", target_path, e))?;

			println!("🌱 Utworzono plik konfiguracji: {}", target_path);
		}
		Ok(())
	}

	/// Zmienia nazwę zepsutego pliku na backup (dodając tag czasu) i tworzy na jego miejsce nowy, domyślny.
	pub fn backup_and_reset_config(target_path: &str) -> Result<(), String> {
		let p = std::path::Path::new(target_path);

		if p.exists() {
			let time_tag = crate::lib::logic::tag_time().0;
			// Sprytne doklejenie tagu: np. "moj_config_backup-R2026...Q43.toml"
			let backup_path = format!("{}_backup-{}.toml", target_path.trim_end_matches(".toml"), time_tag);

			std::fs::rename(p, &backup_path)
				.map_err(|e| format!("Błąd tworzenia backupu dla {}: {}", target_path, e))?;

			println!("⚠️ Uszkodzony plik! Utworzono kopię zapasową: {}", backup_path);
		}

		// Skoro zrobiliśmy miejsce, wzywamy naszą główną metodę do utworzenia świeżego pliku
		Self::ensure_config_exists(target_path)
	}
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// POZOSTAŁE PRZYPDKI
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

impl ValidSaveAsConfig {
	/// Helper: Gwarantuje, że folder docelowy istnieje i zwraca do niego pełną ścieżkę
	pub fn ensure_out_dir(&self) -> std::io::Result<std::path::PathBuf> {
		if !self.out_dir.exists() {
			std::fs::create_dir_all(&self.out_dir)?;
		}
		Ok(self.out_dir.clone())
	}
}

impl ValidColumnItemConfig {
	/// Wygodny builder: parsuje kolekcję stringów i od razu zwraca gotowy config.
	/// Zwraca Result, ponieważ stringi wejściowe mogą być nieprawidłowe.
	//pub fn from_strings<I>(inputs: I) -> Self
	//where
	//	I: IntoIterator,
	//	I::Item: AsRef<str>, {
	//	// 1. Parsujemy stringi na wektor enumów ValidColumnItemParse
	//	let parsed_items = ValidColumnItemParse::parse_vec(inputs);
	//
	//	// 2. Budujemy i zwracamy ostateczny config
	//	Self::get(parsed_items)
	//}

	pub fn format_item(
		&self,
		u: (usize, usize, usize, usize),
		row: &job::ValidResultMainRow,
		tab: &[job::ValidResultMainRow],
	) -> String {
		let (index, num_width, tier_max, name_len_max) = u;
		let mut parts = Vec::new();
		let num_str = format!("{:>width$}.", index + 1, width = num_width);

		// 1. Jeśli num_is_first = true, numer idzie na początek
		if self.num_is_first {
			parts.push(num_str.clone());
		}

		// 2. Struktura Listy
		if self.list != ModeListForValidColumnItem::None {
			let l = job::gens::item_list::draw_list(&self.list, index, tab, tier_max);
			parts.push(l);
		}

		// 3. Ikony
		if self.icons != ModeIconsForValidColumnItem::None {
			let ic = job::gens::draw_icon(&self.icons, &row.node.node, &row.name_with_ext);
			parts.push(ic.to_string());
		}

		// 4. Jeśli num_is_first = false, numer idzie po liście i ikonach (ale przed nazwą)
		if !self.num_is_first {
			parts.push(num_str);
		}

		// 5. Nazwa pliku
		if self.name {
			parts.push(row.name_with_ext.clone());
		}

		let mut base = parts.join(" ");

		// 6. Align End (Trailing space) - dodaje pusty string, co przy join(" ") da spację na końcu
		if self.align_end {
			let p_tier = (tier_max - row.node.tier) * 3;
			let p_name = name_len_max - row.node.name.chars().count();
			base.push_str(&" ".repeat(p_tier + p_name));
		}

		base
	}
}

impl ValidColumnSizeConfig {
	/// Metoda formatująca rozmiar w zależności od wybranego trybu (Dec/Bin)
	pub fn format_size(&self, bytes: u64) -> String {
		let base = match self.mode {
			ValidColumnSizeParse::Decimal => 1000.0,
			ValidColumnSizeParse::Binary => 1024.0,
		};

		let suffix = match self.mode {
			ValidColumnSizeParse::Decimal => ["B ", "kB", "MB", "GB"],
			ValidColumnSizeParse::Binary => ["B ", "KiB", "MiB", "GiB"], // Lub kB/MB/GB wg preferencji
		};

		let bytes_f = bytes as f64;

		if bytes_f < base {
			format!("{:>6} {}", bytes, suffix[0])
		} else if bytes_f < base.powi(2) {
			format!("{:>6.2} {}", bytes_f / base, suffix[1])
		} else if bytes_f < base.powi(3) {
			format!("{:>6.2} {}", bytes_f / base.powi(2), suffix[2])
		} else {
			format!("{:>6.2} {}", bytes_f / base.powi(3), suffix[3])
		}
	}
}

impl ValidColumnDateConfig {
	pub fn format_date(&self, dt: chrono::DateTime<chrono::Local>) -> String { dt.format(&self.format).to_string() }
}

impl ValidColumnTimeConfig {
	pub fn format_time(&self, dt: chrono::DateTime<chrono::Local>) -> String { dt.format(&self.format).to_string() }
}
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// NOTATKI
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
/*
 * pub enum ValidSaveAsParse{
 * 	// w obu plikach musi być ten sam `logic::tag_time().0`,
 * 	out_dir,
 *  				▶️ ścieżka - domyślnie `./target/.cargo-plot/` - niezależnie jaki jest tworzy jeśli nie istnieje (ensure_exist)
 * 	title,
 * 				    ▶️ tytuł - domyślnie `Project Snapshot`
 *                  ▶️ i wtedy-> `format!("# {} {} (v:{}{} [{}])",
 *                          "INDEX (lub) FILES",
 *                          title,
 *                          logic::tag_time().0,
 *                          "SOTC (lub) COTS",
 *                          "Content of the structure (lub) Structure of the content")`
 * 	name,
 *                 ▶️ nazwa pliku - domyślnie name=""
 *                 ▶️ i wtedy-> `format!("{}{}_{}.md",
 *                          logic::tag_time().0,
 *                          "SOTC (lub) COTS",
 *                          name)`
 * 	name_is_prefix,
 *                ▶️ nazwa pliku - domyślnie name=""
 *                ▶️ i wtedy-> `format!("{}_{}{}.md",
 *                          name,
 *                          logic::tag_time().0,
 *                          "SOTC (lub) COTS")`
 * }
 * */
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
/*
 * dodajemy `ValidSortByParse`
 *
 * - "Date" -  `(Date:DateTime, Reverse:bool)`
 * - "Size" -  `(Size:Num, Reverse:bool)`
 * - "Name" -  `(Name:ModeSortName, Reverse:bool, FileGroup:ModeGroup)`
 * - "Path" -  `(Name:ModeSortName, Reverse:bool, DirSplit:bool, FileGroup:ModeGroup)`
 *
 *
 * `ModeSortName`
 * - "[Spec][Num][AZaz]"
 * - "[Spec][Num][azAZ]"
 * - "[Spec][Num][aAzZ]"
 * - "[Spec][Num][AaZz]"
 * - "[Num][Spec][AZaz]"
 * - "[Num][Spec][azAZ]"
 * - "[Num][Spec][aAzZ]"
 * - "[Num][Spec][AaZz]"
 * - "[Spec][AZaz][Num]"
 * - "[Spec][azAZ][Num]"
 * - "[Spec][aAzZ][Num]"
 * - "[Spec][AaZz][Num]"
 * - "[Num][AZaz][Spec]"
 * - "[Num][azAZ][Spec]"
 * - "[Num][aAzZ][Spec]"
 * - "[Num][AaZz][Spec]"
 * - "[AZaz][Num][Spec]"
 * - "[azAZ][Num][Spec]"
 * - "[aAzZ][Num][Spec]"
 * - "[AaZz][Num][Spec]"
 * - "[AZaz][Spec][Num]"
 * - "[azAZ][Spec][Num]"
 * - "[aAzZ][Spec][Num]"
 * - "[AaZz][Spec][Num]"
 *
 * `ModeGroup`
 * - "Name"
 * - "Exte"
 * - "None"
 *
 * `Priority`
 * - "FirstFile"
 * - "FirstDir"
 * - "None"
 *
 * ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
 * oraz dodajemy `ValidSortByConfig`
 *
 * ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
 * Tu kluczowe na metode sortowania będzie miał wpływ tryb `ValidColumnItemParse`
 * - ValidColumnItemParse::ListTree
 * - ValidColumnItemParse::ListFlat = ValidColumnItemParse::ListNone
 *
 * ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
 * */

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

/*
 * kiedyś dodać ubsługe wyśfietlania ukrywania pustych folderów
 * kiedyś dodać ubsługe wyśfietlania ukrywania pustych plików
 *
 *
 *  */
