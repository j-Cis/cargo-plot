// [ENG]: GUI Internationalization module.
// [POL]: Moduł internacjonalizacji interfejsu graficznego.

use cargo_plot::i18n::Lang;

pub struct GuiI18n {
    pub lang: Lang,
}

pub enum GuiText {
    LabelLang,
    LabelScanPath,
    LabelOutFolder,
    LabelSorting,
    LabelViewMode,
    LabelNoRoot,
    HeadingPatterns,
    LabelIgnoreCase,
    LabelNewPattern,
    BtnAddPattern,
    BtnClearAll,
    BtnBrowse,
    MsgNoPatterns,
    FooterDownload,
    FooterInstall,
    FooterUninstall,
    BtnGenerate,
    LabelAddFooter,
    BtnSaveMatch,
    BtnSaveMismatch,
    TabMatch,
    TabMismatch,
    BtnGenerateCode,
    LabelSkipBinary,
}

impl GuiI18n {
    pub fn new(lang: Option<Lang>) -> Self {
        Self {
            lang: lang.unwrap_or(Lang::En),
        }
    }

    pub fn t(&self, text: GuiText) -> &'static str {
        match self.lang {
            Lang::Pl => match text {
                GuiText::LabelLang => "🌍 Język:",
                GuiText::LabelScanPath => "📂 Ścieżka skanowania:",
                GuiText::LabelOutFolder => "💾 Folder zapisu (Output):",
                GuiText::LabelSorting => "Sortowanie",
                GuiText::LabelViewMode => "Tryb widoku",
                GuiText::LabelNoRoot => "Ukryj ROOT w drzewie",
                GuiText::HeadingPatterns => "🔍 Wzorce dopasowań (Patterns)",
                GuiText::LabelIgnoreCase => "🔠 Ignoruj wielkość liter",
                GuiText::LabelNewPattern => "Nowy:",
                GuiText::BtnAddPattern => "➕ Dodaj wzorzec",
                GuiText::BtnClearAll => "💣 Usuń wszystkie",
                GuiText::BtnBrowse => "Wybierz...",
                GuiText::MsgNoPatterns => "Brak wzorców. Dodaj przynajmniej jeden!",
                GuiText::FooterDownload => "Pobierz binarkę (GitHub)",
                GuiText::FooterInstall => "Instalacja:",
                GuiText::FooterUninstall => "Usuwanie:",
                GuiText::BtnGenerate => "🔄 Generuj / Regeneruj",
                GuiText::LabelAddFooter => "Dodaj stopkę (--by)",
                GuiText::BtnSaveMatch => "💾 Zapisz (-m)",
                GuiText::BtnSaveMismatch => "💾 Zapisz (-x)",
                GuiText::TabMatch => "✔ (-m) MATCH",
                GuiText::TabMismatch => "✖ (-x) MISMATCH",
                GuiText::BtnGenerateCode => "🔄 Generuj kod (Cache)",
                GuiText::LabelSkipBinary => "> *(Pominięto plik binarny/graficzny)*",
            },
            Lang::En => match text {
                GuiText::LabelLang => "🌍 Language:",
                GuiText::LabelScanPath => "📂 Scan path:",
                GuiText::LabelOutFolder => "💾 Output folder:",
                GuiText::LabelSorting => "Sorting",
                GuiText::LabelViewMode => "View mode",
                GuiText::LabelNoRoot => "Hide ROOT in tree",
                GuiText::HeadingPatterns => "🔍 Match Patterns",
                GuiText::LabelIgnoreCase => "🔠 Ignore case",
                GuiText::LabelNewPattern => "New:",
                GuiText::BtnAddPattern => "➕ Add pattern",
                GuiText::BtnClearAll => "💣 Clear all",
                GuiText::BtnBrowse => "Browse...",
                GuiText::MsgNoPatterns => "No patterns. Add at least one!",
                GuiText::FooterDownload => "Download binary (GitHub)",
                GuiText::FooterInstall => "Install:",
                GuiText::FooterUninstall => "Uninstall:",
                GuiText::BtnGenerate => "🔄 Generate / Regenerate",
                GuiText::LabelAddFooter => "Add footer (--by)",
                GuiText::BtnSaveMatch => "💾 Save (-m)",
                GuiText::BtnSaveMismatch => "💾 Save (-x)",
                GuiText::TabMatch => "✔ (-m) MATCH",
                GuiText::TabMismatch => "✖ (-x) MISMATCH",
                GuiText::BtnGenerateCode => "🔄 Generate code (Cache)",
                GuiText::LabelSkipBinary => "> *(Binary/graphic file skipped)*",
            },
        }
    }
}
