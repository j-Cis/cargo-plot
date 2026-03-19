pub mod args;
pub mod engine;

use self::args::CargoCli;
use clap::Parser;

// [ENG]: Main entry point for the CLI interface.
// [POL]: Główny punkt wejścia dla interfejsu CLI.
pub fn run_cli() {
    // [POL]: Pobieramy surowe argumenty bezpośrednio z systemu.
    let args_os = std::env::args();
    let mut args: Vec<String> = args_os.collect();

    // [ENG]: Injection trick: If run via 'cargo run -- -d...', 'plot' is missing.
    // We insert it manually so the parser matches the Cargo plugin structure.
    // [POL]: Trik z wstrzyknięciem: Jeśli uruchomiono przez 'cargo run -- -d...', brakuje 'plot'.
    // Wstawiamy go ręcznie, aby parser pasował do struktury wtyczki Cargo.
    if args.len() > 1 && args[1] != "plot" {
        args.insert(1, "plot".to_string());
    }

    // [ENG]: Now parse from the modified list.
    // [POL]: Teraz parsujemy ze zmodyfikowanej listy.
    let CargoCli::Plot(flags) = CargoCli::parse_from(args);

    // [ENG]: Transfer control to our execution engine.
    // [POL]: Przekazanie kontroli do naszego silnika wykonawczego.
    if flags.gui {
        // Jeśli podano -g, od razu ładujemy okienko ze sparsowaną konfiguracją
        crate::interfaces::gui::run_gui(flags);
    } else {
        // Jeśli nie, uruchamiamy standardowy silnik generujący raport w terminalu
        engine::run(flags);
    }
}
