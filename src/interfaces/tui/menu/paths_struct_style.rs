use super::super::i18n::{Prompt, T, Translatable, Txt};
use super::super::state::{Lang, SizeBase, SortOrder, StateTui};
use console::style;

#[derive(Default, Clone, Eq, PartialEq)]
enum ActionStyle {
    SortOrder,
    SizeFiles,
    SizeDirs,
    SizeDirsReal,
    SizeBase,
    Precision,
    #[default]
    Back,
}

//impl Default for ActionStyle {
//    fn default() -> Self { Self::SortOrder }
//}

// =====================================================================
// LOKALNE ENUMY POMOCNICZE
// =====================================================================

#[derive(Clone, Eq, PartialEq)]
enum ActionSort {
    FilesFirst,
    DirsFirst,
    Alphanumeric,
}

enum LocalPrompt {
    SortMode,
    PrecisionInput,
    ErrPrecisionRange,
    ErrPrecisionNotNum,
}

// =====================================================================
// WARSTWA JĘZYKOWO - STYLIZACYJNA
// =====================================================================

impl Translatable for ActionStyle {
    fn trans(&self) -> Txt {
        match self {
            ActionStyle::SortOrder => Txt {
                pol: "Sortowanie",
                eng: "Sort order",
            },
            ActionStyle::SizeFiles => Txt {
                pol: "Rozmiar przy plikach",
                eng: "Size for files",
            },
            ActionStyle::SizeDirs => Txt {
                pol: "Rozmiar przy folderach",
                eng: "Size for directories",
            },
            ActionStyle::SizeDirsReal => Txt {
                pol: "Rzeczywisty rozmiar folderów",
                eng: "Real directory size",
            },
            ActionStyle::SizeBase => Txt {
                pol: "Podstawa rozmiaru",
                eng: "Size base",
            },
            ActionStyle::Precision => Txt {
                pol: "Precyzja",
                eng: "Precision",
            },
            ActionStyle::Back => Txt {
                pol: "Powrót",
                eng: "Back",
            },
        }
    }
}

impl Translatable for ActionSort {
    fn trans(&self) -> Txt {
        match self {
            ActionSort::FilesFirst => Txt {
                pol: "Najpierw pliki",
                eng: "Files first",
            },
            ActionSort::DirsFirst => Txt {
                pol: "Najpierw foldery",
                eng: "Directories first",
            },
            ActionSort::Alphanumeric => Txt {
                pol: "Alfanumerycznie",
                eng: "Alphanumeric",
            },
        }
    }
}

impl Translatable for LocalPrompt {
    fn trans(&self) -> Txt {
        match self {
            LocalPrompt::SortMode => Txt {
                pol: "Wybierz tryb sortowania:",
                eng: "Choose sort mode:",
            },
            LocalPrompt::PrecisionInput => Txt {
                pol: "Podaj precyzję (od 3 do 9):",
                eng: "Enter precision (from 3 to 9):",
            },
            LocalPrompt::ErrPrecisionRange => Txt {
                pol: "Precyzja musi być w przedziale 3-9!",
                eng: "Precision must be between 3 and 9!",
            },
            LocalPrompt::ErrPrecisionNotNum => Txt {
                pol: "To nie jest liczba!",
                eng: "This is not a number!",
            },
        }
    }
}

// =====================================================================
// WIDOK MENU
// =====================================================================

pub fn menu_paths_struct_style(s: &mut StateTui) {
    // Ta linia teraz automatycznie wybierze Back jako start:
    let mut last_action = ActionStyle::default();

    loop {
        let t = T::new(s.lang);
        let header = style(t.raw(Prompt::HeaderStyle))
            .on_white()
            .black()
            .to_string();

        // 1. ZABEZPIECZENIE UX PRZED BUDOWĄ MENU
        // Jeśli rozmiar folderów wyłączony, ukrywamy opcję rzeczywistego rozmiaru
        if !s.struct_config.size_dirs {
            s.struct_config.size_dirs_real = false;
            if last_action == ActionStyle::SizeDirsReal {
                last_action = ActionStyle::SizeDirs;
            }
        }

        // Jeśli rozmiar plików I folderów wyłączony, ukrywamy też bazę i precyzję
        if !s.struct_config.size_files && !s.struct_config.size_dirs {
            s.struct_config.size_base = SizeBase::Base1024; // Wartość domyślna
            s.struct_config.precision = 3; // Wartość domyślna

            if last_action == ActionStyle::SizeBase || last_action == ActionStyle::Precision {
                // Cofamy kursor na cokolwiek, co jest jeszcze widoczne nad nimi
                last_action = ActionStyle::SizeDirs;
            }
        }

        // 2. DYNAMICZNE ETYKIETY
        let sort_str = match s.struct_config.sort {
            SortOrder::FilesFirst => match s.lang {
                Lang::POL => "najpierw pliki",
                Lang::ENG => "files first",
            },
            SortOrder::DirsFirst => match s.lang {
                Lang::POL => "najpierw foldery",
                Lang::ENG => "directories first",
            },
            SortOrder::Alphanumeric => match s.lang {
                Lang::POL => "alfanumerycznie",
                Lang::ENG => "alphanumeric",
            },
        };

        let base_str = match s.struct_config.size_base {
            SizeBase::Base1024 => "1024",
            SizeBase::Base1000 => "1000",
        };

        let toggle = |b: bool| if b { "[x]" } else { "[ ]" };

        let sort_lbl = format!("{} [{}]", t.fmt(ActionStyle::SortOrder), sort_str);
        let s_files_lbl = format!(
            "{} {}",
            t.fmt(ActionStyle::SizeFiles),
            toggle(s.struct_config.size_files)
        );
        let s_dirs_lbl = format!(
            "{} {}",
            t.fmt(ActionStyle::SizeDirs),
            toggle(s.struct_config.size_dirs)
        );
        let s_dirs_real_lbl = format!(
            "{} {}",
            t.fmt(ActionStyle::SizeDirsReal),
            toggle(s.struct_config.size_dirs_real)
        );
        let base_lbl = format!("{} [{}]", t.fmt(ActionStyle::SizeBase), base_str);
        let prec_lbl = format!(
            "{} [{}]",
            t.fmt(ActionStyle::Precision),
            s.struct_config.precision
        );

        // 3. BUDOWANIE MENU
        let mut menu = cliclack::select(header)
            .initial_value(last_action.clone())
            .item(ActionStyle::SortOrder, sort_lbl, "")
            .item(ActionStyle::SizeFiles, s_files_lbl, "")
            .item(ActionStyle::SizeDirs, s_dirs_lbl, "");

        // Warunkowe opcje dla folderów
        if s.struct_config.size_dirs {
            menu = menu.item(ActionStyle::SizeDirsReal, s_dirs_real_lbl, "");
        }

        // Warunkowe opcje globalne dla rozmiarów
        if s.struct_config.size_files || s.struct_config.size_dirs {
            menu = menu.item(ActionStyle::SizeBase, base_lbl, "");
            menu = menu.item(ActionStyle::Precision, prec_lbl, "");
        }

        let action_result = menu
            .item(ActionStyle::Back, t.fmt(ActionStyle::Back), "")
            .interact();

        // 4. OBSŁUGA AKCJI
        match action_result {
            Ok(action) => {
                last_action = action.clone();

                match action {
                    ActionStyle::SortOrder => {
                        let initial_sort_action = match s.struct_config.sort {
                            SortOrder::FilesFirst => ActionSort::FilesFirst,
                            SortOrder::DirsFirst => ActionSort::DirsFirst,
                            SortOrder::Alphanumeric => ActionSort::Alphanumeric,
                        };

                        let val_action = cliclack::select(t.raw(LocalPrompt::SortMode))
                            .initial_value(initial_sort_action)
                            .item(ActionSort::FilesFirst, t.fmt(ActionSort::FilesFirst), "")
                            .item(ActionSort::DirsFirst, t.fmt(ActionSort::DirsFirst), "")
                            .item(
                                ActionSort::Alphanumeric,
                                t.fmt(ActionSort::Alphanumeric),
                                "",
                            )
                            .interact();

                        if let Ok(selected_sort) = val_action {
                            s.struct_config.sort = match selected_sort {
                                ActionSort::FilesFirst => SortOrder::FilesFirst,
                                ActionSort::DirsFirst => SortOrder::DirsFirst,
                                ActionSort::Alphanumeric => SortOrder::Alphanumeric,
                            };
                        }
                    }
                    ActionStyle::SizeFiles => {
                        s.struct_config.size_files = !s.struct_config.size_files
                    }
                    ActionStyle::SizeDirs => s.struct_config.size_dirs = !s.struct_config.size_dirs,
                    ActionStyle::SizeDirsReal => {
                        s.struct_config.size_dirs_real = !s.struct_config.size_dirs_real
                    }
                    ActionStyle::SizeBase => {
                        s.struct_config.size_base = match s.struct_config.size_base {
                            SizeBase::Base1024 => SizeBase::Base1000,
                            SizeBase::Base1000 => SizeBase::Base1024,
                        };
                    }
                    ActionStyle::Precision => {
                        let default_val = s.struct_config.precision.to_string();
                        let val: String = cliclack::input(t.raw(LocalPrompt::PrecisionInput))
                            .default_input(&default_val)
                            .interact()
                            .unwrap_or(default_val);

                        if let Ok(num) = val.parse::<u8>() {
                            if (3..=9).contains(&num) {
                                s.struct_config.precision = num;
                            } else {
                                cliclack::log::error(t.raw(LocalPrompt::ErrPrecisionRange))
                                    .unwrap();
                            }
                        } else {
                            cliclack::log::error(t.raw(LocalPrompt::ErrPrecisionNotNum)).unwrap();
                        }
                    }
                    ActionStyle::Back => {
                        cliclack::clear_screen().unwrap();
                        cliclack::intro(t.raw(Prompt::HeaderEnter)).unwrap();
                        return;
                    }
                }
            }
            Err(_) => {
                cliclack::clear_screen().unwrap();
                let t_err = T::new(s.lang);
                cliclack::intro(t_err.raw(Prompt::HeaderEnter)).unwrap();
                return;
            }
        }
    }
}
