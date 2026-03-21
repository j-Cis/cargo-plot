pub mod args;
pub mod engine;

use self::args::CargoCli;
use clap::Parser;

// [ENG]: Main entry point for the CLI interface and global router.
// [POL]: Główny punkt wejścia dla interfejsu CLI i globalny router.
pub fn run_cli() {
    let args_os = std::env::args();
    let mut args: Vec<String> = args_os.collect();

    // ⚡ NOWOŚĆ: Jeśli wywołano bez żadnych argumentów (samo `cargo plot`),
    // wstrzykujemy domyślnie flagę `-g` (GUI).
    let is_empty = args.len() == 1 || (args.len() == 2 && args[1] == "plot");
    if is_empty {
        args.push("-g".to_string());
    }

    // [ENG]: Injection trick: If run via 'cargo run -- -d...', 'plot' is missing.
    // [POL]: Trik z wstrzyknięciem: Jeśli uruchomiono przez 'cargo run -- -d...', brakuje 'plot'.
    if args.len() > 1 && args[1] != "plot" {
        args.insert(1, "plot".to_string());
    }

    // [ENG]: Parse from the modified list.
    // [POL]: Parsowanie ze zmodyfikowanej listy.
    let CargoCli::Plot(flags) = CargoCli::parse_from(args);

    // [ENG]: Transfer control based on parsed flags.
    // [POL]: Przekazanie kontroli na podstawie sparsowanych flag.
    if flags.gui {
        crate::interfaces::gui::run_gui(flags);
    } else if flags.tui {
        crate::interfaces::tui::run_tui();
    } else {
        engine::run(flags);
    }
}
