use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use crate::interfaces::gui::{CargoPlotApp, PathsTab};
use cargo_plot::addon::TimeTag;
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::execute;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    // [ENG]: Initialize translation engine.
    // [POL]: Inicjalizacja silnika tłumaczeń.
    let gt = GuiI18n::new(app.args.lang);

    // [ENG]: 1. TOP BAR - Generation and Saving controls.
    // [POL]: 1. GÓRNA BELKA - Kontrolki generowania i zapisu.
    ui.horizontal(|ui| {
        if ui.button(gt.t(GT::BtnGenerate)).clicked() {
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);

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

            app.generated_paths_m =
                stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
            app.generated_paths_x =
                stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
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

        // [ENG]: Save MATCH results (-m).
        // [POL]: Zapis wyników MATCH (-m).
        if ui.button(gt.t(GT::BtnSaveMatch)).clicked() {
            let tag = TimeTag::now();
            let filepath = format!(
                "{}plot-address_{}_M.md",
                resolve_dir(&app.args.dir_out),
                tag
            );
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            let cmd_string = app.args.to_command_string();
            cargo_plot::core::save::SaveFile::paths(
                &app.generated_paths_m,
                &filepath,
                &tag,
                app.args.by,
                &i18n,
                &cmd_string,
            );
        }

        ui.add_space(5.0);

        // [ENG]: Save MISMATCH results (-x).
        // [POL]: Zapis wyników MISMATCH (-x).
        if ui.button(gt.t(GT::BtnSaveMismatch)).clicked() {
            let tag = TimeTag::now();
            let filepath = format!(
                "{}plot-address_{}_X.md",
                resolve_dir(&app.args.dir_out),
                tag
            );
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            let cmd_string = app.args.to_command_string();
            cargo_plot::core::save::SaveFile::paths(
                &app.generated_paths_x,
                &filepath,
                &tag,
                app.args.by,
                &i18n,
                &cmd_string,
            );
        }
    });

    ui.separator();

    // [ENG]: 2. BOTTOM BAR - Sub-tabs for switching between Match and Mismatch views.
    // [POL]: 2. DOLNA BELKA - Zakładki do przełączania między widokiem Match i Mismatch.
    egui::TopBottomPanel::bottom("paths_subtabs").show_inside(ui, |ui| {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if app.active_paths_tab == PathsTab::Match {
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

            ui.selectable_value(&mut app.active_paths_tab, PathsTab::Match, m_text);
            ui.add_space(20.0);
            ui.selectable_value(&mut app.active_paths_tab, PathsTab::Mismatch, x_text);
        });
        ui.add_space(8.0);
    });

    // [ENG]: 3. MAIN CONTENT AREA - Scrollable notepad showing generated path data.
    // [POL]: 3. GŁÓWNY OBSZAR TREŚCI - Przewijalny notatnik z wygenerowanymi danymi ścieżek.
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

            let text_buffer = match app.active_paths_tab {
                PathsTab::Match => &mut app.generated_paths_m,
                PathsTab::Mismatch => &mut app.generated_paths_x,
            };

            ui.add(
                egui::TextEdit::multiline(text_buffer)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY),
            );
        });
    });
}
