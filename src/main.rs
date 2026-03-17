// [ENG]: Main entry point switching between interactive TUI and automated CLI.
// [POL]: Główny punkt wejścia przełączający między interaktywnym TUI a automatycznym CLI.

#![allow(clippy::pedantic, clippy::struct_excessive_bools)]

use std::env;
mod interfaces;

fn main() {
    // Rejestrujemy pusty handler Ctrl+C.
    // Dzięki temu system nie zabije programu natychmiast, a `cliclack`
    // przejmie sygnał i bezpiecznie wyjdzie z prompta.
    ctrlc::set_handler(move || {}).expect("Błąd podczas ustawiania handlera Ctrl+C");

    let args: Vec<String> = env::args().collect();

    // [POL]: Uruchom TUI tylko jeśli:
    // 1. Brak argumentów (tylko nazwa pliku binarnego) -> len == 1
    // 2. Wywołanie subkomendy bez flag (cargo-plot plot) -> len == 2 && args[1] == "plot"
    let is_tui = args.len() == 1 || (args.len() == 2 && args[1] == "plot");

    if is_tui {
        interfaces::tui::run_tui();
        return; // ⚡ ODKOMENTOWANE: Zapobiega odpaleniu CLI po wyjściu z TUI
    }

    // Wszystko inne (w tym --help) trafia do parsera CLI
    interfaces::cli::run_cli();
}
