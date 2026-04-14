#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabColumn {
	TreeList, // Symbole drzewa (  ├──• ) // Wcięcia wg głębokości ( │  │  ├──• )
	Number,   // Numeracja pozycji ( 1. )
	Icon,     // Ikona typu ( 📂 / 📝 )
	Size,     // Rozmiar w nawiasach ( [ 1.20 kB] )
	Date,     // Data: | 2026 W14 Sun |
	Time,     // Czas: | 11:08:06.298 PM |
	Path,     // Ścieżka pliku
}

#[derive(Debug, Clone, Copy)]
pub enum TabSortBy {
	Name,
	Size,
	Date,
	Kind,
	FileFirst,
	DirFirst,
	FileFirstMerge,
	DirFirstMerge,
}

#[derive(Debug, Clone, Copy)]
pub enum TabSortOrder {
	Asc,
	Desc,
}

/// Zero-cost konfiguracja widoku tabeli.
#[derive(Debug, Clone)]
pub struct TableSpec {
	pub sort_by: TabSortBy,
    pub sort_order: TabSortOrder,
    pub list_instead_tree: bool,
    pub columns: Vec<TabColumn>,
    pub trim_size: Option<usize>,
    pub trim_page: usize, // Domyślnie 1, działa tylko gdy trim_size jest Some
    pub more_icons: bool,
}

impl Default for TableSpec {
	fn default() -> Self {
		Self {
			sort_by: TabSortBy::Name,
            sort_order: TabSortOrder::Asc,
            list_instead_tree: false, // is_tre
            columns: vec![],
            trim_size: None,
            trim_page: 1,
            more_icons: false,
		}
	}
}

impl TableSpec {
	pub fn sort(mut self, by: TabSortBy, order: TabSortOrder, list_instead_tree: bool) -> Self {
		self.sort_by = by;
		self.sort_order = order;
		self.list_instead_tree = list_instead_tree;
		self
	}

	pub fn columns(mut self, cols: &[TabColumn]) -> Self {
		self.columns = cols.to_vec();
		self
	}

	pub fn trim(mut self, size: usize, page: Option<usize>) -> Self {
        self.trim_size = Some(size);
        if let Some(p) = page {
            self.trim_page = p;
        }
        self
    }

	pub fn more_icons(mut self, enabled: bool) -> Self {
        self.more_icons = enabled;
        self
    }
}



impl TabColumn {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "treelist" | "tree" | "list" => Ok(TabColumn::TreeList),
            "number" => Ok(TabColumn::Number),
            "icon" => Ok(TabColumn::Icon),
            "size" => Ok(TabColumn::Size),
            "date" => Ok(TabColumn::Date),
            "time" => Ok(TabColumn::Time),
            "path" => Ok(TabColumn::Path),
            _ => Err(format!("Nieprawidłowa kolumna: '{}'", s.trim())),
        }
    }
}

impl TabSortBy {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "name" => Ok(TabSortBy::Name),
            "size" => Ok(TabSortBy::Size),
            "date" => Ok(TabSortBy::Date),
            "kind" => Ok(TabSortBy::Kind),
            "file-first" => Ok(TabSortBy::FileFirst),
            "dir-first" => Ok(TabSortBy::DirFirst),
            "file-merge" => Ok(TabSortBy::FileFirstMerge),
            "dir-merge" => Ok(TabSortBy::DirFirstMerge),
            _ => Err(format!("Nieprawidłowe sortowanie: '{}'", s.trim())),
        }
    }
}