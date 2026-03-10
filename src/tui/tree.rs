use super::utils::TaskData;
use cliclack::{confirm, intro, spinner}; // Usunięto outro i select
use lib::fn_filespath::{Task, filespath};
use lib::fn_filestree::filestree;
use lib::fn_plotfiles::plotfiles_cli; // Usunięto ask_for_task_data (jeśli nieużywane bezpośrednio)

pub fn run_tree_flow() {
    intro(" 🌲 Eksplorator Drzewa (Multi-Task) ").unwrap();

    let mut tasks_data: Vec<TaskData> = Vec::new();

    loop {
        tasks_data.push(super::utils::ask_for_task_data(tasks_data.len() + 1));
        if !confirm("Czy dodać kolejną lokalizację (Task)?")
            .initial_value(false)
            .interact()
            .unwrap()
        {
            break;
        }
    }

    let sort = super::utils::select_sort();

    // -- ZMIANA: Wywołujemy nowy konfigurator wag --
    let w_cfg = super::utils::ask_for_weight_config();

    // Prefix '_' mówi Rustowi: "Wiem, że tego nie używam (jeszcze), nie krzycz"
    let _use_custom_style = confirm("Czy użyć niestandardowego stylu gałęzi?")
        .initial_value(false)
        .interact()
        .unwrap();

    let spin = spinner();
    spin.start("Budowanie złożonej struktury...");

    let tasks: Vec<Task> = tasks_data
        .iter()
        .map(|t: &super::utils::TaskData| t.to_api_task())
        .collect();
        
    let nodes = filestree(filespath(&tasks), sort, &w_cfg);

    spin.stop("Skanowanie zakończone:");

    // Generujemy tekst drzewa
    let tree_output = plotfiles_cli(&nodes, "", None);

    if tree_output.trim().is_empty() {
        cliclack::outro_cancel(
            "Brak wyników: Żaden plik nie pasuje do podanych filtrów (sprawdź ścieżki).",
        )
        .unwrap();
    } else {
        // Dodajemy pustą linię przed i po dla czytelności w TUI
        println!("\n{}\n", tree_output);
        cliclack::outro("Drzewo wyrenderowane pomyślnie!").unwrap();
    }
}
