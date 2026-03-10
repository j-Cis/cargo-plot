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
// --- FLOW: DOC (Pełna orkiestracja: Wiele raportów, wiele zadań) ---
fn run_doc_flow() {
    // 1. Katalog wyjściowy dla wszystkich raportów
    let output_dir: String = input("Katalog wyjściowy dla raportów:")
        .default_input("doc")
        .interact()
        .unwrap();

    let mut doc_tasks_data = Vec::new(); // Tu będziemy trzymać dane (Stringi), żeby nie zniknęły

    // 2. PĘTLA DODAWANIA RAPORTÓW (DocTask)
    loop {
        intro(format!(" 📄 Konfiguracja raportu nr {} ", doc_tasks_data.len() + 1)).unwrap();

        let out_name: String = input("Nazwa pliku (prefix):")
            .default_input("code")
            .interact()
            .unwrap();

        let id_style = select_id_style();
        let tree_style = select_tree_style();

        // PĘTLA DODAWANIA ZADAŃ (Task) do tego konkretnego raportu
        let mut sub_tasks_data = Vec::new();
        loop {
            let task = ask_for_full_task_data(sub_tasks_data.len() + 1);
            sub_tasks_data.push(task);

            if !confirm("Czy dodać kolejne zadanie skanowania (Task) DO TEGO raportu?").initial_value(false).interact().unwrap() {
                break;
            }
        }

        // Zapisujemy komplet danych dla jednego DocTask
        doc_tasks_data.push((out_name, id_style, tree_style, sub_tasks_data));

        if !confirm("Czy chcesz zdefiniować KOLEJNY, osobny raport (DocTask)?").initial_value(false).interact().unwrap() {
            break;
        }
    }

    let is_dry = confirm("Czy uruchomić tryb symulacji (Dry-Run)?").initial_value(false).interact().unwrap();

    // 3. KONWERSJA DANYCH NA STRUKTURY API
    // Musimy to zrobić ostrożnie, bo DocTask oczekuje referencji (&str)
    let spin = spinner();
    spin.start("Generowanie wszystkich raportów...");

    let mut final_doc_tasks = Vec::new();

    // Mapujemy nasze zebrane Stringi na struktury DocTask i Task
    for doc_data in &doc_tasks_data {
        let (name, id_s, tree_s, tasks_raw) = doc_data;
        
        let mut api_tasks = Vec::new();
        for t_raw in tasks_raw {
            api_tasks.push(Task {
                path_location: &t_raw.loc,
                path_include_only: t_raw.inc.iter().map(|s| s.as_str()).collect(),
                path_exclude: t_raw.exc.iter().map(|s| s.as_str()).collect(),
                filter_files: t_raw.fil.iter().map(|s| s.as_str()).collect(),
                output_type: t_raw.out_type,
                ..Default::default()
            });
        }

        final_doc_tasks.push(DocTask {
            output_filename: name,
            insert_tree: tree_s,
            id_style: id_s,
            tasks: api_tasks,
        });
    }

    if is_dry {
        spin.stop(format!("Symulacja zakończona. Wygenerowano by {} raportów.", final_doc_tasks.len()));
    } else {
        match generate_docs(final_doc_tasks, &output_dir) {
            Ok(_) => spin.stop(format!("Sukces! Wszystkie raporty zapisano w /{}/", output_dir)),
            Err(e) => spin.error(format!("Błąd krytyczny: {}", e)),
        }
    }
}

// Pomocnicza struktura do przetrzymywania danych Task w TUI (właściciel Stringów)
struct TaskData {
    loc: String,
    inc: Vec<String>,
    exc: Vec<String>,
    fil: Vec<String>,
    out_type: &'static str,
}

// Funkcja zbierająca pełne dane o pojedynczym zadaniu skanowania
fn ask_for_full_task_data(index: usize) -> TaskData {
    println!("\n--- Zadanie skanowania #{} ---", index);
    let loc: String = input("  Ścieżka (loc):").default_input(".").interact().unwrap();
    
    let inc_raw: String = input("  Whitelist (inc) [Enter = wszystko]:").required(false).interact().unwrap_or_default();
    let mut inc = split_and_trim(&inc_raw);
    
    // Stosujemy Twoją logikę rekurencyjną dla folderów w whitelist
    inc = inc.into_iter().map(|s| {
        if s.ends_with('/') || !s.contains('.') {
            format!("{}/**/*", s.trim_end_matches('/'))
        } else { s }
    }).collect();

    let exc_raw: String = input("  Blacklist (exc):").default_input("target, .git, node_modules").required(false).interact().unwrap_or_default();
    let exc = split_and_trim(&exc_raw);

    let fil_raw: String = input("  Filtry plików (fil) [np. *.rs]:").required(false).interact().unwrap_or_default();
    let fil = split_and_trim(&fil_raw);

    let out_type = select_type();

    TaskData { loc, inc, exc, fil, out_type }
}

fn select_id_style() -> &'static str {
    select("Styl nagłówków (ID):")
        .item("id-tag", "Opisowy (tag)", "")
        .item("id-num", "Numerowany (num)", "")
        .item("id-non", "Tylko ścieżka", "")
        .interact().unwrap()
}

fn select_tree_style() -> &'static str {
    select("Spis treści (drzewo):")
        .item("files-first", "Pliki na górze", "")
        .item("dirs-first", "Foldery na górze", "")
        .item("with-out", "Brak drzewa", "")
        .interact().unwrap()
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