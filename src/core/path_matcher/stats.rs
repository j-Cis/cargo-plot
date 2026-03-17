use crate::core::path_view::{PathGrid, PathList, PathTree, ViewMode};

/// [POL]: Podzbiór wyników zawierający surowe ścieżki i wygenerowane widoki.
#[derive(Default)]
pub struct ResultSet {
    pub paths: Vec<String>,
    pub tree: Option<PathTree>,
    pub list: Option<PathList>,
    pub grid: Option<PathGrid>,
}

// [ENG]: Simple stats object to avoid manual counting in the Engine.
// [POL]: Prosty obiekt statystyk, aby uniknąć ręcznego liczenia w Engine.
#[derive(Default)]
pub struct MatchStats {
    pub m_size_matched: usize,
    pub x_size_mismatched: usize,
    pub total: usize,
    pub m_matched: ResultSet,
    pub x_mismatched: ResultSet,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShowMode {
    Include,
    Exclude,
    Context,
}

impl MatchStats {
    /// : Hermetyzacja renderowania po stronie rdzenia.
    /// Zwraca gotowy, złożony ciąg znaków, gotowy do wrzucenia w konsolę lub plik.
    #[must_use]
    pub fn render_output(&self, view_mode: ViewMode, show_mode: ShowMode, print_info: bool, use_color: bool) -> String {
        let mut out = String::new();
        let do_include = show_mode == ShowMode::Include || show_mode == ShowMode::Context;
        let do_exclude = show_mode == ShowMode::Exclude || show_mode == ShowMode::Context;

        match view_mode {
            ViewMode::Grid => {
                if do_include && let Some(grid) = &self.m_matched.grid {
                    if print_info { out.push_str("✅\n");}
                    if use_color { out.push_str( &grid.render_cli()); } else { out.push_str( &grid.render_txt()); }
                }
                if do_exclude && let Some(grid) = &self.x_mismatched.grid {
                    if print_info { out.push_str("❌\n");}
                    if use_color { out.push_str( &grid.render_cli()); } else { out.push_str( &grid.render_txt()); }
                }
            }
            ViewMode::Tree => {
                if do_include && let Some(tree) = &self.m_matched.tree {
                    if print_info { out.push_str("✅\n");}
                    if use_color { out.push_str( &tree.render_cli()); } else { out.push_str( &tree.render_txt()); }
                }
                if do_exclude && let Some(tree) = &self.x_mismatched.tree {
                    if print_info { out.push_str("❌\n");}
                    if use_color { out.push_str( &tree.render_cli()); } else { out.push_str( &tree.render_txt()); }
                }
            }
            ViewMode::List => {
                if do_include && let Some(list) = &self.m_matched.list {
                    if print_info { out.push_str("✅\n");}
                    if use_color { out.push_str( &list.render_cli(true)); } else { out.push_str( &list.render_txt()); }
                }
                if do_exclude && let Some(list) = &self.x_mismatched.list {
                    if print_info { out.push_str("❌\n");}
                    if use_color { out.push_str( &list.render_cli(false)); } else { out.push_str( &list.render_txt()); }
                }
            }
        }

        out
    }
}
