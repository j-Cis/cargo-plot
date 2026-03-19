use eframe::egui;
use crate::interfaces::gui::CargoPlotApp;
use cargo_plot::i18n::Lang;
use crate::interfaces::cli::args::{CliSortStrategy, CliViewMode};

pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    // Sprawdzamy, czy wybrany jest język polski (dla dynamicznych tłumaczeń w locie)
    let is_pl = app.args.lang.unwrap_or(Lang::En) == Lang::Pl;

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(10.0);

        // 1. WYBÓR JĘZYKA (Teraz dynamicznie aktualizuje resztę interfejsu!)
        ui.horizontal(|ui| {
            ui.label(if is_pl { "🌍 Język:" } else { "🌍 Language:" });
            ui.radio_value(&mut app.args.lang, Some(Lang::Pl), "Polski");
            ui.radio_value(&mut app.args.lang, Some(Lang::En), "English");
        });
        ui.separator();

        // 2. WYBÓR FOLDERU (Z działającym przyciskiem okna systemowego)
        ui.horizontal(|ui| {
            ui.label(if is_pl { "📂 Ścieżka skanowania:" } else { "📂 Scan path:" });
            ui.text_edit_singleline(&mut app.args.enter_path);
            
            // ⚡ NATYWNE OKNO WYBORU FOLDERU
            if ui.button(if is_pl { "Wybierz..." } else { "Browse..." }).clicked()
                && let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    // Aktualizujemy ścieżkę i ujednolicamy ukośniki
                    app.args.enter_path = folder.to_string_lossy().replace('\\', "/");
                }
        });
        ui.separator();

        // 3. WZORCE DOPASOWAŃ (Z połączonymi opcjami z linii "X")
        ui.heading(if is_pl { "🔍 Wzorce dopasowań (Patterns)" } else { "🔍 Match Patterns" });
        
        // Trzy opcje logiczne przytulone do wzorców
        //ui.horizontal(|ui| {
            
            //ui.checkbox(&mut app.args.include, if is_pl { "✅ Pokaż dopasowane (m)" } else { "✅ Show matched (m)" });
            //ui.checkbox(&mut app.args.exclude, if is_pl { "❌ Pokaż odrzucone (x)" } else { "❌ Show rejected (x)" });
        //});
        ui.add_space(5.0);

        // Pole dodawania nowego wzorca
        ui.horizontal(|ui| {
            ui.checkbox(&mut app.args.ignore_case, if is_pl { "🔠 Ignoruj wielkość liter" } else { "🔠 Ignore case" });
            ui.label(if is_pl { "Nowy:" } else { "New:" });
            let response = ui.text_edit_singleline(&mut app.new_pattern_input);
            let btn_clicked = ui.button(if is_pl { "➕ Dodaj wzorzec" } else { "➕ Add pattern" }).clicked();
            
            if (btn_clicked || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) 
                && !app.new_pattern_input.trim().is_empty() 
            {
                app.args.patterns.push(app.new_pattern_input.trim().to_string());
                app.new_pattern_input.clear();
                response.request_focus();
            }
        });

        // 4. LISTA DODANYCH WZORCÓW
        ui.add_space(5.0);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_min_height(100.0);

            let mut move_up = None;
            let mut move_down = None;
            let mut remove = None;

            for (i, pat) in app.args.patterns.iter().enumerate() {
                ui.horizontal(|ui| {
                    if ui.button("🗑").clicked() { remove = Some(i); }
                    if ui.button("⬆").clicked() { move_up = Some(i); }
                    if ui.button("⬇").clicked() { move_down = Some(i); }
                    ui.label(pat);
                });
            }

            if let Some(i) = remove { app.args.patterns.remove(i); }
            if let Some(i) = move_up && i > 0 { app.args.patterns.swap(i, i - 1); }
            if let Some(i) = move_down && i + 1 < app.args.patterns.len() { app.args.patterns.swap(i, i + 1); }

            if !app.args.patterns.is_empty() {
                ui.separator();
                if ui.button(if is_pl { "💣 Usuń wszystkie" } else { "💣 Clear all" }).clicked() {
                    app.args.patterns.clear();
                }
            } else {
                let empty_msg = if is_pl { "Brak wzorców. Dodaj przynajmniej jeden!" } else { "No patterns. Add at least one!" };
                ui.label(egui::RichText::new(empty_msg).italics().weak());
            }
        });
        ui.separator();

        // 5. WIDOK I SORTOWANIE
        ui.horizontal(|ui| {
            egui::ComboBox::from_label(if is_pl { "Sortowanie" } else { "Sorting" })
                .selected_text(format!("{:?}", app.args.sort))
                .show_ui(ui, |ui| {
                    // ⚡ PEŁNA LISTA SORTOWAŃ
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::AzFileMerge, "AzFileMerge");
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::ZaFileMerge, "ZaFileMerge");
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::AzDirMerge, "AzDirMerge");
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::ZaDirMerge, "ZaDirMerge");
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::AzFile, "AzFile");
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::ZaFile, "ZaFile");
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::AzDir, "AzDir");
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::ZaDir, "ZaDir");
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::Az, "Az");
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::Za, "Za");
                    ui.selectable_value(&mut app.args.sort, CliSortStrategy::None, "None");
                });

            ui.add_space(15.0);

            egui::ComboBox::from_label(if is_pl { "Tryb widoku" } else { "View mode" })
                .selected_text(format!("{:?}", app.args.view))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.args.view, CliViewMode::Tree, "Tree");
                    ui.selectable_value(&mut app.args.view, CliViewMode::List, "List");
                    ui.selectable_value(&mut app.args.view, CliViewMode::Grid, "Grid");
                });

            ui.add_space(15.0);
            
            ui.checkbox(&mut app.args.no_root, if is_pl { "Ukryj ROOT w drzewie" } else { "Hide ROOT in tree" });
        });

        // ⚡ NOWOŚĆ: ŚCIEŻKA WYNIKOWA (Output path)
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(if is_pl { "💾 Ścieżka zapisu (Output):" } else { "💾 Output path:" });
            if ui.text_edit_singleline(&mut app.out_path_input).changed() {
                app.args.out_path = if app.out_path_input.trim().is_empty() {
                    None
                } else {
                    Some(app.out_path_input.trim().to_string())
                };
            }
            if ui.button(if is_pl { "Wybierz folder..." } else { "Browse folder..." }).clicked() {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    let path = folder.to_string_lossy().replace('\\', "/");
                    app.out_path_input = path.clone();
                    app.args.out_path = Some(path);
                }
            }
        });

        ui.add_space(30.0);
        
        // 6. STOPKA
        ui.separator();
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("📦 cargo-plot v0.2.0-beta").strong());
            ui.separator();
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/cargo-plot");
            ui.separator();
            ui.hyperlink_to(if is_pl { "Pobierz binarkę (GitHub)" } else { "Download binary (GitHub)" }, "https://github.com/j-Cis/cargo-plot/releases");
        });
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(if is_pl { "Instalacja:" } else { "Install:" }).weak());
            ui.code("cargo install cargo-plot");
            ui.separator();
            ui.label(egui::RichText::new(if is_pl { "Usuwanie:" } else { "Uninstall:" }).weak());
            ui.code("cargo uninstall cargo-plot");
        });
        ui.add_space(10.0);
    });
}