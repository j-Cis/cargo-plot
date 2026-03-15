use super::super::i18n::{Prompt, T, Translatable, Txt}; // Importujemy nasz nowy, odchudzony silnik!
use super::super::state::{JobConfig, StateTui};
use super::job_add_custom::menu_job_add_custom;
use console::style;

#[derive(Default, Clone, Eq, PartialEq)]
enum ActionJobAdd {
    #[default]
    JobAddDefault,
    JobAddCustom,
    Back,
}

//impl Default for ActionJobAdd {
//    fn default() -> Self {
//        Self::JobAddDefault
//    }
//}

// =====================================================================
// WARSTWA JĘZYKOWO - STYLIZACYJNA
// =====================================================================

impl Translatable for ActionJobAdd {
    fn trans(&self) -> Txt {
        match self {
            ActionJobAdd::JobAddDefault => Txt {
                pol: "Dodaj zadanie domyślne",
                eng: "Add default job",
            },
            ActionJobAdd::JobAddCustom => Txt {
                pol: "Zdefiniuj zadanie",
                eng: "Customize job",
            },
            ActionJobAdd::Back => Txt {
                pol: "Powrót",
                eng: "Back",
            },
        }
    }

    // Nadpisujemy domyślny styl tylko dla jednego przycisku!
    fn theme(&self, text: String) -> String {
        match self {
            ActionJobAdd::JobAddCustom => style(text).on_white().blue().to_string(),
            _ => text, // Pozostałe opcje zwracają zwykły tekst
        }
    }
}

// =====================================================================
// WIDOK MENU
// =====================================================================

pub fn menu_job_add(s: &mut StateTui) {
    let mut last_action = ActionJobAdd::default();

    loop {
        // 1. INICJUJEMY TŁUMACZA DLA TEJ PĘTLI
        let t = T::new(s.lang);

        // 2. STYLIZUJEMY NAGŁÓWEK (pobierany z globalnych Promptów)
        let header = style(t.raw(Prompt::HeaderJobAdd))
            .on_white()
            .black()
            .to_string();

        // 3. BUDUJEMY MENU (czysto, zwięźle i z podpowiedziami kompilatora)
        let action_result = cliclack::select(header)
            .initial_value(last_action.clone())
            .item(
                ActionJobAdd::JobAddDefault,
                t.fmt(ActionJobAdd::JobAddDefault),
                "",
            )
            .item(
                ActionJobAdd::JobAddCustom,
                t.fmt(ActionJobAdd::JobAddCustom),
                "",
            )
            .item(ActionJobAdd::Back, t.fmt(ActionJobAdd::Back), "")
            .interact();

        match action_result {
            Ok(action) => {
                last_action = action.clone();

                match action {
                    ActionJobAdd::JobAddDefault => {
                        s.add_job(JobConfig::default());

                        // Sukces i wyjście też przepuszczamy przez tłumacza
                        cliclack::log::success(t.raw(Prompt::SuccessJobAdd)).unwrap();
                        cliclack::clear_screen().unwrap();
                        cliclack::intro(t.raw(Prompt::HeaderEnter)).unwrap();
                        return;
                    }
                    ActionJobAdd::JobAddCustom => {
                        let saved = menu_job_add_custom(s);
                        if saved {
                            return;
                        }
                    }
                    ActionJobAdd::Back => {
                        cliclack::clear_screen().unwrap();
                        cliclack::intro(t.raw(Prompt::HeaderEnter)).unwrap();
                        return;
                    }
                }
            }
            Err(_) => {
                cliclack::clear_screen().unwrap();
                let t_err = T::new(s.lang); // Tłumacz na wypadek błędu (np. Ctrl+C)
                cliclack::intro(t_err.raw(Prompt::HeaderEnter)).unwrap();
                return;
            }
        }
    }
}
