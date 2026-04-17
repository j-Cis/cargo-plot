use std::{path::PathBuf, process};

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
 *  */
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// CONFIG VALID: TABLE-PART
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

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

impl ValidTablePartParse {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"MD" | "md" | "match-dirs" => Ok(ValidTablePartParse::MD),
			"MF" | "mf" | "match-files" => Ok(ValidTablePartParse::MF),
			"XD" | "xd" | "mismatch-dirs" => Ok(ValidTablePartParse::XD),
			"XF" | "xf" | "mismatch-files" => Ok(ValidTablePartParse::XF),
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
		inputs.into_iter().map(|s| Self::parse(s.as_ref()).expect("invalid ValidTablePartParse input")).collect()
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

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// CONFIG VALID: COLUMN-ITEM
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

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

#[derive(Debug)]
pub struct ValidColumnItemConfig {
	pub list: ModeListForValidColumnItem,
	pub icons: ModeIconsForValidColumnItem,
	pub name: bool,
	pub align_end: bool,
	pub num_is_first: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModeListForValidColumnItem {
	None,
	Flat,
	Tree,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModeIconsForValidColumnItem {
	Lite,
	More,
	None,
}

impl ValidColumnItemParse {
	pub fn parse(s: &str) -> Result<Self, String> {
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
	pub fn get(items: Vec<ValidColumnItemParse>) -> Self {
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

	/// Wygodny builder: parsuje kolekcję stringów i od razu zwraca gotowy config.
	/// Zwraca Result, ponieważ stringi wejściowe mogą być nieprawidłowe.
	pub fn from_strings<I>(inputs: I) -> Result<Self, String>
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		// 1. Parsujemy stringi na wektor enumów ValidColumnItemParse
		let parsed_items = ValidColumnItemParse::parse_vec(inputs)?;

		// 2. Budujemy i zwracamy ostateczny config
		Ok(Self::get(parsed_items))
	}

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

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// CONFIG VALID: TABLE-COLUMNS
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

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

impl ValidTableColumnsParse {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"date" => Ok(ValidTableColumnsParse::Date),
			"time" => Ok(ValidTableColumnsParse::Time),
			"size" => Ok(ValidTableColumnsParse::Size),
			"item" => Ok(ValidTableColumnsParse::Item),
			"path" => Ok(ValidTableColumnsParse::Path),
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

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// CONFIG VALID: COLUMN-SIZE
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnSizeParse {
	Decimal, // System SI (podstawa 1000)
	Binary,  // System IEC (podstawa 1024)
}

#[derive(Debug, Clone)]
pub struct ValidColumnSizeConfig {
	pub mode: ValidColumnSizeParse,
}

impl ValidColumnSizeParse {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"dec" | "decimal" => Ok(ValidColumnSizeParse::Decimal),
			"bin" | "binary" => Ok(ValidColumnSizeParse::Binary),
			_ => Err(format!("Nieznany system miar: '{}'. Dostępne: dec, bin", s.trim())),
		}
	}

	pub fn parse_vec_as_config<I>(inputs: I) -> ValidColumnSizeConfig
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
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// CONFIG VALID: COLUMN-DATE
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnDateParse {
	Default,
}

#[derive(Debug, Clone)]
pub struct ValidColumnDateConfig {
	pub format: String,
}

impl ValidColumnDateParse {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"date-default" | "default" => Ok(ValidColumnDateParse::Default),
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

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// CONFIG VALID: COLUMN-TIME
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnTimeParse {
	Default,
}

#[derive(Debug, Clone)]
pub struct ValidColumnTimeConfig {
	pub format: String,
}

impl ValidColumnTimeParse {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"time-default" | "default" => Ok(ValidColumnTimeParse::Default),
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
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// CONFIG VALID: PATTERN
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

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

pub struct ValidPatternParse;

impl ValidPatternParse {
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
					"./dist/{**/*,*}.{bat,exe}&/".to_string(),
				],
				ignore_case: ignore_case_sensitive.unwrap_or(false),
			};
		}

		ValidPatternConfig { patterns, ignore_case: ignore_case_sensitive.unwrap_or(false) }
	}
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// CONFIG VALID: WORKSPACE
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

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

pub struct ValidWorkspaceParse;

impl ValidWorkspaceParse {
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
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// CONFIG VALID: SORT-BY
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

/* ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeFileGroupForValidSortBy {
	Name,
	Exte,
	None,
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

impl ValidSortByConfig {
	/// Zwraca gotowy obiekt z warstwy logic:: (np. logic::SortQueries),
	/// gdzie dopiero tam te proste stringi i bool'e zostaną przetworzone
	/// w potężny algorytm (wspomniane 24 warianty).
	pub fn get(&self) -> logic::SortQueries { logic::SortQueries::new(self.clone()) }
}

pub struct ValidSortByParse;

impl ValidSortByParse {
	pub fn parse_vec_as_config<I>(inputs: I) -> ValidSortByConfig
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

/*
 * dodać ubsługe wyśfietlania ukrywania pustych folderów
 * dodać ubsługe wyśfietlania ukrywania pustych plików
 *
 *
 *  */
