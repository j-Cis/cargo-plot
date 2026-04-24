use std::{
	collections::{BTreeSet, HashMap},
	fs,
	path::{Path, PathBuf},
};

use crate::lib::{
	core::{TOML_DEFAULT_MINIMAL, default_config_path, execution_dir},
	logic::{file_backup, file_ensure, file_save_safe_if_changed},
	schema::{RawTomlFileJobs, RawTomlJob},
};

/// Rezultat działania funkcji init
#[derive(Debug, Clone)]
pub struct AnchoredRuntime {
	execution_dir: PathBuf, // Ścieżka, z której uruchomiono program (CWD)
	config_path: PathBuf,
	jobs_store: Vec<RawTomlJob>, // SSoT: Jedyne źródło prawdy o wczytanych zadaniach
}

impl AnchoredRuntime {
	/// Konstruktor wewnętrzny (używany przez funkcje w tym module)
	pub(crate) fn new(execution_dir: PathBuf, config_path: PathBuf, jobs_store: Vec<RawTomlJob>) -> Self {
		Self { execution_dir, config_path, jobs_store }
	}

	/// Bezpieczny dostęp do ścieżki konfiguracji
	pub fn path_xdo(&self) -> &Path { &self.config_path }

	/// Bezpieczny dostęp do ścieżki cwd
	pub fn path_cwd(&self) -> &Path { &self.execution_dir }

	/// Zwraca ilość wczytanych zadań
	pub fn count_jobs(&self) -> usize { self.jobs_store.len() }

	/// Zwraca indeks w formie HashMapy (ID -> Description) do szybkiego wyszukiwania
	pub fn get_jobs_index(&self) -> HashMap<String, String> {
		self.jobs_store.iter().map(|job| (job.id.clone(), job.description.clone().unwrap_or_default())).collect()
	}

	/// Wykonuje podany callback dla każdego zadania w strukturze.
	/// Pętla `for` jest całkowicie ukryta przed użytkownikiem (Inwersja Sterowania).
	pub fn get_jobs<F>(&self, mut f: F)
	where F: FnMut(&RawTomlJob) {
		for job in &self.jobs_store {
			f(job);
		}
	}

	pub fn get_jobs_some<F>(&self, ids: &BTreeSet<String>, mut f: F)
	where F: FnMut(&RawTomlJob) {
		self.jobs_store.iter().filter(|job| ids.contains(&job.id)).for_each(|job| f(job));
	}

	/// Pobiera konkretne zadanie na podstawie ID.
	/// Zwraca Option<&RawTomlJob>, co pozwala obsłużyć przypadek braku zadania.
	pub fn get_job(&self, id: &str) -> Option<&RawTomlJob> { self.jobs_store.iter().find(|job| job.id == id) }
}

// Struktura pomocnicza używana WYŁĄCZNIE do bezpiecznego parsowania z "wycinaniem",
// aby nie utracić reszty konfiguracji podczas usuwania duplikatów
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct InitTomlJobs {
	job: Vec<InitTomlJob>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
struct InitTomlJob {
	id: String,
	// Przechowujemy całą resztę kluczy bez ich analizowania, żeby przy zapisie nic nie zginęło
	#[serde(flatten)]
	extra: HashMap<String, toml::Value>,
}

/// Funkcja inicjująca potok "od zera".
/// Ignoruje ewentualne błędy i stan obecnego pliku - jeśli plik istnieje,
/// ZAWSZE tworzy jego kopię zapasową (.bak) i nadpisuje go minimalną konfiguracją.
pub fn start_blank(path_toml: Option<PathBuf>) -> AnchoredRuntime {
	let config_path = path_toml.unwrap_or_else(default_config_path);

	// Cała logika "skopiuj jeśli istnieje i nadpisz domyślnym"
	// jest zamknięta w naszej bezpiecznej funkcji narzędziowej!
	file_save_safe_if_changed(&config_path, TOML_DEFAULT_MINIMAL);

	build_normalized_result(&config_path)
}

/// Główna funkcja orkiestrująca (Różdżkarz)
pub fn start(path_toml: Option<PathBuf>) -> AnchoredRuntime {
	let config_path = path_toml.unwrap_or_else(default_config_path);

	if !config_path.exists() {
		file_ensure(&config_path, TOML_DEFAULT_MINIMAL);
		return build_normalized_result(&config_path);
	}

	let content = match fs::read_to_string(&config_path) {
		Ok(c) => c,
		Err(_) => {
			handle_corrupted(&config_path);
			return build_normalized_result(&config_path);
		}
	};

	let mut parsed: InitTomlJobs = match toml::from_str(&content) {
		Ok(p) => p,
		Err(_) => {
			handle_corrupted(&config_path);
			return build_normalized_result(&config_path);
		}
	};

	let mut id_counts = HashMap::new();
	for job in &parsed.job {
		*id_counts.entry(job.id.clone()).or_insert(0) += 1;
	}

	let has_any_unique_id = id_counts.values().any(|&count| count == 1);
	let has_duplicates = id_counts.values().any(|&count| count > 1);

	if !has_any_unique_id || parsed.job.is_empty() {
		handle_corrupted(&config_path);
		return build_normalized_result(&config_path);
	}

	if has_duplicates {
		file_backup(&config_path);

		parsed.job.retain(|job| id_counts[&job.id] == 1);

		let fixed_toml = toml::to_string(&parsed).expect("Błąd serializacji naprawionego TOML");
		fs::write(&config_path, fixed_toml).expect("Błąd zapisu naprawionego TOML");
	}

	build_normalized_result(&config_path)
}

fn handle_corrupted(path: &Path) {
	file_backup(path);
	file_ensure(path, TOML_DEFAULT_MINIMAL);
}

fn build_normalized_result(path: &Path) -> AnchoredRuntime {
	let content = fs::read_to_string(path).expect("Krytyczny błąd odczytu znormalizowanego pliku");

	// Ładujemy to już w nasze pełne, twarde modele domenowe (RawTomlFileJobs)
	let parsed: RawTomlFileJobs = toml::from_str(&content).expect("Krytyczny błąd parsowania znormalizowanego pliku");

	AnchoredRuntime::new(execution_dir(), path.to_path_buf(), parsed.job.0)
}
