// Plik: src/tui.rs

use cliclack::{confirm, input, intro, outro, outro_cancel, select, spinner};
use std::process::exit;

// Importy z Twojej biblioteki
use lib::fn_copy_dist::{copy_dist, DistConfig};
use lib::fn_doc_gen::generate_docs;
use lib::fn_doc_models::DocTask;
use lib::fn_filespath::{filespath, Task};
use lib::fn_filestree::filestree;
use lib::fn_plotfiles::plotfiles_cli;

pub fn run_tui() {
    intro(" 📦 cargo-plot - Panel Sterowania ").unwrap();

    loop {
        let action = select("Co chcesz zrobić?")
            .item("tree", "🌲 Wizualizacja Drzewa (Tree)", "Podgląd struktury z filtrami")
            .item("doc", "📄 Generator Raportu (Doc)", "Pełna dokumentacja Markdown")
            .item("dist", "📦 Zarządzanie Wydaniem (Dist)", "Kopiowanie binarek do /dist")
            .item("quit", "❌ Wyjdź", "")
            .interact();

            match action {
              Ok("tree") => run_tree_flow(),
              Ok("doc") => run_doc_flow(),
              Ok("dist") => run_dist_flow(),
              Ok("quit") => {
                  outro("Do zobaczenia!").unwrap();
                  exit(0);
              }
              _ => {
                  // Jeśli użytkownik naciśnie Esc, wyświetlimy czerwoną informację:
                  outro_cancel("Operacja anulowana przez użytkownika.").unwrap();
                  exit(0);
                }
            }

        let stay = confirm("Czy chcesz wykonać kolejną operację?")
            .initial_value(true)
            .interact()
            .unwrap_or(false);

        if !stay {
            outro("Do zobaczenia!").unwrap();
            break;
        }
    }
}

// --- FLOW: TREE ---
fn run_tree_flow() {
    let path: String = input("Ścieżka bazowa (loc):")
        .default_input(".")
        .interact()
        .unwrap();

    let whitelist_raw: String = input("Whitelist: Lokalizacje do UWZGLĘDNIENIA [Enter = wszystko]:")
        .placeholder("src, lib, Cargo.toml")
        .required(false)
        .interact()
        .unwrap_or_default();
    let whitelist: Vec<String> = split_and_trim(&whitelist_raw);

    let blacklist_raw: String = input("Blacklist: Lokalizacje do POMINIĘCIA:")
        .default_input("target, .git, node_modules")
        .required(false)
        .interact()
        .unwrap_or_default();
    let blacklist: Vec<String> = split_and_trim(&blacklist_raw);

    let filter_raw: String = input("Filtry rozszerzeń (np. *.rs) [Enter = wszystko]:")
        .placeholder("*.rs, *.md")
        .required(false)
        .interact()
        .unwrap_or_default();
    let filter: Vec<String> = split_and_trim(&filter_raw);

    let sort = select_sort();
    let show_type = select_type();

    // Logika rekurencyjna dla whitelisty
    let processed_whitelist: Vec<String> = if whitelist.is_empty() {
        vec![]
    } else {
        whitelist.iter().map(|s: &String| {
            if s.ends_with('/') || !s.contains('.') {
                let base = s.trim_end_matches('/');
                format!("{}/**/*", base)
            } else {
                s.clone()
            }
        }).collect()
    };

    let spin = spinner();
    spin.start("Analizowanie struktury...");

    let tasks = vec![Task {
        path_location: &path,
        path_include_only: processed_whitelist.iter().map(|s| s.as_str()).collect(),
        path_exclude: blacklist.iter().map(|s| s.as_str()).collect(),
        filter_files: filter.iter().map(|s| s.as_str()).collect(),
        output_type: show_type,
        //..Default::default()
    }];

    let nodes = filestree(filespath(&tasks), sort);
    spin.stop("Zbudowano drzewo:");
    println!("{}", plotfiles_cli(&nodes, "", None));
}

// --- FLOW: DOC ---
fn run_doc_flow() {
    let out_name: String = input("Nazwa raportu (prefix):")
        .default_input("code")
        .interact()
        .unwrap();

    let id_style = select("Styl nagłówków (ID):")
        .item("id-tag", "Opisowy (tag)", "")
        .item("id-num", "Numerowany (num)", "")
        .item("id-non", "Tylko ścieżka (none)", "")
        .interact().unwrap();

    let tree_style = select("Spis treści (drzewo):")
        .item("files-first", "Pliki na górze", "")
        .item("dirs-first", "Foldery na górze", "")
        .item("with-out", "Brak drzewa", "")
        .interact().unwrap();

    let is_dry = confirm("Czy uruchomić tryb symulacji (Dry-Run)?")
        .initial_value(false)
        .interact().unwrap();

    let spin = spinner();
    spin.start("Generowanie dokumentacji...");

    let tasks = vec![Task {
        path_location: ".",
        path_exclude: vec![".git/", "target/", "node_modules/", ".vs/", ".idea/", ".vscode/"],
        ..Default::default()
    }];

    let doc_task = DocTask {
        output_filename: &out_name,
        insert_tree: tree_style,
        id_style,
        tasks,
    };

    if is_dry {
        spin.stop("Symulacja zakończona (nie zapisano plików).");
    } else {
        match generate_docs(vec![doc_task], "doc") {
            Ok(_) => spin.stop("Raport zapisany w folderze /doc"),
            Err(e) => spin.error(format!("Błąd: {}", e)),
        }
    }
}

// --- FLOW: DIST ---
// --- FLOW: DIST (Poprawiony błąd E0716) ---
fn run_dist_flow() {
    let bins: String = input("Nazwy binarek [Enter = wszystkie]:")
        .placeholder("np. cargo-plot")
        .required(false)
        .interact().unwrap_or_default();
    
    let clear = confirm("Czy wyczyścić folder /dist?")
        .initial_value(true)
        .interact().unwrap();

    let spin = spinner();
    spin.start("Przygotowywanie dystrybucji...");

    // ROZWIĄZANIE E0716: Tworzymy stałą zmienną (binding), która będzie żyć 
    // do końca tej funkcji, dzięki czemu bin_list może bezpiecznie do niej linkować.
    let owned_bin_list = split_and_trim(&bins);
    let bin_list: Vec<&str> = owned_bin_list.iter().map(|s| s.as_str()).collect();

    let config = DistConfig {
        target_dir: "./target",
        dist_dir: "./dist",
        binaries: bin_list, // Teraz bin_list wskazuje na bezpieczne dane w owned_bin_list
        clear_dist: clear,
        overwrite: true,
        dry_run: false,
    };

    match copy_dist(&config) {
        Ok(f) => spin.stop(format!("Skopiowano {} plików.", f.len())),
        Err(e) => spin.error(format!("Błąd: {}", e)),
    }
}

// --- POMOCNICZE ---

fn split_and_trim(input: &str) -> Vec<String> {
    if input.trim().is_empty() {
        vec![]
    } else {
        input.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    }
}

fn select_sort() -> &'static str {
    select("Sortowanie:")
        .item("alpha", "Alfabetyczne", "")
        .item("dirs-first", "Katalogi najpierw", "")
        .item("files-first", "Pliki najpierw", "")
        .interact().unwrap()
}

fn select_type() -> &'static str {
    select("Co wyświetlić?")
        .item("dirs_and_files", "Wszystko", "")
        .item("files", "Tylko pliki (z nadrzędnymi)", "")
        .item("dirs", "Tylko foldery", "")
        .interact().unwrap()
}