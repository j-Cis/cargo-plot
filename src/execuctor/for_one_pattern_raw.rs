use crate::core::path_matcher::matcher::PathMatcher;
use crate::core::path_matcher::stats::MatchStats;
use crate::core::path_store::PathStore;
use crate::core::path_store::context::PathContext;
// [PL]: Reeksportujemy strategię, aby Kokpit nie musiał szukać jej w core.
pub use crate::core::path_matcher::sort::SortStrategy;

/// [POL]: Egzekutor operujący na pojedynczym, surowym wzorcu wpisanym przez użytkownika.
/// [ENG]: Executor operating on a single, raw pattern provided by the user.
// #[allow(clippy::too_many_arguments)]
pub fn execute<OnMatch, OnMismatch>(
    enter_path: &str,
    raw_pattern: &str,
    is_case_sensitive: bool,
    sort_strategy: SortStrategy,
    show_include: bool,
    show_exclude: bool,
    on_match: OnMatch,
    on_mismatch: OnMismatch,
) -> MatchStats
where
    OnMatch: FnMut(&str),
    OnMismatch: FnMut(&str),
{
    // 1. Inicjalizacja kontekstów
    let path_ctx = PathContext::resolve(enter_path).unwrap_or_else(|e| {
        eprintln!("❌ {}", e);
        std::process::exit(1);
    });

    // 2. Logowanie stanu początkowego
    println!("📂 Baza terminala (Absolutna): {}", path_ctx.base_absolute);
    println!("📂 Cel skanowania (Absolutna): {}", path_ctx.entry_absolute);
    println!("📂 Cel skanowania (Relatywna): {}", path_ctx.entry_relative);
    println!("---------------------------------------");
    println!("🔠 Wrażliwość na litery: {}", is_case_sensitive);
    println!("🔍 Wzorzec (RAW): {:?}", raw_pattern);
    println!("---------------------------------------");

    // 3. Budowa silników dopasowujących (Generał)
    let matcher = PathMatcher::new(raw_pattern, is_case_sensitive).expect("Błąd kompilacji wzorca");

    // 4. Skanowanie dysku (Getter)
    // [PL]: Ładujemy dane do rejestru z rdzenia
    let paths_store = PathStore::scan(&path_ctx.entry_absolute);
    // [PL]: Wyciągamy PULĘ ŚCIEŻEK (Encyklopedię)
    let paths_set = paths_store.get_index();

    // 5. Ewaluacja i wykonanie callbacków
    

    // 6. Zwracamy statystyki do Engine'u
    matcher.evaluate(
        &paths_store.list,
        &paths_set,
        sort_strategy,
        show_include,
        show_exclude,
        on_match,
        on_mismatch,
    )
}
