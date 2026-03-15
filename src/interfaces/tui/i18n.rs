use super::state::Lang;

// =====================================================================
// BAZOWY FORMATER
// =====================================================================

// Uniwersalny padding - dodaje spacje po bokach każdego tekstu.
// Zmieniasz to tutaj -> zmienia się w CAŁEJ aplikacji!
fn pad(text: &str) -> String {
    format!("  {}  ", text)
}

// =====================================================================
// STRUKTURA WYMUSZAJĄCA JAWNE NAZWY JĘZYKÓW
// =====================================================================

pub struct Txt {
    pub pol: &'static str,
    pub eng: &'static str,
}

// =====================================================================
// GŁÓWNA CECHA (TRAIT)
// =====================================================================

pub trait Translatable {
    // 1. Zwraca tekst w wielu językach
    fn trans(&self) -> Txt;

    // 2. Opcjonalna stylizacja (kolory itp.).
    // Domyślnie zwraca tekst bez zmian.
    fn theme(&self, text: String) -> String {
        text
    }
}

// =====================================================================
// GLOBALNE TEKSTY (Nagłówki, Prompty, Komunikaty)
// =====================================================================

pub enum Prompt {
    HeaderEnter,
    HeaderJobAdd,
    HeaderJobAddCustom,
    HeaderJobsManager,
    HeaderStyle,
    HeaderOutput,
    EnterDepth,
    NoJobsWarning,
    SuccessJobAdd,
    JobAlreadyAtTop,
    JobAlreadyAtBottom,
    SuccessJobDeleted,
    JobTitlePrefix,
    ExitBye,
    Canceled,
}

impl Translatable for Prompt {
    fn trans(&self) -> Txt {
        match self {
            Prompt::HeaderEnter => Txt {
                pol: "📦 j-Cis/cargo-plot [POL]",
                eng: "📦 j-Cis/cargo-plot [ENG]",
            },
            Prompt::HeaderJobAdd => Txt {
                pol: "Dodawanie zadania",
                eng: "Adding a job",
            },
            Prompt::HeaderJobAddCustom => Txt {
                pol: "Definiowanie zadania",
                eng: "Customizing job",
            },
            Prompt::HeaderJobsManager => Txt {
                pol: "Menadżer zadań",
                eng: "Jobs manager",
            },
            Prompt::HeaderStyle => Txt {
                pol: "Stylizacja struktury",
                eng: "Paths structure styling",
            },
            Prompt::HeaderOutput => Txt {
                pol: "Zapisywanie wyniku",
                eng: "Saving output",
            },
            Prompt::EnterDepth => Txt {
                pol: "Podaj głębokość:",
                eng: "Enter depth:",
            },
            Prompt::NoJobsWarning => Txt {
                pol: "Brak zadań w kolejce!",
                eng: "No jobs in the queue!",
            },
            Prompt::SuccessJobAdd => Txt {
                pol: "Dodano zadanie do kolejki!",
                eng: "Job added to queue!",
            },
            Prompt::JobAlreadyAtTop => Txt {
                pol: "Zadanie jest już na samej górze!",
                eng: "Job is already at the top!",
            },
            Prompt::JobAlreadyAtBottom => Txt {
                pol: "Zadanie jest już na samym dole!",
                eng: "Job is already at the bottom!",
            },
            Prompt::SuccessJobDeleted => Txt {
                pol: "Usunięto zadanie.",
                eng: "Job deleted.",
            },
            Prompt::JobTitlePrefix => Txt {
                pol: "Zadanie: ",
                eng: "Job: ",
            },
            Prompt::ExitBye => Txt {
                pol: "Do widzenia!",
                eng: "Goodbye!",
            },
            Prompt::Canceled => Txt {
                pol: "Anulowano",
                eng: "Canceled",
            },
        }
    }
}

// =====================================================================
// KONTEKST TŁUMACZA (Translator)
// =====================================================================

pub struct T {
    lang: Lang,
}
impl T {
    pub fn new(lang: Lang) -> Self {
        Self { lang }
    }

    fn get_text<I: Translatable>(&self, item: &I) -> String {
        let txt = item.trans();
        let text = match self.lang {
            Lang::POL => txt.pol,
            Lang::ENG => txt.eng,
        };
        pad(text)
    }

    // Zmiana nazwy na `fmt`!
    pub fn fmt<I: Translatable>(&self, item: I) -> String {
        let base_text = self.get_text(&item);
        item.theme(base_text)
    }

    pub fn raw<I: Translatable>(&self, item: I) -> String {
        self.get_text(&item)
    }
}
