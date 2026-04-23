use std::path::PathBuf;

use crate::lib::job::{self};
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// PRZETWARZANIE FALAG NA PARAMETRY
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

pub trait TraitParseFromFlags: Sized {
    /// Ostateczny typ konfiguracji, który zostanie zwrócony do silnika
    type Params;
    
    /// Typ roboczy (zbiorczy) używany podczas procesowania
    type Partial;

    /// Parsuje pojedynczego stringa na wariant Enuma.
    fn parse(s: &str) -> Result<Self, String>;

    /// ZBIERANIE: Odczyt surowych danych i liczenie wystąpień (szukanie sprzeczności)
    fn gather_inputs<I>(inputs: I) -> Self::Partial
    where
        I: IntoIterator,
        I::Item: AsRef<str>;

    /// STRATEGIA: Rozwiązywanie konfliktów i nadawanie priorytetów (czerwony romb)
    fn resolve_conflicts(partial: &mut Self::Partial);

    /// DOMYŚLNE: Ładowanie predefiniowanych wartości, jeśli użytkownik nic nie podał
    fn apply_defaults(partial: &mut Self::Partial);

    /// BUDOWA: Przepisanie ułożonych danych ze struktury Partial do finalnego Params
    fn build_params(partial: Self::Partial) -> Self::Params;

    /// ⚡ ORCHESTRATOR: Metoda wywoływana z zewnątrz, spina proces w całość
    fn parse_vec_as_params<I>(inputs: I) -> Self::Params
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut partial = Self::gather_inputs(inputs);
        Self::resolve_conflicts(&mut partial);
        Self::apply_defaults(&mut partial);
        Self::build_params(partial)
    }
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// EXECUTION
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

impl TraitParseFromFlags for job::schema::ValidExecutionFlags {
    type Params = job::schema::ValidExecutionParams;
    type Partial = job::schema::PartialExecutionRead;

    // --- ETAP 0: PARSOWANIE ---
    fn parse(s: &str) -> Result<Self, String> {
        let lower = s.trim().to_lowercase();
        match lower.as_str() {
            "dry" | "dry-run" => Ok(Self::DryRun(true)),
            "save" => Ok(Self::Save(true)),
            "save-ins" | "save-inspection" => Ok(Self::SaveWithInspection(true)),
            
            "warn" | "warning" | "quiet" | "q" => Ok(Self::PrintOnlyWarning(true)),
            "mute" | "silent" | "none" => Ok(Self::PrintNothing(true)),
            "color" | "colors" => Ok(Self::PrintInColor(true)),
            "debug" | "verbose" | "v" | "print-ins" => Ok(Self::PrintWithInspection(true)),
            
            _ => Err(format!("Nieznana flaga egzekucji: '{}'", s)),
        }
    }

    // --- ETAP 1: ZBIERANIE I WYKRYWANIE SPRZECZNOŚCI ---
    fn gather_inputs<I>(inputs: I) -> Self::Partial
    where
        I: IntoIterator,
        I::Item: AsRef<str>, 
    {
        let mut p = job::schema::PartialExecutionRead { 
            dry_run: false, save: false, save_with_inspection: false, 
            print_only_warning: false, print_nothing: false, print_in_color: false, 
            print_with_inspection: false, has_any: false, has_conflict: false 
        };
        
        for item in inputs {
            if let Ok(parsed) = Self::parse(item.as_ref()) {
                p.has_any = true; 
                match parsed {
                    Self::DryRun(_) => p.dry_run = true,
                    Self::Save(_) => p.save = true,
                    Self::SaveWithInspection(_) => p.save_with_inspection = true,
                    Self::PrintOnlyWarning(_) => p.print_only_warning = true,
                    Self::PrintNothing(_) => p.print_nothing = true,
                    Self::PrintInColor(_) => p.print_in_color = true,
                    Self::PrintWithInspection(_) => p.print_with_inspection = true,
                }
            }
        }

        // 🟥 DETEKCJA SPRZECZNOŚCI
        let conflict_in_save = p.dry_run && (p.save || p.save_with_inspection);
        let conflict_in_print = p.print_nothing && (p.print_only_warning || p.print_in_color || p.print_with_inspection);

        if conflict_in_save || conflict_in_print {
            p.has_conflict = true;
        }

        p
    }

    // --- ETAP 2: STRATEGIA PRIORYTETÓW ---
    fn resolve_conflicts(p: &mut Self::Partial) {
        if p.has_conflict {
            #[cfg(debug_assertions)]
            eprintln!("⚠️ Wykryto sprzeczne flagi sterujące. Stosuję twarde reguły priorytetów (DryRun / PrintNothing).");
        }

        // 🟨 ROZWIĄZYWANIE SPRZECZNOŚCI
        
        // 1. Priorytet `dry_run` nad zapisem
        if p.dry_run {
            p.save = false;
            p.save_with_inspection = false;
        }

        // 2. Priorytet `print_nothing` nad resztą wyświetlania
        if p.print_nothing {
            p.print_only_warning = false;
            p.print_in_color = false;
            p.print_with_inspection = false;
        }
    }

    // --- ETAP 3: WYPEŁNIANIE DOMYŚLNYMI ---
    fn apply_defaults(p: &mut Self::Partial) {
        // 🟨 UZUPEŁNIANIE BRAKUJĄCYCH Z DOMYŚLNYCH
        
        // Zapis (Domyślnie: true)
        // Dajemy true, jeśli użytkownik nie określił żadnego wariantu zapisu 
        // i nie zablokował go podając `dry_run`.
        if !p.save && !p.save_with_inspection && !p.dry_run {
            p.save = true;
        }

        // Wyświetlanie (Domyślnie: print_only_warning = true)
        // Dajemy true, jeśli użytkownik nie określił żadnego trybu widoczności.
        if !p.print_only_warning && !p.print_nothing && !p.print_with_inspection {
            p.print_only_warning = true;
        }
    }

    // --- ETAP 4: BUDOWA OSTATECZNYCH PARAMETRÓW ---
    fn build_params(p: Self::Partial) -> Self::Params {
        job::schema::ValidExecutionParams {
            dry_run: p.dry_run,
            save: p.save,
            save_with_inspection: p.save_with_inspection,
            print_only_warning: p.print_only_warning,
            print_nothing: p.print_nothing,
            print_in_color: p.print_in_color,
            print_with_inspection: p.print_with_inspection,
        }
    }
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// WORKSPACE DIR
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

impl TraitParseFromFlags for job::schema::ValidWorkspaceFlags {
    type Params = job::schema::ValidWorkspaceParams;
    type Partial = job::schema::PartialWorkspaceRead;

    // --- ETAP 0: PARSOWANIE ---
    fn parse(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();
        if trimmed.to_lowercase() == "ignore-case" {
            Ok(Self::IgnoreCase(true))
        } else if !trimmed.is_empty() {
            Ok(Self::Path(trimmed.to_string()))
        } else {
            Err("Pusta flaga workspace".to_string())
        }
    }

    // --- ETAP 1: ZBIERANIE ---
    fn gather_inputs<I>(inputs: I) -> Self::Partial
    where
        I: IntoIterator,
        I::Item: AsRef<str>, 
    {
        let mut p = job::schema::PartialWorkspaceRead { 
            path: None, 
            ignore_case: false, 
            has_any: false, 
            has_conflict: false // Tu nie ma fizycznej możliwości wystąpienia konfliktu ścieżek
        };
        
        for item in inputs {
            if let Ok(parsed) = Self::parse(item.as_ref()) {
                p.has_any = true; 
                match parsed {
                    Self::Path(path_str) => p.path = Some(path_str),
                    Self::IgnoreCase(_) => p.ignore_case = true,
                }
            }
        }

        p
    }

    // --- ETAP 2: STRATEGIA PRIORYTETÓW ---
    fn resolve_conflicts(_p: &mut Self::Partial) {
        // Brak. Workspace to pojedynczy string i pojedynczy boolean. 
    }

    // --- ETAP 3: WYPEŁNIANIE DOMYŚLNYMI ---
    fn apply_defaults(p: &mut Self::Partial) {
        // Jeśli plik TOML był uszkodzony albo pominął zmienną, wstrzykujemy kropkę
        if p.path.is_none() {
            p.path = Some(".".to_string());
        }
    }

    // --- ETAP 4: BUDOWA OSTATECZNYCH PARAMETRÓW ---
    fn build_params(p: Self::Partial) -> Self::Params {
        let raw = p.path.unwrap();
        let w = std::path::Path::new(&raw);
        
        let execution_dir = std::env::current_dir().expect("Nie można odczytać katalogu roboczego (CWD)");
        
        let workspace_dir = std::fs::canonicalize(w).unwrap_or_else(|x| {
            eprintln!("❌ Błąd lokalizacji workspace '{}': {}", raw, x);
            std::process::exit(1);
        });

        job::schema::ValidWorkspaceParams {
            workspace_raw: raw,
            workspace_dir,
            execution_dir,
            ignore_case: p.ignore_case,
        }
    }
}

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// SAVE AS
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

impl TraitParseFromFlags for job::schema::ValidSaveAsFlags {
    type Params = job::schema::ValidSaveAsParams;
    type Partial = job::schema::PartialSaveAsRead;

    // --- ETAP 0: PARSOWANIE ---
    fn parse(s: &str) -> Result<Self, String> {
        let lower = s.trim().to_lowercase();

        if lower.starts_with("outdir=") || lower.starts_with("out_dir=") {
            let val = s.split_once('=').unwrap().1.trim();
            Ok(Self::OutDir(val.to_string()))
        } else if lower.starts_with("title=") {
            let val = s.split_once('=').unwrap().1.trim();
            Ok(Self::Title(val.to_string()))
        } else if lower.starts_with("name=") {
            let val = s.split_once('=').unwrap().1.trim();
            Ok(Self::Name(val.to_string()))
        } else if lower == "name-prefix" || lower == "name_prefix" {
            Ok(Self::NameIsPrefix(true))
        } else if lower == "not-separately" || lower == "not_separately" {
            Ok(Self::NotSeparately(true))
        } else {
            Err(format!("Nieznana flaga konfiguracji zapisu: '{}'", s.trim()))
        }
    }

    // --- ETAP 1: ZBIERANIE ---
    fn gather_inputs<I>(inputs: I) -> Self::Partial
    where
        I: IntoIterator,
        I::Item: AsRef<str>, 
    {
        let mut p = job::schema::PartialSaveAsRead {
            out_dir_str: None,
            title: None,
            name: None,
            name_is_prefix: None,
            not_separately: None,
            has_any: false,
            has_conflict: false,
        };

        for item in inputs {
            if let Ok(parsed) = Self::parse(item.as_ref()) {
                p.has_any = true;
                match parsed {
                    Self::OutDir(v) => p.out_dir_str = Some(v),
                    Self::Title(v) => p.title = Some(v),
                    Self::Name(v) => p.name = Some(v),
                    Self::NameIsPrefix(v) => p.name_is_prefix = Some(v),
                    Self::NotSeparately(v) => p.not_separately = Some(v),
                }
            }
        }
        p
    }

    // --- ETAP 2: STRATEGIA PRIORYTETÓW ---
    fn resolve_conflicts(_p: &mut Self::Partial) {
        // Brak. Wartości nadpisują się naturalnie (ostatnia lub z TOML jako klucz-wartość)
    }

    // --- ETAP 3: WYPEŁNIANIE DOMYŚLNYMI ---
    fn apply_defaults(p: &mut Self::Partial) {
        if p.out_dir_str.is_none() {
            p.out_dir_str = Some("./target/.cargo-plot/".to_string());
        }
        if p.title.is_none() {
            p.title = Some("Project Snapshot".to_string());
        }
        if p.name.is_none() {
            p.name = Some("".to_string());
        }
        if p.name_is_prefix.is_none() {
            p.name_is_prefix = Some(false); // Domyślnie NIE JEST to prefix (czyli zachowuje się jak zwykła nazwa)
        }
        if p.not_separately.is_none() {
            p.not_separately = Some(false);
        }
    }

    // --- ETAP 4: BUDOWA OSTATECZNYCH PARAMETRÓW ---
    fn build_params(p: Self::Partial) -> Self::Params {
        let out_raw = p.out_dir_str.unwrap();
        let out_dir = std::path::PathBuf::from(&out_raw);

        job::schema::ValidSaveAsParams {
            out_dir,
            out_raw,
            title: p.title.unwrap(),
            name: p.name.unwrap(),
            name_is_prefix: p.name_is_prefix.unwrap(),
            not_separately: p.not_separately.unwrap(),
        }
    }
}
impl TraitParseFromFlags for job::schema::ValidTablePartFlags {
	type Config = job::schema::ValidTablePartParams;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"MD" | "md" | "match-dirs" => Ok(job::schema::ValidTablePartFlags::MD),
			"MF" | "mf" | "match-files" => Ok(job::schema::ValidTablePartFlags::MF),
			"XD" | "xd" | "mismatch-dirs" => Ok(job::schema::ValidTablePartFlags::XD),
			"XF" | "xf" | "mismatch-files" => Ok(job::schema::ValidTablePartFlags::XF),
			_ => Err(format!("Nieprawidłowa nazwa: '{}'. Dostępne: md, mf, xd, xf", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let mut md = false;
		let mut mf = false;
		let mut xd = false;
		let mut xf = false;

		let mut has_any_valid = false;

		for item in inputs {
			// Cicho ignorujemy błędy parsowania (zgodnie z założeniem o niepoprawnych elementach)
			if let Ok(parsed) = Self::parse(item.as_ref()) {
				has_any_valid = true;
				match parsed {
					job::schema::ValidTablePartFlags::MD => md = true,
					job::schema::ValidTablePartFlags::MF => mf = true,
					job::schema::ValidTablePartFlags::XD => xd = true,
					job::schema::ValidTablePartFlags::XF => xf = true,
				}
			}
		}

		// Jeśli tablica była pusta lub wszystko było śmieciami, ładujemy domyślne ustawienia
		if !has_any_valid {
			md = true;
			mf = true;
		}

		job::schema::ValidTablePartParams { md, mf, xd, xf }
	}
}
impl TraitParseFromFlags for job::schema::ValidColumnItemFlags {
	type Config = job::schema::ValidColumnItemParams;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"list-none" => Ok(job::schema::ValidColumnItemFlags::ListNone),
			"list-flat" => Ok(job::schema::ValidColumnItemFlags::ListFlat),
			"list-tree" => Ok(job::schema::ValidColumnItemFlags::ListTree),

			"icons-lite" => Ok(job::schema::ValidColumnItemFlags::IconsLite),
			"icons-more" => Ok(job::schema::ValidColumnItemFlags::IconsMore),
			"icons-none" => Ok(job::schema::ValidColumnItemFlags::IconsNone),

			"num-prefix" => Ok(job::schema::ValidColumnItemFlags::NumPrefix),
			"num-suffix" => Ok(job::schema::ValidColumnItemFlags::NumSuffix),

			"name-none" => Ok(job::schema::ValidColumnItemFlags::NameNone),
			"name-show" => Ok(job::schema::ValidColumnItemFlags::NameShow),

			"ws-none" | "whitespace-none" => Ok(job::schema::ValidColumnItemFlags::WhitespaceTrailNone),
			"ws-show" | "whitespace-show" => Ok(job::schema::ValidColumnItemFlags::WhitespaceTrailShow),

			_ => Err(format!("Nieznany job::schema::ValidColumnItemFlags: '{}'", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let items: Vec<job::schema::ValidColumnItemFlags> =
			inputs.into_iter().filter_map(|s| Self::parse(s.as_ref()).ok()).collect();

		job::schema::ValidColumnItemParams::get(items)
	}
}
impl TraitParseFromFlags for job::schema::ValidTableColumnsFlags {
	type Config = job::schema::ValidTableColumnsParams;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"date" => Ok(job::schema::ValidTableColumnsFlags::Date),
			"time" => Ok(job::schema::ValidTableColumnsFlags::Time),
			"size" => Ok(job::schema::ValidTableColumnsFlags::Size),
			"item" => Ok(job::schema::ValidTableColumnsFlags::Item),
			"path" => Ok(job::schema::ValidTableColumnsFlags::Path),
			_ => Err(format!("Nieprawidłowa nazwa kolumny: '{}'. Dostępne: date, time, size, item, path", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let mut columns = Vec::new();

		let mut has_date = false;
		let mut has_time = false;
		let mut has_size = false;
		let mut has_item = false;
		let mut has_path = false;

		let mut has_any_valid = false;

		for item in inputs {
			if let Ok(parsed) = Self::parse(item.as_ref()) {
				has_any_valid = true;
				match parsed {
					job::schema::ValidTableColumnsFlags::Date => {
						if !has_date {
							columns.push(job::schema::ValidTableColumnsFlags::Date);
							has_date = true;
						}
					}
					job::schema::ValidTableColumnsFlags::Time => {
						if !has_time {
							columns.push(job::schema::ValidTableColumnsFlags::Time);
							has_time = true;
						}
					}
					job::schema::ValidTableColumnsFlags::Size => {
						if !has_size {
							columns.push(job::schema::ValidTableColumnsFlags::Size);
							has_size = true;
						}
					}
					job::schema::ValidTableColumnsFlags::Item => {
						if !has_item {
							columns.push(job::schema::ValidTableColumnsFlags::Item);
							has_item = true;
						}
					}
					job::schema::ValidTableColumnsFlags::Path => {
						if !has_path {
							columns.push(job::schema::ValidTableColumnsFlags::Path);
							has_path = true;
						}
					}
				}
			}
		}

		// Jeśli wejście było puste lub same błędy -> wczytujemy konfigurację domyślną
		if !has_any_valid {
			return job::schema::ValidTableColumnsParams {
				columns: vec![
					job::schema::ValidTableColumnsFlags::Date,
					job::schema::ValidTableColumnsFlags::Time,
					job::schema::ValidTableColumnsFlags::Size,
					job::schema::ValidTableColumnsFlags::Item,
					job::schema::ValidTableColumnsFlags::Path,
				],
			};
		}

		// Jeśli podano cokolwiek poprawnego, ale brakuje 'Item', doklejamy na koniec
		if !has_item {
			columns.push(job::schema::ValidTableColumnsFlags::Item);
		}

		job::schema::ValidTableColumnsParams { columns }
	}
}
impl TraitParseFromFlags for job::schema::ValidColumnSizeFlags {
	type Config = job::schema::ValidColumnSizeParams;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"dec" | "decimal" => Ok(job::schema::ValidColumnSizeFlags::Decimal),
			"bin" | "binary" => Ok(job::schema::ValidColumnSizeFlags::Binary),
			_ => Err(format!("Nieznany system miar: '{}'. Dostępne: dec, bin", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let mut mode = job::schema::ValidColumnSizeFlags::Decimal; // Domyślnie Dec

		for item in inputs {
			if let Ok(parsed) = Self::parse(item.as_ref()) {
				mode = parsed;
				break; // Przyjmujemy pierwszy poprawny wpis
			}
		}

		job::schema::ValidColumnSizeParams { mode }
	}
}
impl TraitParseFromFlags for job::schema::ValidColumnDateFlags {
	type Config = job::schema::ValidColumnDateParams;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"date-default" | "default" => Ok(job::schema::ValidColumnDateFlags::Default),
			_ => Err(format!("Nieznany format daty: '{}'", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let format = "%Y W%V %u-%a".to_string(); // Domyślny format: Rok Tydzień Dzień(1-7) SkrótDnia

		for item in inputs {
			if let Ok(_) = Self::parse(item.as_ref()) {
				// Tutaj można dodać logikę wyboru formatu, jeśli pojawi się więcej wariantów w enum
				break;
			}
		}

		job::schema::ValidColumnDateParams { format }
	}
}
impl TraitParseFromFlags for job::schema::ValidColumnTimeFlags {
	type Config = job::schema::ValidColumnTimeParams;
	fn parse(s: &str) -> Result<Self, String> {
		match s.trim().to_lowercase().as_str() {
			"time-default" | "default" => Ok(job::schema::ValidColumnTimeFlags::Default),
			_ => Err(format!("Nieznany format czasu: '{}'", s.trim())),
		}
	}
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let format = "%H:%M:%S.%3f".to_string(); // Domyślny format: Godz:Min:Sek.Milisekundy

		for item in inputs {
			if let Ok(_) = Self::parse(item.as_ref()) {
				break;
			}
		}

		job::schema::ValidColumnTimeParams { format }
	}
}
impl TraitParseFromFlags for job::schema::ValidPatternFlags {
	type Config = job::schema::ValidPatternParams;
	fn parse(s: &str) -> Result<Self, String> { Err(format!("Typ nie wspiera parsowania pojedynczej flagi: '{}'", s)) }
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		let patterns: Vec<String> = inputs.into_iter().map(|s| s.as_ref().to_string()).collect();

		// Jeśli lista jest pusta po zebraniu, ładujemy domyślne
		if patterns.is_empty() {
			return job::schema::ValidPatternParams {
				patterns: vec![
					"./{.rustfmt,Cargo,rust-toolchain,Makefile}.toml&/".to_string(),
					"./**/*.rs&/".to_string(),
					"!./target/**".to_string(),
					"!./.git/**".to_string(),
					"./.{gitattributes,gitignore}".to_string(),
					"./.github/workflows/*.yml&/".to_string(),
					"./.vscode/settings.json&/".to_string(),
					"./{API,ARCHITECTURE,AUTHORS,CHANGELOG,README,ROADMAP,TODO}.md".to_string(),
					"./dist/{**/*,*}.{bat,exe}&/".to_string(),
				],
				//ignore_case: ignore_case_sensitive.unwrap_or(false),
			};
		}

		job::schema::ValidPatternParams { patterns }
	}
}

impl TraitParseFromFlags for job::schema::ValidSortByFlags {
	type Config = job::schema::ValidSortByParams;
	fn parse(s: &str) -> Result<Self, String> { Err(format!("Typ nie wspiera parsowania pojedynczej flagi: '{}'", s)) }
	fn parse_vec_as_config<I>(inputs: I) -> Self::Config
	where
		I: IntoIterator,
		I::Item: AsRef<str>, {
		// 1. Ustawienia domyślne
		let mut strategy_name = "name";
		let mut reverse = false;
		let mut dir_split = true;
		let mut file_group = job::schema::ModeFileGroupForValidSortBy::None;
		let mut mode_sort_name = "[Spec][Num][AZaz]".to_string();

		// 2. Płaskie parsowanie CLI/TOML (bez wchodzenia w detale algorytmów)
		for item in inputs {
			let s = item.as_ref().trim();
			let lower = s.to_lowercase();

			match lower.as_str() {
				"date" => strategy_name = "date",
				"size" => strategy_name = "size",
				"name" => strategy_name = "name",
				"path" => strategy_name = "path",

				"rev" | "reverse" => reverse = true,

				"group-name" => file_group = job::schema::ModeFileGroupForValidSortBy::Name,
				"group-exte" => file_group = job::schema::ModeFileGroupForValidSortBy::Exte,
				"group-none" => file_group = job::schema::ModeFileGroupForValidSortBy::None,

				"dir-split-true" => dir_split = true,
				"dir-split-false" => dir_split = false,

				_ => {
					// Czyste przekazanie definicji maski (np. "[Num][Spec][azAZ]") do logiki
					if s.starts_with('[') && s.ends_with(']') {
						mode_sort_name = s.to_string();
					}
				}
			}
		}

		// 3. Budowa wariantu konfiguracji
		let strategy = match strategy_name {
			"date" => job::schema::StrategyForValidSortBy::Date { reverse },
			"size" => job::schema::StrategyForValidSortBy::Size { reverse },
			"path" => {
				job::schema::StrategyForValidSortBy::Path { mode: mode_sort_name, reverse, dir_split, file_group }
			}
			_ => job::schema::StrategyForValidSortBy::Name { mode: mode_sort_name, reverse, file_group },
		};

		job::schema::ValidSortByParams { strategy }
	}
}
