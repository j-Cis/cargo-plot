use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use crate::interfaces::gui::{CargoPlotApp, CodeTab};
use cargo_plot::addon::TimeTag;
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::execute;
use eframe::egui;

// [ENG]: View function for the Code tab, managing source code extraction and preview.
// [POL]: Funkcja widoku dla karty Kod, zarządzająca ekstrakcją i podglądem kodu źródłowego.
pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    // [ENG]: Initialize translation engine.
    // [POL]: Inicjalizacja silnika tłumaczeń.
    let gt = GuiI18n::new(app.args.lang);

    // [ENG]: 1. TOP BAR - Code generation and archival save controls.
    // [POL]: 1. GÓRNA BELKA - Kontrolki generowania kodu i zapisu archiwalnego.
    ui.horizontal(|ui| {
        if ui.button(gt.t(GT::BtnGenerateCode)).clicked() {
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);

            // [ENG]: Execute scan in Context mode to populate both match and mismatch buffers.
            // [POL]: Wykonaj skanowanie w trybie Context, aby wypełnić bufory dopasowań i odrzuceń.
            let stats = execute::execute(
                &app.args.enter_path,
                &app.args.patterns,
                !app.args.ignore_case,
                app.args.sort.into(),
                ShowMode::Context,
                app.args.view.into(),
                app.args.no_root,
                false,
                true,
                &i18n,
                |_| {},
                |_| {},
            );

            let base_dir = std::path::Path::new(&app.args.enter_path);

            // --- [ENG]: BUILD MATCH BUFFER (-m) ---
            // --- [POL]: BUDOWA BUFORA DOPASOWAŃ (-m) ---
            let tree_m = stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
            let mut content_m = format!("```plaintext\n{}\n```\n\n", tree_m);
            let mut counter_m = 1;
            for p_str in &stats.m_matched.paths {
                if p_str.ends_with('/') {
                    continue;
                }
                let absolute_path = base_dir.join(p_str);
                match std::fs::read_to_string(&absolute_path) {
                    Ok(txt) => content_m.push_str(&format!(
                        "### {:03}: `{}`\n\n```rust\n{}\n```\n\n",
                        counter_m, p_str, txt
                    )),
                    Err(_) => content_m.push_str(&format!(
                        "### {:03}: `{}`\n\n{}\n\n",
                        counter_m,
                        p_str,
                        gt.t(GT::LabelSkipBinary)
                    )),
                }
                counter_m += 1;
            }
            app.generated_code_m = content_m;

            // --- [ENG]: BUILD MISMATCH BUFFER (-x) ---
            // --- [POL]: BUDOWA BUFORA ODRZUCEŃ (-x) ---
            let tree_x = stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
            let mut content_x = format!("```plaintext\n{}\n```\n\n", tree_x);
            let mut counter_x = 1;
            for p_str in &stats.x_mismatched.paths {
                if p_str.ends_with('/') {
                    continue;
                }
                let absolute_path = base_dir.join(p_str);
                match std::fs::read_to_string(&absolute_path) {
                    Ok(txt) => content_x.push_str(&format!(
                        "### {:03}: `{}`\n\n```rust\n{}\n```\n\n",
                        counter_x, p_str, txt
                    )),
                    Err(_) => content_x.push_str(&format!(
                        "### {:03}: `{}`\n\n{}\n\n",
                        counter_x,
                        p_str,
                        gt.t(GT::LabelSkipBinary)
                    )),
                }
                counter_x += 1;
            }
            app.generated_code_x = content_x;
        }

        ui.add_space(15.0);
        ui.checkbox(&mut app.args.by, gt.t(GT::LabelAddFooter));
        ui.add_space(15.0);

        // [ENG]: Helper to resolve output directory from app arguments.
        // [POL]: Pomocnik do wyznaczania folderu zapisu z argumentów aplikacji.
        let resolve_dir = |val: &Option<String>| -> String {
            match val {
                Some(v) if v == "AUTO" => "./other/".to_string(),
                Some(v) => {
                    let mut p = v.replace('\\', "/");
                    if !p.ends_with('/') {
                        p.push('/');
                    }
                    p
                }
                None => "./".to_string(),
            }
        };

        // [ENG]: Save archival code for MATCH results (-m).
        // [POL]: Zapis archiwalnego kodu dla wyników MATCH (-m).
        if ui.button(gt.t(GT::BtnSaveMatch)).clicked() {
            let tag = TimeTag::now();
            let filepath = format!(
                "{}plot-archive_{}_M.md",
                resolve_dir(&app.args.dir_out),
                tag
            );
            let mut final_text = app.generated_code_m.clone();

            if app.args.by {
                let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                let cmd_string = app.args.to_command_string();
                final_text.push_str(&cargo_plot::core::save::SaveFile::generate_by_section(
                    &tag,
                    "codes",
                    &i18n,
                    &cmd_string,
                ));
            }
            let _ = std::fs::write(&filepath, final_text);
        }

        ui.add_space(5.0);

        // [ENG]: Save archival code for MISMATCH results (-x).
        // [POL]: Zapis archiwalnego kodu dla wyników MISMATCH (-x).
        if ui.button(gt.t(GT::BtnSaveMismatch)).clicked() {
            let tag = TimeTag::now();
            let filepath = format!(
                "{}plot-archive_{}_X.md",
                resolve_dir(&app.args.dir_out),
                tag
            );
            let mut final_text = app.generated_code_x.clone();

            if app.args.by {
                let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                let cmd_string = app.args.to_command_string();
                final_text.push_str(&cargo_plot::core::save::SaveFile::generate_by_section(
                    &tag,
                    "codes",
                    &i18n,
                    &cmd_string,
                ));
            }
            let _ = std::fs::write(&filepath, final_text);
        }
    });

    ui.separator();

    // [ENG]: 2. BOTTOM BAR - Sub-tabs for switching between Match and Mismatch code views.
    // [POL]: 2. DOLNA BELKA - Zakładki do przełączania między widokiem kodu Match i Mismatch.
    egui::TopBottomPanel::bottom("code_subtabs").show_inside(ui, |ui| {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if app.active_code_tab == CodeTab::Match {
                ui.visuals_mut().selection.bg_fill = egui::Color32::from_rgb(255, 215, 0);
            }

            let m_text = egui::RichText::new(gt.t(GT::TabMatch))
                .size(18.0)
                .strong()
                .color(egui::Color32::from_rgb(138, 90, 255));

            let x_text = egui::RichText::new(gt.t(GT::TabMismatch))
                .size(18.0)
                .strong()
                .color(egui::Color32::from_rgb(255, 80, 100));

            ui.selectable_value(&mut app.active_code_tab, CodeTab::Match, m_text);
            ui.add_space(20.0);
            ui.selectable_value(&mut app.active_code_tab, CodeTab::Mismatch, x_text);
        });
        ui.add_space(8.0);
    });

    // [ENG]: 3. MAIN CONTENT AREA - Scrollable editor showing extracted file contents.
    // [POL]: 3. GŁÓWNY OBSZAR TREŚCI - Przewijalny edytor pokazujący wyekstrahowaną zawartość plików.
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

            let text_buffer = match app.active_code_tab {
                CodeTab::Match => &mut app.generated_code_m,
                CodeTab::Mismatch => &mut app.generated_code_x,
            };

            ui.add(
                egui::TextEdit::multiline(text_buffer)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY),
            );
        });
    });
}
