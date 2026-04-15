// examples/whitespace-cleaner.rs

use std::{
	fs,
	io::{self, Read},
	path::Path,
	process::Command,
};

use memchr::memchr;
use walkdir::WalkDir;
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let targets = ["./src", "./examples"];

	// 1. Najpierw usuwamy "złe" spacje, przez które fmt wywala błędy
	for target in targets {
		WhitespaceCleaner::clean_project(target)?;
	}

	// 2. Teraz, gdy pliki są "czyste", puszczamy oficjalny formatter
	WhitespaceCleaner::run_cargo_fmt()?;

	Ok(())
}

pub struct WhitespaceCleaner;

impl WhitespaceCleaner {
	/// Skanuje i czyści białe znaki (istniejąca metoda)
	pub fn clean_project<P: AsRef<Path>>(dir: P) -> io::Result<()> {
		let mut cleaned_files = 0;

		for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
			let path = entry.path();

			// Interesują nas tylko pliki Rustowe
			if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") && Self::clean_file(path)? {
				cleaned_files += 1;
			}
		}

		if cleaned_files > 0 {
			println!("🧹 Pomyślnie wyczyszczono białe znaki w {} plikach.", cleaned_files);
		} else {
			println!("✨ Twój kod jest czysty. Żadne pliki nie wymagały czyszczenia.");
		}

		Ok(())
	}

	pub fn run_cargo_fmt() -> io::Result<()> {
		println!("formatting...");

		let status =
			Command::new("cargo").args(["+nightly", "fmt", "--", "--config-path", "./.rustfmt.toml"]).status()?; // Czekamy na zakończenie procesu

		if status.success() {
			println!("✅ formatowanie zakończone pomyślnie.");
		} else {
			eprintln!("❌ formatowanie zwróciło błąd (prawdopodobnie błędy składni w kodzie).");
		}

		Ok(())
	}

	/// Pomocnicza funkcja clean_file (zostaje bez zmian)
	fn clean_file(file_path: &Path) -> io::Result<bool> {
		let content = fs::read_to_string(file_path)?;

		// Szacujemy pojemność, by uniknąć reallokacji (z reguły plik po czyszczeniu
		// będzie nieco mniejszy)
		let mut cleaned = String::with_capacity(content.len());
		let mut is_modified = false;

		for line in content.lines() {
			let trimmed = line.trim_end();
			if trimmed.len() != line.len() {
				is_modified = true;
			}
			cleaned.push_str(trimmed);
			cleaned.push('\n');
		}

		// Zapisujemy tylko, jeśli faktycznie dokonano zmian (chroni to czasy
		// modyfikacji mtime)
		if is_modified {
			fs::write(file_path, cleaned)?;
			println!("   -> Wyczyszczono: {}", file_path.display());
			Ok(true)
		} else {
			Ok(false)
		}
	}
}
