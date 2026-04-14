use super::{
    ConfigJob, ConfigManifest, DocEngine, IoConfig, JobMode, JobSpec, ScanSpec, TabColumn,
    TabPathStructure, TabSortBy, TabSortOrder, TabSpec, MX, RenderFlags
};

/// Orchestrator zadań wsadowych.
/// Pobiera konfigurację z plików TOML, tłumaczy ją na silnie typowane obiekty JobSpec 
/// i zleca ich wykonanie do rdzennego DocEngine.
pub struct DocEngineMultiple {
    pub manifest: ConfigManifest,
}

impl DocEngineMultiple {
    // ============================================================================
    // LOADERY (Odczyt konfiguracji za pomocą IoConfig)
    // ============================================================================

    /// Wczytuje konfigurację z domyślnego pliku (np. ./.x-do.toml)
    pub fn loader_default() -> Result<Self, String> {
        let manifest = IoConfig::default_config_loader()?;
        Ok(Self { manifest })
    }

    /// Wczytuje konfigurację ze wskazanej ścieżki
    pub fn loader_from_toml(path: &str) -> Result<Self, String> {
        let manifest = IoConfig::config_loader_from(path)?;
        Ok(Self { manifest })
    }

    // ============================================================================
    // PARSERY (Tłumaczenie ConfigJob z TOML na nasz domenowy JobSpec)
    // ============================================================================

    /// Parsuje pojedynczy surowy obiekt konfiguracji TOML na silnie typowany JobSpec
    pub fn parse(&self, cfg: &ConfigJob) -> Result<JobSpec, String> {
        // 1. Budujemy specyfikację skanowania
        let scan = ScanSpec::new(&cfg.pattern.work_path) // ⚡ Używamy konstruktora!
            .patterns(cfg.pattern.patterns.iter().map(|s| s.as_str()).collect())
            .ignore_case(cfg.pattern.ignore_case);

        // 2. Dekodujemy kolumny
        let mut columns = Vec::new();
        for col_str in &cfg.layout.columns {
            columns.push(TabColumn::parse(col_str)?);
        }

        // 3. Dekodujemy sortowanie i tryb drzewa/listy
        let sort_by = TabSortBy::parse(&cfg.layout.sort)?;
        let sort_order = if cfg.layout.reverse { TabSortOrder::Desc } else { TabSortOrder::Asc };
        let structure = if cfg.layout.list_instead_tree {
            TabPathStructure::List
        } else {
            TabPathStructure::Tree
        };

        // 4. Budujemy specyfikację tabeli
        let mut table = TabSpec::default()
            .sort(sort_by, sort_order, structure)
            .columns(&columns);

        // 5. Dodajemy trimming jeśli jest zdefiniowany
        if let Some(trim) = &cfg.trimming {
            if let Some(size) = trim.page_size {
                table = table.trim(size, trim.show_page);
            }
        }

        // 6. Składamy pełny JobSpec z flagami z TOML
        let mut job_spec = JobSpec::new(&cfg.id)
            .scan(scan)
            .table(table)
            .mode(JobMode::parse(&cfg.pattern.mode)?)
            .quiet(cfg.render.quiet_work)
            .hide_stats(cfg.render.hide_stats)
            .hide_promo(cfg.render.hide_promo);

        // Uzupełniamy opcjonalne pola tekstowe i eksport
        job_spec.name = cfg.name.clone();
        job_spec.description = cfg.description.clone();

        if let Some(export) = &cfg.export {
            job_spec.save_sotc_at = export.save_sotc_at.clone();
            job_spec.title_sotc = export.title_sotc.clone();
            job_spec.save_cots_at = export.save_cots_at.clone();
            job_spec.title_cots = export.title_cots.clone();
        }

        Ok(job_spec)
    }

    /// Szuka zadania po ID i parsuje je do JobSpec (nie wykonując go)
    pub fn parse_for(&self, id: &str) -> Result<JobSpec, String> {
        let cfg_job = self.manifest.job.iter()
            .find(|j| j.id == id)
            .ok_or_else(|| format!("❌ Nie znaleziono zadania o ID: '{}'", id))?;
            
        self.parse(cfg_job)
    }

    // ============================================================================
    // EXECUTORY (Wykonywanie zadań za pomocą DocEngine)
    // ============================================================================

    /// Mapuje i wykonuje w pętli wszystkie zadania zdefiniowane w pliku TOML
    pub fn jobs(&self) -> Result<(), String> {
        if self.manifest.job.is_empty() {
            return Err("⚠️ Manifest nie zawiera żadnych zadań.".to_string());
        }

        for cfg_job in &self.manifest.job {
            let spec = self.parse(cfg_job)?;
            self.execute_spec(spec)?;
        }
        Ok(())
    }

    /// Wyszukuje zadanie po ID, mapuje na JobSpec i wykonuje je
    pub fn job_id(&self, id: &str) -> Result<(), String> {
        let spec = self.parse_for(id)?;
        self.execute_spec(spec)?;
        Ok(())
    }

    // ============================================================================
    // SILNIK WYKONAWCZY (Delegacja do DocEngine)
    // ============================================================================

    /// Bierze gotowy i bezpieczny JobSpec, a następnie karmi nim DocEngine
    fn execute_spec(&self, spec: JobSpec) -> Result<(), String> {
        if !spec.quiet_work {
            let name = spec.name.as_deref().unwrap_or("Brak nazwy");
            println!("🚀 Uruchamianie zadania: {} ({})", spec.id, name);
        }

        // 1. Przekładamy JobMode na stare MX dla silnika renderującego
        let mx_mode = match spec.mode {
            JobMode::Matched => MX::Matched,
            JobMode::Mismatched => MX::Mismatched,
        };

        // 2. Inicjujemy rdzewny DocEngine przekazując mu od razu obiekt ScanSpec i TabSpec
        let mut engine = DocEngine::new(spec.scan.clone())
            .spec(spec.table.clone());

        // 3. Widok w terminalu (lub cichy stan dla plików)
        if !spec.quiet_work {
            if let Some(size) = spec.table.trim_size {
                let page = Some(spec.table.trim_page);
                engine = engine.view_trimmed(mx_mode, spec.hide_stats, spec.hide_promo, size, page);
            } else {
                engine = engine.view(mx_mode, spec.hide_stats, spec.hide_promo);
            }
        } else {
            // Nawet w trybie cichym zapisujemy konfigurację renderowania na wypadek zapisu do pliku
            engine.last_render = Some(RenderFlags { 
                hide_stats: spec.hide_stats, 
                hide_promo: spec.hide_promo, 
                mode: mx_mode 
            });
        }

        // 4. Eksport do plików (niezależnie od trybu quiet)
        if let Some(sotc_path) = &spec.save_sotc_at {
            engine = engine.save_structure_of_the_content(sotc_path, spec.title_sotc.as_deref());
        }
        if let Some(cots_path) = &spec.save_cots_at {
            engine = engine.save_content_of_the_structure(cots_path, spec.title_cots.as_deref());
        }

		let _ = engine;

        if !spec.quiet_work {
            println!("✨ Ukończono zadanie: {}", spec.id);
        }

        Ok(())
    }
}