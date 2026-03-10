// Plik: src/main.rs
use clap::Parser;

mod cli;
mod tui;

fn main() {
    let cli::args::CargoCli::Plot(plot_args) = cli::args::CargoCli::parse();

    match plot_args.command {
        Some(cmd) => cli::run_command(cmd),
        None => tui::run_tui(),
    }
}