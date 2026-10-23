//! Path: `src/libs/querypath/controllers/gather_claims/structs.rs`
//! Last significant modified: 2026-W43-5 11:39 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

// use std::collections::HashMap;
use crate::libs::querypath::core::{
	features::{HandlePreparedPatterns, HandleVerifiedDirs, HandleWalkScan},
	types::enums::{
		EnumCasePrecedence, EnumCharClass, EnumColumn, EnumDirWeightDisplay, EnumGroupStrategy, EnumNoExtPriority,
		EnumSameNamePriority, EnumUnitSystem, EnumWeightPrecision,
	},
};

#[derive(Debug, Clone, Default)]
pub struct DebugVerboses {
	pub step1b_entrydir: bool,
	pub step1a_patterns: bool,
	pub step1c_walk: bool,
	pub step1d_execute: bool, // Dopisano krok 03 (filtrowanie)
}

/// Znacznik dla całkowicie domyślnych ustawień (szybki start).
pub struct ClaimDefaults;

// --- STRUKTURY DANYCH (CLAIMS) ---
#[derive(Debug, Clone, Default)]
pub struct ClaimPatterns {
	pub entry_dirs: Vec<String>,
	pub patterns_all_matched: Vec<String>,
	pub patterns_no_numerate: Vec<String>,
	pub patterns_no_copypast: Vec<String>,
	pub case_sensitive: bool,
	//pub pattern_temporal_in_tree: &'static str,
	//pub pattern_temporal_in_text: &'static str,
	//pub pattern_temporal_in_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct ClaimOrdering {
	pub order_group_strategy: EnumGroupStrategy,
	pub order_same_name_priority: EnumSameNamePriority,
	pub order_ignore_leading_dot: bool,
	pub order_split_extension: bool,
	pub order_no_ext_priority: EnumNoExtPriority,
	pub order_char_class_order: [EnumCharClass; 3],
	pub order_case_precedence: EnumCasePrecedence,
	pub order_reverse: bool,
	pub numerate_dirs: bool,
	pub numerate_binaries: bool,
	pub numerate_start_from: usize,
}

#[derive(Debug, Clone)]
pub struct ClaimCreating {
	pub wraper_name_width: usize,
	pub wraper_path_width: usize,
	pub column_order_left: Vec<EnumColumn>,
	pub column_order_right: Vec<EnumColumn>,
	pub weight_binary_base: EnumUnitSystem,
	pub weight_dir_display: EnumDirWeightDisplay,
	pub weight_precision: EnumWeightPrecision,
	pub keep_parent: bool,
	pub include_files: bool,
	pub include_binaries_in_matched: bool,
	pub title: &'static str,
	pub title_sufix_timestamp: bool,
	pub saveas: &'static str,
	pub saveas_sufix_timestamp: bool,
	pub output_dir: String,
	pub limit_max_single_file_size: u32,
	pub add_in_header_autoreklama: bool,
	pub add_in_footer_autoreklama: bool,
	pub add_in_header_table_setup: bool,
	pub add_in_footer_table_setup: bool,
	pub add_in_header_table_stats: bool,
	pub add_in_footer_table_stats: bool,
	pub add_in_header_paragraph: &'static str,
	pub add_in_footer_paragraph: &'static str,
}

pub enum Gather {
	Defaults,
	Patterns(ClaimPatterns),
	Ordering(ClaimOrdering),
	Creating(ClaimCreating),
	Verboses(DebugVerboses),
}

pub struct GatherClaims {
	pub patterns: ClaimPatterns,
	pub ordering: ClaimOrdering,
	pub creating: ClaimCreating,
	pub verboses: DebugVerboses,

	// Używamy zaktualizowanych ścieżek z katalogu features
	pub step00_val_entry: Option<HandleVerifiedDirs>,
	pub step01_val_patts: Option<HandlePreparedPatterns>,
	pub step02_val_walk: Option<HandleWalkScan>,
	pub step03_val_execute: Option<crate::libs::querypath::core::features::step04_execute_query::QueryExecute>,
	// step03_val_sorts
	// step04_val_filtr
	// step05_val_add_index
	// step06_val_get_stats
	// step07_val_result_paths
	// i wiele innych
}
