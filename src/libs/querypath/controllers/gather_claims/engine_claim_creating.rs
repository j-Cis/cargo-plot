use crate::libs::querypath::{
	controllers::{
		traits::ApplyClaim,
		types::{ClaimCreating, GatherClaims},
	},
	core::types::enums::{EnumColumn, EnumDirWeightDisplay, EnumUnitSystem, EnumWeightPrecision},
};

// Ręczna implementacja Default rozwiązuje błąd E0277
impl Default for ClaimCreating {
	fn default() -> Self {
		Self {
			wraper_name_width: 25,
			wraper_path_width: 35,
			column_order_left: vec![EnumColumn::Weight, EnumColumn::Temporal],
			column_order_right: vec![EnumColumn::Path],
			weight_binary_base: EnumUnitSystem::Binary,
			weight_dir_display: EnumDirWeightDisplay::MatchedOnly,
			weight_precision: EnumWeightPrecision::Tenths,
			include_files: true,
			include_binaries_in_matched: true,
			title: "CODE SNAPSHOT",
			title_sufix_timestamp: false,
			saveas: "snapshot_",
			saveas_sufix_timestamp: true,
			output_dir: "./dump".to_string(),
			limit_max_single_file_size: 524288,
			add_in_header_autoreklama: false,
			add_in_footer_autoreklama: true,
			add_in_header_table_setup: false,
			add_in_footer_table_setup: true,
			add_in_header_table_stats: false,
			add_in_footer_table_stats: false,
			add_in_header_paragraph: "",
			add_in_footer_paragraph: "",
		}
	}
}

impl ClaimCreating {
	#[must_use]
	pub fn new() -> Self { Self::default() }
}

impl ApplyClaim for ClaimCreating {
	fn apply(self, claims: &mut GatherClaims) { claims.creating = self; }
}
