use crate::interfaces::cli::args::{CliSortStrategy, CliViewMode};
use crate::interfaces::gui::CargoPlotApp;
use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use cargo_plot::i18n::Lang;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    // [ENG]: Initialize the GUI translation engine.
    // [POL]: Inicjalizacja silnika tłumaczeń GUI.
    let gt = GuiI18n::new(app.args.lang);

    // [ENG]: 1. Pinned Footer - Attached to the bottom of the screen.
    // [POL]: 1. Przyklejona stopka - Przypięta do dołu ekranu.
    egui::TopBottomPanel::bottom("settings_footer_panel")
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📦 cargo-plot v0.2.0-beta").strong());
                ui.separator();
                ui.hyperlink_to("Crates.io", "https://crates.io/crates/cargo-plot");
                ui.separator();
                ui.hyperlink_to(
                    gt.t(GT::FooterDownload),
                    "https://github.com/j-Cis/cargo-plot/releases",
                );
            });

            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(gt.t(GT::FooterInstall)).weak());
                ui.code("cargo install cargo-plot");
                ui.separator();
                ui.label(egui::RichText::new(gt.t(GT::FooterUninstall)).weak());
                ui.code("cargo uninstall cargo-plot");
            });
            ui.add_space(10.0);
        });

    // [ENG]: 2. Main Content Area - Scrollable settings.
    // [POL]: 2. Główny obszar treści - Przewijalne ustawienia.
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // ⚡ Ustawiamy globalny limit szerokości (bez ucinania krawędzi)
            ui.set_max_width(600.0);
            ui.add_space(10.0);

            // [ENG]: Language selection.
            // [POL]: Wybór języka.
            ui.horizontal(|ui| {
                ui.label(gt.t(GT::LabelLang));
                ui.radio_value(&mut app.args.lang, Some(Lang::Pl), "Polski");
                ui.radio_value(&mut app.args.lang, Some(Lang::En), "English");
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // [ENG]: Path Selection Grid - Perfectly aligns labels and inputs to the right edge.
            // [POL]: Siatka wyboru ścieżek - Idealnie wyrównuje etykiety i pola do prawej krawędzi.
            egui::Grid::new("path_settings_grid")
                .num_columns(2)
                .spacing([10.0, 10.0])
                .min_col_width(120.0)
                .show(ui, |ui| {
                    // Row 1: Scan path
                    ui.label(gt.t(GT::LabelScanPath));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(gt.t(GT::BtnBrowse)).clicked()
                            && let Some(folder) = rfd::FileDialog::new().pick_folder()
                        {
                            app.args.enter_path = folder.to_string_lossy().replace('\\', "/");
                        }
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut app.args.enter_path),
                        );
                    });
                    ui.end_row();

                    // Row 2: Output folder
                    ui.label(gt.t(GT::LabelOutFolder));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(gt.t(GT::BtnBrowse)).clicked()
                            && let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                let mut path = folder.to_string_lossy().replace('\\', "/");
                                if !path.ends_with('/') {
                                    path.push('/');
                                }
                                app.out_path_input = path.clone();
                                app.args.dir_out = Some(path);
                            }

                        let txt_response = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut app.out_path_input),
                        );
                        if txt_response.changed() {
                            let trimmed = app.out_path_input.trim();
                            app.args.dir_out = if trimmed.is_empty() {
                                None
                            } else {
                                Some(trimmed.to_string())
                            };
                        }
                    });
                    ui.end_row();
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // [ENG]: View and Sorting.
            // [POL]: Widok i sortowanie.
            ui.horizontal(|ui| {
                egui::ComboBox::from_label(gt.t(GT::LabelSorting))
                    .selected_text(format!("{:?}", app.args.sort))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.args.sort,
                            CliSortStrategy::AzFileMerge,
                            "AzFileMerge",
                        );
                        ui.selectable_value(
                            &mut app.args.sort,
                            CliSortStrategy::ZaFileMerge,
                            "ZaFileMerge",
                        );
                        ui.selectable_value(
                            &mut app.args.sort,
                            CliSortStrategy::AzDirMerge,
                            "AzDirMerge",
                        );
                        ui.selectable_value(
                            &mut app.args.sort,
                            CliSortStrategy::ZaDirMerge,
                            "ZaDirMerge",
                        );
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::AzFile, "AzFile");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::ZaFile, "ZaFile");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::AzDir, "AzDir");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::ZaDir, "ZaDir");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::Az, "Az");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::Za, "Za");
                        ui.selectable_value(&mut app.args.sort, CliSortStrategy::None, "None");
                    });

                ui.add_space(15.0);

                egui::ComboBox::from_label(gt.t(GT::LabelViewMode))
                    .selected_text(format!("{:?}", app.args.view))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.args.view, CliViewMode::Tree, "Tree");
                        ui.selectable_value(&mut app.args.view, CliViewMode::List, "List");
                        ui.selectable_value(&mut app.args.view, CliViewMode::Grid, "Grid");
                    });

                ui.add_space(15.0);
                ui.checkbox(&mut app.args.no_root, gt.t(GT::LabelNoRoot));
            });

            ui.add_space(20.0);

            // [ENG]: Match Patterns Section.
            // [POL]: Sekcja wzorców dopasowań.
            ui.heading(gt.t(GT::HeadingPatterns));
            ui.add_space(15.0);

            ui.horizontal(|ui| {
                ui.checkbox(&mut app.args.ignore_case, gt.t(GT::LabelIgnoreCase));
                ui.label(gt.t(GT::LabelNewPattern));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let btn_clicked = ui.button(gt.t(GT::BtnAddPattern)).clicked();
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut app.new_pattern_input),
                    );

                    if (btn_clicked
                        || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                        && !app.new_pattern_input.trim().is_empty()
                    {
                        let input = app.new_pattern_input.trim();

                        // ⚡ FAST-TRACK: Automatyczne parsowanie ciągów z CLI
                        if input.contains("-p ") || input.contains("--pat ") {
                            // Ujednolicamy znacznik flagi
                            let normalized = input.replace("--pat ", "-p ");

                            for part in normalized.split("-p ") {
                                let mut trimmed = part.trim();

                                // Ignorujemy śmieci takie jak komenda bazowa na początku
                                if trimmed.starts_with("cargo") || trimmed.is_empty() {
                                    continue;
                                }

                                // Zdejmujemy cudzysłowy i odcinamy ewentualne inne flagi na końcu ciągu
                                if (trimmed.starts_with('"') && trimmed.ends_with('"'))
                                    || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
                                {
                                    trimmed = &trimmed[1..trimmed.len() - 1]; // Idealne cudzysłowy po obu stronach
                                } else if trimmed.starts_with('"') || trimmed.starts_with('\'') {
                                    // Zaczyna się od cudzysłowu, ale ma śmieci po nim (np. inne flagi -i)
                                    let quote = trimmed.chars().next().unwrap();
                                    if let Some(end_idx) = trimmed[1..].find(quote) {
                                        trimmed = &trimmed[1..=end_idx];
                                    }
                                } else if let Some(space_idx) = trimmed.find(' ') {
                                    // Brak cudzysłowów, ucinamy do pierwszej spacji (inne flagi)
                                    trimmed = &trimmed[..space_idx];
                                }

                                if !trimmed.is_empty() {
                                    app.args.patterns.push(trimmed.to_string());
                                }
                            }
                        } else {
                            // Zwykłe dodanie pojedynczego wzorca wpisanego ręcznie
                            app.args.patterns.push(input.to_string());
                        }

                        app.new_pattern_input.clear();
                        response.request_focus();
                    }
                });
            });

            ui.add_space(5.0);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_height(200.0);
                // ⚡ Naprawa krawędzi: Wypełnia idealnie dostępną przestrzeń (z uwzględnieniem paddingu ramki)
                ui.set_min_width(ui.available_width());

                let mut move_up = None;
                let mut move_down = None;
                let mut remove = None;

                for (i, pat) in app.args.patterns.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.button("🗑").clicked() {
                            remove = Some(i);
                        }
                        if ui.button("⬆").clicked() {
                            move_up = Some(i);
                        }
                        if ui.button("⬇").clicked() {
                            move_down = Some(i);
                        }
                        ui.label(pat);
                    });
                }

                if let Some(i) = remove {
                    app.args.patterns.remove(i);
                }
                if let Some(i) = move_up
                    && i > 0
                {
                    app.args.patterns.swap(i, i - 1);
                }
                if let Some(i) = move_down
                    && i + 1 < app.args.patterns.len()
                {
                    app.args.patterns.swap(i, i + 1);
                }

                if !app.args.patterns.is_empty() {
                    ui.separator();
                    if ui.button(gt.t(GT::BtnClearAll)).clicked() {
                        app.args.patterns.clear();
                    }
                } else {
                    ui.label(
                        egui::RichText::new(gt.t(GT::MsgNoPatterns))
                            .italics()
                            .weak(),
                    );
                }
            });

            ui.add_space(20.0);
        });
    });
}
