use crate::interfaces::cli::args::CliArgs;
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::core::path_view::ViewMode;
use cargo_plot::execute::{self, SortStrategy};
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

    let stats = execute::execute(
        &args.enter_path,
        &args.patterns,
        is_case_sensitive,
        sort_strategy,
        show_mode,
        view_mode,
        args.no_root,
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
    print!("{}", stats.render_output(view_mode, show_mode));

    // 3. PODSUMOWANIE
    println!("----------");
    println!(
        "📊 Podsumowanie: Dopasowano {} z {} ścieżek.",
        stats.m_size_matched, stats.total
    );
    println!(
        "📊 Podsumowanie: Odrzucono {} z {} ścieżek.",
        stats.x_size_mismatched, stats.total
    );
}
