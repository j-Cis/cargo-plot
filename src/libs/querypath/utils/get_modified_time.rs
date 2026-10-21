//! Path: `src/libs/querypath/utils/get_modified_time.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::{fs, time::UNIX_EPOCH};

/// Odczyt czasu modyfikacji z metadanych w formacie UNIX timestamp.
pub fn get_modified_time(path_str: &str) -> Option<u64> {
	fs::metadata(path_str).ok()?.modified().ok()?.duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs())
}
