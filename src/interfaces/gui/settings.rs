use crate::interfaces::cli::args::{CliSortStrategy, CliViewMode};
use crate::interfaces::gui::CargoPlotApp;
use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use cargo_plot::i18n::Lang;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    // [ENG]: Initialize the GUI translation engine based on current settings.
    // [POL]: Inicjalizacja silnika tłumaczeń GUI na podstawie aktualnych ustawień.
    let gt = GuiI18n::new(app.args.lang);

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(10.0);

        // [ENG]: 1. LANGUAGE SELECTION - Dynamically updates the entire UI.
        // [POL]: 1. WYBÓR JĘZYKA - Dynamicznie aktualizuje cały interfejs.
        ui.horizontal(|ui| {
            ui.label(gt.t(GT::LabelLang));
            ui.radio_value(&mut app.args.lang, Some(Lang::Pl), "Polski");
            ui.radio_value(&mut app.args.lang, Some(Lang::En), "English");
        });
        ui.separator();
        ui.add_space(10.0);

        // [ENG]: 2. SCAN FOLDER SELECTION - Uses native system dialog.
        // [POL]: 2. WYBÓR FOLDERU SKANOWANIA - Używa natywnego okna systemowego.
        ui.horizontal(|ui| {
            ui.label(gt.t(GT::LabelScanPath));
            ui.text_edit_singleline(&mut app.args.enter_path);

            if ui.button(gt.t(GT::BtnBrowse)).clicked()
                && let Some(folder) = rfd::FileDialog::new().pick_folder()
            {
                app.args.enter_path = folder.to_string_lossy().replace('\\', "/");
            }
        });
        ui.add_space(10.0);
        ui.separator();

        // [ENG]: 3. OUTPUT FOLDER SELECTION - Common path for paths and archive saves.
        // [POL]: 3. WYBÓR FOLDERU WYNIKOWEGO - Wspólna ścieżka dla zapisu ścieżek i archiwum.
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(gt.t(GT::LabelOutFolder));

            if ui.text_edit_singleline(&mut app.out_path_input).changed() {
                let trimmed = app.out_path_input.trim();
                app.args.dir_out = if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                };
            }

            if ui.button(gt.t(GT::BtnBrowse)).clicked() {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    let mut path = folder.to_string_lossy().replace('\\', "/");
                    if !path.ends_with('/') {
                        path.push('/');
                    }

                    app.out_path_input = path.clone();
                    app.args.dir_out = Some(path);
                }
            }
        });
        ui.add_space(10.0);
        ui.separator();

        // [ENG]: 4. VIEW AND SORTING - Controls the structure of the generated report.
        // [POL]: 4. WIDOK I SORTOWANIE - Kontroluje strukturę generowanego raportu.
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

        // [ENG]: 5. MATCH PATTERNS - Pattern management with real-time list interaction.
        // [POL]: 5. WZORCE DOPASOWAŃ - Zarządzanie wzorcami z interaktywną listą.
        ui.heading(gt.t(GT::HeadingPatterns));
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.checkbox(&mut app.args.ignore_case, gt.t(GT::LabelIgnoreCase));
            ui.label(gt.t(GT::LabelNewPattern));
            let response = ui.text_edit_singleline(&mut app.new_pattern_input);
            let btn_clicked = ui.button(gt.t(GT::BtnAddPattern)).clicked();

            if (btn_clicked
                || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                && !app.new_pattern_input.trim().is_empty()
            {
                app.args
                    .patterns
                    .push(app.new_pattern_input.trim().to_string());
                app.new_pattern_input.clear();
                response.request_focus();
            }
        });

        ui.add_space(5.0);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_min_height(100.0);

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
        ui.separator();

        ui.add_space(50.0);

        // [ENG]: 6. FOOTER - Versioning, links and installation instructions.
        // [POL]: 6. STOPKA - Wersjonowanie, linki i instrukcje instalacji.
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
}
