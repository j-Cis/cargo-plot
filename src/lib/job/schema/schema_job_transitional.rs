use chrono;

use crate::lib::logic::{self};

pub const IS_DEBUG: bool = true;
/// Reprezentuje finalny wiersz gotowy do tabeli, zawierający wszystkie
/// zgromadzone metadane z dysku i z etapu skanowania.
#[derive(Debug, Clone)]
pub struct ValidResultMainRow {
	pub dt_modified: chrono::DateTime<chrono::Local>,
	pub name_with_ext: String,
	pub size_real: u64,
	pub node: logic::ScannedNode,
}

#[derive(Debug, Clone)]
pub struct ValidResultMainTab {
	pub rows: Vec<ValidResultMainRow>,
	pub tier_max: usize,
	pub name_len_max: usize,
	pub path_len_max: usize,
}
