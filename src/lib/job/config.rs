use std::{path::PathBuf, process};

use chrono;

use crate::lib::{
	job::{self},
	logic::{self},
};
// ===================================================================
// CONFIG VALID: TABLE-PART
// ===================================================================

pub enum ValidTablePart {
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

impl ValidTablePart {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"MD" | "md" | "match-dirs" => Ok(ValidTablePart::MD),
			"MF" | "mf" | "match-files" => Ok(ValidTablePart::MF),
			"XD" | "xd" | "mismatch-dirs" => Ok(ValidTablePart::XD),
			"XF" | "xf" | "mismatch-files" => Ok(ValidTablePart::XF),
			_ => Err(format!("Nieprawidłowa nazwa: '{}'. Dostępne: md, mf, xd, xf", s.trim())),
		}
	}

	pub fn parses<'a, I>(inputs: I) -> Result<Vec<Self>, String>
	where I: IntoIterator<Item = &'a str> {
		inputs.into_iter().map(Self::parse).collect()
	}

	pub fn parse_vec<I>(inputs: I) -> Vec<Self>
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		inputs.into_iter().map(|s| Self::parse(s.as_ref()).expect("invalid ValidTablePart input")).collect()
	}
	pub fn parse_vec_as_config<I>(inputs: I) -> ValidTablePartConfig
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
					ValidTablePart::MD => md = true,
					ValidTablePart::MF => mf = true,
					ValidTablePart::XD => xd = true,
					ValidTablePart::XF => xf = true,
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

// ===================================================================
// CONFIG VALID: COLUMN-ITEM
// ===================================================================

pub enum ValidColumnItem {
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

#[derive(Debug)]
pub struct ValidColumnItemConfig {
	pub list: ValidModeItemList,
	pub icons: ValidModeItemIcons,
	pub name: bool,
	pub align_end: bool,
	pub num_is_first: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValidModeItemList {
	None,
	Flat,
	Tree,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValidModeItemIcons {
	Lite,
	More,
	None,
}

impl ValidColumnItem {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"list-none" => Ok(ValidColumnItem::ListNone),
			"list-flat" => Ok(ValidColumnItem::ListFlat),
			"list-tree" => Ok(ValidColumnItem::ListTree),

			"icons-lite" => Ok(ValidColumnItem::IconsLite),
			"icons-more" => Ok(ValidColumnItem::IconsMore),
			"icons-none" => Ok(ValidColumnItem::IconsNone),

			"num-prefix" => Ok(ValidColumnItem::NumPrefix),
			"num-suffix" => Ok(ValidColumnItem::NumSuffix),

			"name-none" => Ok(ValidColumnItem::NameNone),
			"name-show" => Ok(ValidColumnItem::NameShow),

			"ws-none" | "whitespace-none" => Ok(ValidColumnItem::WhitespaceTrailNone),
			"ws-show" | "whitespace-show" => Ok(ValidColumnItem::WhitespaceTrailShow),

			_ => Err(format!("Nieznany ValidColumnItem: '{}'", s.trim())),
		}
	}

	pub fn parse_vec<I>(inputs: I) -> Result<Vec<Self>, String>
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		inputs.into_iter().map(|s| Self::parse(s.as_ref())).collect()
	}

	pub fn parse_vec_as_config<I>(inputs: I) -> Result<ValidColumnItemConfig, String>
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let items = Self::parse_vec(inputs)?;
		Ok(ValidColumnItemConfig::get(items))
	}
}

impl ValidColumnItemConfig {
	pub fn get(items: Vec<ValidColumnItem>) -> Self {
		let mut list: Option<ValidModeItemList> = None;
		let mut icons: Option<ValidModeItemIcons> = None;
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
				ValidColumnItem::ListNone => {
					list = Some(ValidModeItemList::None);
					list_count += 1;
				}
				ValidColumnItem::ListFlat => {
					list = Some(ValidModeItemList::Flat);
					list_count += 1;
				}
				ValidColumnItem::ListTree => {
					list = Some(ValidModeItemList::Tree);
					list_count += 1;
				}

				// NUM
				ValidColumnItem::NumPrefix => {
					num_is_first = Some(true);
					num_count += 1;
				}
				ValidColumnItem::NumSuffix => {
					num_is_first = Some(false);
					num_count += 1;
				}

				// ICONS
				ValidColumnItem::IconsLite => {
					icons = Some(ValidModeItemIcons::Lite);
					icons_count += 1;
				}
				ValidColumnItem::IconsMore => {
					icons = Some(ValidModeItemIcons::More);
					icons_count += 1;
				}
				ValidColumnItem::IconsNone => {
					icons = Some(ValidModeItemIcons::None);
					icons_count += 1;
				}

				// NAME
				ValidColumnItem::NameNone => {
					name = Some(false);
					name_count += 1;
				}
				ValidColumnItem::NameShow => {
					name = Some(true);
					name_count += 1;
				}

				// WHITESPACE (ALIGN_END)
				ValidColumnItem::WhitespaceTrailNone => {
					align_end = Some(false);
					ws_count += 1;
				}
				ValidColumnItem::WhitespaceTrailShow => {
					align_end = Some(true);
					ws_count += 1;
				}
			}
		}

		Self {
			list: if list_count == 1 { list.unwrap() } else { ValidModeItemList::Tree },

			icons: if icons_count == 1 { icons.unwrap() } else { ValidModeItemIcons::Lite },

			num_is_first: if num_count == 1 { num_is_first.unwrap() } else { false },

			name: if name_count == 1 { name.unwrap() } else { false },

			align_end: if ws_count == 1 { align_end.unwrap() } else { false },
		}
	}

	/// Wygodny builder: parsuje kolekcję stringów i od razu zwraca gotowy config.
	/// Zwraca Result, ponieważ stringi wejściowe mogą być nieprawidłowe.
	pub fn from_strings<I>(inputs: I) -> Result<Self, String>
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		// 1. Parsujemy stringi na wektor enumów ValidColumnItem
		let parsed_items = ValidColumnItem::parse_vec(inputs)?;

		// 2. Budujemy i zwracamy ostateczny config
		Ok(Self::get(parsed_items))
	}

	pub fn format_item(&self, u: (usize, usize, usize, usize), row: &job::RawRow, tab: &[job::RawRow]) -> String {
		let (index, num_width, tier_max, name_len_max) = u;
		let mut parts = Vec::new();
		let num_str = format!("{:>width$}.", index + 1, width = num_width);

		// 1. Jeśli num_is_first = true, numer idzie na początek
		if self.num_is_first {
			parts.push(num_str.clone());
		}

		// 2. Struktura Listy
		if self.list != ValidModeItemList::None {
			let l = job::gens::item_list::draw_list(&self.list, index, tab, tier_max);
			parts.push(l);
		}

		// 3. Ikony
		if self.icons != ValidModeItemIcons::None {
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

// ===================================================================
// CONFIG VALID: TABLE-COLUMNS
// ===================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidTableColumns {
	Date,
	Time,
	Size,
	Item,
	Path,
}

#[derive(Debug, Clone)]
pub struct ValidTableColumnsConfig {
	pub columns: Vec<ValidTableColumns>,
}

impl ValidTableColumns {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"date" => Ok(ValidTableColumns::Date),
			"time" => Ok(ValidTableColumns::Time),
			"size" => Ok(ValidTableColumns::Size),
			"item" => Ok(ValidTableColumns::Item),
			"path" => Ok(ValidTableColumns::Path),
			_ => Err(format!("Nieprawidłowa nazwa kolumny: '{}'. Dostępne: date, time, size, item, path", s.trim())),
		}
	}

	pub fn parse_vec<I>(inputs: I) -> Result<Vec<Self>, String>
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		inputs.into_iter().map(|s| Self::parse(s.as_ref())).collect()
	}

	pub fn parse_vec_as_config<I>(inputs: I) -> ValidTableColumnsConfig
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
					ValidTableColumns::Date => {
						if !has_date {
							columns.push(ValidTableColumns::Date);
							has_date = true;
						}
					}
					ValidTableColumns::Time => {
						if !has_time {
							columns.push(ValidTableColumns::Time);
							has_time = true;
						}
					}
					ValidTableColumns::Size => {
						if !has_size {
							columns.push(ValidTableColumns::Size);
							has_size = true;
						}
					}
					ValidTableColumns::Item => {
						if !has_item {
							columns.push(ValidTableColumns::Item);
							has_item = true;
						}
					}
					ValidTableColumns::Path => {
						if !has_path {
							columns.push(ValidTableColumns::Path);
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
					ValidTableColumns::Date,
					ValidTableColumns::Time,
					ValidTableColumns::Size,
					ValidTableColumns::Item,
					ValidTableColumns::Path,
				],
			};
		}

		// Jeśli podano cokolwiek poprawnego, ale brakuje 'Item', doklejamy na koniec
		if !has_item {
			columns.push(ValidTableColumns::Item);
		}

		ValidTableColumnsConfig { columns }
	}
}

// ===================================================================
// CONFIG VALID: COLUMN-SIZE
// ===================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnSize {
	Decimal, // System SI (podstawa 1000)
	Binary,  // System IEC (podstawa 1024)
}

#[derive(Debug, Clone)]
pub struct ValidColumnSizeConfig {
	pub mode: ValidColumnSize,
}

impl ValidColumnSize {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"dec" | "decimal" => Ok(ValidColumnSize::Decimal),
			"bin" | "binary" => Ok(ValidColumnSize::Binary),
			_ => Err(format!("Nieznany system miar: '{}'. Dostępne: dec, bin", s.trim())),
		}
	}

	pub fn parse_vec_as_config<I>(inputs: I) -> ValidColumnSizeConfig
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let mut mode = ValidColumnSize::Decimal; // Domyślnie Dec

		for item in inputs {
			if let Ok(parsed) = Self::parse(item.as_ref()) {
				mode = parsed;
				break; // Przyjmujemy pierwszy poprawny wpis
			}
		}

		ValidColumnSizeConfig { mode }
	}
}

impl ValidColumnSizeConfig {
	/// Metoda formatująca rozmiar w zależności od wybranego trybu (Dec/Bin)
	pub fn format_size(&self, bytes: u64) -> String {
		let base = match self.mode {
			ValidColumnSize::Decimal => 1000.0,
			ValidColumnSize::Binary => 1024.0,
		};

		let suffix = match self.mode {
			ValidColumnSize::Decimal => ["B ", "kB", "MB", "GB"],
			ValidColumnSize::Binary => ["B ", "KiB", "MiB", "GiB"], // Lub kB/MB/GB wg preferencji
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
// ===================================================================
// CONFIG VALID: COLUMN-DATE
// ===================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnDate {
	Default,
}

#[derive(Debug, Clone)]
pub struct ValidColumnDateConfig {
	pub format: String,
}

impl ValidColumnDate {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"date-default" | "default" => Ok(ValidColumnDate::Default),
			_ => Err(format!("Nieznany format daty: '{}'", s.trim())),
		}
	}

	pub fn parse_vec_as_config<I>(inputs: I) -> ValidColumnDateConfig
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

impl ValidColumnDateConfig {
	pub fn format_date(&self, dt: chrono::DateTime<chrono::Local>) -> String { dt.format(&self.format).to_string() }
}

// ===================================================================
// CONFIG VALID: COLUMN-TIME
// ===================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnTime {
	Default,
}

#[derive(Debug, Clone)]
pub struct ValidColumnTimeConfig {
	pub format: String,
}

impl ValidColumnTime {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"time-default" | "default" => Ok(ValidColumnTime::Default),
			_ => Err(format!("Nieznany format czasu: '{}'", s.trim())),
		}
	}

	pub fn parse_vec_as_config<I>(inputs: I) -> ValidColumnTimeConfig
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

impl ValidColumnTimeConfig {
	pub fn format_time(&self, dt: chrono::DateTime<chrono::Local>) -> String { dt.format(&self.format).to_string() }
}
// ===================================================================
// CONFIG VALID: PATTERN
// ===================================================================

#[derive(Debug, Clone)]
pub struct ValidPatternConfig {
	pub patterns: Vec<String>,
	pub ignore_case: bool,
}

impl ValidPatternConfig {
	/// Zwraca gotowy obiekt logic::PatternsQueries używany przez skaner
	pub fn get(&self) -> logic::PatternsQueries {
		// Musimy jawnie stworzyć wektor referencji, aby Rust nie miał wątpliwości co do typu
		let patterns_ref: Vec<&str> = self.patterns.iter().map(|s| s.as_str()).collect();
		logic::PatternsQueries::new(patterns_ref, self.ignore_case)
	}
}

pub struct ValidPattern;

impl ValidPattern {
	pub fn parse_vec_as_config<I>(inputs: I, ignore_case_sensitive: Option<bool>) -> ValidPatternConfig
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
					"./dist/**".to_string(),
				],
				ignore_case: ignore_case_sensitive.unwrap_or(false),
			};
		}

		ValidPatternConfig { patterns, ignore_case: ignore_case_sensitive.unwrap_or(false) }
	}
}

// ===================================================================
// CONFIG VALID: WORKSPACE
// ===================================================================

#[derive(Debug, Clone)]
pub struct ValidWorkspaceConfig {
	pub workspace_dir: String,
}

impl ValidWorkspaceConfig {
	/// Normalizuje ścieżkę i zwraca logic::AnchoredPathsDatum.
	/// W przypadku błędu (np. brak folderu) przerywa program z komunikatem.
	pub fn get(&self) -> logic::AnchoredPathsDatum {
		// 1. Zamiana na PathBuf dla natywnej obsługi OS
		let path = PathBuf::from(&self.workspace_dir);

		// 2. Kanonizacja (rozwiązuje .., ścieżki relatywne i symlinki)
		// Jeśli ścieżka nie istnieje, fs::canonicalize zwróci błąd
		let work_path = match path.canonicalize() {
			Ok(p) => p.to_string_lossy().to_string(),
			Err(_) => {
				// Jeśli canonicalize zawiedzie (np. brak folderu),
				// próbujemy chociaż zwrócić czytelną ścieżkę absolutną
				match std::env::current_dir() {
					Ok(current) => current.join(&path).to_string_lossy().to_string(),
					Err(_) => self.workspace_dir.clone(),
				}
			}
		};

		// 3. Inicjalizacja AnchoredPathsDatum z obsługą błędu
		logic::AnchoredPathsDatum::new(&work_path).unwrap_or_else(|x| {
			eprintln!("❌ Błąd lokalizacji workspace: {}", x);
			process::exit(1);
		})
	}
}

pub struct ValidWorkspace;

impl ValidWorkspace {
	pub fn parse_vec_as_config<I>(inputs: I) -> ValidWorkspaceConfig
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let mut workspace_dir = ".".to_string();

		// Bierzemy pierwszy niepusty element jako ścieżkę workspace
		for item in inputs {
			let s = item.as_ref().trim();
			if !s.is_empty() {
				workspace_dir = s.to_string();
				break;
			}
		}

		ValidWorkspaceConfig { workspace_dir }
	}
}
// ===================================================================
// ===================================================================
