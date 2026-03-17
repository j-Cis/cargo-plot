use crate::interfaces::cli::args::CliArgs;
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::core::path_view::ViewMode;
use cargo_plot::execute::{self, SortStrategy};
use cargo_plot::addon::TimeTag;
use cargo_plot::core::path_store::PathContext;
// use cargo_plot::theme::for_path_list::get_icon_for_path;

/// [ENG]: The execution engine (Cockpit).
/// [POL]: Silnik wykonawczy (Kokpit).
pub fn run(args: CliArgs) {
    let is_case_sensitive = !args.ignore_case;
    let sort_strategy: SortStrategy = args.sort.into();
    let view_mode: ViewMode = args.view.into();

    let show_mode = match (args.include, args.exclude) {
        (true, false) => ShowMode::Include, // Tylko flaga -m
        (false, true) => ShowMode::Exclude, // Tylko flaga -x
        _ => ShowMode::Context,             // Brak flag (lub podane obie) = pokazujemy wszystko
    };

    let stats = execute::execute(
        &args.enter_path,
        &args.patterns,
        is_case_sensitive,
        sort_strategy,
        show_mode,
        view_mode,
        args.no_root,
        args.info,
        |_| {}, //  Closure są puste, bo renderujemy PO zebraniu statystyk
        |_| {},
        // |file_stat| {
        //     if !args.treeview {
        //         println!(
        //             "✅ MATCH:  {} {} ({} B)",
        //             get_icon_for_path(&file_stat.path),
        //             file_stat.path,
        //             file_stat.weight_bytes
        //         );
        //     }
        // },
        // |file_stat| {
        //     if !args.treeview && show_exclude {
        //         println!(
        //             "❌ REJECT: {} {} ({} B)",
        //             get_icon_for_path(&file_stat.path),
        //             file_stat.path,
        //             file_stat.weight_bytes
        //         );
        //     }
        // },
    );

    // 2. RENDEROWANIE WYNIKÓW
    let output_str_cli = stats.render_output(view_mode, show_mode, args.info, true);
    print!("{}", output_str_cli);

    let has_out_paths = args.out_path.is_some();
    let has_out_codes = args.out_code.is_some();

    if has_out_paths || has_out_codes {
        let tag = TimeTag::now();
        let output_str_txt = stats.render_output(view_mode, show_mode, args.info, false);

        // Closure do automatycznego generowania ścieżki
        let resolve_filepath = |val: &str, prefix: &str| -> String {
            if val == "AUTO" {
                format!("./other/{}_{}.md", prefix, tag)
            } else if val.ends_with('/') || val.ends_with('\\') {
                format!("{}{}_{}.md", val, prefix, tag)
            } else {
                let path = std::path::Path::new(val);
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                let ext = path.extension().unwrap_or_default().to_string_lossy();
                let parent = path.parent().unwrap_or_else(|| std::path::Path::new(""));

                let parent_str = parent.to_string_lossy().replace('\\', "/");
                let ext_str = if ext.is_empty() { String::new() } else { format!(".{}", ext) };
                let stem_str = if stem.is_empty() { prefix } else { &stem };

                if parent_str.is_empty() {
                    format!("{}_{}{}", stem_str, tag, ext_str)
                } else {
                    format!("{}/{}_{}{}", parent_str, stem_str, tag, ext_str)
                }
            }
        };

        if let Some(val) = &args.out_path {
            let filepath = resolve_filepath(val, "paths");
            cargo_plot::output::save_path::save(&output_str_txt, &filepath);
        }

        if let Some(val) = &args.out_code {
            let filepath = resolve_filepath(val, "cache");
            if let Ok(ctx) = PathContext::resolve(&args.enter_path) {
                cargo_plot::output::save_code::save(
                    &output_str_txt, 
                    &stats.m_matched.paths, 
                    &ctx.entry_absolute, 
                    &filepath
                );
            }
        }
    }

    // 3. PODSUMOWANIE
    if args.info {
        println!("---------------------------------------");
        println!(
            "📊 Podsumowanie: Dopasowano {} z {} ścieżek.",
            stats.m_size_matched, stats.total
        );
        println!(
            "📊 Podsumowanie: Odrzucono {} z {} ścieżek.",
            stats.x_size_mismatched, stats.total
        );
    } else {
        println!("---------------------------------------");
    }
}
