use crate::lib::logic::{TabColumn, TabSortBy};

pub fn parse_column(s: &str) -> Result<TabColumn, String> {
    // Delegujemy zadanie do jednego źródła prawdy
    TabColumn::parse(s)
}

pub fn parse_sort(s: &str) -> Result<TabSortBy, String> {
    // Delegujemy zadanie do jednego źródła prawdy
    TabSortBy::parse(s)
}