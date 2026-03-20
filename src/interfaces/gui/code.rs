use eframe::egui;
use crate::interfaces::gui::{CargoPlotApp, CodeTab};
use cargo_plot::i18n::Lang;
use cargo_plot::execute;
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::addon::TimeTag;

pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    let is_pl = app.args.lang.unwrap_or(Lang::En) == Lang::Pl;

    // 1. BELKA GÓRNA
    ui.horizontal(|ui| {
        if ui.button(if is_pl { "🔄 Generuj kod (Cache)" } else { "🔄 Generate code" }).clicked() {
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            
            // Wymuszamy pełny skan (Context), żeby mieć oba wyniki od razu
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
            
            let base_dir = std::path::Path::new(&app.args.enter_path);

            // --- BUDOWA BUFORA MATCH (-m) ---
            let tree_m = stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
            let mut content_m = format!("```plaintext\n{}\n```\n\n", tree_m);
            let mut counter_m = 1;
            for p_str in &stats.m_matched.paths {
                if p_str.ends_with('/') { continue; }
                let absolute_path = base_dir.join(p_str);
                match std::fs::read_to_string(&absolute_path) {
                    Ok(txt) => content_m.push_str(&format!("### {:03}: `{}`\n\n```rust\n{}\n```\n\n", counter_m, p_str, txt)),
                    Err(_) => content_m.push_str(&format!("### {:03}: `{}`\n\n> *(Pominięto binarkę)*\n\n", counter_m, p_str)),
                }
                counter_m += 1;
            }
            app.generated_code_m = content_m;

            // --- BUDOWA BUFORA MISMATCH (-x) ---
            let tree_x = stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
            let mut content_x = format!("```plaintext\n{}\n```\n\n", tree_x);
            let mut counter_x = 1;
            for p_str in &stats.x_mismatched.paths {
                if p_str.ends_with('/') { continue; }
                let absolute_path = base_dir.join(p_str);
                match std::fs::read_to_string(&absolute_path) {
                    Ok(txt) => content_x.push_str(&format!("### {:03}: `{}`\n\n```rust\n{}\n```\n\n", counter_x, p_str, txt)),
                    Err(_) => content_x.push_str(&format!("### {:03}: `{}`\n\n> *(Pominięto binarkę)*\n\n", counter_x, p_str)),
                }
                counter_x += 1;
            }
            app.generated_code_x = content_x;
        }

        ui.add_space(15.0);
        ui.checkbox(&mut app.args.by, if is_pl { "Dodaj stopkę (--by)" } else { "Add footer (--by)" });
        ui.add_space(15.0);

        // ⚡ Helper rozwiązujący folder zapisu ze zmiennej out_code
        let resolve_dir = |val: &Option<String>| -> String {
            match val {
                Some(v) if v == "AUTO" => "./other/".to_string(),
                Some(v) => {
                    let mut p = v.replace('\\', "/");
                    if !p.ends_with('/') { p.push('/'); }
                    p
                }
                None => "./".to_string(),
            }
        };

        // ⚡ ZAPIS DLA -m
        if ui.button(if is_pl { "💾 Zapisz (-m)" } else { "💾 Save (-m)" }).clicked() {
            let tag = TimeTag::now();
            let filepath = format!("{}plot-archive_{}_M.md", resolve_dir(&app.args.out_code), tag);
            let mut final_text = app.generated_code_m.clone();
            if app.args.by { final_text.push_str(&format!("\n\n---\n**Wersja raportu:** {}\n---", tag)); }
            let _ = std::fs::write(&filepath, final_text);
        }

        ui.add_space(5.0);

        // ⚡ ZAPIS DLA -x
        if ui.button(if is_pl { "💾 Zapisz (-x)" } else { "💾 Save (-x)" }).clicked() {
            let tag = TimeTag::now();
            let filepath = format!("{}plot-archive_{}_X.md", resolve_dir(&app.args.out_code), tag);
            let mut final_text = app.generated_code_x.clone();
            if app.args.by { final_text.push_str(&format!("\n\n---\n**Wersja raportu:** {}\n---", tag)); }
            let _ = std::fs::write(&filepath, final_text);
        }
    });

    ui.separator();

    // 2. DOLNA BELKA (ZAKŁADKI)
    egui::TopBottomPanel::bottom("code_subtabs").show_inside(ui, |ui| {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if app.active_code_tab == CodeTab::Match {
                ui.visuals_mut().selection.bg_fill = egui::Color32::from_rgb(255, 215, 0); 
            }
            
            let m_text = egui::RichText::new("✔ (-m) MATCH").size(18.0).strong().color(egui::Color32::from_rgb(138, 90, 255));
            let x_text = egui::RichText::new("✖ (-x) MISMATCH").size(18.0).strong().color(egui::Color32::from_rgb(255, 80, 100)); 

            ui.selectable_value(&mut app.active_code_tab, CodeTab::Match, m_text);
            ui.add_space(20.0);
            ui.selectable_value(&mut app.active_code_tab, CodeTab::Mismatch, x_text);
        });
        ui.add_space(8.0);
    });

    // 3. POLE TEKSTOWE / NOTATNIK
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
                    .desired_width(f32::INFINITY)
            );
        });
    });
}