use clap::{ArgGroup, Parser};
use super::table::{parse_column, parse_sort};
use crate::lib::logic::{TabColumn, TabSortBy};

#[derive(Parser, Debug)]
#[command(name = "x-do", author, version, about = "Advanced File Scanner & Code Archiver", long_about = None, arg_required_else_help = true)]
// ⚡ TWORZYMY GRUPĘ: Grupuje flagi trybu, aby móc ich wymagać jako "jedno z dwóch"
#[command(group(
    ArgGroup::new("mode_flags")
        .args(["matched", "mismatched"])
))]
pub struct ArgsCommand {
	// ============================================================================
    // 0. INFORMACJE I POMOC
    // ============================================================================

    /// Wyświetla szczegółową pomoc dotyczącą składni i semantyki wzorców (Patterns)
    #[arg(
        short = 'P', 
        long = "pattern-help", 
        visible_aliases = ["syntax", "pat-help"], 
        exclusive = true, // ⚡ KLUCZOWE: Pozwala odpalić tę flagę bez podawania `-p` (ignoruje required=true)
        help_heading = "Information"
    )]
    pub pattern_help: bool,

    // ============================================================================
    // 1. WEJŚCIE I SKANOWANIE (TARGET & PATTERNS)
    // ============================================================================

    /// Ścieżka katalogu do skanowania
    #[arg(
        short = 'w', 
        long = "work-path", 
        short_alias = 'j',
        visible_aliases = ["entry", "read", "job-path"], 
        required = true,
        help_heading = "Input Options"
    )]
    pub work_path: String,

    /// Wzorce wyszukiwania (glob, rozszerzenia)
    #[arg(
        short = 'p', 
        long = "pattern", 
        visible_aliases = ["pat", "patterns"], 
        required = true,
        help_heading = "Input Options"
    )]
    pub patterns: Vec<String>,

    /// Ignoruj wielkość liter przy dopasowywaniu wzorców
    #[arg(
        short = 'i', 
        long = "ignore-case", 
        help_heading = "Input Options"
    )]
    pub ignore_case: bool,

    
    // ============================================================================
    // 2. KOLEJNOŚĆ i SPOSÓB PREZENCJI STUKTÓRY ZAWARTOŚCI
    // ============================================================================

    /// Aktywuje widok drzewa (zamiast płaskiej listy)
    #[arg(
        short = 't', 
        long = "tree", 
        help_heading = "Layout V Options"
    )]
    pub tree: bool,

    /// Kryterium sortowania
    #[arg(
        long = "sort", 
        value_parser = parse_sort, 
        default_value = "kind",
        help_heading = "Layout V Options"
    )]
    pub sort: TabSortBy,
    
    /// Odwraca kierunek sortowania (Descending)
    #[arg(
        short = 'r', 
        long = "reverse", 
        help_heading = "Layout V Options"
    )]
    pub reverse: bool,

     /// Kolumny do wyświetlenia w tabeli
    #[arg(
        short = 'v', 
        long = "view-columns",
        value_parser = parse_column,
        value_delimiter = ',',
        default_value = "time,number,size,date,time,treelist,icon,path",
        help_heading = "Layout H Options"
    )]
    pub columns: Vec<TabColumn>,   

    /// Włącza rozszerzony zestaw ikon (np. dla konkretnych języków programowania)
    #[arg(
        short = 'e', 
        long = "ext-icons", 
        help_heading = "Layout H Options"
    )]
    pub ext_icons: bool,

    // ============================================================================
    // 3. TRYB PRACY (MATCHED vs MISMATCHED) - GENEROWANIE STRUKTURY ZAWARTOŚCI
    // ============================================================================

    /// Wyświetl pliki odrzucone (Mismatched) zamiast dopasowanych
    #[arg(
        short = 'x', 
        long = "mismatched", 
        short_alias = 'X',
        conflicts_with = "matched",
        help_heading = "Mode Options"
    )]
    pub mismatched: bool,

    /// Wyświetl pliki dopasowane (Matched) - domyślne
    #[arg(
        short = 'm', 
        long = "matched", 
        short_alias = 'M',
        conflicts_with = "mismatched",
        help_heading = "Mode Options"
    )]
    pub matched: bool,

    
    /// Twardy limit wyświetlanych/zapisywanych pozycji
    #[arg(
        short = 'l', 
        long = "size-limit", 
        conflicts_with = "page",
        help_heading = "Limits & Pagination"
    )]
    pub limit: Option<usize>,

    /// Wybierz konkretną stronę wyników
    #[arg(
        long = "page", 
        conflicts_with = "limit",
        help_heading = "Limits & Pagination"
    )]
    pub page: Option<usize>,

    /// Liczba wyników na stronę (wymaga --page)
    #[arg(
        long = "size-page", 
        default_value = "20",
        help_heading = "Limits & Pagination"
    )]
    pub page_size: usize,

    // ============================================================================
    // 4. ZAPIS NA DYSK (SOTC & COTS)
    // ============================================================================

    /// Zapisuje STRUKTURĘ zawartości (tylko tabela i statystyki)
    #[arg(
        short = 's', 
        long = "save-sotc-at", 
        visible_alias = "save-structure-of-the-content-at",
        value_name = "OUT_DIR",
        requires = "mode_flags",
        num_args = 0..=1, // ⚡ Pozwala wywołać flagę -s bez podawania ścieżki
        default_missing_value = "./target/.cargo-plot/", // ⚡ Wartość, gdy podano samo -s
        help_heading = "Export Options"
    )]
    pub save_sotc_at: Option<String>,

    /// Zapisuje ZAWARTOŚĆ struktury (tabela + pełne kody źródłowe)
    #[arg(
        short = 'c', 
        long = "save-cots-at", 
        visible_alias = "save-content-of-the-structure-at",
        value_name = "OUT_DIR",
        requires = "mode_flags",
        num_args = 0..=1, // ⚡ Pozwala wywołać flagę -c bez podawania ścieżki
        default_missing_value = "./target/.cargo-plot/", // ⚡ Wartość, gdy podano samo -c
        help_heading = "Export Options"
    )]
    pub save_cots_at: Option<String>,

    // ============================================================================
    // 5. MODYFIKATORY RENDEROWANIA (RENDER FLAGS)
    // ============================================================================

    /// Ukrywa nagłówek ze statystykami skanowania
    #[arg(
        short = 'a', 
        long = "hide-stats", 
        help_heading = "Render Flags"
    )]
    pub hide_stats: bool,

    /// Ukrywa stopkę promocyjną (info o narzędziu)
    #[arg(
        short = 'b', 
        long = "hide-promo", 
        help_heading = "Render Flags"
    )]
    pub hide_promo: bool,
    
    /// Ukrywa wyjście w terminalu (Cichy tryb, przydatny jeśli zależy nam tylko na plikach)
    #[arg(
        short = 'q',
        long = "quiet",
        help_heading = "Render Flags"
    )]
    pub quiet: bool,
}