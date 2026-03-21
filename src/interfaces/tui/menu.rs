use super::i18n::{Prompt, T};
use super::state::StateTui;
use crate::interfaces::cli::args::CargoCli;
use crate::interfaces::cli::args::{CliSortStrategy, CliViewMode};
use crate::interfaces::cli::engine;
use clap::Parser;
use console::style;

#[derive(Clone, PartialEq, Eq)]
enum Action {
    Lang,
    QuickStart,
    CliMode,
    Paths,
    View,
    Output,
    Filters,
    Help,
    Run,
    Gui,
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

        let out_p = s.args.dir_out.as_deref().unwrap_or("AUTO");
        let lbl_out = format!(
            "{} (dir-out: {}, address: {}, archive: {}, by: {})",
            t.fmt(Prompt::BtnOutput),
            out_p,
            s.args.save_address,
            s.args.save_archive,
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
        let links_hint = style("crates.io/crates/cargo-plot  |  github.com/j-Cis/cargo-plot")
            .dim()
            .to_string();
        let action_result = cliclack::select(header)
            .initial_value(last_action.clone())
            .item(Action::Lang, t.fmt(Prompt::BtnLang), "")
            .item(Action::QuickStart, t.fmt(Prompt::BtnQuickStart), "")
            .item(Action::CliMode, t.fmt(Prompt::BtnCliMode), "")
            .item(Action::Paths, lbl_paths, "")
            .item(Action::View, lbl_view, "")
            .item(Action::Output, lbl_out, "")
            .item(Action::Filters, lbl_filt, "")
            .item(Action::Help, t.fmt(Prompt::BtnHelp), "")
            .item(Action::Run, t.fmt(Prompt::BtnRun), "")
            .item(Action::Gui, t.fmt(Prompt::BtnGui), "")
            .item(Action::Exit, t.fmt(Prompt::BtnExit), links_hint)
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
            Ok(Action::CliMode) => {
                let cmd: String = cliclack::input(t.raw(Prompt::InputCliCommand))
                    .interact()
                    .unwrap_or_default();

                if !cmd.trim().is_empty() {
                    // ⚡ Shlex idealnie tnie stringa jak bash, a jeśli ktoś zgubi cudzysłów, wyłapie błąd
                    if let Some(mut parsed_split) = shlex::split(&cmd) {
                        // Czyścimy początek (wywalamy "cargo", "run", "--", "plot")
                        while !parsed_split.is_empty() {
                            let first = parsed_split[0].to_lowercase();
                            if first == "cargo"
                                || first == "run"
                                || first == "--"
                                || first == "plot"
                                || first.contains("cargo-plot")
                            {
                                parsed_split.remove(0);
                            } else {
                                break;
                            }
                        }

                        // Podajemy do parsera Clap
                        let mut cli_args = vec!["cargo".to_string(), "plot".to_string()];
                        cli_args.extend(parsed_split);

                        match CargoCli::try_parse_from(cli_args) {
                            Ok(CargoCli::Plot(parsed_args)) => {
                                s.args = parsed_args;
                                cliclack::log::success(t.raw(Prompt::SuccessCliParse)).unwrap();
                            }
                            Err(e) => {
                                cliclack::log::error(format!("{}", e)).unwrap();
                            }
                        }
                    } else {
                        // Obsługa błędu ze strony shlex
                        cliclack::log::error(
                            "Błąd parsowania komendy! Prawdopodobnie nie domknięto cudzysłowu.",
                        )
                        .unwrap();
                    }
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
            Ok(Action::Help) => {
                let help_choice = cliclack::select(t.raw(Prompt::SubHelpHeader))
                    .item(1, t.raw(Prompt::HelpPatternsBtn), "")
                    .item(2, t.raw(Prompt::HelpFlagsBtn), "")
                    .item(0, t.raw(Prompt::BtnExit), "")
                    .interact()
                    .unwrap_or(0);

                if help_choice == 1 {
                    cliclack::note("📖 WZORCE / PATTERNS", t.raw(Prompt::HelpTextPatterns))
                        .unwrap();
                    let _: String = cliclack::input(t.raw(Prompt::HelpPause))
                        .required(false) // ⚡ TO POZWALA NA PUSTY ENTER
                        .interact()
                        .unwrap_or_default();
                } else if help_choice == 2 {
                    cliclack::note("⚙️ FLAGI / FLAGS", t.raw(Prompt::HelpTextFlags)).unwrap();
                    let _: String = cliclack::input(t.raw(Prompt::HelpPause))
                        .required(false) // ⚡ TO POZWALA NA PUSTY ENTER
                        .interact()
                        .unwrap_or_default();
                }
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
            Ok(Action::Gui) => {
                // Wyświetlamy komunikat na pożegnanie z terminalem
                cliclack::outro(t.fmt(Prompt::BtnGui)).unwrap();

                // Odpalamy nasze nowe okienko, przekazując mu całą zebraną konfigurację
                crate::interfaces::gui::run_gui(s.args.clone());

                // Zamykamy pętlę TUI - pałeczkę przejmuje egui!
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
    let out_p: String = cliclack::input(t.raw(Prompt::SubDirOut))
        .default_input(s.args.dir_out.as_deref().unwrap_or(""))
        .interact()
        .unwrap_or_default();
    s.args.dir_out = if out_p.trim().is_empty() {
        None
    } else {
        Some(out_p.trim().to_string())
    };

    s.args.save_address = cliclack::confirm(t.raw(Prompt::SubSaveAddress))
        .initial_value(s.args.save_address)
        .interact()
        .unwrap_or(s.args.save_address);

    s.args.save_archive = cliclack::confirm(t.raw(Prompt::SubSaveArchive))
        .initial_value(s.args.save_archive)
        .interact()
        .unwrap_or(s.args.save_archive);

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

/*/
fn split_cli_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';

    for c in input.chars() {
        if in_quotes {
            if c == quote_char {
                in_quotes = false;
            } else {
                current.push(c);
            }
        } else {
            match c {
                '"' | '\'' => {
                    in_quotes = true;
                    quote_char = c;
                }
                ' ' => {
                    if !current.is_empty() {
                        args.push(current.clone());
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }
    }
    if !current.is_empty() {
        args.push(current);
    }
    args
}
    */
