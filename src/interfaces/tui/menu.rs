use super::i18n::{Prompt, T};
use super::state::StateTui;
use crate::interfaces::cli::args::{CliSortStrategy, CliViewMode};
use crate::interfaces::cli::engine;

#[derive(Clone, PartialEq, Eq)]
enum Action {
    Lang,
    QuickStart,
    Paths,
    View,
    Output,
    Filters,
    Run,
    Exit,
}

pub fn menu_main(s: &mut StateTui) {
    let mut last_action = Action::Paths;

    loop {
        let t = T::new(s.lang);
        let header = t.fmt(Prompt::HeaderMain);

        // ⚡ DYNAMICZNE ETYKIETY KOKPITU
        let pat_str = if s.args.patterns.is_empty() {
            "[]".to_string()
        } else {
            format!("[{}...]", s.args.patterns[0])
        };
        let lbl_paths = format!(
            "{} (dir: '{}', pat: {})",
            t.fmt(Prompt::BtnPaths),
            s.args.enter_path,
            pat_str
        );
        let lbl_view = format!(
            "{} (view: {:?}, sort: {:?}, root: {})",
            t.fmt(Prompt::BtnView),
            s.args.view,
            s.args.sort,
            !s.args.no_root
        );

        let out_p = s.args.out_path.as_deref().unwrap_or("NONE");
        let out_c = s.args.out_code.as_deref().unwrap_or("NONE");
        let lbl_out = format!(
            "{} (paths: {}, cache: {}, by: {})",
            t.fmt(Prompt::BtnOutput),
            out_p,
            out_c,
            s.args.by
        );

        let lbl_filt = format!(
            "{} (match: {}, mismatch: {}, info: {})",
            t.fmt(Prompt::BtnFilters),
            s.args.include,
            s.args.exclude,
            s.args.info
        );

        // ⚡ BUDOWA MENU
        let action_result = cliclack::select(header)
            .initial_value(last_action.clone())
            .item(Action::Lang, t.fmt(Prompt::BtnLang), "")
            .item(Action::QuickStart, t.fmt(Prompt::BtnQuickStart), "")
            .item(Action::Paths, lbl_paths, "")
            .item(Action::View, lbl_view, "")
            .item(Action::Output, lbl_out, "")
            .item(Action::Filters, lbl_filt, "")
            .item(Action::Run, t.fmt(Prompt::BtnRun), "")
            .item(Action::Exit, t.fmt(Prompt::BtnExit), "")
            .interact();

        // ⚡ OBSŁUGA AKCJI
        match action_result {
            Ok(Action::Lang) => s.toggle_lang(),
            Ok(Action::QuickStart) => {
                let raw_pat: String = cliclack::input(t.raw(Prompt::InputPatterns))
                    .interact()
                    .unwrap_or_default();
                if !raw_pat.trim().is_empty() {
                    s.args.patterns = split_patterns(&raw_pat);
                    cliclack::outro("🚀 ...").unwrap();
                    engine::run(s.args.clone());
                    return;
                }
            }
            Ok(Action::Paths) => {
                last_action = Action::Paths;
                handle_paths(s, &t);
            }
            Ok(Action::View) => {
                last_action = Action::View;
                handle_view(s, &t);
            }
            Ok(Action::Output) => {
                last_action = Action::Output;
                handle_output(s, &t);
            }
            Ok(Action::Filters) => {
                last_action = Action::Filters;
                handle_filters(s, &t);
            }
            Ok(Action::Run) => {
                if s.args.patterns.is_empty() {
                    cliclack::log::warning(t.raw(Prompt::WarnNoPatterns)).unwrap();
                    continue;
                }
                cliclack::outro("🚀 ...").unwrap();
                engine::run(s.args.clone());
                return;
            }
            Ok(Action::Exit) | Err(_) => {
                cliclack::outro(t.raw(Prompt::ExitBye)).unwrap();
                return;
            }
        }
        cliclack::clear_screen().unwrap();
    }
}

// =====================================================================
// SZYBKIE PODMENU (Helpery modyfikujące stan)
// =====================================================================

fn handle_paths(s: &mut StateTui, t: &T) {
    s.args.enter_path = cliclack::input(t.raw(Prompt::SubBasePath))
        .default_input(&s.args.enter_path)
        .interact()
        .unwrap_or(s.args.enter_path.clone());
    let current_pat = s.args.patterns.join(", ");
    let new_pat: String = cliclack::input(t.raw(Prompt::InputPatterns))
        .default_input(&current_pat)
        .interact()
        .unwrap_or(current_pat);
    s.args.patterns = split_patterns(&new_pat);
    s.args.ignore_case = cliclack::confirm(t.raw(Prompt::SubIgnoreCase))
        .initial_value(s.args.ignore_case)
        .interact()
        .unwrap_or(s.args.ignore_case);
}

fn handle_view(s: &mut StateTui, t: &T) {
    s.args.view = cliclack::select(t.raw(Prompt::SubSelectView))
        .initial_value(s.args.view)
        .item(CliViewMode::Tree, "Tree", "")
        .item(CliViewMode::List, "List", "")
        .item(CliViewMode::Grid, "Grid", "")
        .interact()
        .unwrap_or(s.args.view);

    s.args.sort = cliclack::select(t.raw(Prompt::SubSelectSort))
        .initial_value(s.args.sort)
        .item(
            CliSortStrategy::AzFileMerge,
            "AzFileMerge (Domyślne/Default)",
            "",
        )
        .item(CliSortStrategy::ZaFileMerge, "ZaFileMerge", "")
        .item(CliSortStrategy::AzDirMerge, "AzDirMerge", "")
        .item(CliSortStrategy::ZaDirMerge, "ZaDirMerge", "")
        .item(CliSortStrategy::AzFile, "AzFile (Najpierw pliki)", "")
        .item(CliSortStrategy::ZaFile, "ZaFile", "")
        .item(CliSortStrategy::AzDir, "AzDir (Najpierw foldery)", "")
        .item(CliSortStrategy::ZaDir, "ZaDir", "")
        .item(CliSortStrategy::Az, "Az (Alfanumerycznie)", "")
        .item(CliSortStrategy::Za, "Za (Odwrócone)", "")
        .item(CliSortStrategy::None, "None (Brak sortowania)", "")
        .interact()
        .unwrap_or(s.args.sort);

    s.args.no_root = cliclack::confirm(t.raw(Prompt::SubNoRoot))
        .initial_value(s.args.no_root)
        .interact()
        .unwrap_or(s.args.no_root);
}

fn handle_output(s: &mut StateTui, t: &T) {
    let out_p: String = cliclack::input(t.raw(Prompt::SubOutPaths))
        .default_input(s.args.out_path.as_deref().unwrap_or(""))
        .interact()
        .unwrap_or_default();
    s.args.out_path = if out_p.trim().is_empty() {
        None
    } else {
        Some(out_p.trim().to_string())
    };

    let out_c: String = cliclack::input(t.raw(Prompt::SubOutCode))
        .default_input(s.args.out_code.as_deref().unwrap_or(""))
        .interact()
        .unwrap_or_default();
    s.args.out_code = if out_c.trim().is_empty() {
        None
    } else {
        Some(out_c.trim().to_string())
    };

    s.args.by = cliclack::confirm(t.raw(Prompt::SubBy))
        .initial_value(s.args.by)
        .interact()
        .unwrap_or(s.args.by);
}

fn handle_filters(s: &mut StateTui, t: &T) {
    s.args.include = cliclack::confirm(t.raw(Prompt::SubOnMatch))
        .initial_value(s.args.include)
        .interact()
        .unwrap_or(s.args.include);
    s.args.exclude = cliclack::confirm(t.raw(Prompt::SubOnMismatch))
        .initial_value(s.args.exclude)
        .interact()
        .unwrap_or(s.args.exclude);
    s.args.info = cliclack::confirm(t.raw(Prompt::SubInfo))
        .initial_value(s.args.info)
        .interact()
        .unwrap_or(s.args.info);
}

// =====================================================================
// POMOCNICZY PARSER WZORCÓW
// =====================================================================
fn split_patterns(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_braces = 0;

    for c in input.chars() {
        match c {
            '{' => {
                in_braces += 1;
                current.push(c);
            }
            '}' => {
                if in_braces > 0 {
                    in_braces -= 1;
                }
                current.push(c);
            }
            ',' if in_braces == 0 => {
                if !current.trim().is_empty() {
                    result.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.trim().is_empty() {
        result.push(current.trim().to_string());
    }
    result
}
