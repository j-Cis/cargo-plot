// use super::{ConfigJob, ScanRawJobNew, ConfigLayout, ConfigTrimming,
// ConfigExport, ConfigSpec};
use crate::lib::logic::config_model::*;
// ============================================================================
// ENUMY Z PARSERAMI (SINGLE SOURCE OF TRUTH)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabColumn {
	TreeList, // Symbole drzewa (  ├──• ) // Wcięcia wg głębokości ( │  │  ├──• )
	Number,   // Numeracja pozycji ( 1. )
	Icon,     // Ikona typu ( 📂 / 📝 )
	Size,     // Rozmiar w nawiasach ( [ 1.20 kB] )
	Date,     // Data: | 2026 W14 Sun |
	Time,     // Czas: | 11:08:06.298 PM |
	Path,     // Ścieżka pliku
}

impl TabColumn {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"treelist" | "tree" | "list" => Ok(TabColumn::TreeList),
			"number" => Ok(TabColumn::Number),
			"icon" => Ok(TabColumn::Icon),
			"size" => Ok(TabColumn::Size),
			"date" => Ok(TabColumn::Date),
			"time" => Ok(TabColumn::Time),
			"path" => Ok(TabColumn::Path),
			_ => Err(format!(
				"Nieprawidłowa nazwa kolumny: '{}'. Dostępne: list, tree, number, icon, size, date, time, path",
				s.trim()
			)),
		}
	}
}

//==================================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabSortBy {
	Name,
	Size,
	Date,
	Kind,
	FileFirst,
	DirFirst,
	FileFirstMerge,
	DirFirstMerge,
}

impl TabSortBy {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"name" => Ok(TabSortBy::Name),
			"size" => Ok(TabSortBy::Size),
			"date" => Ok(TabSortBy::Date),
			"kind" => Ok(TabSortBy::Kind),
			"file-first" => Ok(TabSortBy::FileFirst),
			"dir-first" => Ok(TabSortBy::DirFirst),
			"file-merge" => Ok(TabSortBy::FileFirstMerge),
			"dir-merge" => Ok(TabSortBy::DirFirstMerge),
			_ => Err(format!(
				"Nieprawidłowa wartość sortowania: '{}'. Dostępne: name, size, date, kind, file-first, dir-first, file-merge, dir-merge",
				s.trim()
			)),
		}
	}
}

//==================================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabSortOrder {
	Asc,
	Desc,
}

impl TabSortOrder {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"az" | "asc" | "ascending" => Ok(TabSortOrder::Asc),
			"za" | "desc" | "descending" => Ok(TabSortOrder::Desc),
			_ => Err(format!("Nieprawidłowy kierunek sortowania: '{}'. Dostępne: asc, desc", s)),
		}
	}
}
//==================================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabPathStructure {
	List,
	Tree,
}

impl TabPathStructure {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"tree" | "t" | "T" => Ok(TabPathStructure::Tree),
			"list" | "l" | "L" => Ok(TabPathStructure::List),
			_ => Err(format!("Nieprawidłowa struktura: '{}'. Dostępne: tree, list", s)),
		}
	}
}

//==================================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobMode {
	Matched,
	Mismatched,
}

impl JobMode {
	pub fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"m" | "M" | "matched" => Ok(JobMode::Matched),
			"x" | "X" | "mismatched" => Ok(JobMode::Mismatched),
			_ => Err(format!("Nieprawidłowy tryb zadania (mode): '{}'. Dostępne: m, x", s)),
		}
	}
}

//==================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabSpec {
	pub sort_by: TabSortBy,
	pub sort_order: TabSortOrder,
	pub structure: TabPathStructure,
	pub columns: Vec<TabColumn>,
	pub trim_size: Option<usize>,
	pub trim_page: usize, // Domyślnie 1, działa tylko gdy trim_size jest Some
	pub more_icons: bool,
}

impl Default for TabSpec {
	fn default() -> Self {
		Self {
			sort_by: TabSortBy::Name,
			sort_order: TabSortOrder::Asc,
			structure: TabPathStructure::Tree,
			columns: vec![
				TabColumn::Date,
				TabColumn::Time,
				TabColumn::Size,
				TabColumn::TreeList,
				TabColumn::Icon,
				TabColumn::Number,
				TabColumn::Path,
			],
			trim_size: None,
			trim_page: 1,
			more_icons: false,
		}
	}
}

impl TabSpec {
	pub fn from_config(cfg: &ConfigLayout, spec: &ConfigSpec) -> Result<Self, String> {
		let mut columns = Vec::new();
		for col_str in &cfg.columns {
			columns.push(TabColumn::parse(col_str)?);
		}

		let mut tab = Self::default()
			.sort(
				TabSortBy::parse(&cfg.sort_by)?,       // ⚡ BYŁO: cfg.sort
				TabSortOrder::parse(&cfg.sort_order)?, // ⚡ BYŁO: if cfg.reverse...
				TabPathStructure::parse(&cfg.structure)?,
			)
			.columns(&columns)
			.more_icons(cfg.more_icons);

		// Zostawiamy wsparcie dla flagi reverse, jeśli ktoś woli jej używać
		if cfg.reverse {
			tab.sort_order = TabSortOrder::Desc;
		}

		// ⚡ Pobieramy domyślną stronę z głównej sekcji
		tab.trim_page = cfg.trim_page;

		if let Some(trim) = &spec.trimming
			&& let Some(size) = trim.page_size
		{
			tab = tab.trim(size, trim.show_page);
		}
		Ok(tab)
	}

	pub fn sort(mut self, by: TabSortBy, order: TabSortOrder, structure: TabPathStructure) -> Self {
		self.sort_by = by;
		self.sort_order = order;
		self.structure = structure;
		self
	}

	pub fn columns(mut self, cols: &[TabColumn]) -> Self {
		self.columns = cols.to_vec();
		self
	}

	pub fn trim(mut self, size: usize, page: Option<usize>) -> Self {
		self.trim_size = Some(size);
		if let Some(p) = page {
			self.trim_page = p;
		}
		self
	}

	pub fn more_icons(mut self, enabled: bool) -> Self {
		self.more_icons = enabled;
		self
	}
}

//==================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanSpec {
	pub work_path: String,
	pub patterns: Vec<String>,
	pub ignore_case: bool,
}

impl Default for ScanSpec {
	fn default() -> Self {
		Self {
			work_path: ".".to_string(),
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
			ignore_case: false,
		}
	}
}

impl ScanSpec {
	pub fn new(work_path: impl Into<String>) -> Self { Self { work_path: work_path.into(), ..Default::default() } }

	pub fn from_config(cfg: &ScanRawJobNew) -> Self {
		Self::new(&cfg.work_path)
			.patterns(cfg.patterns.iter().map(|s| s.as_str()).collect())
			.ignore_case(cfg.ignore_case)
	}

	pub fn patterns(mut self, pats: Vec<&str>) -> Self {
		self.patterns = pats.into_iter().map(|s| s.to_string()).collect();
		self
	}

	pub fn ignore_case(mut self, ignore: bool) -> Self {
		self.ignore_case = ignore;
		self
	}
}

//==================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobSpec {
	pub id: String,
	pub name: Option<String>,
	pub description: Option<String>,
	//------------------------------
	pub mode: JobMode,
	pub scan: ScanSpec,
	//------------------------------
	pub save_sotc_at: Option<String>,
	pub title_sotc: Option<String>,
	pub save_cots_at: Option<String>,
	pub title_cots: Option<String>,
	//------------------------------
	pub quiet_work: bool,
	pub hide_stats: bool,
	pub hide_promo: bool,
	//------------------------------
	pub table: TabSpec,
}

impl Default for JobSpec {
	fn default() -> Self {
		Self {
			id: "default_job".to_string(),
			name: None,
			description: None,
			//-----------
			mode: JobMode::Matched,
			scan: ScanSpec::default(),
			save_sotc_at: None,
			title_sotc: None,
			save_cots_at: None,
			title_cots: None,
			quiet_work: false,
			hide_stats: false,
			hide_promo: false,
			table: TabSpec::default(), // Domyślna tabela (z pełnymi kolumnami i drzewem)
		}
	}
}

impl JobSpec {
	pub fn new(id: impl Into<String>) -> Self { Self { id: id.into(), ..Default::default() } }

	pub fn from_config(cfg: &ConfigJob) -> Result<Self, String> {
		let scan = ScanSpec::from_config(&cfg.scan);
		let table = TabSpec::from_config(&cfg.table_sotc, &cfg.spec)?;

		let mut job = Self::new(&cfg.id)
			.scan(scan)
			.table(table)
			.mode(JobMode::parse(&cfg.spec.mode)?)
			.quiet(cfg.quiet_work)
			.hide_stats(cfg.spec.hide_stats)
			.hide_promo(cfg.spec.hide_promo);

		job.name = cfg.name.clone();
		job.description = cfg.description.clone();

		if let Some(export) = &cfg.export {
			job.save_sotc_at = export.save_sotc_at.clone();
			job.title_sotc = export.title_sotc.clone();
			job.save_cots_at = export.save_cots_at.clone();
			job.title_cots = export.title_cots.clone();
		}
		Ok(job)
	}

	pub fn work_path(mut self, path: impl Into<String>) -> Self {
		self.scan.work_path = path.into();
		self
	}

	pub fn patterns(mut self, pats: Vec<&str>) -> Self {
		self.scan.patterns = pats.into_iter().map(|s| s.to_string()).collect();
		self
	}

	pub fn ignore_case(mut self, ignore: bool) -> Self {
		self.scan.ignore_case = ignore;
		self
	}

	pub fn scan(mut self, scan_spec: ScanSpec) -> Self {
		self.scan = scan_spec;
		self
	}

	pub fn mode(mut self, mode: JobMode) -> Self {
		self.mode = mode;
		self
	}

	pub fn quiet(mut self, quiet: bool) -> Self {
		self.quiet_work = quiet;
		self
	}

	pub fn hide_stats(mut self, hide: bool) -> Self {
		self.hide_stats = hide;
		self
	}

	pub fn hide_promo(mut self, hide: bool) -> Self {
		self.hide_promo = hide;
		self
	}

	//  Delegujemy trimming prosto do zagnieżdżonego TabSpec
	pub fn trim(mut self, size: usize, page: Option<usize>) -> Self {
		self.table = self.table.trim(size, page);
		self
	}

	// Pozwala wstrzyknąć gotową specyfikację tabeli
	pub fn table(mut self, tab_spec: TabSpec) -> Self {
		self.table = tab_spec;
		self
	}
}

//==================================================================================================
