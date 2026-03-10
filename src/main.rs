// Plik: src/main.rs
use clap::Parser;
use std::env;

mod cli;
mod tui;

fn main() {
    // [QoL Fix]: Jeśli uruchomiono binarkę bez żadnych argumentów (np. czyste `cargo run` 
    // lub podwójne kliknięcie na cargo-plot.exe), pomijamy walidację Clapa i odpalamy TUI.
    if env::args().len() <= 1 {
        tui::run_tui();
        return;
    }

    // Jeśli są argumenty, pozwalamy Clapowi je sparsować (wymaga słowa 'plot')
    let cli::args::CargoCli::Plot(plot_args) = cli::args::CargoCli::parse();

    match plot_args.command {
        Some(cmd) => cli::run_command(cmd),
        None => tui::run_tui(), // Zadziała np. dla `cargo run -- plot`
    }
}