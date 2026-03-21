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

    pub fn footer_tool(&self) -> &str {
        match self.lang {
            Lang::Pl => "Narzędzie",
            _ => "Tool",
        }
    }
    pub fn footer_input(&self) -> &str {
        match self.lang {
            Lang::Pl => "Folder",
            _ => "Input",
        }
    }
    pub fn footer_cmd(&self) -> &str {
        match self.lang {
            Lang::Pl => "Komenda",
            _ => "Command",
        }
    }
    pub fn footer_tag(&self) -> &str {
        match self.lang {
            Lang::Pl => "Tag",
            _ => "TimeTag",
        }
    }
    pub fn footer_links(&self) -> &str {
        match self.lang {
            Lang::Pl => "Linki",
            _ => "Links",
        }
    }
    pub fn footer_help(&self) -> &str {
        match self.lang {
            Lang::Pl => "Pomoc",
            _ => "Help",
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
}
