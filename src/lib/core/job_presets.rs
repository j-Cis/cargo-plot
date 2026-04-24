use std::path::PathBuf;

use crate::lib::schema::{
	SharedJobAttributeToSelect,
	SharedJobOptForAttrItem,
	SharedJobOptForAttrSize,
	SharedJobPart,
	SharedJobRunMode,
	SharedJobStringMode,
};

// ░░░░░░░░░░░░░░░░░░░░ ŚRODOWISKO I ŚCIEŻKI ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

/// Zwraca bezwzględną ścieżkę, z której uruchomiono program (CWD).
pub fn execution_dir() -> PathBuf {
	std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).canonicalize().unwrap_or_else(|_| PathBuf::from("."))
}

/// Zwraca domyślną ścieżkę do pliku konfiguracyjnego:
/// <CWD>/target/.cargo-plot/task/CargoPlot.toml
pub fn default_config_path() -> PathBuf {
	execution_dir().join("target").join(".cargo-plot").join("tasks").join("CargoPlot.toml")
}

/// Domyślna zawartość nowo tworzonego pliku konfiguracyjnego TOML
pub const TOML_DEFAULT_MINIMAL: &str = r#"
[[job]]
id = "a1"
description = "default job"
"#;

// ░░░░░░░░░░░░░░░░░░░░ 【run_mode】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

pub fn default_run_modes() -> Vec<SharedJobRunMode> { vec![SharedJobRunMode::Save, SharedJobRunMode::PrintOnlyWarning] }

// ░░░░░░░░░░░░░░░░░░░░ 【explorer】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

pub fn default_workspace_dir() -> PathBuf { PathBuf::from(".") }

pub const DEFAULT_IGNORE_CASE: bool = false;

pub fn default_explorer_patterns() -> Vec<String> {
	vec![
		"./{.rustfmt,Cargo,rust-toolchain,Makefile}.toml&/".into(),
		"./**/*.rs&/".into(),
		"!./target/**".into(),
		"!./.git/**".into(),
		"./.{gitattributes,gitignore}".into(),
		"./.github/workflows/*.yml&/".into(),
		"./.vscode/settings.json&/".into(),
		"./{API, ARCHITECTURE, AUTHORS, CHANGELOG, README, ROADMAP, TODO}.md".into(),
		"./dist/{**/*,*}.{bat,exe}&/".into(),
	]
}

pub fn default_explorer_parts() -> Vec<SharedJobPart> { vec![SharedJobPart::MD, SharedJobPart::MF] }

// ░░░░░░░░░░░░░░░░░░░░ 【export】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

pub fn default_out_dir() -> PathBuf { PathBuf::from("./target/.cargo-plot/out/") }

pub const DEFAULT_TITLE: &str = "";
pub const DEFAULT_NAME: &str = "";
pub const DEFAULT_NAME_IS_FIRST: bool = false;
pub const DEFAULT_SAVE_SEPARATELY: bool = false;

// ░░░░░░░░░░░░░░░░░░░░ 【attributes】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

pub fn default_attributes_select() -> Vec<SharedJobAttributeToSelect> {
	vec![
		SharedJobAttributeToSelect::Date,
		SharedJobAttributeToSelect::Time,
		SharedJobAttributeToSelect::Size,
		SharedJobAttributeToSelect::Item,
		SharedJobAttributeToSelect::Path,
	]
}

pub fn default_for_item() -> Vec<SharedJobOptForAttrItem> {
	vec![
		SharedJobOptForAttrItem::ListTree,
		SharedJobOptForAttrItem::IconsLite,
		SharedJobOptForAttrItem::NumListAft,
	]
}

pub const DEFAULT_FOR_DATE: &str = "%Y W%V %u-%a";
pub const DEFAULT_FOR_TIME: &str = "%H:%M:%S.%3f";
pub const DEFAULT_FOR_SIZE: SharedJobOptForAttrSize = SharedJobOptForAttrSize::Decimal;

// ░░░░░░░░░░░░░░░░░░░░ 【tuples】 ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

pub const DEFAULT_DIR_FIRST: bool = false;
pub const DEFAULT_SAME_NAME_DIRS_AND_FILES_NEARBY: bool = false;

pub const DEFAULT_REVERSE: bool = false;
pub const DEFAULT_MIRROR: bool = false;

pub fn default_string_strategy() -> Vec<SharedJobStringMode> {
	vec![SharedJobStringMode::AaZz, SharedJobStringMode::Num, SharedJobStringMode::Spec]
}
