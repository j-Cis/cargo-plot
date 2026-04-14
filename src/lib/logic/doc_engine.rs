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
    //config_io,
    //tag_time,
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
			self.path.execut_dir.clone(),
			self.path.select_dir.clone(),
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

/*/
// ============================================================================
// DOC ENGINE MULTIPLE (ORCHESTRATOR ZADAŃ TOML)
// ============================================================================

/// Wzorzec Buildera dla bezpiecznego ładowania konfiguracji
pub struct ConfigLoader {
    path: String,
}

impl ConfigLoader {
    pub fn if_not_exist_create_default(self) -> Result<DocEngineMultiple, String> {
        config_io::create_default_if_missing(&self.path)?;
        let manifest = config_io::load_manifest(&self.path)?;

        Ok(DocEngineMultiple { file_path: self.path, manifest })
    }
}

pub struct DocEngineMultiple {
    pub file_path: String,
    pub manifest: ConfigManifest,
}

impl DocEngineMultiple {
    /// Inicjuje proces ładowania konfiguracji z pliku
    pub fn get_config_from(path: &str) -> ConfigLoader { ConfigLoader { path: path.to_string() } }
    pub fn get_config_from_default() -> ConfigLoader { ConfigLoader { path: "./.x-do.toml".to_string() } }
    /// Zwraca sformatowaną listę dostępnych zadań (jobs)
    pub fn jobs_list(&self) -> String {
        if self.manifest.job.is_empty() {
            return format!("⚠️ Brak zadań w pliku '{}'", self.file_path);
        }

        let mut out = String::new();
        out.push_str(&format!("📜 DOSTĘPNE ZADANIA W: {}\n", self.file_path));
        for job in &self.manifest.job {
            let name = job.name.as_deref().unwrap_or("Brak nazwy");
            let desc = job.description.as_deref().unwrap_or("Brak opisu");
            out.push_str(&format!("🔹 [{}] - {} - {}\n", job.id, name, desc));
        }
        out
    }

    /// Wykonuje wszystkie zadania i zwraca Self dla dalszego chainowania
    pub fn do_jobs(self) -> Result<Self, String> {
        if self.manifest.job.is_empty() {
            return Err(format!("Plik '{}' nie zawiera zadań.", self.file_path));
        }
        for job in &self.manifest.job {
            self.execute_single(job)?;
        }
        Ok(self)
    }

    /// Wykonuje jedno konkretne zadanie o podanym ID
    /// Wykonuje konkretne zadanie i zwraca Self
    pub fn do_job(self, id: &str) -> Result<Self, String> {
        let job = self
            .manifest
            .job
            .iter()
            .find(|j| j.id == id)
            .ok_or_else(|| format!("❌ Nie znaleziono zadania o ID: '{}'", id))?;

        self.execute_single(job)?;
        Ok(self)
    }

    /// Wewnętrzny silnik mapujący ConfigJob na pojedynczy DocEngine
    fn execute_single(&self, job: &ConfigJob) -> Result<(), String> {
        if !job.render.quiet_work {
            let name = job.name.as_deref().unwrap_or("Brak nazwy");
            println!("🚀 Uruchamianie zadania: {} ({})", job.id, name);
        }

        // Dekodowanie kolumn
        let mut columns = Vec::new();
        for col_str in &job.layout.columns {
            columns.push(TabColumn::parse(col_str)?);
        }

        // Dekodowanie sortowania
        let sort_by = TabSortBy::parse(&job.layout.sort)?;
        let sort_order = if job.layout.reverse { TabSortOrder::Desc } else { TabSortOrder::Asc };

        // Tryb pracy
        let mode = match job.pattern.mode.to_lowercase().as_str() {
            "m" | "matched" => MX::Matched,
            "x" | "mismatched" => MX::Mismatched,
            _ => return Err(format!("Nieznany tryb mode: '{}'", job.pattern.mode)),
        };

        // Tworzenie głównego silnika
        let mut engine = DocEngine::new(
            &job.pattern.work_path,
            job.pattern.patterns.iter().map(AsRef::as_ref).collect(),
            job.pattern.ignore_case,
            (sort_by, sort_order, job.layout.list_instead_tree),
            &columns,
        );
        if !job.render.quiet_work {
            // Jeśli NIE JEST quiet -> normalnie renderujemy do terminala
            if let Some(trim) = &job.trimming {
                if let Some(size) = trim.page_size {
                    engine =
                        engine.view_trimmed(mode, job.render.hide_stats, job.render.hide_promo, size, trim.show_page);
                } else {
                    engine = engine.view(mode, job.render.hide_stats, job.render.hide_promo);
                }
            } else {
                engine = engine.view(mode, job.render.hide_stats, job.render.hide_promo);
            }
        } else {
            // Jeśli JEST quiet -> tylko ustawiamy stan silnika pod ewentualny zapis pliku
            // Omijamy metody .view(), żeby nie odpalić print!()
            if let Some(trim) = &job.trimming
                && let Some(size) = trim.page_size
            {
                engine.result.spec = engine.result.spec.trim(size, trim.show_page);
            }
            engine.last_render =
                Some(RenderFlags { hide_stats: job.render.hide_stats, hide_promo: job.render.hide_promo, mode });
        }

        // Eksport (zostaje bez zmian, bo save_... nie mają printa na stdout tabeli)
        if let Some(export) = &job.export {
            if let Some(sotc_path) = &export.save_sotc_at {
                engine = engine.save_structure_of_the_content(sotc_path, export.title_sotc.as_deref());
            }
            if let Some(cots_path) = &export.save_cots_at {
                engine = engine.save_content_of_the_structure(cots_path, export.title_cots.as_deref());
            }
        }
        let _ = engine;
        if !job.render.quiet_work {
            println!("✨ Ukończono zadanie: {}", job.id);
        }

        Ok(())
    }
}

*/