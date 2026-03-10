// Plik: src/tui/utils.rs
use cliclack::{input, select};

pub struct TaskData {
    pub loc: String,
    pub inc: Vec<String>,
    pub exc: Vec<String>,
    pub fil: Vec<String>,
    pub out_type: &'static str,
}

impl TaskData {
    // FIX: Dodaliśmy <'_>, aby uciszyć ostrzeżenie o elidowanych lifetime'ach
    pub fn to_api_task(&self) -> lib::fn_filespath::Task<'_> {
        lib::fn_filespath::Task {
            path_location: &self.loc,
            path_include_only: self.inc.iter().map(|s| s.as_str()).collect(),
            path_exclude: self.exc.iter().map(|s| s.as_str()).collect(),
            filter_files: self.fil.iter().map(|s| s.as_str()).collect(),
            output_type: self.out_type,
            // FIX: Usunięto ..Default::default(), bo wypełniamy wszystkie pola
        }
    }
}

pub fn ask_for_task_data(idx: usize) -> TaskData {
    println!("\n--- Konfiguracja zadania #{} ---", idx);
    let loc: String = input("  Lokalizacja (loc):")
        .default_input(".")
        .interact()
        .unwrap();
    let inc_raw: String = input("  Whitelist (inc):")
        .required(false)
        .interact()
        .unwrap_or_default();
    let exc_raw: String = input("  Blacklist (exc):")
        .required(false)
        .interact()
        .unwrap_or_default();
    let fil_raw: String = input("  Filtry plików (fil):")
        .required(false)
        .interact()
        .unwrap_or_default();

    let out_type = select_type();

    TaskData {
        loc,
        inc: process_inc(split_and_trim(&inc_raw)),
        exc: split_and_trim(&exc_raw),
        fil: split_and_trim(&fil_raw),
        out_type,
    }
}

fn process_inc(list: Vec<String>) -> Vec<String> {
    list.into_iter()
        .map(|s| {
            // FIX na "Brak Wyniku": Usuwamy ./ z początku, bo Glob tego nie lubi
            let cleaned = s.trim_start_matches("./");

            if cleaned.ends_with('/') || !cleaned.contains('.') {
                let base = cleaned.trim_end_matches('/');
                if base.is_empty() {
                    "**/*".to_string()
                } else {
                    format!("{}/**/*", base)
                }
            } else {
                cleaned.to_string()
            }
        })
        .collect()
}

pub fn split_and_trim(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn select_sort() -> &'static str {
    select("Sortowanie:")
        .item("alpha", "Alfabetyczne", "")
        .item("dirs-first", "Katalogi najpierw", "")
        .item("files-first", "Pliki najpierw", "")
        .interact()
        .unwrap()
}

pub fn select_type() -> &'static str {
    select("Co wyświetlić?")
        .item("dirs_and_files", "Wszystko", "")
        .item("files", "Tylko pliki", "")
        .item("dirs", "Tylko foldery", "")
        .interact()
        .unwrap()
}

pub fn select_id_style() -> &'static str {
    select("Styl nagłówków (ID):")
        .item("id-tag", "Opisowy (tag)", "")
        .item("id-num", "Numerowany (num)", "")
        .item("id-non", "Tylko ścieżka", "")
        .interact()
        .unwrap()
}

pub fn select_tree_style() -> &'static str {
    select("Spis treści (drzewo):")
        .item("files-first", "Pliki na górze", "")
        .item("dirs-first", "Foldery na górze", "")
        .item("with-out", "Brak drzewa", "")
        .interact()
        .unwrap()
}
