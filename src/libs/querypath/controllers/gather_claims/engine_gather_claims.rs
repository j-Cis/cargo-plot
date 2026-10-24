use anyhow::{Context, Result};

// use std::collections::HashMap;
use crate::libs::querypath::{
	controllers::{
		traits::ApplyClaim,
		types::{ClaimCreating, ClaimOrdering, ClaimPatterns, ClaimRefining, DebugVerboses, GatherClaims},
	},
	core::features::{HandleCorpus, HandlePreparedPatterns, HandleVerifiedDirs, HandleWalkScan},
};

// ============================================================================
// 3. GŁÓWNY ZBIERACZ (GatherClaims)
// ============================================================================

impl GatherClaims {
	/// Tworzy nowy GatherClaims, odpakowując z użyciem Traita dostarczone
	/// konfiguracje.
	#[must_use]
	pub fn new<T>(configs: T) -> Self
	where T: ApplyClaim {
		let mut instance = Self {
			patterns: ClaimPatterns::default(),
			refining: ClaimRefining::default(),
			ordering: ClaimOrdering::default(),
			creating: ClaimCreating::default(),
			verboses: DebugVerboses::default(),
			step1a_val_patts: None,
			step1b_val_entry: None,
			step1c_val_walk: None,
			step1d_val_execute: None,
		};
		// Metoda apply z Traita ApplyClaim automatycznie podmieni tylko
		// dostarczone pola!
		configs.apply(&mut instance);
		instance
	}

	pub fn step1a_valid_patterns(mut self) -> Result<Self> {
		let patts = HandlePreparedPatterns::build(
			&self.patterns.patterns_all_matched,
			&self.patterns.patterns_no_numerate,
			&self.patterns.patterns_no_copypast,
			self.patterns.case_sensitive,
			self.verboses.step1a_patterns,
		)
		.context("ETAP-1 [Błąd kroku 01]: Nieudana kompilacja wzorców")?;

		self.step1a_val_patts = Some(patts);
		Ok(self)
	}

	pub fn step1b_valid_entry_paths(mut self) -> Result<Self> {
		let entry = HandleVerifiedDirs::build(&self.patterns.entry_dirs, self.verboses.step1b_entrydir)
			.context("ETAP-1 [Błąd kroku 00]: Niepoprawne ścieżki wejściowe (entry_dirs)")?;

		self.step1b_val_entry = Some(entry);
		Ok(self)
	}

	pub fn step1c_valid_scan_walk(mut self) -> Result<Self> {
		let entry = self
			.step1b_val_entry
			.as_ref()
			.context("ETAP-1 [Błąd sekwencji]: Próba uruchomienia skanu przed walidacją wejścia!")?;

		// Skanowanie dysku z bezpośrednim odcięciem szufladek wg ClaimRefining
		let walk = HandleWalkScan::scan(entry, &self.refining, self.verboses.step1c_walk)
			.context("ETAP-1 [Błąd kroku 02]: Skanowanie dysku (HandleWalkScan) zakończone niepowodzeniem")?;

		self.step1c_val_walk = Some(walk);
		Ok(self)
	}

	pub fn step1d_valid_execute_query(mut self) -> Result<Self> {
		let entry = self
			.step1b_val_entry
			.as_ref()
			.context("ETAP-1 [Błąd sekwencji]: Brak katalogów wejściowych (entry_dirs) przed filtrowaniem!")?;

		let patts =
			self.step1a_val_patts.as_ref().context("ETAP-1 [Błąd sekwencji]: Brak wzorców przed filtrowaniem!")?;

		let walk = self
			.step1c_val_walk
			.as_ref()
			.context("ETAP-1 [Błąd sekwencji]: Brak drzewa katalogów przed filtrowaniem!")?;

		let execute = HandleCorpus::apply(
			entry,
			&patts.all_matched,
			walk,
			self.patterns.entry_dirs_ancestors,
			&self.ordering,
			self.verboses.step1d_execute,
		)
		.context("ETAP-1 [Błąd kroku 03]: Filtracja wyników nie powiodła się!")?;

		self.step1d_val_execute = Some(execute);
		Ok(self)
	}
	/// Gwarantuje zebranie w pełni zwalidowanych obiektów i przekazanie ich do
	/// zablokowanego stanu (Read-Only Engine).
	pub fn step99_if_all_valid_corect_then_gather_out_to_prepared_setup(
		mut self,
	) -> Result<crate::libs::querypath::controllers::prepar_setups::PreparedSetup> {
		let entry = self.step1b_val_entry.take().context("Pominięto step1b_valid_entry_paths w łańcuchu walidacji!")?;
		let patts = self.step1a_val_patts.take().context("Pominięto step1a_valid_patterns w łańcuchu walidacji!")?;
		let walk = self.step1c_val_walk.take().context("Pominięto step1c_valid_scan_walk w łańcuchu walidacji!")?;
		let execute =
			self.step1d_val_execute.take().context("Pominięto step1d_valid_execute_query w łańcuchu walidacji!")?;

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
