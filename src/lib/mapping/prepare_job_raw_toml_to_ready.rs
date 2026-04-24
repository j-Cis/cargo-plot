use std::path::PathBuf;

use crate::lib::{
	core::{
		DEFAULT_DIR_FIRST,
		DEFAULT_FOR_DATE,
		DEFAULT_FOR_SIZE,
		DEFAULT_FOR_TIME,
		DEFAULT_IGNORE_CASE,
		DEFAULT_MIRROR,
		DEFAULT_NAME,
		DEFAULT_NAME_IS_FIRST,
		DEFAULT_REVERSE,
		DEFAULT_SAME_NAME_DIRS_AND_FILES_NEARBY,
		DEFAULT_SAVE_SEPARATELY,
		DEFAULT_TITLE,
		default_attributes_select,
		default_explorer_parts,
		default_explorer_patterns,
		default_for_item,
		default_out_dir,
		default_run_modes,
		default_string_strategy,
		default_workspace_dir,
	},
	schema::{
		RawTomlJob,
		RawTomlJobSortTex,
		RawTomlJobTuplesPile,
		RawTomlJobTuplesSort,
		ReadyJob,
		ReadyJobAttributes,
		ReadyJobExplorer,
		ReadyJobExport,
		ReadyJobPileMode,
		ReadyJobSortNum,
		ReadyJobSortTex,
		ReadyJobTuples,
		ReadyJobTuplesPile,
		ReadyJobTuplesSort,
		SharedJobOptForAttrItem,
		SharedJobRunMode,
		SharedJobStringMode,
	},
};

/// Przygotowuje zadanie: pobiera surowy obiekt TOML, aplikuje domyślne ustawienia
/// dla brakujących pól i rozwiązuje ewentualne konflikty logiczne.
pub fn prepare(raw: RawTomlJob) -> ReadyJob {
	let id = raw.id;
	let description = raw.description.unwrap_or_default();

	// ░░░░░░░░░░ 1. RUN MODES ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	let raw_modes = raw.run_mode.unwrap_or_else(default_run_modes);
	let run_modes = resolve_run_modes(raw_modes);

	// ░░░░░░░░░░ 2. EXPLORER ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	let explorer = match raw.explorer {
		Some(ex) => ReadyJobExplorer::new(
			ex.workspace_dir.map(PathBuf::from).unwrap_or_else(default_workspace_dir),
			ex.ignore_case.unwrap_or(DEFAULT_IGNORE_CASE),
			ex.patterns.unwrap_or_else(default_explorer_patterns),
			ex.parts.unwrap_or_else(default_explorer_parts),
		),
		None => ReadyJobExplorer::new(
			default_workspace_dir(),
			DEFAULT_IGNORE_CASE,
			default_explorer_patterns(),
			default_explorer_parts(),
		),
	};

	// ░░░░░░░░░░ 3. EXPORT ░░░░░░░░░░
	let export = match raw.export {
		Some(ex) => ReadyJobExport::new(
			ex.out_dir.map(PathBuf::from).unwrap_or_else(default_out_dir),
			ex.title.unwrap_or_else(|| DEFAULT_TITLE.to_string()),
			ex.name.unwrap_or_else(|| DEFAULT_NAME.to_string()),
			ex.name_is_first.unwrap_or(DEFAULT_NAME_IS_FIRST),
			ex.save_separately.unwrap_or(DEFAULT_SAVE_SEPARATELY),
		),
		None => ReadyJobExport::new(
			default_out_dir(),
			DEFAULT_TITLE.to_string(),
			DEFAULT_NAME.to_string(),
			DEFAULT_NAME_IS_FIRST,
			DEFAULT_SAVE_SEPARATELY,
		),
	};

	// ░░░░░░░░░░ 4. ATTRIBUTES ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	let attributes = match raw.attributes {
		Some(attr) => {
			let select = attr.select.unwrap_or_else(default_attributes_select);
			let (for_item, for_date, for_time, for_size) = match attr.option {
				Some(opt) => (
					resolve_attr_items(opt.for_item.unwrap_or_else(default_for_item)),
					opt.for_date.unwrap_or_else(|| DEFAULT_FOR_DATE.to_string()),
					opt.for_time.unwrap_or_else(|| DEFAULT_FOR_TIME.to_string()),
					opt.for_size.unwrap_or(DEFAULT_FOR_SIZE),
				),
				None => {
					(default_for_item(), DEFAULT_FOR_DATE.to_string(), DEFAULT_FOR_TIME.to_string(), DEFAULT_FOR_SIZE)
				}
			};
			ReadyJobAttributes::new(select, for_item, for_date, for_time, for_size)
		}
		None => ReadyJobAttributes::new(
			default_attributes_select(),
			default_for_item(),
			DEFAULT_FOR_DATE.to_string(),
			DEFAULT_FOR_TIME.to_string(),
			DEFAULT_FOR_SIZE,
		),
	};

	// ░░░░░░░░░░ 5. TUPLES ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	let tuples = match raw.tuples {
		Some(t) => {
			let pile = match t.pile {
				Some(RawTomlJobTuplesPile::Name(p)) => {
					ReadyJobTuplesPile::Name(ReadyJobPileMode::new(p.dir_first, p.same_name_dirs_and_files_nearby))
				}
				Some(RawTomlJobTuplesPile::Exte(p)) => {
					ReadyJobTuplesPile::Exte(ReadyJobPileMode::new(p.dir_first, p.same_name_dirs_and_files_nearby))
				}
				None => ReadyJobTuplesPile::Name(ReadyJobPileMode::new(
					DEFAULT_DIR_FIRST,
					DEFAULT_SAME_NAME_DIRS_AND_FILES_NEARBY,
				)),
			};
			let sort = match t.sort {
				Some(RawTomlJobTuplesSort::Path(s)) => ReadyJobTuplesSort::Path(resolve_sort_tex(s)),
				Some(RawTomlJobTuplesSort::Name(s)) => ReadyJobTuplesSort::Name(resolve_sort_tex(s)),
				Some(RawTomlJobTuplesSort::Date(s)) => ReadyJobTuplesSort::Date(ReadyJobSortNum::new(s.reverse)),
				Some(RawTomlJobTuplesSort::Size(s)) => ReadyJobTuplesSort::Size(ReadyJobSortNum::new(s.reverse)),
				None => ReadyJobTuplesSort::Name(ReadyJobSortTex::new(
					DEFAULT_REVERSE,
					DEFAULT_MIRROR,
					default_string_strategy(),
				)),
			};
			ReadyJobTuples::new(pile, sort)
		}
		None => ReadyJobTuples::new(
			ReadyJobTuplesPile::Name(ReadyJobPileMode::new(DEFAULT_DIR_FIRST, DEFAULT_SAME_NAME_DIRS_AND_FILES_NEARBY)),
			ReadyJobTuplesSort::Name(ReadyJobSortTex::new(DEFAULT_REVERSE, DEFAULT_MIRROR, default_string_strategy())),
		),
	};

	// Finał: Wrzucamy wszystko do bezpiecznego konstruktora
	ReadyJob::new(id, description, run_modes, explorer, export, attributes, tuples)
}

// ░░░░░░░░░░░░░░░░░░░░ REGUŁY BIZNESOWE (Korekta konfliktów) ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

fn resolve_run_modes(modes: Vec<SharedJobRunMode>) -> Vec<SharedJobRunMode> {
	let mut resolved = Vec::new();
	for mode in &modes {
		let keep = match mode {
			SharedJobRunMode::Save | SharedJobRunMode::SaveWithInspection => !modes.contains(&SharedJobRunMode::DryRun),
			SharedJobRunMode::PrintOnlyWarning | SharedJobRunMode::PrintColor => {
				!modes.contains(&SharedJobRunMode::PrintNothing)
			}
			SharedJobRunMode::PrintWithInspection => {
				!modes.contains(&SharedJobRunMode::PrintNothing) && !modes.contains(&SharedJobRunMode::PrintOnlyWarning)
			}
			_ => true,
		};
		if keep && !resolved.contains(mode) {
			resolved.push(*mode);
		}
	}
	resolved
}

fn resolve_attr_items(items: Vec<SharedJobOptForAttrItem>) -> Vec<SharedJobOptForAttrItem> {
	let mut resolved = Vec::new();
	for item in &items {
		let keep = match item {
			SharedJobOptForAttrItem::ListFlat => !items.contains(&SharedJobOptForAttrItem::ListTree),
			SharedJobOptForAttrItem::IconsMore => !items.contains(&SharedJobOptForAttrItem::IconsLite),
			SharedJobOptForAttrItem::NumListAft => !items.contains(&SharedJobOptForAttrItem::NumListBef),
			SharedJobOptForAttrItem::IconsHide => {
				!items.contains(&SharedJobOptForAttrItem::IconsLite)
					&& !items.contains(&SharedJobOptForAttrItem::IconsMore)
			}
			_ => true,
		};
		if keep && !resolved.contains(item) {
			resolved.push(*item);
		}
	}
	resolved
}

fn resolve_sort_tex(tex: RawTomlJobSortTex) -> ReadyJobSortTex {
	let mut strat = Vec::new();
	let raw = tex.string_strategy.0;
	for s in &raw {
		let keep = match s {
			SharedJobStringMode::aAzZ => !raw.contains(&SharedJobStringMode::AaZz),
			SharedJobStringMode::AZaz => {
				!raw.contains(&SharedJobStringMode::AaZz) && !raw.contains(&SharedJobStringMode::aAzZ)
			}
			SharedJobStringMode::azAZ => {
				!raw.contains(&SharedJobStringMode::AaZz)
					&& !raw.contains(&SharedJobStringMode::aAzZ)
					&& !raw.contains(&SharedJobStringMode::AZaz)
			}
			_ => true,
		};
		if keep && !strat.contains(s) {
			strat.push(*s);
		}
	}
	ReadyJobSortTex::new(tex.reverse, tex.mirror, strat)
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
