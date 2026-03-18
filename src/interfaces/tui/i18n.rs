use cargo_plot::i18n::Lang;
use console::style;

fn pad(text: &str) -> String {
    format!("  {}  ", text)
}

pub struct Txt {
    pub pol: &'static str,
    pub eng: &'static str,
}

pub trait Translatable {
    fn trans(&self) -> Txt;
    fn theme(&self, text: String) -> String {
        text
    }
}

pub enum Prompt {
    HeaderMain,
    BtnLang,
    BtnQuickStart,
    BtnPaths,
    BtnView,
    BtnOutput,
    BtnFilters,
    BtnRun,
    BtnExit,
    InputPatterns,
    ExitBye,
    WarnNoPatterns,
    // Prompty dla podmenu (wersje dwujęzyczne w jednej linii dla szybkości)
    SubBasePath,
    SubIgnoreCase,
    SubSelectView,
    SubSelectSort,
    SubNoRoot,
    SubOutPaths,
    SubOutCode,
    SubBy,
    SubOnMatch,
    SubOnMismatch,
    SubInfo,
    BtnCliMode,
    InputCliCommand,
    SuccessCliParse,
}

impl Translatable for Prompt {
    fn trans(&self) -> Txt {
        match self {
            Prompt::HeaderMain => Txt {
                pol: "📦 cargo-plot [POL] - Interaktywny Kreator",
                eng: "📦 cargo-plot [ENG] - Interactive Builder",
            },
            Prompt::BtnLang => Txt {
                pol: "🌍 Zmień język / Change language",
                eng: "🌍 Zmień język / Change language",
            },
            Prompt::BtnQuickStart => Txt {
                pol: "🚀 SZYBKI START (Podaj wzorce i uruchom)",
                eng: "🚀 QUICK START (Enter patterns and run)",
            },
            Prompt::BtnPaths => Txt {
                pol: "🛠️  Ścieżki i Wzorce",
                eng: "🛠️  Paths and Patterns",
            },
            Prompt::BtnView => Txt {
                pol: "👁️  Widok i Sortowanie",
                eng: "👁️  View and Sorting",
            },
            Prompt::BtnOutput => Txt {
                pol: "💾 Zapis plików",
                eng: "💾 Output and Saving",
            },
            Prompt::BtnFilters => Txt {
                pol: "⚙️  Filtry i Opcje",
                eng: "⚙️  Filters and Options",
            },
            Prompt::BtnRun => Txt {
                pol: "▶️  URUCHOM SKANOWANIE",
                eng: "▶️  RUN SCANNER",
            },
            Prompt::BtnExit => Txt {
                pol: "❌ WYJŚCIE",
                eng: "❌ EXIT",
            },
            Prompt::InputPatterns => Txt {
                pol: "Podaj wzorce (oddzielone przecinkiem, np. *.rs, Cargo.toml):",
                eng: "Enter patterns (comma separated, e.g. *.rs, Cargo.toml):",
            },
            Prompt::ExitBye => Txt {
                pol: "Do widzenia!",
                eng: "Goodbye!",
            },
            Prompt::WarnNoPatterns => Txt {
                pol: "Brak wzorców! Podaj przynajmniej jeden.",
                eng: "Missing patterns! Provide at least one.",
            },

            // Podmenu
            Prompt::SubBasePath => Txt {
                pol: "Ścieżka bazowa / Base path:",
                eng: "Ścieżka bazowa / Base path:",
            },
            Prompt::SubIgnoreCase => Txt {
                pol: "Ignorować wielkość liter? / Ignore case?",
                eng: "Ignorować wielkość liter? / Ignore case?",
            },
            Prompt::SubSelectView => Txt {
                pol: "Wybierz widok / Select view:",
                eng: "Wybierz widok / Select view:",
            },
            Prompt::SubSelectSort => Txt {
                pol: "Wybierz sortowanie / Select sorting:",
                eng: "Wybierz sortowanie / Select sorting:",
            },
            Prompt::SubNoRoot => Txt {
                pol: "Ukryć główny folder? / Hide root dir?",
                eng: "Ukryć główny folder? / Hide root dir?",
            },
            Prompt::SubOutPaths => Txt {
                pol: "Plik na ścieżki (puste=Brak, AUTO=domyślny) / Paths output file:",
                eng: "Plik na ścieżki (puste=Brak, AUTO=domyślny) / Paths output file:",
            },
            Prompt::SubOutCode => Txt {
                pol: "Plik na kod (puste=Brak, AUTO=domyślny) / Code output file:",
                eng: "Plik na kod (puste=Brak, AUTO=domyślny) / Code output file:",
            },
            Prompt::SubBy => Txt {
                pol: "Dodać stopkę na dole pliku? / Add info footer?",
                eng: "Dodać stopkę na dole pliku? / Add info footer?",
            },
            Prompt::SubOnMatch => Txt {
                pol: "Pokaż dopasowane? / Show matched?",
                eng: "Pokaż dopasowane? / Show matched?",
            },
            Prompt::SubOnMismatch => Txt {
                pol: "Pokaż odrzucone? / Show rejected?",
                eng: "Pokaż odrzucone? / Show rejected?",
            },
            Prompt::SubInfo => Txt {
                pol: "Pokaż statystyki? / Show info stats?",
                eng: "Pokaż statystyki? / Show info stats?",
            },
            Prompt::BtnCliMode => Txt { pol: "⌨️  Wklej komendę (Raw CLI)", eng: "⌨️  Paste command (Raw CLI)" },
            Prompt::InputCliCommand => Txt { pol: "Wklej flagi lub całą komendę (np. -d ./ -m):", eng: "Paste flags or full command (e.g. -d ./ -m):" },
            Prompt::SuccessCliParse => Txt { pol: "Wczytano konfigurację!", eng: "Configuration loaded!" },
        }
    }

    fn theme(&self, text: String) -> String {
        match self {
            Prompt::BtnQuickStart => style(text).on_blue().white().bold().to_string(),
            Prompt::BtnRun => style(text).on_green().black().bold().to_string(),
            Prompt::BtnLang => style(text).cyan().to_string(),
            Prompt::BtnExit => style(text).red().to_string(),
            Prompt::HeaderMain => style(text).on_white().black().bold().to_string(),
            Prompt::BtnCliMode => style(text).on_black().yellow().bold().to_string(),
            _ => text,
        }
    }
}

pub struct T {
    lang: Lang,
}

impl T {
    pub fn new(lang: Lang) -> Self {
        Self { lang }
    }
    pub fn fmt<I: Translatable>(&self, item: I) -> String {
        let txt = item.trans();
        let text = match self.lang {
            Lang::Pl => txt.pol,
            Lang::En => txt.eng,
        };
        item.theme(pad(text))
    }
    pub fn raw<I: Translatable>(&self, item: I) -> String {
        let txt = item.trans();
        match self.lang {
            Lang::Pl => txt.pol.to_string(),
            Lang::En => txt.eng.to_string(),
        }
    }
}
