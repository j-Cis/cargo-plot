//! Path: `examples/example01.rs`
//! Last significant modified: 2026-W43-3 22:13:37 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use anyhow::Result;
use cargo_plot::querypath::{ClaimPatterns, DebugVerboses, GatherClaims};

fn main() -> Result<()> {
	println!("🚀 Start ETAPU-1: Konfiguracja i Walidacja\n");

	// 1. Zbieranie deklaracji (Roszczeń)
	// Używamy krotki (Tuple), aby nadpisać tylko wzorce i logowanie.
	// Reszta ustawień (Ordering, Creating) załaduje się domyślnie.
	let claims = GatherClaims::new((
		ClaimPatterns::new()
			.entry_dirs(["./{src|examples}/"])
			.patterns_all_matched(["**/temporal/**"])
			//.patterns_no_copypast(["*.svg"])
			.case_sensitive(false),
		DebugVerboses::new().step0_entrydir(false).step1_patterns(true).step2_walk(true).step3_execute(true),
	));

	// 2. Twarda walidacja (Pipeline)
	// Każdy etap weryfikuje dane i natychmiast wyrzuca błąd (?) w razie
	// pomyłki.
	let _prepared_setup = claims
		.step00_valid_entry_paths()?
		.step01_valid_patterns()?
		.step02_valid_scan_walk()?
		.step03_valid_execute_query()?
		.step99_if_all_valid_corect_then_gather_out_to_prepared_setup()?;

	println!("\n✅ ETAP-1 Zakończony. Otrzymano twardy `PreparedSetup`!");

	// ========================================================================
	// ETAP-2: Z tego miejsca zwalidowany setup zasili logikę rysowania drzewa
	// i generowania raportów.
	// ========================================================================

	// let a = prepared_setup.prepare_get_paths();
	// println!("Zeskanowano poprawnie i przygotowano pakiety konfiguracyjne.");

	Ok(())
}
