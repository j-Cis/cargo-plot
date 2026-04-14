// use clap::{ArgGroup, Parser};
//
// use crate::lib::logic::{TabColumn, TabSortBy};
//
// #[derive(Parser, Debug)]
// #[command(name = "x-do", author, version, about = "Advanced File Scanner &
// Code Archiver", long_about = None, arg_required_else_help = true)]
// ⚡ TWORZYMY GRUPĘ: Grupuje flagi trybu, aby móc ich wymagać jako "jedno z
// dwóch" #[command(group(
// ArgGroup::new("mode_flags")
// .args(["matched", "mismatched"])
// ))]
// pub struct ArgsCommand {
// ============================================================================
// 0. INFORMACJE I POMOC
// ============================================================================
// Wyświetla szczegółową pomoc dotyczącą składni i semantyki wzorców
// (Patterns)
// #[arg(
// short = 'P',
// long = "pattern-help",
// visible_aliases = ["syntax", "pat-help"],
// exclusive = true, // ⚡ KLUCZOWE: Pozwala odpalić tę flagę bez podawania `-p`
// (ignoruje required=true) help_heading = "Information"
// )]
// pub pattern_help: bool,
//
// #[arg(
// short = 'G',
// long = "config-help",
// visible_aliases = ["toml-syntax", "toml-help"],
// help_heading = "Information"
// )]
// pub config_help: bool,
// ============================================================================
// 1. WEJŚCIE I SKANOWANIE (TARGET & PATTERNS)
// ============================================================================
// Ścieżka katalogu do skanowania
// #[arg(
// short = 'w',
// long = "work-path",
// visible_aliases = ["entry", "read", "job-path"],
// required = true,
// help_heading = "Input Options"
// )]
// pub work_path: String,
//
// Wzorce wyszukiwania (glob, rozszerzenia)
// #[arg(
// short = 'p',
// long = "pattern",
// visible_aliases = ["pat", "patterns"],
// required = true,
// help_heading = "Input Options"
// )]
// pub patterns: Vec<String>,
//
// Ignoruj wielkość liter przy dopasowywaniu wzorców
// #[arg(short = 'i', long = "ignore-case", help_heading = "Input Options")]
// pub ignore_case: bool,
//
// ============================================================================
// 2. KOLEJNOŚĆ i SPOSÓB PREZENCJI STUKTÓRY ZAWARTOŚCI
// ============================================================================
// Dezaktywuje widok drzewa (płaska lista)
// #[arg(short = 'l', long = "list", help_heading = "Layout V Options")]
// pub list_instead_tree: bool,
//
// Kryterium sortowania
// #[arg(
// long = "sort",
//  value_parser = parse_sort,
// default_value = "kind",
// help_heading = "Layout V Options"
// )]
// pub sort: TabSortBy,
//
// Odwraca kierunek sortowania (Descending)
// #[arg(short = 'r', long = "reverse", help_heading = "Layout V Options")]
// pub reverse: bool,
//
// Kolumny do wyświetlenia w tabeli
// #[arg(
// short = 'v',
// long = "view-columns",
//  value_parser = parse_column,
// value_delimiter = ',',
// default_value = "date,time,size,treelist,icon,number,path",
// help_heading = "Layout H Options"
// )]
// pub columns: Vec<TabColumn>,
//
// Włącza rozszerzony zestaw ikon (np. dla konkretnych języków
// programowania)
// #[arg(short = 'e', long = "ext-icons", help_heading = "Layout H Options")]
// pub ext_icons: bool,
//
// ============================================================================
// 3. TRYB PRACY (MATCHED vs MISMATCHED) - GENEROWANIE STRUKTURY ZAWARTOŚCI
// ============================================================================
// Wyświetl pliki odrzucone (Mismatched) zamiast dopasowanych
// #[arg(
// short = 'x',
// long = "mismatched",
// short_alias = 'X',
// conflicts_with = "matched",
// help_heading = "Mode Options"
// )]
// pub mismatched: bool,
//
// Wyświetl pliki dopasowane (Matched) - domyślne
// #[arg(
// short = 'm',
// long = "matched",
// short_alias = 'M',
// conflicts_with = "mismatched",
// help_heading = "Mode Options"
// )]
// pub matched: bool,
//
// Twardy limit wyświetlanych/zapisywanych pozycji
// #[arg(short = 'T', long = "trim-size", help_heading = "Limits & Pagination")]
// pub trim_size: Option<usize>,
//
// Wybierz konkretną stronę wyników
// #[arg(
// short = 't',
// long = "show-trimmed-page",
// visible_alias = "page",
// help_heading = "Limits & Pagination"
// )]
// pub trim_page: Option<usize>,
//
// ============================================================================
// 4. ZAPIS NA DYSK (SOTC & COTS)
// ============================================================================
// Zapisuje STRUKTURĘ zawartości (tylko tabela i statystyki)
// #[arg(
// short = 's',
// long = "save-sotc-at",
// visible_alias = "save-structure-of-the-content-at",
// value_name = "OUT_DIR",
// requires = "mode_flags",
// num_args = 0..=1, // ⚡ Pozwala wywołać flagę -s bez podawania ścieżki
// default_missing_value = "./target/.cargo-plot/", // ⚡ Wartość, gdy podano
// samo -s help_heading = "Export Options"
// )]
// pub save_sotc_at: Option<String>,
//
// Opcjonalny tytuł dokumentu (np. nazwa projektu i wersja) wstawiany do
// nagłówka
// #[arg(
// short = 'S',
// long = "title-sotc",
// visible_alias = "title-structure-of-the-content",
// value_name = "STRING",
// requires = "save_sotc_at",
// help_heading = "Export Options"
// )]
// pub title_sotc: Option<String>,
//
// Zapisuje ZAWARTOŚĆ struktury (tabela + pełne kody źródłowe)
// #[arg(
// short = 'c',
// long = "save-cots-at",
// visible_alias = "save-content-of-the-structure-at",
// value_name = "OUT_DIR",
// requires = "mode_flags",
// num_args = 0..=1, // ⚡ Pozwala wywołać flagę -c bez podawania ścieżki
// default_missing_value = "./target/.cargo-plot/", // ⚡ Wartość, gdy podano
// samo -c help_heading = "Export Options"
// )]
// pub save_cots_at: Option<String>,
//
// Opcjonalny tytuł dokumentu (np. nazwa projektu i wersja) wstawiany do
// nagłówka
// #[arg(
// short = 'C',
// long = "title-cots",
// visible_alias = "title-content-of-the-structure",
// requires = "save_cots_at",
// value_name = "STRING",
// help_heading = "Export Options"
// )]
// pub title_cots: Option<String>,
//
// ============================================================================
// 5. MODYFIKATORY RENDEROWANIA (RENDER FLAGS)
// ============================================================================
// Ukrywa nagłówek ze statystykami skanowania
// #[arg(short = 'u', long = "hide-stats", help_heading = "Render Flags")]
// pub hide_stats: bool,
//
// Ukrywa stopkę promocyjną (info o narzędziu)
// #[arg(short = 'o', long = "hide-promo", help_heading = "Render Flags")]
// pub hide_promo: bool,
//
// Ukrywa wyjście w terminalu (Cichy tryb, przydatny jeśli zależy nam tylko
// na plikach)
// #[arg(short = 'q', long = "quiet", visible_alias = "clear-work", help_heading
// = "Render Flags")] pub quiet: bool,
//
// ============================================================================
// 6. AUTOMATYZACJA
// ============================================================================
// Wczytuje i wykonuje zadanie (job) z pliku konfiguracyjnego
// #[arg(
// short = 'g',
// long = "config-get",
// visible_alias = "get",
// value_name = "FILE",
// num_args = 0..=1, // ⚡ Pozwala wywołać flagę -g bez podawania ścieżki
// default_missing_value = "./.x-do.toml", // ⚡ Ścieżka użyta, gdy podano samo
// -g help_heading = "Automatization"
// )]
// pub config_get: Option<String>,
//
// Generuje domyślny, skomentowany plik konfiguracyjny
// #[arg(
// short = 'I',
// long = "config-init",
// visible_alias = "init",
// value_name = "FILE",
// num_args = 0..=1, // ⚡ Pozwala wywołać flagę -I bez podawania ścieżki
// default_missing_value = "./.x-do.toml", // ⚡ Ścieżka użyta, gdy podano samo
// -I help_heading = "Automatization"
// )]
// pub config_init: Option<String>,
//
// Wyświetla listę dostępnych zadań (jobs) z pliku konfiguracyjnego
// #[arg(
// short = 'J',
// long = "config-jobs",
// visible_alias = "jobs",
// value_name = "FILE",
// num_args = 0..=1, // ⚡ Jeśli wywołane bez ścieżki, szuka w domyślnym
// default_missing_value = "./.x-do.toml",
// help_heading = "Automatization"
// )]
// pub config_jobs: Option<String>,
//
// Uruchamia konkretne zadanie z pliku konfiguracyjnego na podstawie jego
// ID
// #[arg(
// short = 'j',
// long = "do-job",
// visible_alias = "job",
// value_name = "JOB_ID",
// help_heading = "Automatization"
// )]
// pub do_job: Option<String>,
// }
//
