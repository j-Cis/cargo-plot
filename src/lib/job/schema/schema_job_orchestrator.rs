use crate::lib::job::{self, TraitParseFromFlags};

//#[derive(Debug, Clone)]
//pub struct ValidHashParams {
//	pub tag_time: crate::lib::logic::TagTime,
//}

#[derive(Debug, Clone)]
pub struct ValidPreparedJobParams {
	pub hash: String,
	pub exec: job::schema::ValidExecutionParams,
	pub work: job::schema::ValidWorkspaceParams,
	pub save: job::schema::ValidSaveAsParams,

	pub patt: job::schema::ValidPatternParams,
	pub part: job::schema::ValidTablePartParams,
	pub item: job::schema::ValidColumnItemParams,
	pub cols: job::schema::ValidTableColumnsParams,

	pub size: job::schema::ValidColumnSizeParams,
	pub date: job::schema::ValidColumnDateParams,
	pub time: job::schema::ValidColumnTimeParams,
	pub sort: job::schema::ValidSortByParams,
}
impl Default for ValidPreparedJobParams {
	fn default() -> Self {
		let blank: Vec<&str> = Vec::new();

		Self {
			hash: crate::lib::logic::tag_time().0,
			// 0. Execution Config (Domyślnie: Debug: true, SaveAs: true, CliColor: true)
			exec: job::schema::ValidExecutionFlags::parse_vec_as_config(&blank),
			// 1. Workspace (Domyślnie ".")
			work: job::schema::ValidWorkspaceFlags::parse_vec_as_config(["."]),
			// 10. Save As (Domyślnie "./target/.cargo-plot/", "Project Snapshot", SOTC/COTS names)
			save: job::schema::ValidSaveAsFlags::parse_vec_as_config(&blank),


			// 2. Patterns (Domyślnie zbiór filtrów zdefiniowany w config.rs)
			patt: job::schema::ValidPatternFlags::parse_vec_as_config(&blank),
			// 3. Table Parts (Domyślnie md + mf)
			part: job::schema::ValidTablePartFlags::parse_vec_as_config(&blank),
			// 4. Column Item (Domyślnie Tree, IconsLite, Name: false, Align: false)
			item: job::schema::ValidColumnItemFlags::parse_vec_as_config([
				"list-tree",
				"name-show",
				"ws-show",
				"icons-lite",
			]),
			// 5. Table Columns (Domyślnie Date, Time, Size, Item, Path)
			cols: job::schema::ValidTableColumnsFlags::parse_vec_as_config(&blank),
			// 6. Column Date (Domyślnie "%Y W%V %u-%a")
			date: job::schema::ValidColumnDateFlags::parse_vec_as_config(&blank),
			// 7. Column Time (Domyślnie "%H:%M:%S.%3f")
			time: job::schema::ValidColumnTimeFlags::parse_vec_as_config(&blank),
			// 8. Column Size (Domyślnie Decimal / SI)
			size: job::schema::ValidColumnSizeFlags::parse_vec_as_config(&blank),
			// 9. Sort By (Domyślnie Name, [Spec][Num][AZaz], Reverse: false)
			sort: job::schema::ValidSortByFlags::parse_vec_as_config(&blank),
		}
	}
}
