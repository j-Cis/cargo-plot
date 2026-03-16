use crate::core::path_view::{PathList, PathTree, PathGrid};

/// [PL]: Podzbiór wyników zawierający surowe ścieżki i wygenerowane widoki.
#[derive(Default)]
pub struct ResultSet {
    pub paths: Vec<String>,
    pub tree: Option<PathTree>,
    pub list: Option<PathList>,
    pub grid: Option<PathGrid>,
}

// [EN]: Simple stats object to avoid manual counting in the Engine.
// [PL]: Prosty obiekt statystyk, aby uniknąć ręcznego liczenia w Engine.
#[derive(Default)]
pub struct MatchStats {
    pub matched: usize,
    pub rejected: usize,
    pub total: usize,
    pub included: ResultSet,
    pub excluded: ResultSet,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShowMode {
    Include,
    Exclude,
    Context,
}