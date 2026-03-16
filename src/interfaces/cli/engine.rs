use crate::interfaces::cli::args::CliArgs;
use cargo_plot::execuctor::for_many_patterns_tok::{self, SortStrategy};
// lub dla jednego wzorca:
// use cargo_plot::execuctor::for_one_pattern_raw::{self, SortStrategy};
use cargo_plot::theme::for_path_list::get_icon_for_path;
use cargo_plot::core::path_treeview::{PathTree, WeightConfig};
use cargo_plot::core::path_store::context::PathContext;

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
        |file_stat| { 
            if !args.treeview {
                println!(
                    "✅ MATCH:  {} {} ({} B)", 
                    get_icon_for_path(&file_stat.path), 
                    file_stat.path, 
                    file_stat.weight_bytes
                );
            }
        },
        |file_stat| {
            if !args.treeview && show_exclude {
                println!(
                    "❌ REJECT: {} {} ({} B)", 
                    get_icon_for_path(&file_stat.path), 
                    file_stat.path, 
                    file_stat.weight_bytes
                );
            }
        },
    );

    // ⚡ Logika Drzewa vs Listy
    if args.treeview {
        println!("🌲 Widok Drzewa:");
        
        let weight_cfg = WeightConfig::default();
        
        // ⚡ Rozwiązujemy ścieżkę z argumentów CLI w locie, żeby wiedzieć, gdzie szukać wag
        let path_ctx = PathContext::resolve(&args.enter_path).unwrap_or_else(|e| {
            eprintln!("❌ Błąd ścieżki: {}", e);
            std::process::exit(1);
        });
        
        // Teraz mamy dostęp do `path_ctx.entry_absolute` i możemy przekazać to do Buildera!
        let tree = PathTree::build(
            &stats.included, 
            &path_ctx.entry_absolute, // ⚡ Używamy odzyskanej ścieżki
            "dirs-first", 
            &weight_cfg
        );
        
        print!("{}", tree.render_cli());
    }

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
