use cliclack::{confirm, input, intro, spinner};
use lib::fn_doc_gen::generate_docs;
use lib::fn_doc_models::DocTask;
use lib::fn_filespath::Task;

// Importujemy niezbędne narzędzia z modułu utils
use super::utils::{TaskData, ask_for_task_data};

pub fn run_doc_flow() {
    let output_dir: String = input("Katalog wyjściowy dla raportów:")
        .default_input("doc")
        .interact()
        .unwrap();

    let mut reports_configs = Vec::new();

    loop {
        intro(format!(
            " 📄 Konfiguracja raportu nr {} ",
            reports_configs.len() + 1
        ))
        .unwrap();

        let name: String = input("Nazwa pliku (prefix):")
            .default_input("code")
            .interact()
            .unwrap();

        let id_s = super::utils::select_id_style();
        let tree_s = super::utils::select_tree_style();

        let mut tasks_for_this_report = Vec::new();
        loop {
            // Teraz funkcja jest zaimportowana, więc zadziała bezpośrednio
            tasks_for_this_report.push(ask_for_task_data(tasks_for_this_report.len() + 1));

            if !confirm("Czy dodać kolejne zadanie skanowania (Task) DO TEGO raportu?")
                .initial_value(false)
                .interact()
                .unwrap()
            {
                break;
            }
        }

        reports_configs.push((name, id_s, tree_s, tasks_for_this_report));

        if !confirm("Czy chcesz zdefiniować KOLEJNY, osobny raport (DocTask)?")
            .initial_value(false)
            .interact()
            .unwrap()
        {
            break;
        }
    }

    let is_dry = confirm("Czy uruchomić tryb symulacji (Dry-Run)?")
        .initial_value(false)
        .interact()
        .unwrap();

    let spin = spinner();
    spin.start("Generowanie wszystkich raportów...");

    let mut final_doc_tasks = Vec::new();

    for r in &reports_configs {
        // Pomagamy Rustowi, określając typ 't' jako &TaskData
        let api_tasks: Vec<Task> = r.3.iter().map(|t: &TaskData| t.to_api_task()).collect();

        final_doc_tasks.push(DocTask {
            output_filename: &r.0,
            insert_tree: r.2,
            id_style: r.1,
            tasks: api_tasks,
        });
    }

    if is_dry {
        spin.stop(format!(
            "Symulacja zakończona. Wygenerowano by {} raportów.",
            final_doc_tasks.len()
        ));
    } else {
        match generate_docs(final_doc_tasks, &output_dir) {
            Ok(_) => spin.stop(format!("Wszystkie raporty zapisano w /{}/", output_dir)),
            Err(e) => spin.error(format!("Błąd krytyczny: {}", e)),
        }
    }
}
