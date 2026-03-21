use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use crate::interfaces::gui::shared::{draw_editor, draw_footer, draw_tabs, resolve_dir};
use crate::interfaces::gui::{CargoPlotApp, PathsTab, TreeStats};
use cargo_plot::addon::TimeTag;
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::core::file_stats::weight::{WeightConfig, UnitSystem};
use cargo_plot::core::save::is_blacklisted_extension;
use cargo_plot::core::file_stats::FileStats;
use cargo_plot::execute;
use eframe::egui;

/// [ENG]: View function for the Paths tab, managing structure generation and unit toggling.
/// [POL]: Funkcja widoku dla karty Ścieżki, zarządzająca generowaniem struktury i przełączaniem jednostek.
pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    let gt = GuiI18n::new(app.args.lang);

    // [ENG]: 1. TOP TABS - Sub-navigation for Match/Mismatch results.
    // [POL]: 1. GÓRNE ZAKŁADKI - Podnawigacja dla wyników Match/Mismatch.
    let mut is_match = app.active_paths_tab == PathsTab::Match;
    draw_tabs(ui, &gt, &mut is_match);
    app.active_paths_tab = if is_match { PathsTab::Match } else { PathsTab::Mismatch };

    ui.separator();

    // [ENG]: 2. ACTION BAR - Controls for generation, unit systems, and file saving.
    // [POL]: 2. PASEK AKCJI - Sterowanie generowaniem, systemami jednostek i zapisem plików.
    ui.horizontal(|ui| {
        // [ENG]: Logic for triggering data generation.
        // [POL]: Logika wyzwalająca generowanie danych.
        if ui.button(gt.t(GT::BtnGenerate)).clicked() {
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            let show_mode = if is_match { ShowMode::Include } else { ShowMode::Exclude };

            // [ENG]: Construct WeightConfig based on current application settings (-u and -a flags).
            // [POL]: Konstrukcja WeightConfig na podstawie bieżących ustawień aplikacji (flagi -u oraz -a).
            let weight_cfg = WeightConfig {
                system: if app.args.unit == crate::interfaces::cli::args::CliUnitSystem::Bin { UnitSystem::Binary } else { UnitSystem::Decimal },
                dir_sum_included: !app.args.all,
                ..WeightConfig::default()
            };

            let mut st_m = TreeStats::default();
            let mut st_x = TreeStats::default();

            // [ENG]: Execute scan with statistics collectors via closures.
            // [POL]: Wykonanie skanowania z kolektorami statystyk przez domknięcia.
            let stats = execute::execute(
                &app.args.enter_path,
                &app.args.patterns,
                !app.args.ignore_case,
                app.args.sort.into(),
                show_mode,
                app.args.view.into(),
                weight_cfg,
                app.args.no_root,
                false,
                app.args.no_emoji,
                &i18n,
                |f: &FileStats| {
                    if f.weight_bytes == 0 { st_m.empty_count += 1; }
                    if !f.path.ends_with('/') {
                        let ext = f.absolute.extension().unwrap_or_default().to_string_lossy().to_lowercase();
                        if is_blacklisted_extension(&ext) { st_m.bin_count += 1; st_m.bin_weight += f.weight_bytes; }
                        else { st_m.txt_count += 1; st_m.txt_weight += f.weight_bytes; }
                    }
                },
                |f: &FileStats| {
                    if f.weight_bytes == 0 { st_x.empty_count += 1; }
                    if !f.path.ends_with('/') {
                        let ext = f.absolute.extension().unwrap_or_default().to_string_lossy().to_lowercase();
                        if is_blacklisted_extension(&ext) { st_x.bin_count += 1; st_x.bin_weight += f.weight_bytes; }
                        else { st_x.txt_count += 1; st_x.txt_weight += f.weight_bytes; }
                    }
                },
            );

            // [ENG]: Update application state with results and calculated statistics.
            // [POL]: Aktualizacja stanu aplikacji o wyniki i obliczone statystyki.
            st_m.matched_count = stats.m_size_matched;
            st_m.total_count = stats.total;
            st_x.matched_count = stats.x_size_mismatched;
            st_x.total_count = stats.total;

            app.stats_m = st_m;
            app.stats_x = st_x;

            if is_match {
                app.generated_paths_m = stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
            } else {
                app.generated_paths_x = stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
            }
        }

        ui.add_space(10.0);
        ui.checkbox(&mut app.args.by, gt.t(GT::LabelAddFooter));
        
        ui.add_space(15.0);

        // [ENG]: Live unit system toggle. Label is pre-calculated to avoid borrow-checker conflicts.
        // [POL]: Przełącznik systemu jednostek na żywo. Etykieta obliczona wcześniej, by uniknąć konfliktów borrow-checkera.
        let mut is_bin = app.args.unit == crate::interfaces::cli::args::CliUnitSystem::Bin;
        let unit_label = if is_bin { "IEC (Bin)" } else { "SI (Dec)" };
        
        if ui.checkbox(&mut is_bin, unit_label).on_hover_text("B/KB vs B/KiB").changed() {
            app.args.unit = if is_bin { crate::interfaces::cli::args::CliUnitSystem::Bin } else { crate::interfaces::cli::args::CliUnitSystem::Dec };
        }

        ui.add_space(15.0);

        // [ENG]: Handle contextual save actions.
        // [POL]: Obsługa kontekstowych akcji zapisu.
        if is_match {
            if ui.button(gt.t(GT::BtnSaveMatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!("{}plot-address_{}_M.md", resolve_dir(&app.args.dir_out, &app.args.enter_path), tag);
                let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                let cmd_string = app.args.to_command_string(true, false, true, false);
                cargo_plot::core::save::SaveFile::paths(&app.generated_paths_m, &filepath, &tag, app.args.by, &i18n, &cmd_string, &app.args.enter_path);
            }
        } else {
            if ui.button(gt.t(GT::BtnSaveMismatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!("{}plot-address_{}_X.md", resolve_dir(&app.args.dir_out, &app.args.enter_path), tag);
                let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                let cmd_string = app.args.to_command_string(false, true, true, false);
                cargo_plot::core::save::SaveFile::paths(&app.generated_paths_x, &filepath, &tag, app.args.by, &i18n, &cmd_string, &app.args.enter_path);
            }
        }
    });

    ui.separator();

    // [ENG]: 3. FOOTER - Statistics display.
    // [POL]: 3. STOPKA - Wyświetlanie statystyk.
    let current_stats = if is_match { &app.stats_m } else { &app.stats_x };
    draw_footer(ui, "paths_stats_footer", current_stats); 

    // [ENG]: 4. MAIN EDITOR - Generated content area.
    // [POL]: 4. GŁÓWNY EDYTOR - Obszar wygenerowanej treści.
    let text_buffer = match app.active_paths_tab {
        PathsTab::Match => &mut app.generated_paths_m,
        PathsTab::Mismatch => &mut app.generated_paths_x,
    };
    draw_editor(ui, text_buffer);
}