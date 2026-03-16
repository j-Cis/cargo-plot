use crate::interfaces::cli::args::CliArgs;
use cargo_plot::execuctor::for_many_patterns_tok::{self, SortStrategy};
use cargo_plot::core::path_view::ViewMode;
use cargo_plot::core::path_matcher::stats::ShowMode;
// lub dla jednego wzorca:
// use cargo_plot::execuctor::for_one_pattern_raw::{self, SortStrategy};
// use cargo_plot::theme::for_path_list::get_icon_for_path;

/// [EN]: The execution engine (Cockpit).
/// [PL]: Silnik wykonawczy (Kokpit).
pub fn run(args: CliArgs) {
    let is_case_sensitive = !args.ignore_case;
    let sort_strategy: SortStrategy = args.sort.into();
    let view_mode: ViewMode = args.view.into();

    let show_mode = match (args.include, args.exclude) {
        (true, false) => ShowMode::Include, // Tylko flaga -m
        (false, true) => ShowMode::Exclude, // Tylko flaga -x
        _ => ShowMode::Context,             // Brak flag (lub podane obie) = pokazujemy wszystko
    };

    let stats = for_many_patterns_tok::execute(
        &args.enter_path,
        &args.patterns,
        is_case_sensitive,
        sort_strategy,
        show_mode,
        view_mode,
        args.no_root,
        |_| {}, // ⚡ Closure są puste, bo renderujemy PO zebraniu statystyk
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

    let do_include = show_mode == ShowMode::Include || show_mode == ShowMode::Context;
    let do_exclude = show_mode == ShowMode::Exclude || show_mode == ShowMode::Context;

    // 2. RENDEROWANIE WYNIKÓW
    match view_mode {
        ViewMode::Grid => {
            if do_include {
                if let Some(grid) = &stats.included.grid {
                    println!("🌲 Widok Siatki (DOPASOWANE):");
                    print!("{}", grid.render_cli());
                }
            }
            if do_exclude {
                if let Some(grid) = &stats.excluded.grid {
                    println!("🌲 Widok Siatki (ODRZUCONE):");
                    print!("{}", grid.render_cli());
                }
            }
        }
        ViewMode::Tree => {
            if do_include {
                if let Some(tree) = &stats.included.tree {
                    println!("🌲 Widok Drzewa (DOPASOWANE):");
                    print!("{}", tree.render_cli());
                }
            }
            if do_exclude {
                if let Some(tree) = &stats.excluded.tree {
                    println!("🌲 Widok Drzewa (ODRZUCONE):");
                    print!("{}", tree.render_cli());
                }
            }
        }
        ViewMode::List => {
            if do_include {
                if let Some(list) = &stats.included.list {
                    // render_cli(true) -> dodaje zielony znaczek ✅
                    print!("{}", list.render_cli(true));
                }
            }
            if do_exclude {
                if let Some(list) = &stats.excluded.list {
                    // render_cli(false) -> dodaje czerwony znaczek ❌
                    print!("{}", list.render_cli(false));
                }
            }
        }
    }

    // 3. PODSUMOWANIE
    println!("----------");
    println!(
        "📊 Podsumowanie: Dopasowano {} z {} ścieżek.",
        stats.matched, stats.total
    );
    println!(
        "📊 Podsumowanie: Odrzucono {} z {} ścieżek.",
        stats.rejected, stats.total
    );
}