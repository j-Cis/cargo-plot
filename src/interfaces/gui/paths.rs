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
            
            // Skanujemy dysk tylko raz, wyciągając pełny kontekst
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
            
            // ⚡ Od razu zapisujemy oba wyniki do niezależnych buforów
            app.generated_paths_m = stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
            app.generated_paths_x = stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
        }

        ui.add_space(15.0);
        ui.checkbox(&mut app.args.by, if is_pl { "Dodaj stopkę (--by) przy zapisie" } else { "Add footer (--by) on save" });
        ui.add_space(15.0);

        if ui.button(if is_pl { "💾 Zapisz do pliku" } else { "💾 Save to file" }).clicked() {
            let tag = TimeTag::now();
            let filepath = format!("./paths_{}.md", tag);
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            
            // ⚡ Zapisujemy tylko ten bufor, na który użytkownik aktualnie patrzy!
            let current_text = if app.active_paths_tab == PathsTab::Match {
                &app.generated_paths_m
            } else {
                &app.generated_paths_x
            };

            cargo_plot::core::save::SaveFile::paths(current_text, &filepath, &tag, app.args.by, &i18n);
        }
    });

    ui.separator();

    // 2. DOLNA BELKA (ZAKŁADKI) - Przypinamy ją do dołu ekranu
    egui::TopBottomPanel::bottom("paths_subtabs").show_inside(ui, |ui| {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            
            // ⚡ ZMIENIAMY TŁO TYLKO DLA LEWEGO GUZIKA (MATCH)
            if app.active_paths_tab == PathsTab::Match {
                ui.visuals_mut().selection.bg_fill = egui::Color32::from_rgb(255, 215, 0); // Złoty
            }
            // Jeśli aktywny jest Mismatch, egui użyje swojego domyślnego, nienaruszonego tła!
            
            let m_text = egui::RichText::new("✔ (-m) MATCH")
                .size(18.0).strong().color(egui::Color32::from_rgb(138, 90, 255)); // Fiolet
            
            let x_text = egui::RichText::new("✖ (-x) MISMATCH")
                .size(18.0).strong().color(egui::Color32::from_rgb(255, 80, 100)); // Czerwień

            ui.selectable_value(&mut app.active_paths_tab, PathsTab::Match, m_text);
            ui.add_space(20.0);
            ui.selectable_value(&mut app.active_paths_tab, PathsTab::Mismatch, x_text);
        });
        ui.add_space(8.0);
    });

    // 3. ŚRODKOWA PRZESTRZEŃ (NOTATNIK) - Wypełnia całą resztę miejsca między górą a dołem
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            // ⚡ Wyłączamy łamanie wierszy - włącza się poziomy scroll!
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            
            // Wybieramy odpowiedni bufor tekstowy do edycji i podglądu
            let text_buffer = match app.active_paths_tab {
                PathsTab::Match => &mut app.generated_paths_m,
                PathsTab::Mismatch => &mut app.generated_paths_x,
            };

            ui.add(
                egui::TextEdit::multiline(text_buffer)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY) // Rozciąga na max szerokość
                    // desired_rows() usunięte - teraz bierze całą wolną wysokość okna!
            );
        });
    });
}