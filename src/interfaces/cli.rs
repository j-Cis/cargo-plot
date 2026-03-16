pub mod args;
pub mod engine;

use self::args::CargoCli;
use clap::Parser;

// [EN]: Main entry point for the CLI interface.
// [PL]: Główny punkt wejścia dla interfejsu CLI.
pub fn run_cli() {
    // [PL]: Pobieramy surowe argumenty bezpośrednio z systemu.
    let args_os = std::env::args();
    let mut args: Vec<String> = args_os.collect();

    // [EN]: Injection trick: If run via 'cargo run -- -d...', 'plot' is missing.
    // We insert it manually so the parser matches the Cargo plugin structure.
    // [PL]: Trik z wstrzyknięciem: Jeśli uruchomiono przez 'cargo run -- -d...', brakuje 'plot'.
    // Wstawiamy go ręcznie, aby parser pasował do struktury wtyczki Cargo.
    if args.len() > 1 && args[1] != "plot" {
        args.insert(1, "plot".to_string());
    }

    // [EN]: Now parse from the modified list.
    // [PL]: Teraz parsujemy ze zmodyfikowanej listy.
    let CargoCli::Plot(flags) = CargoCli::parse_from(args);

    // [EN]: Transfer control to our execution engine.
    // [PL]: Przekazanie kontroli do naszego silnika wykonawczego.
    engine::run(flags);
}
