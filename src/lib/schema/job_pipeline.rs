use chrono::{DateTime, Local};

use crate::lib::logic::ScanNodeScanned;
// ============================================================================
// MODELE DANYCH POTOKU (Zastępują stare job::ValidResultMainRow)
// ============================================================================

#[derive(Debug, Clone)]
pub struct PipelineJobRow {
	pub dt_modified: DateTime<Local>,
	pub name_with_ext: String,
	pub size_real: u64,
	pub node: ScanNodeScanned,
}

#[derive(Debug, Clone)]
pub struct PipelineJobTab {
	pub rows: Vec<PipelineJobRow>,
	pub tier_max: usize,
	pub name_len_max: usize,
	pub path_len_max: usize,
}
