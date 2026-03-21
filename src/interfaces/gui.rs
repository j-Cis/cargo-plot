pub mod code;
pub mod i18n;
pub mod paths;
pub mod settings;
pub mod shared;

use crate::interfaces::cli::args::CliArgs;
use eframe::egui;

#[derive(PartialEq)]
pub enum Tab {
    Settings,
    Paths,
    Code,
}

#[derive(PartialEq)]
pub enum PathsTab {
    Match,
    Mismatch,
}

// ⚡ Dodana zakładka dla karty "Kod"
#[derive(PartialEq)]
pub enum CodeTab {
    Match,
    Mismatch,
}

#[derive(Default, Clone)]
pub struct TreeStats {
    pub txt_count: usize,
    pub txt_weight: u64,
    pub bin_count: usize,
    pub bin_weight: u64,
    pub err_count: usize,
    pub err_weight: u64,
    pub empty_count: usize,
    pub matched_count: usize,
    pub total_count: usize,
}

pub struct CargoPlotApp {
    pub args: CliArgs,
    pub active_tab: Tab,
    pub active_paths_tab: PathsTab,
    pub active_code_tab: CodeTab,
    pub new_pattern_input: String,
    pub out_path_input: String,
    pub generated_paths_m: String,
    pub generated_paths_x: String,
    pub generated_code_m: String,
    pub generated_code_x: String,
    pub stats_m: TreeStats,
    pub stats_x: TreeStats,
    pub ui_scale: f32,
}

impl CargoPlotApp {
    pub fn new(args: CliArgs) -> Self {
        let default_out = args.dir_out.clone().unwrap_or_default();
        Self {
            args,
            active_tab: Tab::Settings,
            active_paths_tab: PathsTab::Match,
            active_code_tab: CodeTab::Match, // ⚡ Domyślnie ładujemy zakładkę MATCH
            new_pattern_input: String::new(),
            out_path_input: default_out, // Inicjalizacja ścieżki
            generated_paths_m: String::new(),
            generated_paths_x: String::new(),
            generated_code_m: String::new(), // ⚡ Pusty na start
            generated_code_x: String::new(), // ⚡ Pusty na start
            stats_m: TreeStats::default(),
            stats_x: TreeStats::default(),
            ui_scale: 1.0,
        }
    }
}

impl eframe::App for CargoPlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(self.ui_scale);

        // GÓRNY PANEL (Teraz tylko 3 karty)
        egui::TopBottomPanel::top("top_tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, Tab::Settings, "Setting\nUstawienia");
                ui.selectable_value(&mut self.active_tab, Tab::Paths, "Paths\nŚcieżki");
                ui.selectable_value(&mut self.active_tab, Tab::Code, "Code\nKod");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);

                    if ui
                        .button("➕")
                        .on_hover_text("Powiększ (Zoom in)")
                        .clicked()
                    {
                        self.ui_scale += 0.1; // Powiększa o 10%
                    }

                    if ui
                        .button("🔄")
                        .on_hover_text("Resetuj skalę (100%)")
                        .clicked()
                    {
                        self.ui_scale = 1.0; // Wraca do standardu
                    }

                    if ui
                        .button("➖")
                        .on_hover_text("Pomniejsz (Zoom out)")
                        .clicked()
                        && self.ui_scale > 0.6
                    {
                        // Zabezpieczenie, żeby nie zmniejszyć za bardzo
                        self.ui_scale -= 0.1;
                    }

                    // Wyświetla aktualny procent powiększenia (np. "120%")
                    ui.label(
                        egui::RichText::new(format!("🔍 Skala: {:.0}%", self.ui_scale * 100.0))
                            .weak(),
                    );
                });
            });
        });

        // ŚRODEK OKNA
        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            Tab::Settings => settings::show(ui, self),
            Tab::Paths => paths::show(ui, self),
            Tab::Code => code::show(ui, self),
        });
    }
}

pub fn run_gui(args: CliArgs) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("cargo-plot"),
        ..Default::default()
    };
    eframe::run_native(
        "cargo-plot",
        options,
        Box::new(|_cc| Ok(Box::new(CargoPlotApp::new(args)))),
    )
    .unwrap();
}
