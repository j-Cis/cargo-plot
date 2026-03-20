use eframe::egui;
use crate::interfaces::gui::{CargoPlotApp, PathsTab};
use cargo_plot::i18n::Lang;
use cargo_plot::execute;
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::addon::TimeTag;

pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    let is_pl = app.args.lang.unwrap_or(Lang::En) == Lang::Pl;

    // 1. GÓRNA BELKA (Generowanie i Zapis)
    ui.horizontal(|ui| {
        if ui.button(if is_pl { "🔄 Generuj / Regeneruj" } else { "🔄 Generate" }).clicked() {
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
                |_| {}, |_| {},
            );
            
            app.generated_paths_m = stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
            app.generated_paths_x = stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
        }

        ui.add_space(15.0);
        ui.checkbox(&mut app.args.by, if is_pl { "Dodaj stopkę (--by)" } else { "Add footer (--by)" });
        ui.add_space(15.0);

        // ⚡ Helper rozwiązujący folder zapisu ze zmiennej out_path
        let resolve_dir = |val: &Option<String>| -> String {
            match val {
                Some(v) if v == "AUTO" => "./other/".to_string(), // Jeśli AUTO, wrzuć do ./other/
                Some(v) => {
                    let mut p = v.replace('\\', "/");
                    if !p.ends_with('/') { p.push('/'); }
                    p
                }
                None => "./".to_string(), // Domyślnie główny folder projektu
            }
        };

        // ⚡ ZAPIS DLA -m
        if ui.button(if is_pl { "💾 Zapisz (-m)" } else { "💾 Save (-m)" }).clicked() {
            let tag = TimeTag::now();
            let filepath = format!("{}plot-address_{}_M.md", resolve_dir(&app.args.out_path), tag);
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            cargo_plot::core::save::SaveFile::paths(&app.generated_paths_m, &filepath, &tag, app.args.by, &i18n);
        }

        ui.add_space(5.0);

        // ⚡ ZAPIS DLA -x
        if ui.button(if is_pl { "💾 Zapisz (-x)" } else { "💾 Save (-x)" }).clicked() {
            let tag = TimeTag::now();
            let filepath = format!("{}plot-address_{}_X.md", resolve_dir(&app.args.out_path), tag);
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            cargo_plot::core::save::SaveFile::paths(&app.generated_paths_x, &filepath, &tag, app.args.by, &i18n);
        }
    });

    ui.separator();

    // 2. DOLNA BELKA (ZAKŁADKI)
    egui::TopBottomPanel::bottom("paths_subtabs").show_inside(ui, |ui| {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            
            if app.active_paths_tab == PathsTab::Match {
                ui.visuals_mut().selection.bg_fill = egui::Color32::from_rgb(255, 215, 0); 
            }
            
            let m_text = egui::RichText::new("✔ (-m) MATCH")
                .size(18.0).strong().color(egui::Color32::from_rgb(138, 90, 255)); 
            
            let x_text = egui::RichText::new("✖ (-x) MISMATCH")
                .size(18.0).strong().color(egui::Color32::from_rgb(255, 80, 100)); 

            ui.selectable_value(&mut app.active_paths_tab, PathsTab::Match, m_text);
            ui.add_space(20.0);
            ui.selectable_value(&mut app.active_paths_tab, PathsTab::Mismatch, x_text);
        });
        ui.add_space(8.0);
    });

    // 3. NOTATNIK
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
                    .desired_width(f32::INFINITY)
            );
        });
    });
}