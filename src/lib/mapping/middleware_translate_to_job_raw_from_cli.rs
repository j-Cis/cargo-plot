use crate::lib::{
	core::{default_for_item, default_run_modes, default_string_strategy},
	schema::{
		RawCliJob,
		RawTomlJob,
		RawTomlJobAttributes,
		RawTomlJobAttributesOptions,
		RawTomlJobExplorer,
		RawTomlJobExport,
		RawTomlJobPileMode,
		RawTomlJobSortNum,
		RawTomlJobSortTex,
		RawTomlJobStringStrategy,
		RawTomlJobTuples,
		RawTomlJobTuplesPile,
		RawTomlJobTuplesSort,
		SharedJobOptForAttrItem,
		SharedJobOptForAttrSize,
		SharedJobPart,
		SharedJobRunMode,
		SharedJobStringMode,
	},
};

/// MIDDLEWARE: Nakłada flagi z CLI na pusty obiekt RawTomlJob
pub fn translate(mut raw: RawTomlJob, cli: &RawCliJob) -> RawTomlJob {
	// 1. RUN MODES (Wstrzykujemy flagi jako nadpisanie modów uruchomieniowych)
	let mut modes = raw.run_mode.clone().unwrap_or_else(default_run_modes);
	if cli.dry_run.unwrap_or(false) {
		modes.push(SharedJobRunMode::DryRun);
	}
	if cli.export_nothing.unwrap_or(false) {
		modes.retain(|m| *m != SharedJobRunMode::Save && *m != SharedJobRunMode::SaveWithInspection);
	}
	if cli.print_nothing.unwrap_or(false) {
		modes.push(SharedJobRunMode::PrintNothing);
	}
	if cli.print_warnings.unwrap_or(false) {
		modes.push(SharedJobRunMode::PrintOnlyWarning);
	}
	if cli.inspection_in_export.unwrap_or(false) {
		modes.retain(|m| *m != SharedJobRunMode::Save);
		modes.push(SharedJobRunMode::SaveWithInspection);
	}
	if cli.inspection_in_print.unwrap_or(false) {
		modes.push(SharedJobRunMode::PrintWithInspection);
	}
	if cli.show_colors.unwrap_or(false) {
		modes.push(SharedJobRunMode::PrintColor);
	}
	raw.run_mode = Some(modes);

	// 2. EXPLORER (Wstrzykujemy ścieżki i wzorce)
	let mut exp = raw.explorer.unwrap_or_else(|| RawTomlJobExplorer {
		workspace_dir: None,
		ignore_case: None,
		patterns: None,
		parts: None,
	});
	exp.workspace_dir = Some(cli.dir_to_scan.clone());
	if !cli.patterns.is_empty() {
		exp.patterns = Some(cli.patterns.clone());
	}
	if !cli.query_parts.is_empty() {
		let mut parts = Vec::new();
		for cp in &cli.query_parts {
			match cp.to_lowercase().as_str() {
				"md" | "matched-dirs" => parts.push(SharedJobPart::MD),
				"mf" | "matched-files" => parts.push(SharedJobPart::MF),
				"xd" | "mismatched-dirs" => parts.push(SharedJobPart::XD),
				"xf" | "mismatched-files" => parts.push(SharedJobPart::XF),
				_ => {}
			}
		}
		if !parts.is_empty() {
			exp.parts = Some(parts);
		}
	}
	if let Some(ic) = cli.ignore_case {
		exp.ignore_case = Some(ic);
	}
	raw.explorer = Some(exp);

	// 3. EXPORT (Wstrzykujemy informacje wyjściowe)
	let mut export = raw.export.unwrap_or_else(|| RawTomlJobExport {
		out_dir: None,
		title: None,
		name: None,
		name_is_first: None,
		save_separately: None,
	});
	export.out_dir = Some(cli.out_dir.clone());
	if let Some(t) = &cli.title {
		export.title = Some(t.clone());
	}
	if let Some(n) = &cli.name {
		export.name = Some(n.clone());
	}
	if let Some(nif) = cli.name_is_first {
		export.name_is_first = Some(nif);
	}
	if let Some(ss) = cli.save_separately {
		export.save_separately = Some(ss);
	}
	raw.export = Some(export);

	// 4. ATTRIBUTES (Wstrzykujemy opcje formatowania tabeli)
	let mut attr = raw.attributes.unwrap_or_else(|| RawTomlJobAttributes { select: None, option: None });
	if let Some(sel) = &cli.attributes {
		attr.select = Some(sel.clone());
	}

	let mut opt = attr.option.unwrap_or_else(|| RawTomlJobAttributesOptions {
		for_item: None,
		for_date: None,
		for_time: None,
		for_size: None,
	});
	let mut for_item = opt.for_item.unwrap_or_else(default_for_item);

	if cli.tree_hide.unwrap_or(false) {
		for_item.retain(|i| *i != SharedJobOptForAttrItem::ListTree);
		if !for_item.contains(&SharedJobOptForAttrItem::ListFlat) {
			for_item.push(SharedJobOptForAttrItem::ListFlat);
		}
	}
	if cli.icon_hide.unwrap_or(false) {
		for_item.retain(|i| *i != SharedJobOptForAttrItem::IconsLite && *i != SharedJobOptForAttrItem::IconsMore);
		for_item.push(SharedJobOptForAttrItem::IconsHide);
	} else if cli.icon_more.unwrap_or(false) {
		for_item.retain(|i| *i != SharedJobOptForAttrItem::IconsLite);
		for_item.push(SharedJobOptForAttrItem::IconsMore);
	}
	if cli.name_hide.unwrap_or(false) {
		for_item.push(SharedJobOptForAttrItem::NameHide);
	}
	if cli.align_hide.unwrap_or(false) {
		for_item.push(SharedJobOptForAttrItem::AlignHide);
	}
	if cli.num_before.unwrap_or(false) {
		for_item.retain(|i| *i != SharedJobOptForAttrItem::NumListAft);
		for_item.push(SharedJobOptForAttrItem::NumListBef);
	}
	opt.for_item = Some(for_item);

	if let Some(fd) = &cli.fmt_date {
		opt.for_date = Some(fd.clone());
	}
	if let Some(ft) = &cli.fmt_time {
		opt.for_time = Some(ft.clone());
	}
	if cli.size_binary.unwrap_or(false) {
		opt.for_size = Some(SharedJobOptForAttrSize::Binary);
	}

	attr.option = Some(opt);
	raw.attributes = Some(attr);

	// 5. TUPLES (Wstrzykujemy sortowanie i grupowanie)
	if cli.pile.is_some() || cli.sort.is_some() {
		let mut tuples = raw.tuples.unwrap_or_else(|| RawTomlJobTuples { pile: None, sort: None });

		if let Some(pile_type) = &cli.pile {
			let pile_mode = RawTomlJobPileMode {
				dir_first: cli.dir_first.unwrap_or(false),
				same_name_dirs_and_files_nearby: cli.name_nearby.unwrap_or(false),
			};
			match pile_type.as_str() {
				"name" => tuples.pile = Some(RawTomlJobTuplesPile::Name(pile_mode)),
				"exte" => tuples.pile = Some(RawTomlJobTuplesPile::Exte(pile_mode)),
				_ => {}
			}
		}

		if let Some(sort_type) = &cli.sort {
			let rev = cli.reverse.unwrap_or(false);
			let mir = cli.mirror.unwrap_or(false);
			let strat = cli.strategy.as_deref().map(map_string_strategy).unwrap_or_else(default_string_strategy);
			let strat_arr = RawTomlJobStringStrategy([strat[0], strat[1], strat[2]]);

			match sort_type.as_str() {
				"date" => tuples.sort = Some(RawTomlJobTuplesSort::Date(RawTomlJobSortNum { reverse: rev })),
				"size" => tuples.sort = Some(RawTomlJobTuplesSort::Size(RawTomlJobSortNum { reverse: rev })),
				"path" => {
					tuples.sort = Some(RawTomlJobTuplesSort::Path(RawTomlJobSortTex {
						reverse: rev,
						mirror: mir,
						string_strategy: strat_arr.clone(),
					}))
				}
				"name" => {
					tuples.sort = Some(RawTomlJobTuplesSort::Name(RawTomlJobSortTex {
						reverse: rev,
						mirror: mir,
						string_strategy: strat_arr,
					}))
				}
				_ => {}
			}
		}
		raw.tuples = Some(tuples);
	}

	raw
}

fn map_string_strategy(s: &str) -> Vec<SharedJobStringMode> {
	let mut vec: Vec<_> = s
		.split(',')
		.filter_map(|part| match part.trim() {
			"Spec" => Some(SharedJobStringMode::Spec),
			"Num" => Some(SharedJobStringMode::Num),
			"AaZz" => Some(SharedJobStringMode::AaZz),
			"aAzZ" => Some(SharedJobStringMode::aAzZ),
			"AZaz" => Some(SharedJobStringMode::AZaz),
			"azAZ" => Some(SharedJobStringMode::azAZ),
			_ => None,
		})
		.collect();

	// Gwarantujemy, że zwrócimy dokładnie 3 elementy, aby uniknąć paniki przy konwersji do tablicy [T; 3]
	while vec.len() < 3 {
		vec.push(SharedJobStringMode::Spec);
	}
	vec
}
