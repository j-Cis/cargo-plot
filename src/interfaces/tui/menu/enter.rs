use super::super::i18n::{Prompt, T, Translatable, Txt};
use super::super::state::{Lang, StateTui};
use super::job_add::menu_job_add;
use super::jobs_manager::menu_jobs_manager;
use super::output_save::menu_output_save;
use super::paths_struct_style::menu_paths_struct_style;
use console::style;

#[derive(Default, Clone, Eq, PartialEq)]
enum ActionEnter {
    #[default]
    ChangeLang,
    JobAdd,
    JobsManager,
    PathsStructStyle,
    PathsStructPrint,
    OutputSave,
    CommandViewStructure,
    CommandSaveDocuments,
    Exit,
}

//impl Default for ActionEnter {
//    fn default() -> Self {
//        Self::ChangeLang
//    }
//}

// =====================================================================
// WARSTWA JĘZYKOWO - STYLIZACYJNA
// =====================================================================

impl Translatable for ActionEnter {
    fn trans(&self) -> Txt {
        match self {
            ActionEnter::ChangeLang => Txt {
                pol: "ustaw POL język",
                eng: "set ENG lang",
            },
            ActionEnter::JobAdd => Txt {
                pol: "dodaj zadanie",
                eng: "add job",
            },
            ActionEnter::JobsManager => Txt {
                pol: "menadżer zadań",
                eng: "manager jobs",
            },
            ActionEnter::PathsStructStyle => Txt {
                pol: "stylizacja struktury ścieżek",
                eng: "style of paths structure",
            },
            ActionEnter::PathsStructPrint => Txt {
                pol: "wyświetl strukturę ścieżek",
                eng: "print the path structure",
            },
            ActionEnter::OutputSave => Txt {
                pol: "zapisz wynik",
                eng: "save output",
            },
            ActionEnter::CommandViewStructure => Txt {
                pol: "komenda podglądu struktury",
                eng: "command print structure",
            },
            ActionEnter::CommandSaveDocuments => Txt {
                pol: "komenda zapisu dokumentacji",
                eng: "command save documents",
            },
            ActionEnter::Exit => Txt {
                pol: "wyjście",
                eng: "exit",
            },
        }
    }

    fn theme(&self, text: String) -> String {
        match self {
            // Wyróżniamy "Dodaj zadanie" na białym tle
            ActionEnter::JobAdd => style(text).on_white().black().to_string(),

            // Wyróżniamy akcje związane z wynikiem mocnym niebieskim
            ActionEnter::PathsStructPrint | ActionEnter::OutputSave => {
                style(text).bold().on_blue().white().to_string()
            }

            // NOWOŚĆ: Fioletowe tło (magenta) i biały tekst dla komendy
            ActionEnter::CommandViewStructure => {
                style(text).bold().on_magenta().white().to_string()
            }
            // NOWOŚĆ: Fioletowe tło (magenta) i biały tekst dla komendy
            ActionEnter::CommandSaveDocuments => {
                style(text).bold().on_magenta().white().to_string()
            }

            // Reszta bez specjalnego formatowania
            _ => text,
        }
    }
}

// =====================================================================
// WIDOK MENU GŁÓWNEGO
// =====================================================================

pub fn menu_enter(s: &mut StateTui) {
    let mut last_action = ActionEnter::default();

    loop {
        // 1. INICJUJEMY TŁUMACZA DLA TEJ PĘTLI
        let t = T::new(s.lang);

        // 2. STYLIZUJEMY GŁÓWNY NAGŁÓWEK (Pobierany z globalnych Promptów)
        let header = style(t.raw(Prompt::HeaderEnter))
            .on_white()
            .black()
            .to_string();

        // 3. BUDUJEMY MENU
        let action_result = cliclack::select(header)
            .initial_value(last_action.clone())
            .item(ActionEnter::ChangeLang, t.fmt(ActionEnter::ChangeLang), "")
            .item(ActionEnter::JobAdd, t.fmt(ActionEnter::JobAdd), "")
            .item(
                ActionEnter::JobsManager,
                t.fmt(ActionEnter::JobsManager),
                "",
            )
            .item(
                ActionEnter::PathsStructStyle,
                t.fmt(ActionEnter::PathsStructStyle),
                "",
            )
            .item(
                ActionEnter::PathsStructPrint,
                t.fmt(ActionEnter::PathsStructPrint),
                "",
            )
            .item(ActionEnter::OutputSave, t.fmt(ActionEnter::OutputSave), "")
            .item(
                ActionEnter::CommandViewStructure,
                t.fmt(ActionEnter::CommandViewStructure),
                "",
            )
            .item(
                ActionEnter::CommandSaveDocuments,
                t.fmt(ActionEnter::CommandSaveDocuments),
                "",
            )
            .item(ActionEnter::Exit, t.fmt(ActionEnter::Exit), "")
            .interact();

        // 4. OBSŁUGA AKCJI
        match action_result {
            Ok(action) => {
                last_action = action.clone();

                match action {
                    ActionEnter::ChangeLang => {
                        s.lang = match s.lang {
                            Lang::POL => Lang::ENG,
                            Lang::ENG => Lang::POL,
                        };
                        let t_new = T::new(s.lang);
                        cliclack::intro(t_new.raw(Prompt::HeaderEnter)).unwrap();
                    }
                    ActionEnter::JobAdd => {
                        menu_job_add(s);
                    }
                    ActionEnter::JobsManager => {
                        menu_jobs_manager(s);
                    }
                    ActionEnter::PathsStructStyle => {
                        menu_paths_struct_style(s);
                    }
                    ActionEnter::PathsStructPrint => {
                        // TUTAJ MIEJSCE NA FUNKCJE WYŚWIETLANIA DRZEWA (np. pager minus)
                    }
                    ActionEnter::OutputSave => {
                        menu_output_save(s);
                    }
                    ActionEnter::CommandViewStructure => {
                        // Na razie opcja nic nie robi - wraca z powrotem do pętli menu głównego
                    }
                    ActionEnter::CommandSaveDocuments => {
                        // Na razie opcja nic nie robi - wraca z powrotem do pętli menu głównego
                    }
                    ActionEnter::Exit => {
                        cliclack::outro(t.raw(Prompt::ExitBye)).unwrap();
                        break;
                    }
                }
            }
            Err(_) => {
                cliclack::outro_cancel(t.raw(Prompt::Canceled)).unwrap();
                break;
            }
        }
    }
}
