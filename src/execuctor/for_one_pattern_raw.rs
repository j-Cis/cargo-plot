use crate::core::path_matcher::matcher::PathMatcher;
use crate::core::path_store::store::PathStore;
use crate::core::path_store::context::PathContext;
// [PL]: Reeksportujemy strategię, aby Kokpit nie musiał szukać jej w core.
pub use crate::core::path_matcher::sort::SortStrategy;
use crate::core::path_matcher::stats::MatchStats;
use crate::core::path_view::{PathList, PathTree, PathGrid};
use crate::core::file_stats::weight::WeightConfig;
use crate::core::file_stats::FileStats;
use std::path::Path;

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
    is_treeview: bool, 
    is_gridview: bool,
    no_root: bool,   
    mut on_match: OnMatch,
    mut on_mismatch: OnMismatch,
) -> MatchStats
where
    OnMatch: FnMut(&FileStats),
    OnMismatch: FnMut(&FileStats),
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

    let entry_abs = path_ctx.entry_absolute.clone();

    // 5. Ewaluacja i wykonanie callbacków
    let mut stats = matcher.evaluate(
        &paths_store.list,
        &paths_set,
        sort_strategy,
        show_include,
        show_exclude,
        |raw_path| {
            let s = FileStats::fetch(raw_path, &entry_abs);
            on_match(&s);
        },
        |raw_path| {
            let s = FileStats::fetch(raw_path, &entry_abs);
            on_mismatch(&s);
        },
    );
    // 6. ⚡ MAGIA BUDOWANIA WIDOKÓW
    let weight_cfg = WeightConfig::default();
    let root_name = if no_root {
        None
    } else {
        Path::new(&path_ctx.entry_absolute).file_name().and_then(|n| n.to_str())
    };
    
    if is_gridview {
        if show_include {
            stats.included.grid = Some(PathGrid::build(&stats.included.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg, root_name));
        }
        if show_exclude {
            stats.excluded.grid = Some(PathGrid::build(&stats.excluded.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg, root_name));
        }
    } else if is_treeview {
        if show_include {
            stats.included.tree = Some(PathTree::build(&stats.included.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg, root_name));
        }
        if show_exclude {
            stats.excluded.tree = Some(PathTree::build(&stats.excluded.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg, root_name));
        }
    } else {
        if show_include {
            stats.included.list = Some(PathList::build(&stats.included.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg));
        }
        if show_exclude {
            stats.excluded.list = Some(PathList::build(&stats.excluded.paths, &path_ctx.entry_absolute, sort_strategy, &weight_cfg));
        }
    }

    // 7. Zwracamy statystyki do Engine'u
    stats
}
