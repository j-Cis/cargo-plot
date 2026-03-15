use super::super::i18n::{Prompt, T, Translatable, Txt};
use super::super::state::{Lang, StateTui};
use console::style;

#[derive(Default, Clone, Eq, PartialEq)]
enum ActionOutput {
    #[default]
    ExecuteSave,
    SaveFileF,
    SaveFileD,
    SaveFileC,
    AddStructToDoc,
    AddCmdToStruct,
    AddCmdToDoc,
    AutoSectionNum,
    TimestampInFile,
    TimestampInFilename,
    SelfPromo,
    OutputFolder,
    SectionPrefix,
    DocTitle,
    Back,
}

//impl Default for ActionOutput {
//    fn default() -> Self { Self::ExecuteSave }
//}

// =====================================================================
// WARSTWA JĘZYKOWO - STYLIZACYJNA
// =====================================================================

impl Translatable for ActionOutput {
    fn trans(&self) -> Txt {
        match self {
            ActionOutput::ExecuteSave => Txt {
                pol: "--- WYKONAJ ZAPIS (START) ---",
                eng: "--- EXECUTE SAVE (START) ---",
            },
            ActionOutput::SaveFileF => Txt {
                pol: "Zapisz plik *f* (struktura)",
                eng: "Save file *f* (structure)",
            },
            ActionOutput::SaveFileD => Txt {
                pol: "Zapisz plik *d* (dokumentacja)",
                eng: "Save file *d* (documentation)",
            },
            ActionOutput::SaveFileC => Txt {
                pol: "Zapisz plik *c* (komenda)",
                eng: "Save file *c* (command)",
            },
            ActionOutput::AddStructToDoc => Txt {
                pol: "Dodaj strukturę do dokumentacji",
                eng: "Add structure to doc",
            },
            ActionOutput::AddCmdToStruct => Txt {
                pol: "Dodaj komendę do struktury",
                eng: "Add command to structure",
            },
            ActionOutput::AddCmdToDoc => Txt {
                pol: "Dodaj komendę do dokumentacji",
                eng: "Add command to doc",
            },
            ActionOutput::AutoSectionNum => Txt {
                pol: "Autonumeracja sekcji",
                eng: "Auto section numbering",
            },
            ActionOutput::TimestampInFile => Txt {
                pol: "Znacznik czasu w każdym pliku",
                eng: "Timestamp in every file",
            },
            ActionOutput::TimestampInFilename => Txt {
                pol: "Znacznik czasu jako prefix nazwy",
                eng: "Timestamp as filename prefix",
            },
            ActionOutput::SelfPromo => Txt {
                pol: "Autoreklama w każdym pliku",
                eng: "Self-promo in every file",
            },
            ActionOutput::OutputFolder => Txt {
                pol: "Folder na pliki",
                eng: "Output folder",
            },
            ActionOutput::SectionPrefix => Txt {
                pol: "Prefix sekcji pliku",
                eng: "Section prefix",
            },
            ActionOutput::DocTitle => Txt {
                pol: "Tytuł dokumentu",
                eng: "Document title",
            },
            ActionOutput::Back => Txt {
                pol: "Powrót",
                eng: "Back",
            },
        }
    }

    fn theme(&self, text: String) -> String {
        match self {
            // Wyróżniamy główny przycisk akcji!
            ActionOutput::ExecuteSave => style(text).on_green().black().bold().to_string(),
            _ => text,
        }
    }
}

// =====================================================================
// WIDOK MENU
// =====================================================================

pub fn menu_output_save(s: &mut StateTui) {
    let mut last_action = ActionOutput::default();

    loop {
        let t = T::new(s.lang);
        let oc = &s.output_config; // Skrót dla wygody czytania

        let header = style(t.raw(Prompt::HeaderOutput))
            .on_white()
            .black()
            .to_string();

        // Formaty przełączników [x] / [ ]
        let toggle = |b: bool| if b { "[x]" } else { "[ ]" };

        let f_lbl = format!(
            "{} {}",
            t.fmt(ActionOutput::SaveFileF),
            toggle(oc.save_file_f)
        );
        let d_lbl = format!(
            "{} {}",
            t.fmt(ActionOutput::SaveFileD),
            toggle(oc.save_file_d)
        );
        let c_lbl = format!(
            "{} {}",
            t.fmt(ActionOutput::SaveFileC),
            toggle(oc.save_file_c)
        );

        let s2d_lbl = format!(
            "{} {}",
            t.fmt(ActionOutput::AddStructToDoc),
            toggle(oc.add_struct_to_doc)
        );
        let c2s_lbl = format!(
            "{} {}",
            t.fmt(ActionOutput::AddCmdToStruct),
            toggle(oc.add_cmd_to_struct)
        );
        let c2d_lbl = format!(
            "{} {}",
            t.fmt(ActionOutput::AddCmdToDoc),
            toggle(oc.add_cmd_to_doc)
        );

        let auto_num_lbl = format!(
            "{} {}",
            t.fmt(ActionOutput::AutoSectionNum),
            toggle(oc.auto_section_num)
        );
        let ts_file_lbl = format!(
            "{} {}",
            t.fmt(ActionOutput::TimestampInFile),
            toggle(oc.timestamp_in_file)
        );
        let ts_name_lbl = format!(
            "{} {}",
            t.fmt(ActionOutput::TimestampInFilename),
            toggle(oc.timestamp_in_filename)
        );
        let promo_lbl = format!(
            "{} {}",
            t.fmt(ActionOutput::SelfPromo),
            toggle(oc.self_promo)
        );

        let folder_lbl = format!(
            "{} [{}]",
            t.fmt(ActionOutput::OutputFolder),
            oc.output_folder
        );
        let prefix_lbl = format!(
            "{} [{}]",
            t.fmt(ActionOutput::SectionPrefix),
            oc.section_prefix
        );
        let title_lbl = format!("{} [{}]", t.fmt(ActionOutput::DocTitle), oc.doc_title);

        // Tłumaczenia hintów (podpowiedzi)
        let hint_f = match s.lang {
            Lang::POL => "Zapisuje drzewo katalogów",
            Lang::ENG => "Saves directory tree",
        };
        let hint_d = match s.lang {
            Lang::POL => "Zapisuje zawartość plików",
            Lang::ENG => "Saves file contents",
        };
        let hint_c = match s.lang {
            Lang::POL => "Zapisuje komendę terminala",
            Lang::ENG => "Saves terminal command",
        };

        let action_result = cliclack::select(header)
            .initial_value(last_action.clone())
            .item(
                ActionOutput::ExecuteSave,
                t.fmt(ActionOutput::ExecuteSave),
                "",
            )
            .item(ActionOutput::SaveFileF, f_lbl, hint_f)
            .item(ActionOutput::SaveFileD, d_lbl, hint_d)
            .item(ActionOutput::SaveFileC, c_lbl, hint_c)
            .item(ActionOutput::AddStructToDoc, s2d_lbl, "")
            .item(ActionOutput::AddCmdToStruct, c2s_lbl, "")
            .item(ActionOutput::AddCmdToDoc, c2d_lbl, "")
            .item(ActionOutput::AutoSectionNum, auto_num_lbl, "")
            .item(ActionOutput::TimestampInFile, ts_file_lbl, "")
            .item(ActionOutput::TimestampInFilename, ts_name_lbl, "")
            .item(ActionOutput::SelfPromo, promo_lbl, "")
            .item(ActionOutput::OutputFolder, folder_lbl, "")
            .item(ActionOutput::SectionPrefix, prefix_lbl, "")
            .item(ActionOutput::DocTitle, title_lbl, "")
            .item(ActionOutput::Back, t.fmt(ActionOutput::Back), "")
            .interact();

        match action_result {
            Ok(action) => {
                last_action = action.clone();

                match action {
                    // Natychmiastowe przełączniki (negacja obecnej wartości)
                    ActionOutput::SaveFileF => {
                        s.output_config.save_file_f = !s.output_config.save_file_f
                    }
                    ActionOutput::SaveFileD => {
                        s.output_config.save_file_d = !s.output_config.save_file_d
                    }
                    ActionOutput::SaveFileC => {
                        s.output_config.save_file_c = !s.output_config.save_file_c
                    }
                    ActionOutput::AddStructToDoc => {
                        s.output_config.add_struct_to_doc = !s.output_config.add_struct_to_doc
                    }
                    ActionOutput::AddCmdToStruct => {
                        s.output_config.add_cmd_to_struct = !s.output_config.add_cmd_to_struct
                    }
                    ActionOutput::AddCmdToDoc => {
                        s.output_config.add_cmd_to_doc = !s.output_config.add_cmd_to_doc
                    }
                    ActionOutput::AutoSectionNum => {
                        s.output_config.auto_section_num = !s.output_config.auto_section_num
                    }
                    ActionOutput::TimestampInFile => {
                        s.output_config.timestamp_in_file = !s.output_config.timestamp_in_file
                    }
                    ActionOutput::TimestampInFilename => {
                        s.output_config.timestamp_in_filename =
                            !s.output_config.timestamp_in_filename
                    }
                    ActionOutput::SelfPromo => {
                        s.output_config.self_promo = !s.output_config.self_promo
                    }

                    // Pola tekstowe
                    ActionOutput::OutputFolder => {
                        let val: String = cliclack::input(t.raw(ActionOutput::OutputFolder))
                            .default_input(&s.output_config.output_folder)
                            .interact()
                            .unwrap_or(s.output_config.output_folder.clone());
                        s.output_config.output_folder = val;
                    }
                    ActionOutput::SectionPrefix => {
                        let val: String = cliclack::input(t.raw(ActionOutput::SectionPrefix))
                            .default_input(&s.output_config.section_prefix)
                            .interact()
                            .unwrap_or(s.output_config.section_prefix.clone());
                        s.output_config.section_prefix = val;
                    }
                    ActionOutput::DocTitle => {
                        let val: String = cliclack::input(t.raw(ActionOutput::DocTitle))
                            .default_input(&s.output_config.doc_title)
                            .interact()
                            .unwrap_or(s.output_config.doc_title.clone());
                        s.output_config.doc_title = val;
                    }

                    ActionOutput::ExecuteSave => {
                        // TUTAJ W PRZYSZŁOŚCI WYWOŁAMY WŁAŚCIWY ZAPIS PLIKÓW
                        let msg = match s.lang {
                            Lang::POL => format!(
                                "Zapisano pliki do folderu: {}",
                                s.output_config.output_folder
                            ),
                            Lang::ENG => {
                                format!("Saved files to folder: {}", s.output_config.output_folder)
                            }
                        };
                        cliclack::log::success(msg).unwrap();
                        cliclack::clear_screen().unwrap();
                        cliclack::intro(t.raw(Prompt::HeaderEnter)).unwrap();
                        return;
                    }
                    ActionOutput::Back => {
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
