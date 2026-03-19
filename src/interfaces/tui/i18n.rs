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
    BtnHelp, 
    HelpPause, 
    SubHelpHeader, HelpPatternsBtn, HelpFlagsBtn, HelpTextPatterns, HelpTextFlags, 
    BtnGui,
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
            Prompt::BtnCliMode => Txt {
                pol: "⌨️  Wklej komendę (Raw CLI)",
                eng: "⌨️  Paste command (Raw CLI)",
            },
            Prompt::InputCliCommand => Txt {
                pol: "Wklej flagi lub całą komendę (np. -d ./ -m):",
                eng: "Paste flags or full command (e.g. -d ./ -m):",
            },
            Prompt::SuccessCliParse => Txt {
                pol: "Wczytano konfigurację!",
                eng: "Configuration loaded!",
            },
            Prompt::BtnHelp => Txt { 
                pol: "❓ Pomoc (Wzorce i Flagi)", 
                eng: "❓ Help (Patterns & Flags)" 
            },
            Prompt::SubHelpHeader => Txt {
                pol: "Wybierz temat pomocy:",
                eng: "Choose help topic:"
            },
            Prompt::HelpPatternsBtn => Txt {
                pol: "Składnia Wzorców",
                eng: "Patterns Syntax"
            },
            Prompt::HelpFlagsBtn => Txt {
                pol: "Opis Flag i Opcji",
                eng: "Flags & Options Description"
            },
            Prompt::HelpTextPatterns => Txt {
                pol: "=== WZORCE DOPASOWAŃ ===
* - Dowolne znaki (np. *.rs)
** - Dowolne zagnieżdżenie (np. src/**/*.rs)
{a,b}   - Rozwinięcie klamrowe (np. {src,tests}/*.rs)
!       - Negacja / Odrzucenie (np. !*test*)
+       - Tryb głęboki: cała zawartość folderu (np. src/+)
@       - Rodzeństwo: wymaga pary plik + folder o tej samej nazwie
$       - Sierota: dopasowuje TYLKO, gdy brakuje pary plik/folder

=== PRZYKŁADY ===
*.rs               -> Pokaż wszystkie pliki .rs
!@tui{.rs,/}+      -> Wyklucz plik tui.rs oraz folder tui/ z całą zawartością (+)",
                
                eng: "=== PATTERN SYNTAX ===
* - Any characters (e.g. *.rs)
** - Any dir depth (e.g. src/**/*.rs)
{a,b}   - Brace expansion (e.g. {src,tests}/*.rs)
!       - Negation / Reject (e.g. !*test*)
+       - Deep mode: all contents of a directory (e.g. src/+)
@       - Sibling: requires file + dir pair with the same name
$       - Orphan: matches ONLY when file/dir pair is missing

=== EXAMPLES ===
*.rs               -> Show all .rs files
!@tui{.rs,/}+      -> Exclude tui.rs file and tui/ dir with all its contents (+)"
            },
            Prompt::HelpTextFlags => Txt {
                pol: "=== FLAGI I OPCJE (W TUI JAKO PRZEŁĄCZNIKI) ===
-d, --dir          : Ścieżka bazowa skanowania (Domyślnie: ./)
-p, --pat          : Wzorce dopasowań (wymagane, oddzielane przecinkiem)
-s, --sort         : Strategia sortowania wyników (np. AzFileMerge)
-v, --view         : Widok wyników (Tree, List, Grid)
-m, --on-match     : Pokaż tylko dopasowane ścieżki
-x, --on-mismatch  : Pokaż tylko odrzucone ścieżki
-o, --out-paths    : Zapisz wynik jako listę ścieżek (Markdown)
-c, --out-cache    : Zapisz wynik wraz z kodem plików (Markdown Cache)
-b, --by           : Dodaj stopkę informacyjną z komendą na końcu pliku
-i, --info         : Pokaż statystyki skanowania (Dopasowano/Odrzucono)
--ignore-case      : Ignoruj wielkość liter we wzorcach
--treeview-no-root : Ukryj główny folder roboczy w widoku drzewa",
                
                eng: "=== FLAGS & OPTIONS (TOGGLES IN TUI) ===
-d, --dir          : Base input path to scan (Default: ./)
-p, --pat          : Match patterns (required, comma separated)
-s, --sort         : Sorting strategy (e.g. AzFileMerge)
-v, --view         : Results view (Tree, List, Grid)
-m, --on-match     : Show only matched paths
-x, --on-mismatch  : Show only rejected paths
-o, --out-paths    : Save result as paths list (Markdown)
-c, --out-cache    : Save result with file codes (Markdown Cache)
-b, --by           : Add info footer with command at the end of file
-i, --info         : Show scan statistics (Matched/Rejected)
--ignore-case      : Ignore case in patterns
--treeview-no-root : Hide main working directory in tree view"
            },
            Prompt::HelpPause => Txt { 
                pol: "Naciśnij [Enter], aby wrócić do menu...", 
                eng: "Press [Enter] to return to menu..." 
            },
            Prompt::BtnGui => Txt {
                pol: "🖥️  Otwórz w oknie (GUI)",
                eng: "🖥️  Open in window (GUI)",
            },
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
            Prompt::BtnHelp => style(text).magenta().bold().to_string(),
            Prompt::BtnGui => style(text).on_magenta().white().bold().to_string(),
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
