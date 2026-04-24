use serde::{Deserialize, Serialize};

// ░░░░░░░░░░░░░░░░░░░░ all【jobs】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RawTomlFileJobs {
	pub job: RawTomlJobs,
}

#[derive(Debug, Deserialize)]
pub struct RawTomlValidJobs {
	pub job: Vec<RawTomlValidJob>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RawTomlJobs(pub Vec<RawTomlJob>);

// ░░░░░░░░░░░░░░░░░░░░ one【job】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize)]
pub struct RawTomlValidJob {
	pub id: String,
	pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RawTomlJob {
	pub id: String,
	pub description: Option<String>,
	pub run_mode: Option<Vec<SharedJobRunMode>>,
	pub explorer: Option<RawTomlJobExplorer>,
	pub export: Option<RawTomlJobExport>,
	pub attributes: Option<RawTomlJobAttributes>,
	pub tuples: Option<RawTomlJobTuples>,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【run_mode】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SharedJobRunMode {
	DryRun,
	Save,
	SaveWithInspection,
	PrintNothing,
	PrintOnlyWarning,
	PrintWithInspection,
	PrintColor,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【explorer】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RawTomlJobExplorer {
	pub workspace_dir: Option<String>,
	pub ignore_case: Option<bool>,
	pub patterns: Option<Vec<String>>,
	pub parts: Option<Vec<SharedJobPart>>,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【explorer】➔【part】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SharedJobPart {
	MD,
	MF,
	XD,
	XF,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【export】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RawTomlJobExport {
	pub out_dir: Option<String>,
	pub title: Option<String>,
	pub name: Option<String>,
	pub name_is_first: Option<bool>,
	pub save_separately: Option<bool>,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【attributes】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RawTomlJobAttributes {
	pub select: Option<Vec<SharedJobAttributeToSelect>>,
	pub option: Option<RawTomlJobAttributesOptions>,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【attributes】➔【select】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SharedJobAttributeToSelect {
	Date,
	Time,
	Size,
	Item,
	Path,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【attributes】➔【options】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RawTomlJobAttributesOptions {
	pub for_item: Option<Vec<SharedJobOptForAttrItem>>,
	pub for_date: Option<String>,
	pub for_time: Option<String>,
	pub for_size: Option<SharedJobOptForAttrSize>,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【attributes】➔【options】➔【item】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SharedJobOptForAttrItem {
	ListTree,
	ListFlat,
	IconsLite,
	IconsMore,
	IconsHide,
	NumListBef,
	NumListAft,
	NameHide,
	AlignHide,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【attributes】➔【options】➔【size】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SharedJobOptForAttrSize {
	Decimal,
	Binary,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RawTomlJobTuples {
	pub pile: Option<RawTomlJobTuplesPile>,
	pub sort: Option<RawTomlJobTuplesSort>,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【pile】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum RawTomlJobTuplesPile {
	Name(RawTomlJobPileMode),
	Exte(RawTomlJobPileMode),
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【pile】➔【mode】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RawTomlJobPileMode {
	pub dir_first: bool,
	pub same_name_dirs_and_files_nearby: bool,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【sort】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum RawTomlJobTuplesSort {
	Date(RawTomlJobSortNum),
	Size(RawTomlJobSortNum),
	Path(RawTomlJobSortTex),
	Name(RawTomlJobSortTex),
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【sort】➔【num】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RawTomlJobSortNum {
	pub reverse: bool,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【sort】➔【tex】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RawTomlJobSortTex {
	pub reverse: bool,
	pub mirror: bool,
	pub string_strategy: RawTomlJobStringStrategy,
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【sort】➔【tex】➔【strategy】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawTomlJobStringStrategy(pub [SharedJobStringMode; 3]);

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【sort】➔【tex】➔【strategy】➔【mode】 ░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SharedJobStringMode {
	Spec,
	Num,
	AaZz,
	aAzZ,
	AZaz,
	azAZ,
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
