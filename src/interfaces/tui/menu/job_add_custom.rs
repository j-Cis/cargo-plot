use super::super::i18n::{Prompt, T, Translatable, Txt};
use super::super::state::{JobConfig, Lang, StateTui};
use console::style;

#[derive(Default, Clone, Eq, PartialEq)]
enum ActionJobAddCustom {
    #[default]
    JobTitle,
    PathEnter,
    PathIncludeParentFile,
    GlobPathWhiteList,
    ClearWhiteList,
    GlobPathBlackList,
    ClearBlackList,
    FileTypes,
    ClearFileTypes,
    DirsIncludeEmpty,
    DirsOnly,
    DirsKeepExcludedAsEmptyToDepth,
    Save,
    Back,
}

// =====================================================================
// WARSTWA JĘZYKOWO - STYLIZACYJNA
// =====================================================================

impl Translatable for ActionJobAddCustom {
    fn trans(&self) -> Txt {
        match self {
            ActionJobAddCustom::JobTitle => Txt {
                pol: "Tytuł zadania",
                eng: "Job title",
            },
            ActionJobAddCustom::PathEnter => Txt {
                pol: "Ścieżka wejściowa",
                eng: "Enter path",
            },
            ActionJobAddCustom::PathIncludeParentFile => Txt {
                pol: "Dołącz plik o tej samej nazwie poziom wyżej",
                eng: "Include file with same name one level up",
            },
            ActionJobAddCustom::GlobPathWhiteList => Txt {
                pol: "Biała lista (include)",
                eng: "White list (include)",
            },
            ActionJobAddCustom::ClearWhiteList => Txt {
                pol: "   [ Wyczyść białą listę ]",
                eng: "   [ Clear white list ]",
            },
            ActionJobAddCustom::GlobPathBlackList => Txt {
                pol: "Czarna lista (exclude)",
                eng: "Black list (exclude)",
            },
            ActionJobAddCustom::ClearBlackList => Txt {
                pol: "   [ Wyczyść czarną listę ]",
                eng: "   [ Clear black list ]",
            },
            ActionJobAddCustom::FileTypes => Txt {
                pol: "Filtry plików w podkatalogach",
                eng: "Subdirectory file filters",
            },
            ActionJobAddCustom::ClearFileTypes => Txt {
                pol: "   [ Wyczyść filtry plików ]",
                eng: "   [ Clear file filters ]",
            },
            ActionJobAddCustom::DirsIncludeEmpty => Txt {
                pol: "Uwzględnij puste ścieżki",
                eng: "Include empty paths",
            },
            ActionJobAddCustom::DirsOnly => Txt {
                pol: "Zachowuj tylko foldery - bez plików",
                eng: "Keep only directories without files",
            },
            ActionJobAddCustom::DirsKeepExcludedAsEmptyToDepth => Txt {
                pol: "Zachowuj wykluczone katalogi jako puste aż do określonej głębokości",
                eng: "Keep excluded directories as empty up to a specified depth",
            },
            ActionJobAddCustom::Save => Txt {
                pol: "--- ZAPISZ ZADANIE ---",
                eng: "--- SAVE JOB ---",
            },
            ActionJobAddCustom::Back => Txt {
                pol: "Powrót",
                eng: "Back",
            },
        }
    }

    fn theme(&self, text: String) -> String {
        match self {
            ActionJobAddCustom::Save => style(text).on_green().black().bold().to_string(),
            ActionJobAddCustom::ClearWhiteList
            | ActionJobAddCustom::ClearBlackList
            | ActionJobAddCustom::ClearFileTypes => style(text).yellow().italic().to_string(),
            _ => text,
        }
    }
}

// =====================================================================
// WIDOK MENU
// =====================================================================

pub fn menu_job_add_custom(s: &mut StateTui) -> bool {
    let mut last_action = ActionJobAddCustom::default();
    let mut current_job = JobConfig::default();

    loop {
        let t = T::new(s.lang);
        let header = style(t.raw(Prompt::HeaderJobAddCustom))
            .on_white()
            .black()
            .to_string();

        // 1. Zabezpieczenie UX (Ukrywanie opcji zależnych od czarnej listy)
        if current_job.glob_excludes.is_empty() {
            current_job.dirs_only = false;
            current_job.dirs_keep_excluded_as_empty_to_depth = 0;

            if last_action == ActionJobAddCustom::DirsOnly
                || last_action == ActionJobAddCustom::DirsKeepExcludedAsEmptyToDepth
            {
                last_action = ActionJobAddCustom::GlobPathBlackList;
            }
        }

        // 2. Przygotowanie etykiet
        let toggle = |b: bool| if b { "[x]" } else { "[ ]" };
        let rules_txt = match s.lang {
            Lang::POL => "reguł",
            Lang::ENG => "rules",
        };
        let format_list = |v: &Vec<String>| {
            if v.is_empty() {
                "[]".to_string()
            } else {
                format!("[{}]", v.join(", "))
            }
        };

        let title_lbl = format!(
            "{} [{}]",
            t.fmt(ActionJobAddCustom::JobTitle),
            current_job.title
        );
        let path_lbl = format!(
            "{} [{}]",
            t.fmt(ActionJobAddCustom::PathEnter),
            current_job.path_enter
        );
        let parent_file_lbl = format!(
            "{} {}",
            t.fmt(ActionJobAddCustom::PathIncludeParentFile),
            toggle(current_job.path_include_parent_file)
        );

        let include_lbl = format!(
            "{} {}",
            t.fmt(ActionJobAddCustom::GlobPathWhiteList),
            format_list(&current_job.glob_includes)
        );
        let exclude_lbl = format!(
            "{} {}",
            t.fmt(ActionJobAddCustom::GlobPathBlackList),
            format_list(&current_job.glob_excludes)
        );
        let file_types_lbl = format!(
            "{} ({} {}) {}",
            t.fmt(ActionJobAddCustom::FileTypes),
            current_job.file_types.len(),
            rules_txt,
            format_list(&current_job.file_types)
        );

        let dirs_empty_lbl = format!(
            "{} {}",
            t.fmt(ActionJobAddCustom::DirsIncludeEmpty),
            toggle(current_job.dirs_include_empty)
        );
        let dirs_only_lbl = format!(
            "{} {}",
            t.fmt(ActionJobAddCustom::DirsOnly),
            toggle(current_job.dirs_only)
        );

        // 3. Budowanie Menu
        let mut menu = cliclack::select(header)
            .initial_value(last_action.clone())
            .item(ActionJobAddCustom::JobTitle, title_lbl, "")
            .item(ActionJobAddCustom::PathEnter, path_lbl, "")
            .item(
                ActionJobAddCustom::PathIncludeParentFile,
                parent_file_lbl,
                "",
            )
            .item(ActionJobAddCustom::GlobPathWhiteList, include_lbl, "");

        if !current_job.glob_includes.is_empty() {
            menu = menu.item(
                ActionJobAddCustom::ClearWhiteList,
                t.fmt(ActionJobAddCustom::ClearWhiteList),
                "",
            );
        }

        menu = menu.item(ActionJobAddCustom::GlobPathBlackList, exclude_lbl, "");

        if !current_job.glob_excludes.is_empty() {
            menu = menu.item(
                ActionJobAddCustom::ClearBlackList,
                t.fmt(ActionJobAddCustom::ClearBlackList),
                "",
            );
        }

        menu = menu.item(ActionJobAddCustom::FileTypes, file_types_lbl, "");

        if !current_job.file_types.is_empty() {
            menu = menu.item(
                ActionJobAddCustom::ClearFileTypes,
                t.fmt(ActionJobAddCustom::ClearFileTypes),
                "",
            );
        }

        menu = menu.item(ActionJobAddCustom::DirsIncludeEmpty, dirs_empty_lbl, "");

        if !current_job.glob_excludes.is_empty() {
            menu = menu.item(ActionJobAddCustom::DirsOnly, dirs_only_lbl, "");
            menu = menu.item(
                ActionJobAddCustom::DirsKeepExcludedAsEmptyToDepth,
                t.fmt(ActionJobAddCustom::DirsKeepExcludedAsEmptyToDepth),
                "",
            );
        }

        let action_result = menu
            .item(
                ActionJobAddCustom::Save,
                t.fmt(ActionJobAddCustom::Save),
                "",
            )
            .item(
                ActionJobAddCustom::Back,
                t.fmt(ActionJobAddCustom::Back),
                "",
            )
            .interact();

        // 4. Obsługa akcji
        match action_result {
            Ok(action) => {
                last_action = action.clone();

                match action {
                    ActionJobAddCustom::JobTitle => {
                        let val: String = cliclack::input(t.raw(ActionJobAddCustom::JobTitle))
                            .default_input(&current_job.title)
                            .interact()
                            .unwrap_or(current_job.title.clone());
                        current_job.title = val;
                    }
                    ActionJobAddCustom::PathEnter => {
                        let val: String = cliclack::input(t.raw(ActionJobAddCustom::PathEnter))
                            .default_input(&current_job.path_enter)
                            .interact()
                            .unwrap_or(current_job.path_enter.clone());
                        current_job.path_enter = val;
                    }
                    ActionJobAddCustom::PathIncludeParentFile => {
                        current_job.path_include_parent_file =
                            !current_job.path_include_parent_file;
                    }
                    ActionJobAddCustom::GlobPathWhiteList => {
                        let val: String =
                            cliclack::input(t.raw(ActionJobAddCustom::GlobPathWhiteList))
                                .multiline()
                                .default_input(&current_job.glob_includes.join("\n"))
                                .interact()
                                .unwrap_or(current_job.glob_includes.join("\n"));
                        current_job.glob_includes = val
                            .lines()
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    ActionJobAddCustom::ClearWhiteList => {
                        current_job.glob_includes.clear();
                        last_action = ActionJobAddCustom::GlobPathWhiteList;
                    }
                    ActionJobAddCustom::GlobPathBlackList => {
                        let val: String =
                            cliclack::input(t.raw(ActionJobAddCustom::GlobPathBlackList))
                                .multiline()
                                .default_input(&current_job.glob_excludes.join("\n"))
                                .interact()
                                .unwrap_or(current_job.glob_excludes.join("\n"));
                        current_job.glob_excludes = val
                            .lines()
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    ActionJobAddCustom::ClearBlackList => {
                        current_job.glob_excludes.clear();
                        last_action = ActionJobAddCustom::GlobPathBlackList;
                    }
                    ActionJobAddCustom::FileTypes => {
                        let val: String = cliclack::input(t.raw(ActionJobAddCustom::FileTypes))
                            .multiline()
                            .default_input(&current_job.file_types.join("\n"))
                            .interact()
                            .unwrap_or(current_job.file_types.join("\n"));
                        current_job.file_types = val
                            .lines()
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    ActionJobAddCustom::ClearFileTypes => {
                        current_job.file_types.clear();
                        last_action = ActionJobAddCustom::FileTypes;
                    }
                    ActionJobAddCustom::DirsIncludeEmpty => {
                        current_job.dirs_include_empty = !current_job.dirs_include_empty;
                    }
                    ActionJobAddCustom::DirsOnly => {
                        current_job.dirs_only = !current_job.dirs_only;
                    }
                    ActionJobAddCustom::DirsKeepExcludedAsEmptyToDepth => {
                        let val: String = cliclack::input(t.raw(Prompt::EnterDepth))
                            .default_input(
                                &current_job.dirs_keep_excluded_as_empty_to_depth.to_string(),
                            )
                            .interact()
                            .unwrap_or_else(|_| {
                                current_job.dirs_keep_excluded_as_empty_to_depth.to_string()
                            });
                        if let Ok(num) = val.parse::<u32>() {
                            current_job.dirs_keep_excluded_as_empty_to_depth = num;
                        }
                    }
                    ActionJobAddCustom::Save => {
                        s.add_job(current_job);
                        cliclack::clear_screen().unwrap();
                        cliclack::intro(t.raw(Prompt::HeaderEnter)).unwrap();
                        return true;
                    }
                    ActionJobAddCustom::Back => {
                        cliclack::clear_screen().unwrap();
                        cliclack::intro(t.raw(Prompt::HeaderJobAdd)).unwrap();
                        return false;
                    }
                }
                cliclack::clear_screen().unwrap();
                cliclack::intro(t.raw(Prompt::HeaderJobAddCustom)).unwrap();
            }
            Err(_) => {
                cliclack::clear_screen().unwrap();
                cliclack::intro(T::new(s.lang).raw(Prompt::HeaderJobAdd)).unwrap();
                return false;
            }
        }
    }
}
