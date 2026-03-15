use super::super::i18n::{Prompt, T, Translatable, Txt};
use super::super::state::StateTui;
use console::style;

#[derive(Default, Clone, Eq, PartialEq)]
enum ActionJobManage {
    MoveUp,
    MoveDown,
    View,
    Delete,
    #[default]
    Back,
}

// =====================================================================
// WARSTWA JĘZYKOWO - STYLIZACYJNA
// =====================================================================

impl Translatable for ActionJobManage {
    fn trans(&self) -> Txt {
        match self {
            ActionJobManage::MoveUp => Txt {
                pol: "Przesuń wyżej",
                eng: "Move up",
            },
            ActionJobManage::MoveDown => Txt {
                pol: "Przesuń niżej",
                eng: "Move down",
            },
            ActionJobManage::View => Txt {
                pol: "Podgląd",
                eng: "View",
            },
            ActionJobManage::Delete => Txt {
                pol: "Usuń zadanie",
                eng: "Delete job",
            },
            ActionJobManage::Back => Txt {
                pol: "Powrót",
                eng: "Back",
            },
        }
    }

    fn theme(&self, text: String) -> String {
        match self {
            ActionJobManage::Delete => style(text).on_red().white().bold().to_string(), // Ostrzegawczy czerwony!
            _ => text,
        }
    }
}

// =====================================================================
// WIDOK MENU GŁÓWNEGO MENADŻERA
// =====================================================================

pub fn menu_jobs_manager(s: &mut StateTui) {
    loop {
        let t = T::new(s.lang);

        if s.jobs.is_empty() {
            cliclack::log::warning(t.raw(Prompt::NoJobsWarning)).unwrap();
            cliclack::clear_screen().unwrap();
            cliclack::intro(t.raw(Prompt::HeaderEnter)).unwrap();
            return;
        }

        let header = style(t.raw(Prompt::HeaderJobsManager))
            .on_white()
            .black()
            .to_string();

        // USTAWIALNY START: usize::MAX to nasz przycisk "Wstecz"
        let mut menu = cliclack::select(header).initial_value(usize::MAX);

        for (index, job) in s.jobs.iter().enumerate() {
            let styled_title = style(format!(" [{}] {}", index, job.title))
                .yellow()
                .to_string();
            menu = menu.item(index, styled_title, "");
        }

        menu = menu.item(usize::MAX, t.fmt(ActionJobManage::Back), "");

        let selection = menu.interact();

        match selection {
            Ok(usize::MAX) | Err(_) => {
                cliclack::clear_screen().unwrap();
                cliclack::intro(t.raw(Prompt::HeaderEnter)).unwrap();
                return;
            }
            Ok(job_index) => {
                // Odpalamy podmenu dla wybranego zadania
                manage_single_job(s, job_index);
            }
        }
    }
}

// =====================================================================
// WIDOK ZARZĄDZANIA POJEDYNCZYM ZADANIEM
// =====================================================================

fn manage_single_job(s: &mut StateTui, index: usize) {
    loop {
        let t = T::new(s.lang);

        if index >= s.jobs.len() {
            return;
        }

        let job_title = &s.jobs[index].title;
        // Sklejamy: "Zadanie: " + "Tytuł"
        let prompt_title = format!("{}{}", t.raw(Prompt::JobTitlePrefix), job_title);

        let action_result = cliclack::select(prompt_title)
            .item(ActionJobManage::MoveUp, t.fmt(ActionJobManage::MoveUp), "")
            .item(
                ActionJobManage::MoveDown,
                t.fmt(ActionJobManage::MoveDown),
                "",
            )
            .item(ActionJobManage::View, t.fmt(ActionJobManage::View), "")
            .item(ActionJobManage::Delete, t.fmt(ActionJobManage::Delete), "")
            .item(ActionJobManage::Back, t.fmt(ActionJobManage::Back), "")
            .interact();

        match action_result {
            Ok(ActionJobManage::MoveUp) => {
                if index > 0 {
                    s.jobs.swap(index, index - 1);
                    return;
                } else {
                    cliclack::log::warning(t.raw(Prompt::JobAlreadyAtTop)).unwrap();
                }
            }
            Ok(ActionJobManage::MoveDown) => {
                if index < s.jobs.len() - 1 {
                    s.jobs.swap(index, index + 1);
                    return;
                } else {
                    cliclack::log::warning(t.raw(Prompt::JobAlreadyAtBottom)).unwrap();
                }
            }
            Ok(ActionJobManage::View) => {
                let job_info = format!("{:#?}", s.jobs[index]);
                cliclack::log::info(job_info).unwrap();
            }
            Ok(ActionJobManage::Delete) => {
                s.jobs.remove(index);
                cliclack::log::success(t.raw(Prompt::SuccessJobDeleted)).unwrap();
                return;
            }
            Ok(ActionJobManage::Back) | Err(_) => {
                cliclack::clear_screen().unwrap();
                cliclack::intro(t.raw(Prompt::HeaderJobsManager)).unwrap();
                return;
            }
        }
    }
}
