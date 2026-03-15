use std::env;

mod interfaces;

fn main() {
    // Rejestrujemy pusty handler Ctrl+C.
    // Dzięki temu system nie zabije programu natychmiast, a `cliclack`
    // przejmie sygnał i bezpiecznie wyjdzie z prompta.
    ctrlc::set_handler(move || {}).expect("Błąd podczas ustawiania handlera Ctrl+C");

    // [EN]: Start TUI if no arguments are provided.
    if env::args().len() <= 2 {
        interfaces::tui::run_tui();
        // return;
    }
}
