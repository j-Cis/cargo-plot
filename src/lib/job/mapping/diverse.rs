use chrono;

use crate::lib::{
	job::{self},
	logic::{self},
};

pub trait TraitConfigGetSetup {
	/// Typ zwracany przez metodę get (różny dla każdej struktury)
	type Output;
	/// Przekształca konfigurację w gotowy obiekt warstwy logicznej
	fn get(&self) -> Self::Output;
}
// impl TraitConfigGetSetup for ValidExecutionParams {}
// impl TraitConfigGetSetup for ValidSaveAsParams {}
// impl TraitConfigGetSetup for ValidTablePartParams {}
// impl TraitConfigGetSetup for ValidColumnItemParams {}
impl job::schema::ValidColumnItemParams {
	pub fn get(items: Vec<job::schema::ValidColumnItemFlags>) -> job::schema::ValidColumnItemParams {
		let mut list: Option<job::schema::ModeListForValidColumnItem> = None;
		let mut icons: Option<job::schema::ModeIconsForValidColumnItem> = None;
		let mut num_is_first: Option<bool> = None;
		let mut name: Option<bool> = None;
		let mut align_end: Option<bool> = None;

		let mut list_count = 0usize;
		let mut icons_count = 0usize;
		let mut num_count = 0usize;
		let mut name_count = 0usize;
		let mut ws_count = 0usize;

		for item in items {
			match item {
				// LIST
				job::schema::ValidColumnItemFlags::ListNone => {
					list = Some(job::schema::ModeListForValidColumnItem::None);
					list_count += 1;
				}
				job::schema::ValidColumnItemFlags::ListFlat => {
					list = Some(job::schema::ModeListForValidColumnItem::Flat);
					list_count += 1;
				}
				job::schema::ValidColumnItemFlags::ListTree => {
					list = Some(job::schema::ModeListForValidColumnItem::Tree);
					list_count += 1;
				}

				// NUM
				job::schema::ValidColumnItemFlags::NumPrefix => {
					num_is_first = Some(true);
					num_count += 1;
				}
				job::schema::ValidColumnItemFlags::NumSuffix => {
					num_is_first = Some(false);
					num_count += 1;
				}

				// ICONS
				job::schema::ValidColumnItemFlags::IconsLite => {
					icons = Some(job::schema::ModeIconsForValidColumnItem::Lite);
					icons_count += 1;
				}
				job::schema::ValidColumnItemFlags::IconsMore => {
					icons = Some(job::schema::ModeIconsForValidColumnItem::More);
					icons_count += 1;
				}
				job::schema::ValidColumnItemFlags::IconsNone => {
					icons = Some(job::schema::ModeIconsForValidColumnItem::None);
					icons_count += 1;
				}

				// NAME
				job::schema::ValidColumnItemFlags::NameNone => {
					name = Some(false);
					name_count += 1;
				}
				job::schema::ValidColumnItemFlags::NameShow => {
					name = Some(true);
					name_count += 1;
				}

				// WHITESPACE (ALIGN_END)
				job::schema::ValidColumnItemFlags::WhitespaceTrailNone => {
					align_end = Some(false);
					ws_count += 1;
				}
				job::schema::ValidColumnItemFlags::WhitespaceTrailShow => {
					align_end = Some(true);
					ws_count += 1;
				}
			}
		}

		Self {
			list: if list_count == 1 { list.unwrap() } else { job::schema::ModeListForValidColumnItem::Tree },

			icons: if icons_count == 1 { icons.unwrap() } else { job::schema::ModeIconsForValidColumnItem::Lite },

			num_is_first: if num_count == 1 { num_is_first.unwrap() } else { false },

			name: if name_count == 1 { name.unwrap() } else { false },

			align_end: if ws_count == 1 { align_end.unwrap() } else { false },
		}
	}
}
// impl TraitConfigGetSetup for ValidTableColumnsParams {}
// impl TraitConfigGetSetup for ValidColumnSizeParams {}
// impl TraitConfigGetSetup for ValidColumnDateParams {}
// impl TraitConfigGetSetup for ValidColumnTimeParams {}
// impl TraitConfigGetSetup for ValidPatternParams {}
impl job::schema::ValidPatternParams {
	/// Zwraca gotowy obiekt logic::PatternsQueries używany przez skaner.
	/// ⚡ UWAGA: Przyjmuje teraz ignore_case z zewnątrz (z Workspace)!
	pub fn get(&self, ignore_case: bool) -> logic::PatternsQueries {
		let patterns_ref: Vec<&str> = self.patterns.iter().map(|s| s.as_str()).collect();
		logic::PatternsQueries::new(patterns_ref, ignore_case)
	}
}
// impl TraitConfigGetSetup for ValidWorkspaceParams { }
impl TraitConfigGetSetup for job::schema::ValidSortByParams {
	type Output = logic::SortQueries;
	/// Zwraca gotowy obiekt z warstwy logic:: (np. logic::SortQueries),
	/// gdzie dopiero tam te proste stringi i bool'e zostaną przetworzone
	/// w potężny algorytm (wspomniane 24 warianty).
	fn get(&self) -> Self::Output { logic::SortQueries::new(self.clone()) }
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// POZOSTAŁE PRZYPDKI
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

impl job::schema::ValidColumnItemParams {
	pub fn format_item(
		&self,
		u: (usize, usize, usize, usize),
		row: &job::ValidResultMainRow,
		tab: &[job::ValidResultMainRow],
	) -> String {
		let (index, num_width, tier_max, name_len_max) = u;
		let mut parts = Vec::new();
		let num_str = format!("{:>width$}.", index + 1, width = num_width);

		// 1. Jeśli num_is_first = true, numer idzie na początek
		if self.num_is_first {
			parts.push(num_str.clone());
		}

		// 2. Struktura Listy
		if self.list != job::schema::ModeListForValidColumnItem::None {
			let l = job::view::item_list::draw_list(&self.list, index, tab, tier_max);
			parts.push(l);
		}

		// 3. Ikony
		if self.icons != job::schema::ModeIconsForValidColumnItem::None {
			let ic = job::view::draw_icon(&self.icons, &row.node.node, &row.name_with_ext);
			parts.push(ic.to_string());
		}

		// 4. Jeśli num_is_first = false, numer idzie po liście i ikonach (ale przed nazwą)
		if !self.num_is_first {
			parts.push(num_str);
		}

		// 5. Nazwa pliku
		if self.name {
			parts.push(row.name_with_ext.clone());
		}

		let mut base = parts.join(" ");

		// 6. Align End (Trailing space) - dodaje pusty string, co przy join(" ") da spację na końcu
		if self.align_end {
			let p_tier = (tier_max - row.node.tier) * 3;
			let p_name = name_len_max - row.node.name.chars().count();
			base.push_str(&" ".repeat(p_tier + p_name));
		}

		base
	}
}

impl job::schema::ValidColumnSizeParams {
	/// Metoda formatująca rozmiar w zależności od wybranego trybu (Dec/Bin)
	pub fn format_size(&self, bytes: u64) -> String {
		let base = match self.mode {
			job::schema::ValidColumnSizeFlags::Decimal => 1000.0,
			job::schema::ValidColumnSizeFlags::Binary => 1024.0,
		};

		let suffix = match self.mode {
			job::schema::ValidColumnSizeFlags::Decimal => ["B ", "kB", "MB", "GB"],
			job::schema::ValidColumnSizeFlags::Binary => ["B ", "KiB", "MiB", "GiB"], // Lub kB/MB/GB wg preferencji
		};

		let bytes_f = bytes as f64;

		if bytes_f < base {
			format!("{:>6} {}", bytes, suffix[0])
		} else if bytes_f < base.powi(2) {
			format!("{:>6.2} {}", bytes_f / base, suffix[1])
		} else if bytes_f < base.powi(3) {
			format!("{:>6.2} {}", bytes_f / base.powi(2), suffix[2])
		} else {
			format!("{:>6.2} {}", bytes_f / base.powi(3), suffix[3])
		}
	}
}

impl job::schema::ValidColumnDateParams {
	pub fn format_date(&self, dt: chrono::DateTime<chrono::Local>) -> String { dt.format(&self.format).to_string() }
}

impl job::schema::ValidColumnTimeParams {
	pub fn format_time(&self, dt: chrono::DateTime<chrono::Local>) -> String { dt.format(&self.format).to_string() }
}
