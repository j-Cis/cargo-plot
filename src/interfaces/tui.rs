// [ENG]: Interactive Terminal User Interface (TUI) module registry.
// [POL]: Rejestr modułu interaktywnego interfejsu tekstowego (TUI).

pub mod i18n;
pub mod menu;
pub mod state;

pub fn run_tui() {
    let mut s = state::StateTui::new();
    cliclack::clear_screen().unwrap();
    //cliclack::intro(" 📖 https://crates.io/crates/cargo-plot").unwrap();
    menu::menu_main(&mut s);
}
