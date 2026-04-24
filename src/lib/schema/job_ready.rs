use std::path::PathBuf;

use serde::Serialize;

use super::{
	SharedJobAttributeToSelect,
	SharedJobOptForAttrItem,
	SharedJobOptForAttrSize,
	SharedJobPart,
	SharedJobRunMode,
	SharedJobStringMode,
};

// ░░░░░░░░░░░░░░░░░░░░ one【job】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReadyJob {
	id: String,
	description: String,
	#[serde(rename = "run-mode")]
	run_modes: Vec<SharedJobRunMode>,
	explorer: ReadyJobExplorer,
	export: ReadyJobExport,
	attributes: ReadyJobAttributes,
	tuples: ReadyJobTuples,
}

impl ReadyJob {
	pub fn id(&self) -> &str { &self.id }
	pub fn description(&self) -> &str { &self.description }
	pub fn run_modes(&self) -> &[SharedJobRunMode] { &self.run_modes }
	pub fn explorer(&self) -> &ReadyJobExplorer { &self.explorer }
	pub fn export(&self) -> &ReadyJobExport { &self.export }
	pub fn attributes(&self) -> &ReadyJobAttributes { &self.attributes }
	pub fn tuples(&self) -> &ReadyJobTuples { &self.tuples }

	// Konstruktor tylko dla warstwy mapującej
	pub(crate) fn new(
		id: String,
		description: String,
		run_modes: Vec<SharedJobRunMode>,
		explorer: ReadyJobExplorer,
		export: ReadyJobExport,
		attributes: ReadyJobAttributes,
		tuples: ReadyJobTuples,
	) -> Self {
		Self { id, description, run_modes, explorer, export, attributes, tuples }
	}

	/// Generuje sformatowaną reprezentację zadania w formacie TOML.
	/// Metoda ta pozwala na łatwe "wstrzyknięcie" gotowego zadania z powrotem do pliku konfiguracyjnego.
	pub fn insert_as_toml(&self) -> String {
        // Tworzymy lokalną strukturę jednorazową specjalnie pod serializację
        #[derive(serde::Serialize)]
        struct TomlWrapper<'a> {
            job: Vec<&'a ReadyJob>, 
        }

        let wrapper = TomlWrapper { job: vec![self] };

        // Serializujemy owijkę. Serde samo wstawi [[job]] i [job.explorer]!
        toml::to_string_pretty(&wrapper).unwrap_or_else(|_| format!("id = \"{}\"", self.id))
    }
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【explorer】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReadyJobExplorer {
	workspace_dir: PathBuf,
	ignore_case: bool,
	patterns: Vec<String>,
	parts: Vec<SharedJobPart>,
}

impl ReadyJobExplorer {
	pub fn workspace_dir(&self) -> &PathBuf { &self.workspace_dir }
	pub fn ignore_case(&self) -> bool { self.ignore_case }
	pub fn patterns(&self) -> &[String] { &self.patterns }
	pub fn parts(&self) -> &[SharedJobPart] { &self.parts }

	pub(crate) fn new(
		workspace_dir: PathBuf,
		ignore_case: bool,
		patterns: Vec<String>,
		parts: Vec<SharedJobPart>,
	) -> Self {
		Self { workspace_dir, ignore_case, patterns, parts }
	}
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【export】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReadyJobExport {
	out_dir: PathBuf,
	title: String,
	name: String,
	name_is_first: bool,
	save_separately: bool,
}

impl ReadyJobExport {
	pub fn out_dir(&self) -> &PathBuf { &self.out_dir }
	pub fn title(&self) -> &str { &self.title }
	pub fn name(&self) -> &str { &self.name }
	pub fn name_is_first(&self) -> bool { self.name_is_first }
	pub fn save_separately(&self) -> bool { self.save_separately }

	pub(crate) fn new(
		out_dir: PathBuf,
		title: String,
		name: String,
		name_is_first: bool,
		save_separately: bool,
	) -> Self {
		Self { out_dir, title, name, name_is_first, save_separately }
	}
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【attributes】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReadyJobAttributes {
	select: Vec<SharedJobAttributeToSelect>,
	for_item: Vec<SharedJobOptForAttrItem>,
	for_date: String,
	for_time: String,
	for_size: SharedJobOptForAttrSize,
}

impl ReadyJobAttributes {
	pub fn select(&self) -> &[SharedJobAttributeToSelect] { &self.select }
	pub fn for_item(&self) -> &[SharedJobOptForAttrItem] { &self.for_item }
	pub fn for_date(&self) -> &str { &self.for_date }
	pub fn for_time(&self) -> &str { &self.for_time }
	pub fn for_size(&self) -> SharedJobOptForAttrSize { self.for_size }

	pub(crate) fn new(
		select: Vec<SharedJobAttributeToSelect>,
		for_item: Vec<SharedJobOptForAttrItem>,
		for_date: String,
		for_time: String,
		for_size: SharedJobOptForAttrSize,
	) -> Self {
		Self { select, for_item, for_date, for_time, for_size }
	}
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReadyJobTuples {
	pile: ReadyJobTuplesPile,
	sort: ReadyJobTuplesSort,
}

impl ReadyJobTuples {
	pub fn pile(&self) -> &ReadyJobTuplesPile { &self.pile }
	pub fn sort(&self) -> &ReadyJobTuplesSort { &self.sort }

	pub(crate) fn new(pile: ReadyJobTuplesPile, sort: ReadyJobTuplesSort) -> Self { Self { pile, sort } }
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【pile】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ReadyJobTuplesPile {
	Name(ReadyJobPileMode),
	Exte(ReadyJobPileMode),
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【pile】➔【mode】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReadyJobPileMode {
	dir_first: bool,
	same_name_dirs_and_files_nearby: bool,
}

impl ReadyJobPileMode {
	pub fn dir_first(&self) -> bool { self.dir_first }
	pub fn same_name_dirs_and_files_nearby(&self) -> bool { self.same_name_dirs_and_files_nearby }

	pub(crate) fn new(dir_first: bool, same_name_dirs_and_files_nearby: bool) -> Self {
		Self { dir_first, same_name_dirs_and_files_nearby }
	}
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【sort】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ReadyJobTuplesSort {
	Path(ReadyJobSortTex),
	Name(ReadyJobSortTex),
	Date(ReadyJobSortNum),
	Size(ReadyJobSortNum),
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【sort】➔【num】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReadyJobSortNum {
	reverse: bool,
}

impl ReadyJobSortNum {
	pub fn reverse(&self) -> bool { self.reverse }

	pub(crate) fn new(reverse: bool) -> Self { Self { reverse } }
}

// ░░░░░░░░░░░░░░░░░░░░ 【job】➔【tuples】➔【sort】➔【tex】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReadyJobSortTex {
	reverse: bool,
	mirror: bool,
	string_strategy: Vec<SharedJobStringMode>,
}

impl ReadyJobSortTex {
	pub fn reverse(&self) -> bool { self.reverse }
	pub fn mirror(&self) -> bool { self.mirror }
	pub fn string_strategy(&self) -> &[SharedJobStringMode] { &self.string_strategy }

	pub(crate) fn new(reverse: bool, mirror: bool, string_strategy: Vec<SharedJobStringMode>) -> Self {
		Self { reverse, mirror, string_strategy }
	}
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
