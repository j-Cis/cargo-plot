// [ENG]: Main entry point switching between interfaces.
// [POL]: Główny punkt wejścia przełączający między interfejsami.

#![allow(clippy::pedantic, clippy::struct_excessive_bools)]

mod interfaces;

fn main() {
    // [ENG]: Register an empty Ctrl+C handler to prevent abrupt termination.
    // [POL]: Rejestrujemy pusty handler Ctrl+C, zapobiegając natychmiastowemu zabiciu programu.
    ctrlc::set_handler(move || {}).expect("Błąd podczas ustawiania handlera Ctrl+C");

    // [ENG]: Pass execution directly to the CLI parser and router.
    // [POL]: Przekazanie wykonania bezpośrednio do parsera i routera CLI.
    interfaces::cli::run_cli();
}
