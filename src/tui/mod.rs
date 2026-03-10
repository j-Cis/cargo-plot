use cliclack::{confirm, intro, outro, outro_cancel, select};
use std::process::exit;

mod dist;
mod doc;
mod stamp;
mod tree;
mod utils;

pub fn run_tui() {
    intro(" 📦 cargo-plot - Profesjonalny Panel Sterowania ").unwrap();

    loop {
        let action = select("Wybierz moduł API:")
            .item(
                "tree",
                "🌲 Tree Explorer",
                "Wizualizacja struktur (Multi-Task)",
            )
            .item(
                "doc",
                "📄 Doc Orchestrator",
                "Generowanie raportów Markdown",
            )
            .item("dist", "📦 Dist Manager", "Zarządzanie paczkami binarnymi")
            .item("stamp", "🕒 Stamp Tool", "Generator sygnatur czasowych")
            .item("quit", "❌ Wyjdź", "")
            .interact();

        match action {
            Ok("tree") => tree::run_tree_flow(),
            Ok("doc") => doc::run_doc_flow(),
            Ok("dist") => dist::run_dist_flow(),
            Ok("stamp") => stamp::run_stamp_flow(),
            Ok("quit") => {
                outro("Zamykanie panelu...").unwrap();
                exit(0);
            }
            _ => {
                outro_cancel("Przerwano.").unwrap();
                exit(0);
            }
        }

        if !confirm("Czy chcesz wykonać inną operację?")
            .initial_value(true)
            .interact()
            .unwrap_or(false)
        {
            outro("Do zobaczenia!").unwrap();
            break;
        }
    }
}
