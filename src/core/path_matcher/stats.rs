use crate::core::path_view::{PathList, PathTree, PathGrid, ViewMode};

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

impl MatchStats {
    /// ⚡ NOWOŚĆ: Hermetyzacja renderowania po stronie rdzenia.
    /// Zwraca gotowy, złożony ciąg znaków, gotowy do wrzucenia w konsolę lub plik.
    #[must_use]
    pub fn render_output(&self, view_mode: ViewMode, show_mode: ShowMode) -> String {
        let mut out = String::new();
        let do_include = show_mode == ShowMode::Include || show_mode == ShowMode::Context;
        let do_exclude = show_mode == ShowMode::Exclude || show_mode == ShowMode::Context;

        match view_mode {
            ViewMode::Grid => {
                if do_include {
                    if let Some(grid) = &self.included.grid {
                        out.push_str("✅ DOPASOWANIA\n");
                        out.push_str(&grid.render_cli());
                    }
                }
                if do_exclude {
                    if let Some(grid) = &self.excluded.grid {
                        out.push_str("❌ ODRZUCENIA\n");
                        out.push_str(&grid.render_cli());
                    }
                }
            }
            ViewMode::Tree => {
                if do_include {
                    if let Some(tree) = &self.included.tree {
                        out.push_str("✅ DOPASOWANIA\n");
                        out.push_str(&tree.render_cli());
                    }
                }
                if do_exclude {
                    if let Some(tree) = &self.excluded.tree {
                        out.push_str("❌ ODRZUCENIA\n");
                        out.push_str(&tree.render_cli());
                    }
                }
            }
            ViewMode::List => {
                if do_include {
                    if let Some(list) = &self.included.list {
                        out.push_str("✅ DOPASOWANIA\n");
                        out.push_str(&list.render_cli(true));
                    }
                }
                if do_exclude {
                    if let Some(list) = &self.excluded.list {
                        out.push_str("❌ ODRZUCENIA\n");
                        out.push_str(&list.render_cli(false));
                    }
                }
            }
        }

        out
    }
}