use crate::interfaces::cli::args::CliArgs;
use cargo_plot::execuctor::for_many_patterns_tok::{self, SortStrategy};
use cargo_plot::theme::for_path_list::get_icon_for_path;
// lub dla jednego wzorca:
// use cargo_plot::execuctor::for_one_pattern_raw;

/// [EN]: The execution engine (Cockpit).
/// [PL]: Silnik wykonawczy (Kokpit).
pub fn run(args: CliArgs) {
    let is_case_sensitive = !args.ignore_case;
    let sort_strategy: SortStrategy = args.sort.into();

    let mut show_include = args.include;
    let mut show_exclude = args.exclude;
    if !show_include && !show_exclude {
        show_include = true;
        show_exclude = true;
    }

    let stats = for_many_patterns_tok::execute(
        &args.enter_path,
        &args.patterns,
        is_case_sensitive,
        sort_strategy,
        show_include,
        show_exclude,
        |path| println!("✅ MATCH:  {} {}", get_icon_for_path(path), path),
        |path| println!("❌ REJECT: {} {}", get_icon_for_path(path), path),
    );

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
