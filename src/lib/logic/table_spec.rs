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
	pub is_tree: bool,
	pub columns: Vec<TabColumn>,
	pub limit: Option<usize>,
	pub page: Option<usize>,
	pub page_size: Option<usize>,
	pub extended_icons: bool,
}

impl Default for TableSpec {
	fn default() -> Self {
		Self {
			sort_by: TabSortBy::Name,
			sort_order: TabSortOrder::Asc,
			is_tree: false,
			columns: vec![],
			limit: None,
			page: None,
			page_size: None,
			extended_icons: false,
		}
	}
}

impl TableSpec {
	pub fn sort(mut self, by: TabSortBy, order: TabSortOrder, is_tree: bool) -> Self {
		self.sort_by = by;
		self.sort_order = order;
		self.is_tree = is_tree;
		self
	}

	pub fn columns(mut self, cols: &[TabColumn]) -> Self {
		self.columns = cols.to_vec();
		self
	}

	pub fn limit(mut self, n: usize) -> Self {
		self.limit = Some(n);
		self
	}

	pub fn paginate(mut self, page: usize, size: usize) -> Self {
		self.page = Some(page);
		self.page_size = Some(size);
		self
	}

	pub fn extended_icons(mut self, enabled: bool) -> Self {
		self.extended_icons = enabled;
		self
	}
}
