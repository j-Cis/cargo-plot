// [ENG]: Interactive Terminal User Interface (TUI) module registry.
// [POL]: Rejestr modułu interaktywnego interfejsu tekstowego (TUI).

pub mod i18n;
pub mod menu;
pub mod state;

use state::StateTui;

pub fn run_tui() {
    let mut s = StateTui::new();
    cliclack::intro(" 📖 https://crates.io/crates/cargo-plot").unwrap();

    menu::enter::menu_enter(&mut s);
}
