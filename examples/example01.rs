//! Path: `examples/example01.rs`
use anyhow::Result;
use cargo_plot::querypath::{ClaimPatterns, DebugVerboses, GatherClaims, ClaimRefining};

fn main() -> Result<()> {
	let claims = GatherClaims::new((
		ClaimPatterns::new()
			.patterns_all_matched(["*"])
			//.patterns_no_copypast(["*.svg"])
			.entry_dirs(["./{src/libs/timer|examples}/"])
			.entry_dirs_ancestors(false)
			.case_sensitive(false),
		ClaimRefining::new().skip_nodoted_withext_files_text(false),
		DebugVerboses::new()
			.step1a_patterns(false)
			.step1b_entrydir(false)
			.step1c_walk(false)
			.step1d_execute(true),
	));

	let _prepared_setup = claims
		.step1a_valid_patterns()?
		.step1b_valid_entry_paths()?
		.step1c_valid_scan_walk()?
		.step1d_valid_execute_query()?
		.step99_if_all_valid_corect_then_gather_out_to_prepared_setup()?;
	Ok(())
}


