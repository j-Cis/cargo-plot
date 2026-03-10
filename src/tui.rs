// Plik: src/tui.rs

use cliclack::{confirm, input, intro, outro, outro_cancel, select, spinner};
use std::process::exit;

// Importujemy funkcje i struktury z lib.rs
use lib::fn_copy_dist::{copy_dist, DistConfig};
use lib::fn_doc_gen::generate_docs;
use lib::fn_doc_models::DocTask;
use lib::fn_filespath::{filespath, Task};
use lib::fn_filestree::filestree;
use lib::fn_plotfiles::plotfiles_cli;

pub fn run_tui() {
    intro(" 📦 cargo-plot - Interaktywny Kreator ").unwrap();

    let action = select("Wybierz akcję, którą chcesz wykonać:")
        .item("tree", "🌲 Rysuj drzewo projektu (Tree)", "Szybki podgląd w konsoli")
        .item("doc", "📄 Generuj dokumentację (Doc)", "Tworzy raporty Markdown")
        .item("dist", "📦 Przygotuj dystrybucję (Dist)", "Kopiuje binarki do /dist")
        .item("quit", "❌ Wyjdź", "")
        .interact();

    match action {
        Ok(choice) => match choice {
            "tree" => {
                let spin = spinner();
                spin.start("Skanowanie plików...");

                let tasks = vec![Task {
                    path_location: ".",
                    path_exclude: vec![".git/", "target/", "node_modules/", ".vs/", ".idea/", ".vscode/"],
                    output_type: "dirs_and_files",
                    ..Default::default()
                }];

                let paths = filespath(&tasks);
                let nodes = filestree(paths, "alpha");
                
                spin.stop("Zbudowano drzewo projektu:");
                println!("{}", plotfiles_cli(&nodes, "", None));
                outro("Koniec pracy. Wpisz `cargo plot --help`, by poznać zaawansowane opcje CLI!").unwrap();
            }

            "doc" => {
                // 1. Pytamy o nazwę pliku
                let out_name: String = input("Podaj bazową nazwę pliku wyjściowego:")
                    .default_input("code")
                    .interact()
                    .unwrap_or_else(|_| "code".to_string());

                // 2. Pytamy o styl spisu treści
                let insert_tree = select("Jak wstawić wizualizację drzewa do raportu?")
                    .item("files-first", "Pliki na górze (files-first)", "")
                    .item("dirs-first", "Katalogi na górze (dirs-first)", "")
                    .item("with-out", "Całkowicie bez drzewa", "")
                    .interact()
                    .unwrap_or("files-first");

                let spin = spinner();
                spin.start("Skanowanie kodu i generowanie raportu Markdown...");

                let tasks = vec![Task {
                    path_location: ".",
                    path_exclude: vec![".git/", "target/", "node_modules/", ".vs/", ".idea/", ".vscode/"],
                    output_type: "dirs_and_files",
                    ..Default::default()
                }];

                let doc_task = DocTask {
                    output_filename: &out_name,
                    insert_tree,
                    id_style: "id-tag", // Używamy domyślnego, czytelnego stylu
                    tasks,
                };

                match generate_docs(vec![doc_task], "doc") {
                    Ok(_) => {
                        spin.stop("Zapisano dokumentację w katalogu 'doc/'!");
                        outro("Twój raport jest gotowy do przeglądu!").unwrap();
                    }
                    Err(e) => {
                        spin.error(format!("Błąd zapisu pliku: {}", e));
                        exit(1);
                    }
                }
            }

            "dist" => {
                // 1. Pytamy o binarne artefakty
                let bin_input: String = input("Jakie pliki skopiować? (zostaw puste, by skopiować WSZYSTKIE binarki)")
                    .placeholder("np. cargo-plot, serwer")
                    .interact()
                    .unwrap_or_default();

                // 2. Pytamy czy czyścić katalog (T/n)
                let clear_dist: bool = confirm("Czy usunąć stare pliki z folderu /dist przed kopiowaniem?")
                    .initial_value(true)
                    .interact()
                    .unwrap_or(true);

                let spin = spinner();
                spin.start("Szukanie binarek w folderze /target...");

                // Przygotowujemy wektor binarek odrzucając puste spacje
                let bin_refs: Vec<&str> = if bin_input.trim().is_empty() {
                    vec![]
                } else {
                    bin_input.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
                };

                let config = DistConfig {
                    target_dir: "./target",
                    dist_dir: "./dist",
                    binaries: bin_refs,
                    clear_dist,
                    overwrite: true,
                    dry_run: false,
                };

                match copy_dist(&config) {
                    Ok(files) => {
                        if files.is_empty() {
                            spin.error("Nie znaleziono żadnych plików! Spróbuj najpierw wpisać `cargo build`.");
                            exit(1);
                        } else {
                            spin.stop(format!("Skopiowano {} plików do struktury dystrybucyjnej!", files.len()));
                            for (s, d) in files {
                                println!("   └── {} -> {}", s.file_name().unwrap_or_default().to_string_lossy(), d.display());
                            }
                            outro("Katalog ./dist jest gotowy na premierę!").unwrap();
                        }
                    }
                    Err(e) => {
                        spin.error(format!("Krytyczny błąd kopiowania: {}", e));
                        exit(1);
                    }
                }
            }

            _ => {
                outro_cancel("Anulowano przez użytkownika.").unwrap();
                exit(0);
            }
        },
        Err(_) => {
            outro_cancel("Wymuszono zamknięcie programu (Ctrl+C).").unwrap();
            exit(0);
        }
    }
}