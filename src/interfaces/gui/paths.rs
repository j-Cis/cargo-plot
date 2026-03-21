use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use crate::interfaces::gui::{CargoPlotApp, PathsTab};
use crate::interfaces::gui::shared::{resolve_dir, draw_tabs, draw_footer, draw_editor};
use cargo_plot::addon::TimeTag;
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::execute;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    let gt = GuiI18n::new(app.args.lang);

    // [ENG]: 1. TOP TABS - Shared 50/50 layout.
    // [POL]: 1. GÓRNE ZAKŁADKI - Współdzielony układ 50/50.
    let mut is_match = app.active_paths_tab == PathsTab::Match;
    draw_tabs(ui, &gt, &mut is_match);
    app.active_paths_tab = if is_match { PathsTab::Match } else { PathsTab::Mismatch };

    ui.separator();

    // [ENG]: 2. ACTION BAR - Dynamic save and isolated generation.
    // [POL]: 2. PASEK AKCJI - Dynamiczny zapis i izolowane generowanie.
    ui.horizontal(|ui| {
        if ui.button(gt.t(GT::BtnGenerate)).clicked() {
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            
            // ⚡ OPTYMALIZACJA: Generujemy tylko to, czego w danej chwili potrzebujesz!
            let show_mode = if is_match { ShowMode::Include } else { ShowMode::Exclude };

            let stats = execute::execute(
                &app.args.enter_path, &app.args.patterns, !app.args.ignore_case, app.args.sort.into(),
                show_mode, // ⚡ Oszczędzamy procesor, używając precyzyjnego trybu
                app.args.view.into(), app.args.no_root, false, true, &i18n, |_| {}, |_| {},
            );

            if is_match {
                app.generated_paths_m = stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
            } else {
                app.generated_paths_x = stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
            }
        }

        ui.add_space(15.0);
        ui.checkbox(&mut app.args.by, gt.t(GT::LabelAddFooter));
        ui.add_space(15.0);

        // ⚡ Wyświetla tylko przycisk zapisu odpowiadający otwartej zakładce
        if is_match {
            if ui.button(gt.t(GT::BtnSaveMatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!("{}plot-address_{}_M.md", resolve_dir(&app.args.dir_out, &app.args.enter_path), tag);
                let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                let cmd_string = app.args.to_command_string(true, false, true, false); 
                cargo_plot::core::save::SaveFile::paths(&app.generated_paths_m, &filepath, &tag, app.args.by, &i18n, &cmd_string);
            }
        } else {
            if ui.button(gt.t(GT::BtnSaveMismatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!("{}plot-address_{}_X.md", resolve_dir(&app.args.dir_out, &app.args.enter_path), tag);
                let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                let cmd_string = app.args.to_command_string(false, true, true, false); 
                cargo_plot::core::save::SaveFile::paths(&app.generated_paths_x, &filepath, &tag, app.args.by, &i18n, &cmd_string);
            }
        }
    });

    ui.separator();

    // [ENG]: 3. FOOTER - Shared statistics block.
    // [POL]: 3. STOPKA - Współdzielony blok statystyk.
    draw_footer(ui, "paths_stats_footer");

    // [ENG]: 4. MAIN EDITOR - Shared notepad UI.
    // [POL]: 4. GŁÓWNY EDYTOR - Współdzielony interfejs notatnika.
    let text_buffer = match app.active_paths_tab {
        PathsTab::Match => &mut app.generated_paths_m,
        PathsTab::Mismatch => &mut app.generated_paths_x,
    };
    draw_editor(ui, text_buffer);
}