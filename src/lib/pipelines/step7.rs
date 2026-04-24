use std::io;

use crate::lib::{
    pipelines::{render_table, FormattedRow},
    schema::{ReadyJob, SharedJobRunMode},
};


// ============================================================================
// VIEWER (Step 7) - PODGLĄD W KONSOLI
// ============================================================================

pub fn engine_step7_data_view(job: &ReadyJob, formatted_rows: &[FormattedRow]) -> io::Result<()> {
    let modes = job.run_modes();

    // ⚡ BLOKADA 1: Całkowita cisza (--clear)
    if modes.contains(&SharedJobRunMode::PrintNothing) {
        return Ok(()); 
    }

    // ⚡ BLOKADA 2: Tylko ostrzeżenia (--only-warnings) -> nie drukujemy podglądu danych
    if modes.contains(&SharedJobRunMode::PrintOnlyWarning) {
        return Ok(());
    }

    // Ograniczamy podgląd do pierwszych 100 elementów
    let limit = formatted_rows.len().min(100);
    let preview_rows = &formatted_rows[..limit];

    // Używamy tego samego generatora tabeli (render_table)
    let table_plain_text = render_table(preview_rows, job.attributes().select());

    println!("{}", table_plain_text);

    // Mały dodatek UX – informacja o przycięciu wyników
    if formatted_rows.len() > limit {
        println!("... (pokazano {} z {} elementów)", limit, formatted_rows.len());
    }

    // ⚡ INSPEKCJA: Dodatkowe informacje debug dla terminala (--inspection-in-print)
    if modes.contains(&SharedJobRunMode::PrintWithInspection) {
        println!("🔍 [DEBUG] Całkowita liczba przetworzonych wierszy: {}", formatted_rows.len());
        println!("🔍 [DEBUG] Tryb kolorów włączony: {}", modes.contains(&SharedJobRunMode::PrintColor));
    }

    Ok(())
}