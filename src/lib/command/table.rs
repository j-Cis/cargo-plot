use crate::lib::logic::{TabColumn, TabSortBy};

pub fn parse_column(s: &str) -> Result<TabColumn, String> {
    match s.trim().to_lowercase().as_str() {
        "treelist" | "tree" | "list" => Ok(TabColumn::TreeList),
        "number" => Ok(TabColumn::Number),
        "icon" => Ok(TabColumn::Icon),
        "size" => Ok(TabColumn::Size),
        "date" => Ok(TabColumn::Date),
        "time" => Ok(TabColumn::Time),
        "path" => Ok(TabColumn::Path),
        _ => Err(format!(
            "Nieprawidłowa nazwa kolumny: '{}'. Dostępne: list, tree, number, icon, size, date, time, path",
            s.trim()
        )),
    }
}

pub fn parse_sort(s: &str) -> Result<TabSortBy, String> {
    match s.trim().to_lowercase().as_str() {
        "name" => Ok(TabSortBy::Name),
        "size" => Ok(TabSortBy::Size),
        "date" => Ok(TabSortBy::Date),
        "kind" => Ok(TabSortBy::Kind),
        "file-first" => Ok(TabSortBy::FileFirst),
        "dir-first" => Ok(TabSortBy::DirFirst),
        "file-merge" => Ok(TabSortBy::FileFirstMerge),
        "dir-merge" => Ok(TabSortBy::DirFirstMerge),
        _ => Err(format!(
            "Nieprawidłowa wartość sortowania: '{}'. Dostępne: name, size, date, kind, file-first, dir-first, file-merge, dir-merge",
            s.trim()
        )),
    }
}