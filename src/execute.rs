use crate::core::file_stats::FileStats;
use crate::core::file_stats::weight::WeightConfig;
pub use crate::core::path_matcher::SortStrategy;
use crate::core::path_matcher::{MatchStats, PathMatchers, ShowMode};
use crate::core::path_store::{PathContext, PathStore};
use crate::core::path_view::{PathGrid, PathList, PathTree, ViewMode};
use crate::core::patterns_expand::PatternContext;
use std::path::Path;

/// [ENG]: Primary execution function that coordinates scanning, matching, and view building.
/// [POL]: Główna funkcja wykonawcza koordynująca skanowanie, dopasowywanie i budowanie widoków.
pub fn execute<OnMatch, OnMismatch>(
    enter_path: &str,
    patterns: &[String],
    is_case_sensitive: bool,
    sort_strategy: SortStrategy,
    show_mode: ShowMode,
    view_mode: ViewMode,
    weight_cfg: WeightConfig, // ⚡ Używamy konfiguracji przekazanej z CLI/GUI
    no_root: bool,
    print_info: bool,
    no_emoji: bool,
    i18n: &crate::i18n::I18n,
    mut on_match: OnMatch,
    mut on_mismatch: OnMismatch,
) -> MatchStats
where
    OnMatch: FnMut(&FileStats),
    OnMismatch: FnMut(&FileStats),
{
    // [ENG]: 1. Initialize contexts.
    // [POL]: 1. Inicjalizacja kontekstów.
    let pattern_ctx = PatternContext::new(patterns);
    let path_ctx = PathContext::resolve(enter_path).unwrap_or_else(|e| {
        eprintln!("❌ {}", e);
        std::process::exit(1);
    });

    // [ENG]: 2. Initial state logging (Restored full verbosity).
    // [POL]: 2. Logowanie stanu początkowego (Przywrócono pełną szczegółowość).
    if print_info {
        println!("{}", i18n.cli_base_abs(&path_ctx.base_absolute));
        println!("{}", i18n.cli_target_abs(&path_ctx.entry_absolute));
        println!("{}", i18n.cli_target_rel(&path_ctx.entry_relative));
        println!("---------------------------------------");
        println!("{}", i18n.cli_case_sensitive(is_case_sensitive));
        println!(
            "{}",
            i18n.cli_patterns_raw(&format!("{:?}", pattern_ctx.raw))
        );
        println!(
            "{}",
            i18n.cli_patterns_tok(&format!("{:?}", pattern_ctx.tok))
        );
        println!("---------------------------------------");
    } else {
        println!("---------------------------------------");
    }

    // [ENG]: 3. Build matchers.
    // [POL]: 3. Budowa silników dopasowujących.
    let matchers =
        PathMatchers::new(&pattern_ctx.tok, is_case_sensitive).expect("Błąd kompilacji wzorców");

    // [ENG]: 4. Scan disk.
    // [POL]: 4. Skanowanie dysku.
    let paths_store = PathStore::scan(&path_ctx.entry_absolute);
    let paths_set = paths_store.get_index();
    let entry_abs = path_ctx.entry_absolute.clone();

    // [ENG]: 6. Evaluate paths and fetch stats via callbacks.
    // [POL]: 6. Ewaluacja ścieżek i pobieranie statystyk przez callbacki.
    let mut stats = matchers.evaluate(
        &paths_store.list,
        &paths_set,
        sort_strategy,
        show_mode,
        |raw_path| {
            let stats = FileStats::fetch(raw_path, &entry_abs);
            on_match(&stats);
        },
        |raw_path| {
            let stats = FileStats::fetch(raw_path, &entry_abs);
            on_mismatch(&stats);
        },
    );

    // [ENG]: 7. Build views using the provided weight configuration.
    // [POL]: 7. Budowa widoków przy użyciu dostarczonej konfiguracji wagi.
    let root_name = if no_root {
        None
    } else {
        Path::new(&path_ctx.entry_absolute)
            .file_name()
            .and_then(|n| n.to_str())
    };

    let do_include = show_mode == ShowMode::Include || show_mode == ShowMode::Context;
    let do_exclude = show_mode == ShowMode::Exclude || show_mode == ShowMode::Context;

    match view_mode {
        ViewMode::Grid => {
            if do_include {
                stats.m_matched.grid = Some(PathGrid::build(
                    &stats.m_matched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    root_name,
                    no_emoji,
                ));
            }
            if do_exclude {
                stats.x_mismatched.grid = Some(PathGrid::build(
                    &stats.x_mismatched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    root_name,
                    no_emoji,
                ));
            }
        }
        ViewMode::Tree => {
            if do_include {
                stats.m_matched.tree = Some(PathTree::build(
                    &stats.m_matched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    root_name,
                    no_emoji,
                ));
            }
            if do_exclude {
                stats.x_mismatched.tree = Some(PathTree::build(
                    &stats.x_mismatched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    root_name,
                    no_emoji,
                ));
            }
        }
        ViewMode::List => {
            if do_include {
                stats.m_matched.list = Some(PathList::build(
                    &stats.m_matched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    no_emoji,
                ));
            }
            if do_exclude {
                stats.x_mismatched.list = Some(PathList::build(
                    &stats.x_mismatched.paths,
                    &path_ctx.entry_absolute,
                    sort_strategy,
                    &weight_cfg,
                    no_emoji,
                ));
            }
        }
    }

    stats
}
