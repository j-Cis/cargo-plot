use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use crate::interfaces::gui::{CargoPlotApp, CodeTab};
use crate::interfaces::gui::shared::{resolve_dir, draw_tabs, draw_footer, draw_editor};
use cargo_plot::addon::TimeTag;
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::execute;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    let gt = GuiI18n::new(app.args.lang);

    let mut is_match = app.active_code_tab == CodeTab::Match;
    draw_tabs(ui, &gt, &mut is_match);
    app.active_code_tab = if is_match { CodeTab::Match } else { CodeTab::Mismatch };

    ui.separator();

    ui.horizontal(|ui| {
        if ui.button(gt.t(GT::BtnGenerateCode)).clicked() {
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            
            // ⚡ OPTYMALIZACJA: Generowanie skanowania i odczytów plików tylko dla żądanej sekcji.
            let show_mode = if is_match { ShowMode::Include } else { ShowMode::Exclude };

            let stats = execute::execute(
                &app.args.enter_path, &app.args.patterns, !app.args.ignore_case, app.args.sort.into(),
                show_mode, app.args.view.into(), app.args.no_root, false, true, &i18n, |_| {}, |_| {},
            );

            let base_dir = std::path::Path::new(&app.args.enter_path);

            if is_match {
                let tree_m = stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
                let mut content_m = format!("```plaintext\n{}\n```\n\n", tree_m);
                let mut counter_m = 1;
                for p_str in &stats.m_matched.paths {
                    if p_str.ends_with('/') { continue; }
                    let absolute_path = base_dir.join(p_str);
                    match std::fs::read_to_string(&absolute_path) {
                        Ok(txt) => content_m.push_str(&format!("### {:03}: `{}`\n\n```rust\n{}\n```\n\n", counter_m, p_str, txt)),
                        Err(_) => content_m.push_str(&format!("### {:03}: `{}`\n\n{}\n\n", counter_m, p_str, gt.t(GT::LabelSkipBinary))),
                    }
                    counter_m += 1;
                }
                app.generated_code_m = content_m;
            } else {
                let tree_x = stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
                let mut content_x = format!("```plaintext\n{}\n```\n\n", tree_x);
                let mut counter_x = 1;
                for p_str in &stats.x_mismatched.paths {
                    if p_str.ends_with('/') { continue; }
                    let absolute_path = base_dir.join(p_str);
                    match std::fs::read_to_string(&absolute_path) {
                        Ok(txt) => content_x.push_str(&format!("### {:03}: `{}`\n\n```rust\n{}\n```\n\n", counter_x, p_str, txt)),
                        Err(_) => content_x.push_str(&format!("### {:03}: `{}`\n\n{}\n\n", counter_x, p_str, gt.t(GT::LabelSkipBinary))),
                    }
                    counter_x += 1;
                }
                app.generated_code_x = content_x;
            }
        }

        ui.add_space(15.0);
        ui.checkbox(&mut app.args.by, gt.t(GT::LabelAddFooter));
        ui.add_space(15.0);

        if is_match {
            if ui.button(gt.t(GT::BtnSaveMatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!("{}plot-archive_{}_M.md", resolve_dir(&app.args.dir_out, &app.args.enter_path), tag);
                let mut final_text = app.generated_code_m.clone();
                if app.args.by { 
                    let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                    let cmd_string = app.args.to_command_string(true, false, false, true);
                    final_text.push_str(&cargo_plot::core::save::SaveFile::generate_by_section(&tag, "codes", &i18n, &cmd_string)); 
                }
                let _ = std::fs::write(&filepath, final_text);
            }
        } else {
            if ui.button(gt.t(GT::BtnSaveMismatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!("{}plot-archive_{}_X.md", resolve_dir(&app.args.dir_out, &app.args.enter_path), tag);
                let mut final_text = app.generated_code_x.clone();
                if app.args.by { 
                    let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                    let cmd_string = app.args.to_command_string(false, true, false, true);
                    final_text.push_str(&cargo_plot::core::save::SaveFile::generate_by_section(&tag, "codes", &i18n, &cmd_string)); 
                }
                let _ = std::fs::write(&filepath, final_text);
            }
        }
    });

    ui.separator();

    draw_footer(ui, "code_stats_footer");

    let text_buffer = match app.active_code_tab {
        CodeTab::Match => &mut app.generated_code_m,
        CodeTab::Mismatch => &mut app.generated_code_x,
    };
    draw_editor(ui, text_buffer);
}