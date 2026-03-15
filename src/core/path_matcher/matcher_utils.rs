/// [POL]: Wykonuje rozwinięcie klamer we wzorcu (np. {a,b} -> [a, b]). Obsługuje rekurencję.
/// [ENG]: Performs brace expansion in the pattern (e.g. {a,b} -> [a, b]). Supports recursion.
pub fn expand_braces(pattern: &str) -> Vec<String> {
    if let (Some(start), Some(end)) = (pattern.find('{'), pattern.find('}')) {
        if start < end {
            let prefix = &pattern[..start];
            let suffix = &pattern[end + 1..];
            let options = &pattern[start + 1..end];

            let mut expanded = Vec::new();
            for opt in options.split(',') {
                let new_pattern = format!("{}{}{}", prefix, opt, suffix);
                expanded.extend(expand_braces(&new_pattern));
            }
            return expanded;
        }
    }
    vec![pattern.to_string()]
}


/// [POL]: Definiuje dostępne strategie sortowania kolekcji ścieżek.
/// [ENG]: Defines available sorting strategies for path collections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortStrategy {
    /// [POL]: Brak stosowania algorytmu sortowania.
    /// [ENG]: No sorting algorithm applied.
    None,

    /// [POL]: Sortowanie alfanumeryczne w porządku rosnącym.
    /// [ENG]: Alphanumeric sorting in ascending order.
    Az,

    /// [POL]: Sortowanie alfanumeryczne w porządku malejącym.
    /// [ENG]: Alphanumeric sorting in descending order.
    Za,

    /// [POL]: Priorytet dla plików, następnie sortowanie alfanumeryczne rosnąco.
    /// [ENG]: Priority for files, followed by alphanumeric ascending sort.
    AzFileFirst,

    /// [POL]: Priorytet dla plików, następnie sortowanie alfanumeryczne malejąco.
    /// [ENG]: Priority for files, followed by alphanumeric descending sort.
    ZaFileFirst,

    /// [POL]: Priorytet dla katalogów, następnie sortowanie alfanumeryczne rosnąco.
    /// [ENG]: Priority for directories, followed by alphanumeric ascending sort.
    AzDirFirst,

    /// [POL]: Priorytet dla katalogów, następnie sortowanie alfanumeryczne malejąco.
    /// [ENG]: Priority for directories, followed by alphanumeric descending sort.
    ZaDirFirst,


    /// [POL]: Sortowanie alfanumeryczne rosnąco, grupujące logiczne pary plik-katalog (np. moduły) z priorytetem dla plików.
    /// [ENG]: Alphanumeric ascending sort grouping logical file-directory pairs (e.g. modules), prioritising files.
    AzFileFirstMerge,

    /// [POL]: Sortowanie alfanumeryczne malejąco, grupujące logiczne pary plik-katalog z priorytetem dla plików.
    /// [ENG]: Alphanumeric descending sort grouping logical file-directory pairs, prioritising files.
    ZaFileFirstMerge,

    /// [POL]: Sortowanie alfanumeryczne rosnąco, grupujące logiczne pary plik-katalog z priorytetem dla katalogów.
    /// [ENG]: Alphanumeric ascending sort grouping logical file-directory pairs, prioritising directories.
    AzDirFirstMerge,

    /// [POL]: Sortowanie alfanumeryczne malejąco, grupujące logiczne pary plik-katalog z priorytetem dla katalogów.
    /// [ENG]: Alphanumeric descending sort grouping logical file-directory pairs, prioritising directories.
    ZaDirFirstMerge,
}