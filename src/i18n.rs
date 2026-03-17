use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Lang {
    Pl,
    En,
}

impl Lang {
    pub fn detect() -> Self {
        if env::var("LANG")
            .unwrap_or_default()
            .to_lowercase()
            .starts_with("pl")
        {
            Self::Pl
        } else {
            Self::En
        }
    }
}

pub struct I18n {
    pub lang: Lang,
}

impl I18n {
    pub fn new(lang: Option<Lang>) -> Self {
        Self {
            lang: lang.unwrap_or_else(Lang::detect),
        }
    }

    // =====================================================================
    // 1. TOP - TEKST OGÓLNY
    // =====================================================================
    pub fn save_success(&self, name: &str, path: &str) -> String {
        match self.lang {
            Lang::Pl => format!("💾 Pomyślnie zapisano {} do pliku: {}", name, path),
            Lang::En => format!("💾 Successfully saved {} to file: {}", name, path),
        }
    }
    pub fn save_err(&self, name: &str, path: &str, err: &str) -> String {
        match self.lang {
            Lang::Pl => format!("❌ Błąd zapisu {} do pliku {}: {}", name, path, err),
            Lang::En => format!("❌ Error saving {} to file {}: {}", name, path, err),
        }
    }
    pub fn dir_create_err(&self, dir: &str, err: &str) -> String {
        match self.lang {
            Lang::Pl => format!("❌ Błąd: Nie można utworzyć katalogu {} ({})", dir, err),
            Lang::En => format!("❌ Error: Cannot create directory {} ({})", dir, err),
        }
    }

    // =====================================================================
    // 2. LIB / CORE - LOGIKA BAZOWA
    // =====================================================================
    pub fn skip_binary(&self) -> &'static str {
        match self.lang {
            Lang::Pl => "> *(Plik binarny/graficzny - pominięto zawartość)*",
            Lang::En => "> *(Binary/graphic file - content skipped)*",
        }
    }
    pub fn read_err(&self) -> &'static str {
        match self.lang {
            Lang::Pl => "> *(Błąd odczytu / plik nie jest UTF-8)*",
            Lang::En => "> *(Read error / file is not UTF-8)*",
        }
    }
    pub fn by_title(&self, typ: &str) -> String {
        match self.lang {
            Lang::Pl => format!("## Command - Query ({typ})"),
            Lang::En => format!("## Command - Query ({typ})"),
        }
    }
    pub fn by_cmd(&self) -> &'static str {
        match self.lang {
            Lang::Pl => "**Wywołana komenda:**",
            Lang::En => "**Executed command:**",
        }
    }
    pub fn by_instructions(&self) -> &'static str {
        match self.lang {
            Lang::Pl => {
                "**Krótka instrukcja flag:**\n- `-d, --dir <PATH>` : Ścieżka wejściowa do skanowania (domyślnie: `.`)\n- `-p, --pat <PATTERNS>...` : Wzorce dopasowań (wymagane)\n- `-s, --sort <STRATEGY>` : Strategia sortowania (np. `az-file-merge`)\n- `-v, --view <MODE>` : Widok wyników (`tree`, `list`, `grid`)\n- `-m, --on-match` : Pokaż tylko dopasowane ścieżki\n- `-x, --on-mismatch` : Pokaż tylko odrzucone ścieżki\n- `-o, --out-paths [PATH]` : Zapisz ścieżki do pliku (AUTO: `./other/`)\n- `-c, --out-cache [PATH]` : Zapisz kod do pliku (AUTO: `./other/`)\n- `-i, --info` : Tryb gadatliwy w terminalu\n- `-b, --by` : Dodaj sekcję informacyjną na końcu pliku\n- `--ignore-case` : Ignoruj wielkość liter we wzorcach\n- `--treeview-no-root` : Ukryj główny folder w widoku drzewa"
            }
            Lang::En => {
                "**Short flags manual:**\n- `-d, --dir <PATH>` : Input path to scan (default: `.`)\n- `-p, --pat <PATTERNS>...` : Match patterns (required)\n- `-s, --sort <STRATEGY>` : Sorting strategy (e.g. `az-file-merge`)\n- `-v, --view <MODE>` : Results view (`tree`, `list`, `grid`)\n- `-m, --on-match` : Show only matched paths\n- `-x, --on-mismatch` : Show only rejected paths\n- `-o, --out-paths [PATH]` : Save paths to file (AUTO: `./other/`)\n- `-c, --out-cache [PATH]` : Save code to file (AUTO: `./other/`)\n- `-i, --info` : Verbose terminal mode\n- `-b, --by` : Add info section at end of file\n- `--ignore-case` : Ignore case in patterns\n- `--treeview-no-root` : Hide root directory in tree view"
            }
        }
    }
    pub fn by_link(&self) -> &'static str {
        match self.lang {
            Lang::Pl => {
                "[📊 Sprawdź `cargo-plot` na crates.io](https://crates.io/crates/cargo-plot)"
            }
            Lang::En => "[📊 Check `cargo-plot` on crates.io](https://crates.io/crates/cargo-plot)",
        }
    }
    pub fn by_version(&self, tag: &str) -> String {
        match self.lang {
            Lang::Pl => format!("**Wersja raportu:** {tag}"),
            Lang::En => format!("**Report version:** {tag}"),
        }
    }

    // =====================================================================
    // 3. CLI - INTERFEJS TERMINALOWY
    // =====================================================================
    pub fn cli_base_abs(&self, path: &str) -> String {
        match self.lang {
            Lang::Pl => format!("📂 Baza terminala (Absolutna): {}", path),
            Lang::En => format!("📂 Terminal base (Absolute): {}", path),
        }
    }
    pub fn cli_target_abs(&self, path: &str) -> String {
        match self.lang {
            Lang::Pl => format!("📂 Cel skanowania (Absolutna): {}", path),
            Lang::En => format!("📂 Scan target (Absolute): {}", path),
        }
    }
    pub fn cli_target_rel(&self, path: &str) -> String {
        match self.lang {
            Lang::Pl => format!("📂 Cel skanowania (Relatywna): {}", path),
            Lang::En => format!("📂 Scan target (Relative): {}", path),
        }
    }
    pub fn cli_case_sensitive(&self, val: bool) -> String {
        match self.lang {
            Lang::Pl => format!("🔠 Wrażliwość na litery: {}", val),
            Lang::En => format!("🔠 Case sensitive: {}", val),
        }
    }
    pub fn cli_patterns_raw(&self, pat: &str) -> String {
        match self.lang {
            Lang::Pl => format!("🔍 Wzorce (RAW): {}", pat),
            Lang::En => format!("🔍 Patterns (RAW): {}", pat),
        }
    }
    pub fn cli_patterns_tok(&self, pat: &str) -> String {
        match self.lang {
            Lang::Pl => format!("⚙️ Wzorce (TOK): {}", pat),
            Lang::En => format!("⚙️ Patterns (TOK): {}", pat),
        }
    }
    pub fn cli_summary_matched(&self, count: usize, total: usize) -> String {
        match self.lang {
            Lang::Pl => format!("📊 Podsumowanie: Dopasowano {} z {} ścieżek.", count, total),
            Lang::En => format!("📊 Summary: Matched {} of {} paths.", count, total),
        }
    }
    pub fn cli_summary_rejected(&self, count: usize, total: usize) -> String {
        match self.lang {
            Lang::Pl => format!("📊 Podsumowanie: Odrzucono {} z {} ścieżek.", count, total),
            Lang::En => format!("📊 Summary: Rejected {} of {} paths.", count, total),
        }
    }

    // =====================================================================
    // 4. TUI - INTERAKTYWNY PANEL
    // =====================================================================
    // (Zostawiamy tu miejsce na przyszłość)
}
