mod cli;

use anyhow::Result;
use clap::Parser;
use cli::{CargoCliRoot, PlotArgs};

use querypath::QueryPath;
use querypath_fmt::{
    CasePrecedence, CharClass, Column, DirWeightDisplay, GroupStrategy, NoExtPriority, NodeGroup,
    Numeration, QueryPathFmt, SameNamePriority, Sorting, StatsTemporal, StatsWeight, UnitSystem,
    WeightPrecision,
};
use querypath_snapshot::Snapshot;

fn main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] != "plot" {
        args.insert(1, "plot".to_string());
    }
    if args.len() == 1 || (args.len() == 2 && args[1] == "plot") {
        if args.len() == 1 {
            args.push("plot".to_string());
        }
        args.push("-h".to_string());
    }

    let CargoCliRoot::Plot(cli) = CargoCliRoot::parse_from(args);
    run_engine(cli)?;

    Ok(())
}

fn run_engine(cli: PlotArgs) -> Result<()> {
    // =========================================================================
    // ETAP 1: SKANOWANIE (querypath)
    // =========================================================================
    println!("🔍 [cargo-plot] Skanowanie ścieżek: {:?}", cli.dirs_to_scan);
    
    let mut scanner = QueryPath::new().scan_at(&cli.dirs_to_scan).keep_parent(true);

    if !cli.patterns.is_empty() {
        scanner = scanner.match_pattern(&cli.patterns);
    }
    if cli.ignore_case {
        scanner = scanner.ignore_case(true);
    }

    let results = scanner.run()?;

    println!(
        "📦 Znaleziono: {} plików, {} katalogów",
        results.scanned_files, results.scanned_dirs
    );

    // =========================================================================
    // ETAP 2: PARSOWANIE FLAGA PO FLADZE
    // =========================================================================
    
    // --- 2.1 Kolumny ---
    let mut left_cols = Vec::new();
    let mut right_cols = Vec::new();
    let mut is_right_side = false;

    for ch in cli.columns.to_uppercase().chars() {
        match ch {
            'H' => is_right_side = true,
            'W' => { let col = Column::Weight; if is_right_side { right_cols.push(col); } else { left_cols.push(col); } }
            'D' => { let col = Column::Temporal; if is_right_side { right_cols.push(col); } else { left_cols.push(col); } }
            'P' => { let col = Column::Path; if is_right_side { right_cols.push(col); } else { left_cols.push(col); } }
            ' ' | ',' | '_' | '-' => {} 
            _ => eprintln!("⚠️ Zignorowano symbol kolumny: '{}'", ch),
        }
    }

    // --- 2.2 Strategia Grupowania (T D B) ---
    let mut group_order = Vec::new();
    for ch in cli.group_order.to_uppercase().chars() {
        match ch {
            'T' if !group_order.contains(&NodeGroup::TextFile) => group_order.push(NodeGroup::TextFile),
            'D' if !group_order.contains(&NodeGroup::Directory) => group_order.push(NodeGroup::Directory),
            'B' if !group_order.contains(&NodeGroup::BinaryFile) => group_order.push(NodeGroup::BinaryFile),
            ' ' | ',' | '_' | '-' => {}
            _ => eprintln!("⚠️ Zignorowano symbol grupy: '{}'", ch),
        }
    }
    if !group_order.contains(&NodeGroup::TextFile) { group_order.push(NodeGroup::TextFile); }
    if !group_order.contains(&NodeGroup::Directory) { group_order.push(NodeGroup::Directory); }
    if !group_order.contains(&NodeGroup::BinaryFile) { group_order.push(NodeGroup::BinaryFile); }

    // --- 2.3 Kolejność znaków (S D L) ---
    let mut char_classes = Vec::new();
    for ch in cli.char_order.to_uppercase().chars() {
        match ch {
            'S' if !char_classes.contains(&CharClass::Special) => char_classes.push(CharClass::Special),
            'D' if !char_classes.contains(&CharClass::Digit) => char_classes.push(CharClass::Digit),
            'L' if !char_classes.contains(&CharClass::Letter) => char_classes.push(CharClass::Letter),
            ' ' | ',' | '_' | '-' => {}
            _ => eprintln!("⚠️ Zignorowano symbol klasy: '{}'", ch),
        }
    }
    if !char_classes.contains(&CharClass::Special) { char_classes.push(CharClass::Special); }
    if !char_classes.contains(&CharClass::Digit) { char_classes.push(CharClass::Digit); }
    if !char_classes.contains(&CharClass::Letter) { char_classes.push(CharClass::Letter); }
    let char_class_arr = [char_classes[0], char_classes[1], char_classes[2]];

    // --- 2.4 Enumy ---
    let dir_display = match cli.dir_weight.to_lowercase().as_str() {
        "matched" => DirWeightDisplay::MatchedOnly,
        "none" => DirWeightDisplay::None,
        _ => DirWeightDisplay::Both,
    };

    let same_name_priority = match cli.same_name_priority.to_lowercase().as_str() {
        "dir" => SameNamePriority::DirectoryFirst,
        _ => SameNamePriority::FileFirst,
    };

    let no_ext_priority = match cli.no_ext_priority.to_lowercase().as_str() {
        "below" => NoExtPriority::Below,
        _ => NoExtPriority::Above,
    };

    // ⚡ TUTAJ NAPRAWIONO: UpperFirst zamiast UppercaseFirst
    let case_precedence = if cli.case_sensitive { CasePrecedence::UpperFirst } else { CasePrecedence::Insensitive };
    let unit_sys = if cli.size_si { UnitSystem::Decimal } else { UnitSystem::Binary };

    // =========================================================================
    // ETAP 3: FORMATOWANIE I WIDOK (querypath-fmt)
    // =========================================================================
    let sorting = Sorting::new()
        .enabled(true)
        .group_strategy(GroupStrategy::Custom(group_order))
        .same_name_priority(same_name_priority)
        .no_ext_priority(no_ext_priority)
        .ignore_leading_dot(cli.ignore_leading_dot)
        .char_class_order(char_class_arr)
        .case_precedence(case_precedence);

    let fmt = QueryPathFmt::new()
        .name_width(cli.name_width)
        .path_width(cli.path_width)
        .column_order_left(left_cols)
        .column_order_right(right_cols)
        .numeration(
            Numeration::new()
                .enabled(!cli.no_numeration)
                .numerate_dirs(cli.numerate_dirs)
                .numerate_binaries(cli.numerate_binaries)
                .start_from(1),
        )
        .stats_weight(
            StatsWeight::new()
                .enabled(true)
                .unit_system(unit_sys)
                .dir_display(dir_display)
                .precision(WeightPrecision::Tenths),
        )
        .stats_temporal(StatsTemporal::new().enabled(true).pattern(cli.time_format))
        .sorting(sorting);

    let rendered_tree = fmt.format(&results);
    println!("\n{}", rendered_tree);

    // =========================================================================
    // ETAP 4: EKSPORT (querypath-snapshot)
    // =========================================================================
    if let Some(out_dir) = cli.out_dir {
        println!("🚀 Generowanie migawki (Snapshot)...");
        
        let snapshot = Snapshot::new()
            .title(cli.title)
            .output_dir(out_dir)
            .version_pattern("WYYY-WW-SSS-D_hhmmssttqq")
            .file_name_pattern("snapshot_{VERSION}.md")
            .max_single_file_size(cli.max_file_size);

        let saved_path = snapshot.generate_and_save(&results, &fmt)?;
        println!("✅ Zapisano raport do: {}", saved_path.display());
    }

    Ok(())
}