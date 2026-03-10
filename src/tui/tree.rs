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

    let save_to_file =
        confirm("Czy zapisać wynikowe drzewo do pliku .md (zamiast pokazywać w konsoli)?")
            .initial_value(false)
            .interact()
            .unwrap();

    let md_path = if save_to_file {
        // Wymuszamy typowanie bezpośrednio na zmiennej wejściowej 'path', tak jak to robiliśmy w innych miejscach
        let path: String = cliclack::input("Podaj nazwę pliku (np. drzewo.md):")
            .default_input("drzewo.md")
            .interact()
            .unwrap();
        Some(path)
    } else {
        None
    };

    let spin = spinner();
    spin.start("Budowanie złożonej struktury...");

    let tasks: Vec<Task> = tasks_data
        .iter()
        .map(|t: &super::utils::TaskData| t.to_api_task())
        .collect();

    let nodes = filestree(filespath(&tasks), sort, &w_cfg);

    spin.stop("Skanowanie zakończone:");

    // Generujemy tekst drzewa
    if let Some(path) = md_path {
        let txt = lib::fn_plotfiles::plotfiles_txt(&nodes, "", None);
        std::fs::write(&path, format!("```text\n{}\n```\n", txt)).unwrap();
        cliclack::outro(format!("Sukces! Drzewo zapisano do pliku: {}", path)).unwrap();
    } else {
        let tree_output = plotfiles_cli(&nodes, "", None);
        if tree_output.trim().is_empty() {
            cliclack::outro_cancel("Brak wyników: Żaden plik nie pasuje do podanych filtrów.")
                .unwrap();
        } else {
            println!("\n{}\n", tree_output);
            cliclack::outro("Drzewo wyrenderowane pomyślnie!").unwrap();
        }
    }
}
