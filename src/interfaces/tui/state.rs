use crate::interfaces::cli::args::{CliArgs, CliSortStrategy, CliViewMode};
use cargo_plot::i18n::Lang;

pub struct StateTui {
    pub lang: Lang,
    pub args: CliArgs,
}

impl StateTui {
    pub fn new() -> Self {
        let lang = Lang::detect();
        Self {
            lang,
            args: CliArgs {
                // Domyślne wartości, dokładnie takie jak w CLI
                enter_path: ".".to_string(),
                patterns: vec![],
                sort: CliSortStrategy::AzFileMerge,
                view: CliViewMode::Tree,
                include: true, // Domyślnie pokazujemy dopasowania (-m)
                exclude: false,
                out_path: None,
                out_code: None,
                by: false,
                ignore_case: false,
                no_root: false,
                info: true, // Domyślnie włączamy statystyki (-i)
                lang: Some(lang),
            },
        }
    }

    /// Aktualizuje język w interfejsie i w argumentach dla silnika
    pub fn toggle_lang(&mut self) {
        self.lang = match self.lang {
            Lang::Pl => Lang::En,
            Lang::En => Lang::Pl,
        };
        self.args.lang = Some(self.lang);
    }
}
