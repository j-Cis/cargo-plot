use eframe::egui;
use crate::interfaces::gui::CargoPlotApp;
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
            let show_mode = match (app.args.include, app.args.exclude) {
                (true, false) => ShowMode::Include,
                (false, true) => ShowMode::Exclude,
                _ => ShowMode::Context,
            };
            
            let stats = execute::execute(
                &app.args.enter_path,
                &app.args.patterns,
                !app.args.ignore_case,
                app.args.sort.into(),
                show_mode,
                app.args.view.into(),
                app.args.no_root,
                false, 
                true,
                &i18n,
                |_| {}, |_| {},
            );
            
            let tree_text = stats.render_output(app.args.view.into(), show_mode, false, false);
            let mut content = format!("```plaintext\n{}\n```\n\n", tree_text);
            
            let base_dir = std::path::Path::new(&app.args.enter_path);
            let mut counter = 1;

            // Ręcznie budujemy podgląd kodu dla GUI
            for p_str in &stats.m_matched.paths {
                if p_str.ends_with('/') { continue; }
                let absolute_path = base_dir.join(p_str);
                
                match std::fs::read_to_string(&absolute_path) {
                    Ok(txt) => {
                        content.push_str(&format!("### {:03}: `{}`\n\n```rust\n{}\n```\n\n", counter, p_str, txt));
                    }
                    Err(_) => {
                        content.push_str(&format!("### {:03}: `{}`\n\n> *(Pominięto binarkę)*\n\n", counter, p_str));
                    }
                }
                counter += 1;
            }
            app.generated_code = content;
        }

        ui.add_space(15.0);
        ui.checkbox(&mut app.args.by, if is_pl { "Dodaj stopkę (--by) przy zapisie" } else { "Add footer (--by) on save" });
        ui.add_space(15.0);

        if ui.button(if is_pl { "💾 Zapisz do pliku" } else { "💾 Save to file" }).clicked() {
            let tag = TimeTag::now();
            let filepath = format!("./cache_{}.md", tag);
            
            // Prosty zapis tego, co użytkownik ma w polu tekstowym
            let mut final_text = app.generated_code.clone();
            if app.args.by {
                final_text.push_str(&format!("\n\n---\n**Wersja raportu:** {}\n---", tag));
            }
            
            let _ = std::fs::write(&filepath, final_text);
        }
    });

    ui.separator();

    // 2. POLE TEKSTOWE / NOTATNIK
    egui::ScrollArea::both().show(ui, |ui| {
        ui.add(
            egui::TextEdit::multiline(&mut app.generated_code)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY)
                .desired_rows(30),
        );
    });
}