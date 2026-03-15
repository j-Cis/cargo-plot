use cargo_plot::core::path_class::get_icon_for_path;
use cargo_plot::core::path_getter::get_paths;
use cargo_plot::core::path_matcher::{/*PathMatcher,*/ PathMatchers, expand_braces};
use std::collections::HashSet;
use std::env;
use std::process;

/// Wyciąga flagi `-x` z argumentów wywołania.
/// Jeśli nie znajdzie żadnych wzorców, wypisuje instrukcję i kończy program.
fn get_patterns_from_cli() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut patterns = Vec::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "-x" && i + 1 < args.len() {
            patterns.push(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }

    if patterns.is_empty() {
        eprintln!("⚠️ Nie podano żadnych wzorców!");
        eprintln!("💡 Użycie: cargo run --example skaner -- -x \"src/\" -x \"src/**\"");
        process::exit(1); // Brutalne przerwanie programu (z kodem błędu 1)
    }

    patterns
}

fn main() {
    // 1. ODCZYT ARGUMENTÓW Z KONSOLI
    let patterns_raw = get_patterns_from_cli();
    println!("🔍 Wzorce wejściowe (RAW): {:?}", patterns_raw);

    // 🔴 NOWOŚĆ: Przepuszczamy wzorce przez middleware w celach wizualnych
    let mut patterns_tok = Vec::new();
    for pat in &patterns_raw {
        patterns_tok.extend(expand_braces(pat));
    }
    println!("⚙️  Wzorce po middleware (TOK): {:?}", patterns_tok);

    let is_case_sensitive = false;
    let matchers =
        PathMatchers::new(&patterns_raw, is_case_sensitive).expect("Błąd kompilacji wzorców");
    // let paths_to_test: Vec<&str> = include!("data.rs");
    let paths_to_test = get_paths("./src");
    // let wycinek = &paths_to_test[..std::cmp::min(25, paths_to_test.len())];
    // println!("{:#?}", wycinek);
    // for path in paths_to_test.iter().take(25) {
    //     println!("{}", path);
    // }

    // 🔴 NOWOŚĆ: Budujemy mapę środowiska z naszej listy ścieżek.
    // Używamy .copied(), bo elements w Vec<&str> to &str, a chcemy mieć HashSet<&str>
    let environment: HashSet<&str> = paths_to_test.iter().map(|s| s.as_str()).collect();

    let mut dopasowane = 0;
    let total = paths_to_test.len();

    // Ewaluacja
    matchers.evaluate(
        &paths_to_test,
        &environment, // 🔴 NOWOŚĆ: Wstrzykujemy środowisko do silnika!
        |path| {
            // 🔥 Używamy naszej nowej, czystej funkcji!
            let icon = get_icon_for_path(path);
            println!("✅ MATCH: {} {}", icon, path);
            dopasowane += 1;
        },
        |_path| {
            // Miejsce na logikę dla odrzuconych ścieżek
        },
    );

    println!("----------");
    println!(
        "📊 Podsumowanie: Dopasowano {} z {} ścieżek.",
        dopasowane, total
    );
}
