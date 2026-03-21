use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use crate::interfaces::gui::shared::{draw_editor, draw_footer, draw_tabs, resolve_dir};
use crate::interfaces::gui::{CargoPlotApp, CodeTab, TreeStats};
use cargo_plot::addon::TimeTag;
use cargo_plot::core::file_stats::FileStats;
use cargo_plot::core::file_stats::weight::{UnitSystem, WeightConfig};
use cargo_plot::core::path_matcher::stats::ShowMode;
use cargo_plot::core::save::is_blacklisted_extension;
use cargo_plot::execute;
use eframe::egui;

/// [ENG]: View function for the Code tab, managing source extraction and statistics.
/// [POL]: Funkcja widoku dla karty Kod, zarządzająca ekstrakcją źródeł i statystykami.
pub fn show(ui: &mut egui::Ui, app: &mut CargoPlotApp) {
    let gt = GuiI18n::new(app.args.lang);

    // [ENG]: 1. TOP TABS - Navigation between matched and mismatched code buffers.
    // [POL]: 1. GÓRNE ZAKŁADKI - Nawigacja między buforami kodu dopasowanego i odrzuconego.
    let mut is_match = app.active_code_tab == CodeTab::Match;
    draw_tabs(ui, &gt, &mut is_match);
    app.active_code_tab = if is_match {
        CodeTab::Match
    } else {
        CodeTab::Mismatch
    };

    ui.separator();

    // [ENG]: 2. ACTION BAR - Controls for code generation and archival save.
    // [POL]: 2. PASEK AKCJI - Sterowanie generowaniem kodu i zapisem archiwalnym.
    ui.horizontal(|ui| {
        if ui.button(gt.t(GT::BtnGenerateCode)).clicked() {
            let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
            let show_mode = if is_match {
                ShowMode::Include
            } else {
                ShowMode::Exclude
            };

            // [ENG]: Weight configuration remains fixed for code extraction to ensure consistency.
            // [POL]: Konfiguracja wagi pozostaje stała dla ekstrakcji kodu, aby zapewnić spójność.
            let weight_cfg = WeightConfig {
                system: if app.args.unit == crate::interfaces::cli::args::CliUnitSystem::Bin {
                    UnitSystem::Binary
                } else {
                    UnitSystem::Decimal
                },
                dir_sum_included: !app.args.all,
                ..WeightConfig::default()
            };

            let mut st_m = TreeStats::default();
            let mut st_x = TreeStats::default();

            // [ENG]: Execute main engine with closures for statistics and file classification.
            // [POL]: Wykonanie głównego silnika z domknięciami dla statystyk i klasyfikacji plików.
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
                    if f.weight_bytes == 0 {
                        st_m.empty_count += 1;
                    }
                    if !f.path.ends_with('/') {
                        let ext = f
                            .absolute
                            .extension()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_lowercase();
                        if is_blacklisted_extension(&ext) {
                            st_m.bin_count += 1;
                            st_m.bin_weight += f.weight_bytes;
                        } else {
                            st_m.txt_count += 1;
                            st_m.txt_weight += f.weight_bytes;
                        }
                    }
                },
                |f: &FileStats| {
                    if f.weight_bytes == 0 {
                        st_x.empty_count += 1;
                    }
                    if !f.path.ends_with('/') {
                        let ext = f
                            .absolute
                            .extension()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_lowercase();
                        if is_blacklisted_extension(&ext) {
                            st_x.bin_count += 1;
                            st_x.bin_weight += f.weight_bytes;
                        } else {
                            st_x.txt_count += 1;
                            st_x.txt_weight += f.weight_bytes;
                        }
                    }
                },
            );

            st_m.matched_count = stats.m_size_matched;
            st_m.total_count = stats.total;
            st_x.matched_count = stats.x_size_mismatched;
            st_x.total_count = stats.total;

            app.stats_m = st_m;
            app.stats_x = st_x;

            let base_dir = std::path::Path::new(&app.args.enter_path);

            // [ENG]: Process code extraction for the selected result set.
            // [POL]: Przetwarzanie ekstrakcji kodu dla wybranego zestawu wyników.
            if is_match {
                let tree_m =
                    stats.render_output(app.args.view.into(), ShowMode::Include, false, false);
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
            } else {
                let tree_x =
                    stats.render_output(app.args.view.into(), ShowMode::Exclude, false, false);
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
        }

        ui.add_space(15.0);
        ui.checkbox(&mut app.args.by, gt.t(GT::LabelAddFooter));
        ui.add_space(15.0);

        // [ENG]: Archival saving with metadata table.
        // [POL]: Zapis archiwalny z tabelą metadanych.
        if is_match {
            if ui.button(gt.t(GT::BtnSaveMatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!(
                    "{}plot-archive_{}_M.md",
                    resolve_dir(&app.args.dir_out, &app.args.enter_path),
                    tag
                );
                let mut final_text = app.generated_code_m.clone();
                if app.args.by {
                    let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                    let cmd_string = app.args.to_command_string(true, false, false, true);
                    final_text.push_str(&cargo_plot::core::save::SaveFile::generate_by_section(
                        &tag,
                        &app.args.enter_path,
                        &i18n,
                        &cmd_string,
                    ));
                }
                let _ = std::fs::write(&filepath, final_text);
            }
        } else {
            if ui.button(gt.t(GT::BtnSaveMismatch)).clicked() {
                let tag = TimeTag::now();
                let filepath = format!(
                    "{}plot-archive_{}_X.md",
                    resolve_dir(&app.args.dir_out, &app.args.enter_path),
                    tag
                );
                let mut final_text = app.generated_code_x.clone();
                if app.args.by {
                    let i18n = cargo_plot::i18n::I18n::new(app.args.lang);
                    let cmd_string = app.args.to_command_string(false, true, false, true);
                    final_text.push_str(&cargo_plot::core::save::SaveFile::generate_by_section(
                        &tag,
                        &app.args.enter_path,
                        &i18n,
                        &cmd_string,
                    ));
                }
                let _ = std::fs::write(&filepath, final_text);
            }
        }
    });

    ui.separator();

    // [ENG]: 3. FOOTER - Update statistics pinned to the bottom.
    // [POL]: 3. STOPKA - Aktualizacja statystyk przypiętych do dołu.
    let current_stats = if is_match { &app.stats_m } else { &app.stats_x };
    draw_footer(ui, "code_stats_footer", current_stats);

    // [ENG]: 4. MAIN EDITOR - Display extracted file contents.
    // [POL]: 4. GŁÓWNY EDYTOR - Widok wyekstrahowanej zawartości plików.
    let text_buffer = match app.active_code_tab {
        CodeTab::Match => &mut app.generated_code_m,
        CodeTab::Mismatch => &mut app.generated_code_x,
    };
    draw_editor(ui, text_buffer);
}
