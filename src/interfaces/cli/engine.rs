use std::path::Path;
use crate::interfaces::cli::args::CliArgs;

// [EN]: Imports from our library core.
// [PL]: Importy z rdzenia naszej biblioteki.
use cargo_plot::core::path_class::get_icon_for_path;
use cargo_plot::core::path_store::{PathContext, PathStore};
use cargo_plot::core::path_matcher::{PathMatchers, SortStrategy};
use cargo_plot::core::patterns_expand::PatternContext;

/// [EN]: The execution engine (Cockpit).
/// [PL]: Silnik wykonawczy (Kokpit).
pub fn run(args: CliArgs) {
    let mut show_include = args.include;
    let mut show_exclude = args.exclude;
    if !show_include && !show_exclude {
        show_include = true;
        show_exclude = true;
    }
    
    let is_case_sensitive = !args.ignore_case;
    let sort_strategy: SortStrategy = args.sort.into();
    let pattern_ctx = PatternContext::new(&args.patterns);
    let path_ctx = PathContext::resolve(&args.enter_path).unwrap_or_else(|e| {
        eprintln!("❌ {}", e);
        std::process::exit(1);
    });
    
    println!("📂 Baza terminala (Absolutna): {}", path_ctx.base_absolute);
    println!("📂 Cel skanowania (Absolutna): {}", path_ctx.entry_absolute);
    println!("📂 Cel skanowania (Relatywna): {}", path_ctx.entry_relative);
    println!("---------------------------------------");    
    println!("🔠 Wrażliwość na litery: {}", is_case_sensitive);
    println!("🔍 Wzorce (RAW): {:?}", pattern_ctx.raw);
    println!("⚙️ Wzorce (TOK): {:?}",  pattern_ctx.tok);
    println!("---------------------------------------");    
    
    //let path_obj = Path::new(&args.enter_path);
    //if path_obj.is_relative() {
    //    let abs_path = resolve_absolute_path(&args.enter_path)
    //        .unwrap_or_else(|| "[Nie można ustalić ścieżki]".to_string());
    //
    //    println!("📂 Ścieżka wejściowa: {} (Absolutna: {})", args.enter_path, abs_path);
    //} else {
    //    println!("📂 Ścieżka wejściowa: {}", args.enter_path);
    //}

    

    let matchers = PathMatchers::new(&pattern_ctx.tok, is_case_sensitive)
        .expect("Błąd kompilacji wzorców");


    // [PL]: Ładujemy dane do rejestru z rdzenia
    let paths_store = PathStore::scan(&path_ctx.entry_absolute);   
    // [PL]: Wyciągamy PULĘ ŚCIEŻEK
    let paths_set = paths_store.get_index();

    let stats = matchers.evaluate(
        &paths_store.list,
        &paths_set,
        sort_strategy,
        show_include,
        show_exclude,
        |path| println!("✅ MATCH:  {} {}", get_icon_for_path(path), path),
        |path| println!("❌ REJECT: {} {}", get_icon_for_path(path), path),
    );

    println!("----------");
    println!("📊 Podsumowanie: Dopasowano {} z {} ścieżek.", stats.matched, stats.total);
    println!("📊 Podsumowanie: Odrzucono {} z {} ścieżek.", stats.rejected, stats.total);
}