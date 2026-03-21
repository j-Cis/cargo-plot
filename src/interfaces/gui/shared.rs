use crate::interfaces::gui::i18n::{GuiI18n, GuiText as GT};
use eframe::egui;

// [ENG]: Helper to resolve output directory from app arguments.
// [POL]: Pomocnik do wyznaczania folderu zapisu z argumentów aplikacji.
pub fn resolve_dir(val: &Option<String>, base_path: &str) -> String {
    let is_auto = val
        .as_ref()
        .is_none_or(|v| v.trim().is_empty() || v == "AUTO");
    if is_auto {
        let mut b = base_path.replace('\\', "/");
        if !b.ends_with('/') {
            b.push('/');
        }
        format!("{}.cargo-plot/", b)
    } else {
        let mut p = val.as_ref().unwrap().replace('\\', "/");
        if !p.ends_with('/') {
            p.push('/');
        }
        p
    }
}

// [ENG]: UI component: 50/50 Match & Mismatch tabs stretching across the top.
// [POL]: Komponent UI: Zakładki Match i Mismatch 50/50 rozciągnięte na górze.
pub fn draw_tabs(ui: &mut egui::Ui, gt: &GuiI18n, is_match: &mut bool) {
    ui.horizontal(|ui| {
        let item_width = (ui.available_width() - 8.0) / 2.0;

        // --- MATCH (-m) ---
        let mut m_color = egui::Color32::from_rgb(150, 150, 150);
        let mut m_bg = egui::Color32::TRANSPARENT;
        if *is_match {
            m_color = egui::Color32::from_rgb(138, 90, 255);
            m_bg = egui::Color32::from_rgb(40, 40, 40);
        }

        let m_btn = ui.add_sized(
            [item_width, 40.0],
            egui::Button::new(
                egui::RichText::new(gt.t(GT::TabMatch))
                    .size(16.0)
                    .strong()
                    .color(m_color),
            )
            .fill(m_bg),
        );
        if m_btn.clicked() {
            *is_match = true;
        }

        ui.add_space(8.0);

        // --- MISMATCH (-x) ---
        let mut x_color = egui::Color32::from_rgb(150, 150, 150);
        let mut x_bg = egui::Color32::TRANSPARENT;
        if !*is_match {
            x_color = egui::Color32::from_rgb(255, 80, 100);
            x_bg = egui::Color32::from_rgb(40, 40, 40);
        }

        let x_btn = ui.add_sized(
            [item_width, 40.0],
            egui::Button::new(
                egui::RichText::new(gt.t(GT::TabMismatch))
                    .size(16.0)
                    .strong()
                    .color(x_color),
            )
            .fill(x_bg),
        );
        if x_btn.clicked() {
            *is_match = false;
        }
    });
}

// [ENG]: UI component: Statistics footer placeholder.
// [POL]: Komponent UI: Stopka ze statystykami.
pub fn draw_footer(ui: &mut egui::Ui, panel_id: &'static str, stats: &crate::interfaces::gui::TreeStats) {
    let fmt_bytes = |b: u64| -> String {
        let kb = b as f64 / 1024.0;
        if kb < 1.0 { format!("{} B", b) }
        else if kb < 1024.0 { format!("{:.1} KB", kb) }
        else { format!("{:.2} MB", kb / 1024.0) }
    };

    egui::TopBottomPanel::bottom(panel_id).show_inside(ui, |ui| {
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label(format!("📝 Txt: {} ({})", stats.txt_count, fmt_bytes(stats.txt_weight))); ui.separator();
            ui.label(format!("📦 Bin: {} ({})", stats.bin_count, fmt_bytes(stats.bin_weight))); ui.separator();
            
            if stats.err_count > 0 { // ⚡ Zaznacza się na czerwono, jeśli są błędy
                ui.label(egui::RichText::new(format!("🚫 Err: {} ({})", stats.err_count, fmt_bytes(stats.err_weight))).color(egui::Color32::RED)); ui.separator();
            } else {
                ui.label("🚫 Err: 0 (0 B)"); ui.separator();
            }
            
            ui.label(format!("🕳️ Empty: {}", stats.empty_count)); ui.separator();
            ui.label(format!("🎯 Matched: {} / {}", stats.matched_count, stats.total_count));
        });
        ui.add_space(5.0);
    });
}

// [ENG]: UI component: Central scrollable editor.
// [POL]: Komponent UI: Centralny przewijalny edytor.
pub fn draw_editor(ui: &mut egui::Ui, text_buffer: &mut String) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            ui.add(
                egui::TextEdit::multiline(text_buffer)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY),
            );
        });
    });
}
