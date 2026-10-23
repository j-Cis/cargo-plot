//! Path: `src/libs/querypath/structs/anchored_path.rs`
//! Last significant modified: 2026-W43-3 13:57:35 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use anyhow::{Context, Result};
use std::{
	fs,
	path::{Path, PathBuf},
};

use crate::querypath::core::utils::{diff_paths, normalize_path};
// ============================================================================
// RELACJE I ZAKOTWICZENIE (ANCHORING)
// ============================================================================

/// Odwzorowanie relacji pomiędzy miejscem odpalenia programu (CWD),
/// a fizycznym celem skanowania, wyliczające zawsze bezpieczną ścieżkę
/// relatywną.
#[derive(Debug, Clone)]
pub struct PathAnchored {
	pub buf: PathBuf, // kanoniczna ścieżka absolutna
	pub str: String,  // normalize_path(&buf)
	pub rel: String,  // Ścieżka pomostowa względem CWD
}

impl PathAnchored {
	/// Tworzy nowy punkt zakotwiczenia. Zwraca błąd, jeśli ścieżka docelowa
	/// nie istnieje na dysku (wczesna walidacja z kontekstem).
	pub fn resolve<P: AsRef<Path>>(input: P, cwd: &Path) -> Result<Self> {
		let input_ref = input.as_ref();

		// Kanonizacja rozwija m.in. symlinki, '..' i weryfikuje istnienie na
		// dysku.
		let buf = fs::canonicalize(input_ref)
			.with_context(|| format!("Ścieżka docelowa nie istnieje lub jest niedostępna: {:?}", input_ref))?;
		let str = normalize_path(&buf);
		let is_dir = buf.is_dir();

		// Wyliczamy relację (jak dostać się z CWD do Targetu).
		/*
		 * let rel = match buf.strip_prefix(cwd) {
		 * 	Ok(rel) => {
		 * 		let clean_rel = normalize_path(rel).trim_matches('/').to_string();
		 * 		if clean_rel.is_empty() {
		 * 			"./".to_string()
		 * 		} else if is_dir {
		 * 			format!("./{clean_rel}/")
		 * 		} else {
		 * 			format!("./{clean_rel}")
		 * 		}
		 * 	}
		 * 	Err(_) => {
		 * 		let mut s = normalize_path(&buf);
		 * 		if is_dir && !s.ends_with('/') {
		 * 			s.push('/');
		 * 		}
		 * 		s
		 * 	}
		 * };
		 * Ok(Self { buf, str, rel })
		 *
		 * */
		let rel = match diff_paths(&buf, cwd) {
			Some(rel_path) => {
				let clean_rel = normalize_path(&rel_path).trim_matches('/').to_string();
				let prefix = if clean_rel == ".." || clean_rel.starts_with("../") { "" } else { "./" };

				if clean_rel.is_empty() {
					"./".to_string()
				} else if is_dir {
					format!("{prefix}{clean_rel}/")
				} else {
					format!("{prefix}{clean_rel}")
				}
			}
			None => {
				let mut s = str.clone();
				if is_dir && !s.ends_with('/') {
					s.push('/');
				}
				s
			}
		};

		Ok(Self { buf, str, rel })
	}
}
