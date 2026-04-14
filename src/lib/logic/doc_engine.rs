use super::{
	TabColumn,
	DocMarkdown,
	PathCanonicalCtx,
	PathScan,
	PathsPatterns,
	ResultScanPatterns,
	TabSortBy,
	TabSortOrder,
	TableOutput,
	TagTime,
	tag_time,
};

#[derive(Clone, Copy)]
pub enum MX {
	M,
	X,
	Matched,
	Mismatched,
}

#[derive(Clone, Copy)]
pub struct RenderFlags {
	pub hide_stats: bool,
	pub hide_promo: bool,
	pub mode: MX,
}

/// Główny silnik spinający skanowanie, filtrowanie i konfigurację widoku.
pub struct DocEngine {
	pub path: PathCanonicalCtx,
	pub tagtime: TagTime,
	pub result: ResultScanPatterns,
	pub last_render: Option<RenderFlags>,
}

impl DocEngine {
	pub fn new(
		target_to_scan: &str,
		paths_patterns: Vec<&str>,
		ignore_case_sensitive: bool,
		view_row: (TabSortBy, TabSortOrder, bool),
		view_col: &[TabColumn],
	) -> Self {
		let dir = PathCanonicalCtx::new(target_to_scan).unwrap_or_else(|x| {
			eprintln!("❌ {}", x);
			std::process::exit(1);
		});

		let ctx = PathScan::scan(&dir);
		let cfg = PathsPatterns::new(paths_patterns, ignore_case_sensitive);
		let tab = ResultScanPatterns::new(ctx, cfg).sort(view_row.0, view_row.1, view_row.2).columns(view_col);

		Self { path: dir, tagtime: tag_time(), result: tab, last_render: None }
	}

	// ============================================================================
	// Wewnętrzny silnik generujący
	// ============================================================================

	fn section_header(&self, border: String) -> String {
		let pats = format!("{:?}", self.result.patterns.patterns.0);
		let m_len = self.result.m.paths.len();
		let x_len = self.result.x.paths.len();

		let stat = &self.result.scanner.stat;
		let dir_path = format!("\"{}\"", stat.relation.select_dir.buf.display());

		let mut header = String::new();
		header.push_str(&format!("{}\n", border));
		header.push_str(&format!(
			"📊 | 📝 {} | 📂 {} | ⭕ {} | ✔️  {} | ✖️  {} |\n",
			stat.count_files, stat.count_folder, stat.count_empty, m_len, x_len
		));
		header.push_str(&format!("🔎 {}\n", pats));
		header.push_str(&format!("🗃️  {}\n", dir_path));
		header.push_str(&border);
		header
	}

	fn section_footer(&self, border: String) -> String {
		let mut footer = String::new();
		footer.push_str("📚 | [Crates.io](https://crates.io/crates/cargo-plot) |\n");
		footer.push_str("📚 | [GitHub](https://github.com/j-Cis/cargo-plot/releases) |\n");
		// footer.push_str(&format!("Wersja: {}\n", self.version));
		footer.push_str(&border);
		footer
	}

	/// Składa w całość ostateczny ciąg znaków
	fn build_structure_of_the_content(&self, flags: &RenderFlags) -> String {
		let tab_out = match flags.mode {
			MX::M | MX::Matched => self.result.build_matched(),
			MX::X | MX::Mismatched => self.result.build_mismatched(),
		};

		let mut parts = Vec::with_capacity(3);

		if !flags.hide_stats {
			parts.push(self.section_header("░".repeat(80)));
			parts.push(format!("{}", tab_out));
			parts.push("░".repeat(80));
		} else {
			parts.push(format!("{}", tab_out));
		}

		if !flags.hide_promo {
			parts.push(self.section_footer("░".repeat(80)));
		}

		// Łączymy wszystko znakami nowej linii i dodajemy na końcu
		parts.join("\n") + "\n"
	}

	// ============================================================================
	// KONFIGURACJA WIDOKU (Builder API)
	// ============================================================================

	#[inline]
	fn finalize_view_structure_of_the_content(&mut self, flags: RenderFlags) {
		self.last_render = Some(flags);
		print!("{}", self.build_structure_of_the_content(&flags));
	}

	/// REZULTAT PEŁNY
	pub fn view(mut self, tab_mode: MX, hide_stats: bool, hide_promo: bool) -> Self {
		self.finalize_view_structure_of_the_content(RenderFlags { hide_stats, hide_promo, mode: tab_mode });
		self
	}

	/// REZULTAT OGRANICZONY PRZEZ LIMIT
	pub fn view_limit(mut self, tab_mode: MX, limit: usize, hide_stats: bool, hide_promo: bool) -> Self {
		self.result = self.result.limit(limit);
		self.finalize_view_structure_of_the_content(RenderFlags { hide_stats, hide_promo, mode: tab_mode });
		self
	}

	/// REZULTAT OGRANICZONY PRZEZ PAGINACJE
	pub fn view_pages(
		mut self,
		tab_mode: MX,
		page: usize,
		page_size: usize,
		hide_stats: bool,
		hide_promo: bool,
	) -> Self {
		self.result.spec = self.result.spec.paginate(page, page_size);
		self.finalize_view_structure_of_the_content(RenderFlags { hide_stats, hide_promo, mode: tab_mode });
		self
	}

	// ============================================================================
	// UTILS "DRY"
	// ============================================================================

	/// Pobiera aktualne flagi renderowania lub zwraca domyślne
	fn current_flags(&self) -> RenderFlags {
		self.last_render.unwrap_or(RenderFlags { hide_stats: false, hide_promo: false, mode: MX::M })
	}

	/// Generuje surowy obiekt TableOutput na podstawie trybu z flag
	fn generate_table(&self, mode: MX) -> TableOutput {
		match mode {
			MX::M | MX::Matched => self.result.build_matched(),
			MX::X | MX::Mismatched => self.result.build_mismatched(),
		}
	}

	/// Składa w jedną całość inicjalizację obiektu Markdown (redukcja powtórzeń
	/// w metodach save)
	fn init_markdown(&self, content: String, table: TableOutput) -> DocMarkdown {
		DocMarkdown::new(
			content,
			table,
			self.path.execut_dir.clone(),
			self.path.select_dir.clone(),
			self.tagtime.clone(),
		)
	}

	// ============================================================================
	// ZAPIS DO PLIKU
	// ============================================================================

	pub fn save_structure_of_the_content(self, rel_path: &str) -> Self {
		let flags = self.current_flags();
		let table_output = self.generate_table(flags.mode);
		let raw_out_str = self.build_structure_of_the_content(&flags);
		let md = self.init_markdown(raw_out_str, table_output);

		if let Err(e) = md.structure_of_the_content_save_as(rel_path) {
			eprintln!("❌ Błąd zapisu SOTC (Struktura Zawartości): {}", e);
		}
		self
	}

	pub fn save_content_of_the_structure(self, rel_path: &str) -> Self {
		let flags = self.current_flags();
		let table_output = self.generate_table(flags.mode);
		let raw_out_str = self.build_structure_of_the_content(&flags);
		let md = self.init_markdown(raw_out_str, table_output);

		if let Err(e) = md.content_of_the_structure_save_as(rel_path) {
			eprintln!("❌ Błąd zapisu COTS (Zawartość Struktury): {}", e);
		}
		self
	}
}
