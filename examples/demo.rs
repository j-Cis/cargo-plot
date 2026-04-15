// ./examples/demo.rs

use plot::lib::logic::{
	DocEngine,
	DocEngineMultiple,
	IoConfig,
	MX,
	ScanSpec,
	TabPathStructure,
	TabSortBy,
	TabSortOrder,
	TabSpec,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("=== TEST API BEZPOŚREDNIEGO ===");

	// 1. Zamiast ręcznie wpisywać potężny wektor wzorców i ścieżek, korzystamy z
	//    wbudowanego, idealnie skrojonego pod Rusta Defaulta!
	// let scan_config = ScanSpec::default();
	// Jeśli chciałbyś inną ścieżkę, po prostu: ScanSpec::new("./src")

	// 2. Konfiguracja tabeli (bierzemy domyślne kolumny i tylko nadpisujemy
	//    sortowanie)
	// let table_config = TabSpec::default()
	//    .sort(TabSortBy::Name, TabSortOrder::Desc, TabPathStructure::List);

	// 3. Odpalenie głównego silnika
	// DocEngine::new(scan_config)
	//    .spec(table_config)
	//    .view(MX::M, false, false)
	//    .save_structure_of_the_content("./docs/raport-file", Some("file"))
	//    .view(MX::M, true, false)
	//    .save_content_of_the_structure("./docs/raport-docs", Some("docs"));

	// println!("\n=== TEST API KONFIGURACYJNEGO (TOML) ===");

	// 4. Inicjalizacja pliku TOML za pomocą nowej, bezpiecznej struktury IoConfig
	// Tworzy ./.x-do.toml, jeśli jeszcze go nie ma (i ładuje do niego domyślne
	// JobSpec)
	IoConfig::default_config_init_if_missing(IoConfig::DEFAULT_PATH)?;

	// 5. Wczytywanie konfiguracji i wykonanie zadań
	let orchestrator = DocEngineMultiple::loader_default()?;
	println!("{}", orchestrator.manifest);
	// Jeśli chcesz odpalić konkretne zadanie (pamiętaj, w Default nazywa się
	// "default_job", a nie "p1"!): orchestrator.job_id("default_job")?;

	// A najprościej odpalić po prostu wszystkie zadania z pliku z automatu:
	orchestrator.jobs()?;

	Ok(())
}
