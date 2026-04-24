use std::path::PathBuf;

use crate::lib::{
	core::{print_help_for_pattern_syntax_and_semantics, print_help_for_toml_config, start, start_blank},
	logic::file_save_safe_if_changed,
	mapping::{prepare, translate},
	schema::{RawCliJob, ReadyJob},
};

/// GŁÓWNY ROUTER CLI - tu zapadają decyzje i delegowane jest wykonanie potoku (callback)
pub fn route_and_execute<F>(cli: RawCliJob, mut execute_pipeline: F)
where F: FnMut(&ReadyJob) -> bool // ⚡ CALLBACK: Zwraca true, jeśli pipeline przeszedł bez błędów
{
	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	// 🚩 OBSŁUGA FLAGI: `-P` // POMOC
	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	if cli.help_for_pattern {
		print_help_for_pattern_syntax_and_semantics();
		return;
	}

	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	// 🚩 OBSŁUGA FLAGI: `-T` // POMOC
	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	if cli.help_for_config {
		print_help_for_toml_config();
		return;
	}

	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	// 🚩 OBSŁUGA FLAGI: `-L` // POMOC
	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	if let Some(path_to_show) = &cli.jobs_load_only_show {
		let path_opt = if path_to_show.is_empty() { None } else { Some(PathBuf::from(path_to_show)) };
		let s = start(path_opt);
		s.get_jobs(|job_raw| {
			let desc = job_raw.description.as_deref().unwrap_or("---");
			println!("   [ID: {}] Opis: {}", job_raw.id, desc);
		});
		return;
	}

	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	// 🚩 OBSŁUGA FLAG: `-l` oraz opcjonalnie `-t` // AUTOMATYZACJA
	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	if let Some(path_to_load) = &cli.jobs_load {
		let path_opt = if path_to_load.is_empty() { None } else { Some(PathBuf::from(path_to_load)) };
		let s = start(path_opt);
		let mut successful_jobs_toml = String::new();

		if let Some(ids) = &cli.task_id {
			s.get_jobs_some(ids, |job_raw| {
				let job = prepare(job_raw.clone());
				// ▶▶ CALLBACK Z UNIWERSALNY DLA PIPELINE
				if execute_pipeline(&job) {
					successful_jobs_toml.push_str("\n");
					successful_jobs_toml.push_str(&job.insert_as_toml());
				}
			});
		} else {
			s.get_jobs(|job_raw| {
				let job = prepare(job_raw.clone());
				// ▶▶ CALLBACK Z UNIWERSALNY DLA PIPELINE
				if execute_pipeline(&job) {
					successful_jobs_toml.push_str("\n");
					successful_jobs_toml.push_str(&job.insert_as_toml());
				}
			});
		}

		dump_last_successful_jobs(&s, &successful_jobs_toml);
		return;
	}

	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	// 🚩 OBSŁUGA FLAGI: `-b` // MANUAL
	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	if cli.job_blank {
		let s = start_blank(None);
		let mut successful_jobs_toml = String::new();

		if cli.speed {
			// ----------------------------------------------------------------
			// 🚀 SZYBKA ŚCIEŻKA MANUALNA (-b -s -d -o -p -q)
			// ----------------------------------------------------------------
			println!("⚙️ Tryb: SZYBKI (Fast Manual)");

			s.get_jobs(|job_blank_like_raw_toml| {
				// TODO: Tu wjedzie Twój zoptymalizowany, lekki middleware,
				// który bierze pod uwagę TYLKO flagi b/d/o/p/q i ignoruje resztę
				let job_raw_cli = translate(job_blank_like_raw_toml.clone(), &cli);
				let job = prepare(job_raw_cli);

				if execute_pipeline(&job) {
					successful_jobs_toml.push_str("\n");
					successful_jobs_toml.push_str(&job.insert_as_toml());
				}
			});
		} else {
			// ----------------------------------------------------------------
			// 🛠️ PEŁNY MANUAL (-b -d -o -p -q i ewentualne zaawansowane modyfikatory)
			// ----------------------------------------------------------------
			println!("⚙️ Tryb: PEŁNY (Full Manual)");

			s.get_jobs(|job_blank_like_raw_toml| {
				let job_raw_cli = translate(job_blank_like_raw_toml.clone(), &cli);
				let job = prepare(job_raw_cli);

				if execute_pipeline(&job) {
					successful_jobs_toml.push_str("\n");
					successful_jobs_toml.push_str(&job.insert_as_toml());
				}
			});
		}

		dump_last_successful_jobs(&s, &successful_jobs_toml);
		return;
	}
}

// ----------------------------------------------------------------------------

fn dump_last_successful_jobs(runtime: &crate::lib::core::AnchoredRuntime, jobs_toml_content: &str) {
	// Jeśli string jest pusty, nic się nie wykonało poprawnie
	if jobs_toml_content.is_empty() {
		return;
	}

	let mut out_path = runtime.path_xdo().to_path_buf();
	out_path.set_file_name("CargoPlotLast.toml");

	let mut final_content = String::from("# Automatyczny zapis poprawnie wykonanych zadań (ReadyJob)\n");
	final_content.push_str(jobs_toml_content);

	// UŻYWAMY TWOJEGO GOTOWCA! Robi backup i bezpiecznie nadpisuje:
	file_save_safe_if_changed(&out_path, &final_content);

	println!("💾 Zapisano recepturę zadania.");
	println!("📂 Ścieżka zapisu: {}", out_path.display());
}
