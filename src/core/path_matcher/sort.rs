use std::cmp::Ordering;

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

impl SortStrategy {
    /// [POL]: Sortuje kolekcję ścieżek w miejscu (in-place) na podstawie wybranej strategii.
    /// [ENG]: Sorts a collection of paths in-place based on the selected strategy.
    pub fn apply<S: AsRef<str>>(&self, paths: &mut [S]) {
        if *self == SortStrategy::None {
            return;
        }

        paths.sort_by(|a_s, b_s| {
            let a = a_s.as_ref();
            let b = b_s.as_ref();

            let a_is_dir = a.ends_with('/');
            let b_is_dir = b.ends_with('/');

            // Wywołujemy naszą prywatną, hermetyczną metodę
            let a_merge = Self::get_merge_key(a);
            let b_merge = Self::get_merge_key(b);

            match self {
                SortStrategy::None => Ordering::Equal,
                SortStrategy::Az => a.cmp(b),
                SortStrategy::Za => b.cmp(a),
                SortStrategy::AzFileFirst => (a_is_dir, a).cmp(&(b_is_dir, b)),
                SortStrategy::ZaFileFirst => (a_is_dir, b).cmp(&(b_is_dir, a)),
                SortStrategy::AzDirFirst => (!a_is_dir, a).cmp(&(!b_is_dir, b)),
                SortStrategy::ZaDirFirst => (!a_is_dir, b).cmp(&(!b_is_dir, a)),
                SortStrategy::AzFileFirstMerge => {
                    (a_merge, a_is_dir, a).cmp(&(b_merge, b_is_dir, b))
                }
                SortStrategy::ZaFileFirstMerge => {
                    (b_merge, a_is_dir, b).cmp(&(a_merge, b_is_dir, a))
                }
                SortStrategy::AzDirFirstMerge => {
                    (a_merge, !a_is_dir, a).cmp(&(b_merge, !b_is_dir, b))
                }
                SortStrategy::ZaDirFirstMerge => {
                    (b_merge, !a_is_dir, b).cmp(&(a_merge, !b_is_dir, a))
                }
            }
        });
    }

    /// [POL]: Prywatna metoda. Ekstrahuje rdzenną nazwę ścieżki dla strategii Merge.
    /// [ENG]: Private method. Extracts the core path name for Merge strategies.
    fn get_merge_key(path: &str) -> &str {
        let trimmed = path.trim_end_matches('/');
        if let Some(idx) = trimmed.rfind('.')
            && idx > 0
            && trimmed.as_bytes()[idx - 1] != b'/'
        {
            return &trimmed[..idx];
        }
        trimmed
    }
}
