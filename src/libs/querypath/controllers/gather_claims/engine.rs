
use anyhow::{Context, Result};

// use std::collections::HashMap;

use crate::libs::querypath::{
	controllers::{
		traits::ApplyClaim,
		types::{ClaimCreating, /*ClaimDefaults,*/ ClaimOrdering, ClaimPatterns, DebugVerboses, /*Gather,*/ GatherClaims},
	},
	core::{
		features::{HandlePreparedPatterns, HandleVerifiedDirs, HandleWalkScan},
		types::enums::{
			EnumCasePrecedence, EnumCharClass, EnumColumn, EnumDirWeightDisplay, EnumGroupStrategy, EnumNoExtPriority,
			EnumSameNamePriority, EnumUnitSystem, EnumWeightPrecision,
		},
	},
};

// ============================================================================
// ============================================================================

impl DebugVerboses {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn step0_entrydir(mut self, enable: bool) -> Self {
		self.step0_entrydir = enable;
		self
	}

	#[must_use]
	pub fn step1_patterns(mut self, enable: bool) -> Self {
		self.step1_patterns = enable;
		self
	}

	#[must_use]
	pub fn step2_walk(mut self, enable: bool) -> Self {
		self.step2_walk = enable;
		self
	}

	#[must_use]
	pub fn step3_execute(mut self, enable: bool) -> Self {
		self.step3_execute = enable;
		self
	}
}

impl ApplyClaim for DebugVerboses {
	fn apply(self, claims: &mut GatherClaims) {
		claims.verboses = self;
	}
}

// ============================================================================
// 2. STRUKTURY DANYCH I BUDOWNICZOWIE (Ukrycie Default::default)
// ============================================================================

impl ClaimPatterns {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn entry_dirs<I, S>(mut self, dirs: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.entry_dirs = dirs.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn patterns_all_matched<I, S>(mut self, patterns: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.patterns_all_matched = patterns.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn patterns_no_numerate<I, S>(mut self, patterns: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.patterns_no_numerate = patterns.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn patterns_no_copypast<I, S>(mut self, patterns: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.patterns_no_copypast = patterns.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn case_sensitive(mut self, sensitive: bool) -> Self {
		self.case_sensitive = sensitive;
		self
	}
}

impl ApplyClaim for ClaimPatterns {
	fn apply(self, claims: &mut GatherClaims) {
		claims.patterns = self;
	}
}

// ============================================================================
// ============================================================================

// Ręczna implementacja Default rozwiązuje błąd E0277 dla enumów
impl Default for ClaimOrdering {
	fn default() -> Self {
		Self {
			order_group_strategy: EnumGroupStrategy::Mixed,
			order_same_name_priority: EnumSameNamePriority::DirectoryFirst,
			order_ignore_leading_dot: false,
			order_split_extension: true,
			order_no_ext_priority: EnumNoExtPriority::Above,
			order_char_class_order: [EnumCharClass::Special, EnumCharClass::Digit, EnumCharClass::Letter],
			order_case_precedence: EnumCasePrecedence::Insensitive,
			order_reverse: false,
			numerate_dirs: true,
			numerate_binaries: true,
			numerate_start_from: 1,
		}
	}
}

impl ClaimOrdering {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn order_reverse(mut self, reverse: bool) -> Self {
		self.order_reverse = reverse;
		self
	}

	#[must_use]
	pub fn numerate_start_from(mut self, start: usize) -> Self {
		self.numerate_start_from = start;
		self
	}
}

impl ApplyClaim for ClaimOrdering {
	fn apply(self, claims: &mut GatherClaims) {
		claims.ordering = self;
	}
}

// ============================================================================
// ============================================================================

// Ręczna implementacja Default rozwiązuje błąd E0277
impl Default for ClaimCreating {
	fn default() -> Self {
		Self {
			wraper_name_width: 25,
			wraper_path_width: 35,
			column_order_left: vec![EnumColumn::Weight, EnumColumn::Temporal],
			column_order_right: vec![EnumColumn::Path],
			weight_binary_base: EnumUnitSystem::Binary,
			weight_dir_display: EnumDirWeightDisplay::MatchedOnly,
			weight_precision: EnumWeightPrecision::Tenths,
			keep_parent: true,
			include_files: true,
			include_binaries_in_matched: true,
			title: "CODE SNAPSHOT",
			title_sufix_timestamp: false,
			saveas: "snapshot_",
			saveas_sufix_timestamp: true,
			output_dir: "./dump".to_string(),
			limit_max_single_file_size: 524288,
			add_in_header_autoreklama: false,
			add_in_footer_autoreklama: true,
			add_in_header_table_setup: false,
			add_in_footer_table_setup: true,
			add_in_header_table_stats: false,
			add_in_footer_table_stats: false,
			add_in_header_paragraph: "",
			add_in_footer_paragraph: "",
		}
	}
}

impl ClaimCreating {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}
}

impl ApplyClaim for ClaimCreating {
	fn apply(self, claims: &mut GatherClaims) {
		claims.creating = self;
	}
}

// ============================================================================
// 3. GŁÓWNY ZBIERACZ (GatherClaims)
// ============================================================================

impl GatherClaims {
	/// Tworzy nowy GatherClaims, odpakowując z użyciem Traita dostarczone
	/// konfiguracje.
	#[must_use]
	pub fn new<T>(configs: T) -> Self
	where
		T: ApplyClaim,
	{
		let mut instance = Self {
			patterns: ClaimPatterns::default(),
			ordering: ClaimOrdering::default(),
			creating: ClaimCreating::default(),
			verboses: DebugVerboses::default(),
			step00_val_entry: None,
			step01_val_patts: None,
			step02_val_walk: None,
			step03_val_execute: None,
		};
		// Metoda apply z Traita ApplyClaim automatycznie podmieni tylko
		// dostarczone pola!
		configs.apply(&mut instance);
		instance
	}

	pub fn step00_valid_entry_paths(mut self) -> Result<Self> {
		let entry = HandleVerifiedDirs::build(&self.patterns.entry_dirs, self.verboses.step0_entrydir)
			.context("ETAP-1 [Błąd kroku 00]: Niepoprawne ścieżki wejściowe (entry_dirs)")?;

		self.step00_val_entry = Some(entry);
		Ok(self)
	}

	pub fn step01_valid_patterns(mut self) -> Result<Self> {
		let patts = HandlePreparedPatterns::build(
			&self.patterns.patterns_all_matched,
			&self.patterns.patterns_no_numerate,
			&self.patterns.patterns_no_copypast,
			self.patterns.case_sensitive,
			self.verboses.step1_patterns,
		)
		.context("ETAP-1 [Błąd kroku 01]: Nieudana kompilacja wzorców")?;

		self.step01_val_patts = Some(patts);
		Ok(self)
	}

	pub fn step02_valid_scan_walk(mut self) -> Result<Self> {
		let entry = self
			.step00_val_entry
			.as_ref()
			.context("ETAP-1 [Błąd sekwencji]: Próba uruchomienia skanu przed walidacją wejścia!")?;

		let walk = HandleWalkScan::scan(entry, self.verboses.step2_walk)
			.context("ETAP-1 [Błąd kroku 02]: Skanowanie dysku (HandleWalkScan) zakończone niepowodzeniem")?;

		self.step02_val_walk = Some(walk);
		Ok(self)
	}

	/// Filtrowanie zeskanowanych ścieżek z użyciem wzorców regex / globu.
	pub fn step03_valid_execute_query(mut self) -> Result<Self> {
		let patts =
			self.step01_val_patts.as_ref().context("ETAP-1 [Błąd sekwencji]: Brak wzorców przed filtrowaniem!")?;

		let walk = self
			.step02_val_walk
			.as_ref()
			.context("ETAP-1 [Błąd sekwencji]: Brak drzewa katalogów przed filtrowaniem!")?;

		let execute = crate::libs::querypath::core::features::step04_execute_query::QueryExecute::apply(
			&patts.all_matched, // <-- ZMIANA TUTAJ: Wskazujemy konkretną grupę filtrów (PathsPatterns)
			walk,
			self.creating.keep_parent,
			self.verboses.step3_execute,
		)
		.context("ETAP-1 [Błąd kroku 03]: Filtracja wyników nie powiodła się!")?;

		self.step03_val_execute = Some(execute);
		Ok(self)
	}

	/// Gwarantuje zebranie w pełni zwalidowanych obiektów i przekazanie ich do
	/// zablokowanego stanu (Read-Only Engine).
	pub fn step99_if_all_valid_corect_then_gather_out_to_prepared_setup(
		mut self,
	) -> Result<crate::libs::querypath::controllers::prepar_setups::PreparedSetup> {
		let entry = self.step00_val_entry.take().context("Pominięto step00_valid_entry_paths w łańcuchu walidacji!")?;
		let patts = self.step01_val_patts.take().context("Pominięto step01_valid_patterns w łańcuchu walidacji!")?;
		let walk = self.step02_val_walk.take().context("Pominięto step02_valid_scan_walk w łańcuchu walidacji!")?;
		let execute =
			self.step03_val_execute.take().context("Pominięto step03_valid_execute_query w łańcuchu walidacji!")?;

		Ok(crate::libs::querypath::controllers::prepar_setups::PreparedSetup {
			entry,
			patts,
			walk,
			execute,
			ordering: self.ordering,
			creating: self.creating,
			verboses: self.verboses,
		})
	}
}
