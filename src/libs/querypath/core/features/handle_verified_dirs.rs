//! Path: `src/libs/querypath/features/handle_verified_dirs.rs`
//! Last significant modified: 2026-W43-3 14:55:46 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use crate::querypath::core::{features::VerboseDisplay, structs::PathAnchored, utils::normalize_path};
use anyhow::{Context, Result, bail};
use std::{env, fmt, fs, path::PathBuf};

/// Menadżer zarządzający wszystkimi lokalizacjami startowymi skanowania.
/// Rozwija alternatywy `{a|b}` oraz weryfikuje ich istnienie.
#[derive(Debug, Clone)]
pub struct HandleVerifiedDirs {
	/// Napisy podane przez użytkownika (po `trim`, puste zamienione na `./`).
	pub raw: Vec<String>,
	pub handles: Vec<PathAnchored>,
	pub cwd_buf: PathBuf,
	pub cwd_str: String,
}

impl HandleVerifiedDirs {
	/// Inicjuje listę celów skanowania na podstawie podanych przez użytkownika
	/// stringów. Jeśli lista jest pusta lub zawiera puste napisy (`""`),
	/// domyślnie używa katalogu roboczego (`./`).
	pub fn build<I, S>(inputs: I, debug: bool) -> Result<Self>
	where
		I: IntoIterator<Item = S>,
		S: AsRef<str>,
	{
		// 1. Ustalenie kanonicznego CWD (również przepuszczonego przez
		//    fs::canonicalize)
		let raw_cwd = env::current_dir().context("Nie można odczytać katalogu roboczego (CWD)")?;
		let cwd_buf = fs::canonicalize(&raw_cwd).unwrap_or(raw_cwd);
		let cwd_str = normalize_path(&cwd_buf);

		let mut raw_inputs: Vec<String> = Vec::new();
		for input in inputs {
			let s = input.as_ref().trim();
			if s.is_empty() {
				raw_inputs.push("./".to_string());
			} else {
				raw_inputs.push(s.to_string());
			}
		}

		// 2. Jeśli po przefiltrowaniu lista jest pusta - domyślnie badamy CWD
		if raw_inputs.is_empty() {
			raw_inputs.push("./".to_string());
		}

		let raw = raw_inputs.clone();

		let mut expanded_paths = Vec::new();
		for input in raw_inputs {
			expanded_paths.extend(Self::expand_alternatives(&input));
		}

		// 3. Budowanie punktów zakotwiczenia.
		let mut handles: Vec<PathAnchored> = Vec::new();
		for expanded in expanded_paths {
			if let Ok(anchored) = PathAnchored::resolve(&expanded, &cwd_buf)
				&& !handles.iter().any(|handle| handle.buf == anchored.buf)
			{
				handles.push(anchored);
			}
		}

		if handles.is_empty() {
			bail!("Żadna z podanych lub rozwiniętych ścieżek wejściowych nie istnieje na dysku.");
		}

		let instance = Self { raw, handles, cwd_buf, cwd_str };

		if debug {
			println!("{}", instance.display_debug()); // używasz swojego traitu
		}

		Ok(instance)
	}

	/// Rekurencyjnie rozwija alternatywne bloki zapisane w klamrach,
	/// np.: `./{abc/ao/|def/}` -> `./abc/ao/` oraz `./def/`
	fn expand_alternatives(input: &str) -> Vec<String> {
		let Some(end) = input.find('}') else {
			return vec![input.to_string()];
		};

		let Some(sub) = input.get(..end) else {
			return vec![input.to_string()];
		};

		let Some(start) = sub.rfind('{') else {
			return vec![input.to_string()];
		};

		let (Some(prefix), Some(suffix), Some(options)) =
			(input.get(..start), input.get(end + 1..), input.get(start + 1..end))
		else {
			return vec![input.to_string()];
		};

		let mut expanded = Vec::new();

		for opt in options.split('|') {
			let merged = format!("{prefix}{opt}{suffix}");
			expanded.extend(Self::expand_alternatives(&merged));
		}

		expanded
	}
}

// ============================================================================
// FORMATOWANIE WYNIKOWE
// ============================================================================

impl VerboseDisplay for HandleVerifiedDirs {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "1b📕 [ structs: HandleVerifiedDirs, PathAnchored ] 🔰")?;

		writeln!(f, "cwd str 👀: {:?}", self.cwd_str)?;
		writeln!(f, "cwd buf 👀: {:?}", self.cwd_buf.to_string_lossy())?;

		let raw = self.raw.join(", ");
		writeln!(f, "raw paths 💡: {:?}", raw)?;
		for (i, handle) in self.handles.iter().enumerate() {
			let num = format!("{:>2}", i + 1);
			writeln!(f, "path {num} rel 🎯: {:?}", handle.rel)?;
			writeln!(f, "path {num} str 🎯: {:?}", handle.str)?;
			writeln!(f, "path {num} buf 🎯: {:?}", handle.buf.to_string_lossy())?;
		}
		writeln!(f, "1b📕 [ structs: HandleVerifiedDirs, PathAnchored ] 🏁")?;

		Ok(())
	}
}
