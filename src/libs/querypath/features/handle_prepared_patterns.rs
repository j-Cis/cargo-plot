//! Path: `src/libs/querypath/features/handle_prepared_patterns.rs`
//! Last significant modified: 2026-W43-3 22:13:17 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use crate::querypath::{features::VerboseDisplay, structs::PathsPatterns};
use anyhow::{Context, Result};
use std::fmt;

/// Komplet wzorców jednego zapytania, podzielony na grupy.
#[derive(Debug, Clone)]
pub struct HandlePreparedPatterns {
	pub all_matched: PathsPatterns,
	pub no_numerate: PathsPatterns,
	pub no_copypast: PathsPatterns,
}

impl HandlePreparedPatterns {
	pub fn build(
		all_matched: &[String],
		no_numerate: &[String],
		no_copypast: &[String],
		case_sensitive: bool,
		debug: bool,
	) -> Result<Self> {
		let instance = Self {
			all_matched: PathsPatterns::build(all_matched, case_sensitive)
				.context("Grupa 'all_matched': nieudana kompilacja wzorców")?,
			no_numerate: PathsPatterns::build(no_numerate, case_sensitive)
				.context("Grupa 'no_numerate': nieudana kompilacja wzorców")?,
			no_copypast: PathsPatterns::build(no_copypast, case_sensitive)
				.context("Grupa 'no_copypast': nieudana kompilacja wzorców")?,
		};

		if debug {
			println!("{}", instance.display_debug());
		}

		Ok(instance)
	}

	fn fmt_group(f: &mut fmt::Formatter<'_>, name: &str, group: &PathsPatterns) -> fmt::Result {
		if group.rules_count() == 0 {
			return Ok(());
		}
		writeln!(f, "── {name} ({} reguł, case_sensitive: {}) ──", group.rules_count(), group.case_sensitive)?;

		let mut cursor = 0;
		for (r_idx, raw) in group.raw.0.iter().enumerate() {
			let r_num = format!("{:02}", r_idx + 1);
			if r_idx > 0 {
				writeln!(f)?;
			}
			writeln!(f, "{r_num} raw 🔎: {raw:?}")?;

			for (e_idx, exp) in PathsPatterns::expand_braces(raw).iter().enumerate() {
				let e_num = format!("{:02}", e_idx + 1);
				if let Some(rule) = group.compiled.get(cursor) {
					writeln!(f, "{r_num}:{e_num}  🪄: 【{} 】 🧿: {exp:?}", rule.regex)?;
					writeln!(
						f,
						"{r_num}:{e_num}  ⚗️ : base name: {:?}, is negated: {} , dir only: {} , requires sibling: {} , requires orphan: {} ,",
						rule.base_name,
						if rule.is_negated { "✔️" } else { "❌" },
						if rule.targets_dir_only { "✔️" } else { "❌" },
						if rule.requires_sibling { "✔️" } else { "❌" },
						if rule.requires_orphan { "✔️" } else { "❌" }
					)?;
				}
				cursor += 1;
			}
		}
		Ok(())
	}
}

impl VerboseDisplay for HandlePreparedPatterns {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(
			f,
			"002📕 [ structs: HandlePreparedPatterns, PathsPatterns, PattRaw, PattExp, PatternCompiled, Regex ] 🔰"
		)?;

		let mut has_printed = false;
		for (name, group) in [
			("all_matched", &self.all_matched),
			("no_numerate", &self.no_numerate),
			("no_copypast", &self.no_copypast),
		] {
			if group.rules_count() > 0 {
				if has_printed {
					writeln!(f)?;
				}
				Self::fmt_group(f, name, group)?;
				has_printed = true;
			}
		}

		writeln!(
			f,
			"002📕 [ structs: HandlePreparedPatterns, PathsPatterns, PattRaw, PattExp, PatternCompiled, Regex ] 🏁"
		)?;
		Ok(())
	}
}
