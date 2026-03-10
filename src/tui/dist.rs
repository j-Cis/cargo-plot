use cliclack::{confirm, input, intro, spinner};
use lib::fn_copy_dist::{DistConfig, copy_dist};

pub fn run_dist_flow() {
    intro(" 📦 Zarządzanie Dystrybucją ").unwrap();

    let target: String = input("Katalog kompilacji (target):")
        .default_input("./target")
        .interact()
        .unwrap();
    let dist: String = input("Katalog docelowy (dist):")
        .default_input("./dist")
        .interact()
        .unwrap();
    let bins: String = input("Binarki (przecinek) [Enter = wszystkie]:")
        .required(false)
        .interact()
        .unwrap_or_default();

    let clear = confirm("Wyczyścić katalog docelowy?")
        .initial_value(true)
        .interact()
        .unwrap();
    let dry = confirm("Tryb symulacji (Dry Run)?")
        .initial_value(false)
        .interact()
        .unwrap();

    let spin = spinner();
    spin.start("Kopiowanie artefaktów...");

    let owned_bins = super::utils::split_and_trim(&bins);
    let bin_refs: Vec<&str> = owned_bins.iter().map(|s| s.as_str()).collect();

    let config = DistConfig {
        target_dir: &target,
        dist_dir: &dist,
        binaries: bin_refs,
        clear_dist: clear,
        overwrite: true,
        dry_run: dry,
    };

    match copy_dist(&config) {
        Ok(f) => spin.stop(format!("Zakończono. Przetworzono {} plików.", f.len())),
        Err(e) => spin.error(format!("Błąd: {}", e)),
    }
}
