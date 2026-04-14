// examples/whitespace-cleaner.rs
use plot::lib::util::WhitespaceCleaner;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let targets = ["./src", "./examples"];

    // 1. Najpierw usuwamy "złe" spacje, przez które fmt wywala błędy
    for target in targets {
        WhitespaceCleaner::clean_project(target)?;
    }

    // 2. Teraz, gdy pliki są "czyste", puszczamy oficjalny formatter
    WhitespaceCleaner::run_cargo_fmt()?;

    Ok(())
}