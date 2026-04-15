use crate::lib::logic::{
	AnchoredPathsDatum,
	DocMarkdown,
	PartitioningResult,
	PatternsToApply,
	ScanSpec,
	ScannedToApply,
	TabSpec,
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
	pub path: AnchoredPathsDatum,
	pub tagtime: TagTime,
	pub result: PartitioningResult,
	pub last_render: Option<RenderFlags>,
}

impl DocEngine {
	pub fn new(scan: ScanSpec) -> Self {
		let anchored_paths_datum = AnchoredPathsDatum::new(&scan.work_path).unwrap_or_else(|x| {
			eprintln!("❌ {}", x);
			std::process::exit(1);
		});

		let scanned_to_apply = ScannedToApply::scan(&anchored_paths_datum);
		let ref_patterns: Vec<&str> = scan.patterns.iter().map(|s| s.as_str()).collect();
		let patterns_to_apply = PatternsToApply::new(ref_patterns, scan.ignore_case);

		// let mut tab = PartitioningResult::new(scanned_to_apply, patterns_to_apply)
		//    .sort(spec.sort_by, spec.sort_order, spec.structure)
		//    .columns(&spec.columns);
		// tab.spec = spec;
		let partitioning_result = PartitioningResult::new(scanned_to_apply, patterns_to_apply);

		Self { path: anchored_paths_datum, tagtime: tag_time(), result: partitioning_result, last_render: None }
	}

	pub fn spec(mut self, spec: TabSpec) -> Self {
		self.result.spec = spec;
		self
	}

	// ============================================================================
	// Wewnętrzny silnik generujący
	// ============================================================================

	fn section_header(&self, border: String) -> String {
		let pats = format!("{:?}", self.result.patterns.patterns.0);
		let m_len = self.result.m.paths.len();
		let x_len = self.result.x.paths.len();

		let stat = &self.result.scanner.stat;
		let dir_path = format!("\"{}\"", stat.relation.workspace_dir.buf.display());

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

	/// REZULTAT PRZYCIĘTY/POCIĘTY (Limit / Paginacja)
	pub fn view_trimmed(
		mut self,
		tab_mode: MX,
		hide_stats: bool,
		hide_promo: bool,
		size: usize,
		page: Option<usize>, // Jeśli None, działa jak dawny Limit (strona 1)
	) -> Self {
		self.result.spec = self.result.spec.trim(size, page);
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
			self.path.execution_dir.clone(),
			self.path.workspace_dir.clone(),
			self.tagtime.clone(),
		)
	}

	// ============================================================================
	// ZAPIS DO PLIKU
	// ============================================================================

	pub fn save_structure_of_the_content(self, rel_path: &str, title: Option<&str>) -> Self {
		let flags = self.current_flags();
		let table_output = self.generate_table(flags.mode);
		let raw_out_str = self.build_structure_of_the_content(&flags);
		let md = self.init_markdown(raw_out_str, table_output);

		if let Err(e) = md.structure_of_the_content_save_as(rel_path, title) {
			eprintln!("❌ Błąd zapisu SOTC (Struktura Zawartości): {}", e);
		}
		self
	}

	pub fn save_content_of_the_structure(self, rel_path: &str, title: Option<&str>) -> Self {
		let flags = self.current_flags();
		let table_output = self.generate_table(flags.mode);
		let raw_out_str = self.build_structure_of_the_content(&flags);
		let md = self.init_markdown(raw_out_str, table_output);

		if let Err(e) = md.content_of_the_structure_save_as(rel_path, title) {
			eprintln!("❌ Błąd zapisu COTS (Zawartość Struktury): {}", e);
		}
		self
	}
}
