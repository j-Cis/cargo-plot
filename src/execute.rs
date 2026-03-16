pub use crate::core::path_matcher::SortStrategy;
use crate::core::path_matcher::{PathMatchers,MatchStats, ShowMode};
use crate::core::path_store::{PathContext,PathStore};
use crate::core::patterns_expand::PatternContext;
use crate::core::path_view::{PathList, PathTree, PathGrid, ViewMode};
use crate::core::file_stats::weight::WeightConfig;
use crate::core::file_stats::FileStats;
use std::path::Path;

/// [POL]: Egzekutor operujący na wielu wzorcach (wersja po rozwinięciu klamer/tokenizacji).
/// [ENG]: Executor operating on multiple patterns (post brace expansion/tokenisation).
pub fn execute<OnMatch, OnMismatch>(
    enter_path: &str,
    patterns: &[String],
    is_case_sensitive: bool,
    sort_strategy: SortStrategy,
    show_mode: ShowMode,
    view_mode: ViewMode,
    no_root: bool,
    mut on_match: OnMatch,
    mut on_mismatch: OnMismatch,
) -> MatchStats
where
    // OnMatch: FnMut(&str),
    // OnMismatch: FnMut(&str),
    // ⚡ Teraz callbacki oczekują bogatego obiektu, a nie tylko tekstu
    OnMatch: FnMut(&FileStats),
    OnMismatch: FnMut(&FileStats),
{
    // 1. Inicjalizacja kontekstów
    let pattern_ctx = PatternContext::new(patterns);
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
    println!("🔍 Wzorce (RAW): {:?}", pattern_ctx.raw);
    println!("⚙️ Wzorce (TOK): {:?}", pattern_ctx.tok);
    println!("---------------------------------------");

    // 3. Budowa silników dopasowujących (Generał)
    let matchers =
        PathMatchers::new(&pattern_ctx.tok, is_case_sensitive).expect("Błąd kompilacji wzorców");

    // 4. Skanowanie dysku (Getter)
    // [PL]: Ładujemy dane do rejestru z rdzenia
    let paths_store = PathStore::scan(&path_ctx.entry_absolute);
    // [PL]: Wyciągamy PULĘ ŚCIEŻEK (Encyklopedię)
    let paths_set = paths_store.get_index();

   
    
    let entry_abs = path_ctx.entry_absolute.clone();
    // 6. Zwracamy statystyki do Engine'u
    let mut stats = matchers.evaluate(
        &paths_store.list,
        &paths_set,
        sort_strategy,
        show_mode,
        |raw_path| {
            // Pośrednik pobiera statystyki
            let stats = FileStats::fetch(raw_path, &entry_abs);
            on_match(&stats);
        },
        |raw_path| {
            // Pośrednik pobiera statystyki
            let stats = FileStats::fetch(raw_path, &entry_abs);
            on_mismatch(&stats);
        },
    );

    // 7. ⚡ MAGIA BUDOWANIA WIDOKÓW
    let weight_cfg = WeightConfig::default();
    let root_name = if no_root {
        None
    } else {
        Path::new(&path_ctx.entry_absolute).file_name().and_then(|n| n.to_str())
    };

    // Pomocnicze flagi do budowania (żeby kod w match był krótki)
    let do_include = show_mode == ShowMode::Include || show_mode == ShowMode::Context;
    let do_exclude = show_mode == ShowMode::Exclude || show_mode == ShowMode::Context;

    // ⚡ Czysty match dla widoków (Grid, Tree, List)
    match view_mode {
        ViewMode::Grid => {
            if do_include {
                stats.included.grid = Some(PathGrid::build(&stats.included.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg, root_name));
            }
            if do_exclude {
                stats.excluded.grid = Some(PathGrid::build(&stats.excluded.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg, root_name));
            }
        }
        ViewMode::Tree => {
            if do_include {
                stats.included.tree = Some(PathTree::build(&stats.included.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg, root_name));
            }
            if do_exclude {
                stats.excluded.tree = Some(PathTree::build(&stats.excluded.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg, root_name));
            }
        }
        ViewMode::List => {
            if do_include {
                stats.included.list = Some(PathList::build(&stats.included.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg));
            }
            if do_exclude {
                stats.excluded.list = Some(PathList::build(&stats.excluded.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg));
            }
        }
    }

    stats
}