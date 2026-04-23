use std::path::PathBuf;

// ░░░░░░░░░░░░░░░░░░░░ EXECUTION / START ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidExecutionFlags {
	DryRun(bool),
	Save(bool),
	SaveWithInspection(bool),
	PrintOnlyWarning(bool),
	PrintNothing(bool),
	PrintInColor(bool),
	PrintWithInspection(bool),
}

#[derive(Debug, Clone)]
pub struct ValidExecutionParams {
	pub dry_run: bool,
    pub save: bool,
    pub save_with_inspection: bool,
	pub print_only_warning: bool,
	pub print_nothing: bool,
	pub print_in_color: bool,
	pub print_with_inspection: bool,
}

pub struct PartialExecutionRead {
    pub dry_run: bool,
    pub save: bool,
    pub save_with_inspection: bool,
    pub print_only_warning: bool,
    pub print_nothing: bool,
    pub print_in_color: bool,
    pub print_with_inspection: bool,
    pub has_any: bool,
	pub has_conflict: bool,
}

// ░░░░░░░░░░░░░░░░░░░░ IO: WORKSPACE_DIR ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidWorkspaceFlags {
    Path(String),
    IgnoreCase(bool),
}

#[derive(Debug, Clone)]
pub struct ValidWorkspaceParams {
	pub workspace_raw: String,
	pub workspace_dir: PathBuf,
	pub execution_dir: PathBuf,
	pub ignore_case: bool,
}

pub struct PartialWorkspaceRead {
    pub path: Option<String>,
    pub ignore_case: bool,
    pub has_any: bool,
	pub has_conflict: bool,
}

// ░░░░░░░░░░░░░░░░░░░░ IO: SAVE AS ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidSaveAsFlags {
	OutDir(String),
	Title(String),
	Name(String),
	NameIsPrefix(bool),
	NotSeparately(bool),
}

//#[derive(Debug, Clone)]
//pub struct ValidSaveAsParams {
//	pub out_dir: PathBuf,
//	pub out_raw: String,
//	pub title: String,
//	pub name: String,
//	pub name_is_prefix: bool,
//	pub not_separately: bool,
//}

//pub struct PartialSaveAsRead {
//    pub out_dir_str: Option<String>,
//    pub title: Option<String>,
//    pub name: Option<String>,
//    pub name_is_prefix: Option<bool>,
//    pub not_separately: Option<bool>,
//    pub has_any: bool,//
//	pub has_conflict: bool,
//}

// ░░░░░░░░░░░░░░░░░░░░ MAIN PARAMS ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone)]
pub enum ValidTablePartFlags {
	MD,
	MF,
	XD,
	XF,
}
#[derive(Debug, Clone)]
pub struct ValidTablePartParams {
	pub md: bool,
	pub mf: bool,
	pub xd: bool,
	pub xf: bool,
}

pub struct PartialTablePartRead {
    pub md: bool,
    pub mf: bool,
    pub xd: bool,
    pub xf: bool,
    pub has_any: bool,
	pub has_conflict: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidPatternFlags {
    Pattern(String),
}

#[derive(Debug, Clone)]
pub struct ValidPatternParams {
	pub patterns: Vec<String>,
}

pub struct PartialPatternRead {
    pub patterns: Vec<String>,
    pub has_any: bool,
	pub has_conflict: bool,
}

// ░░░░░░░░░░░░░░░░░░░░ STYLE MAIN ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidTableColumnsFlags {
	Date,
	Time,
	Size,
	Item,
	Path,
}

#[derive(Debug, Clone)]
pub struct ValidTableColumnsParams {
	pub columns: Vec<ValidTableColumnsFlags>,
}
pub struct PartialTableColumnsRead {
    pub columns: Vec<ValidTableColumnsFlags>,
    pub has_date: bool,
    pub has_time: bool,
    pub has_size: bool,
    pub has_item: bool,
    pub has_path: bool,
    pub has_any: bool,
	pub has_conflict: bool,
}

#[derive(Debug, Clone)]
pub enum ValidColumnItemFlags {
	ListNone,
	ListTree,
	ListFlat,
	IconsNone,
	IconsLite,
	IconsMore,

	NumPrefix,
	NumSuffix,
	NameNone,
	NameShow,
	WhitespaceTrailNone,
	WhitespaceTrailShow,
}

#[derive(Debug, Clone)]
pub struct ValidColumnItemParams {
	pub list: ModeListForValidColumnItem,
	pub icons: ModeIconsForValidColumnItem,
	pub name: bool,
	pub align_end: bool,
	pub num_is_first: bool,
}

pub struct PartialColumnItemRead {
    pub list: Option<ModeListForValidColumnItem>,
    pub icons: Option<ModeIconsForValidColumnItem>,
    pub num_is_first: Option<bool>,
    pub name: Option<bool>,
    pub align_end: Option<bool>,
    // Liczniki dla strategii rozwiązywania sprzeczności
    pub list_count: usize,
    pub icons_count: usize,
    pub num_count: usize,
    pub name_count: usize,
    pub ws_count: usize,
    pub has_any: bool,
	pub has_conflict: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeListForValidColumnItem {
	None,
	Flat,
	Tree,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeIconsForValidColumnItem {
	Lite,
	More,
	None,
}

// ░░░░░░░░░░░░░░░░░░░░ STYLE PLUS ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnSizeFlags {
	Decimal, // System SI (podstawa 1000)
	Binary,  // System IEC (podstawa 1024)
}

#[derive(Debug, Clone)]
pub struct ValidColumnSizeParams {
	pub mode: ValidColumnSizeFlags,
}

pub struct PartialColumnSizeRead {
    pub mode: Option<crate::lib::job::schema::ValidColumnSizeFlags>,
    pub has_any: bool,
	pub has_conflict: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnDateFlags {
	Default,
}

#[derive(Debug, Clone)]
pub struct ValidColumnDateParams {
	pub format: String,
}

pub struct PartialColumnDateRead {
    pub format: Option<String>,
    pub has_any: bool,
	pub has_conflict: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidColumnTimeFlags {
	Default,
}

#[derive(Debug, Clone)]
pub struct ValidColumnTimeParams {
	pub format: String,
}

pub struct PartialColumnTimeRead {
    pub format: Option<String>,
    pub has_any: bool,
	pub has_conflict: bool,
}
