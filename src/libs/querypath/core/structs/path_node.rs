//! Path: `src/libs/querypath/structs/path_node.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use crate::querypath::core::utils::normalize_path;
use std::path::PathBuf;

/// Reprezentacja pojedynczego węzła ścieżki przechowująca zarówno
/// oryginalny obiekt systemowy (`PathBuf`), jak i szybką, znormalizowaną
/// postać tekstową (`String`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathNode {
	pub buf: PathBuf,
	pub str: String,
}

impl PathNode {
	pub fn new(buf: PathBuf) -> Self {
		let str = normalize_path(&buf);
		Self { buf, str }
	}
}
