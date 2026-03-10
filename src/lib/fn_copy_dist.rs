// src/lib/fn_copy_dist.rs
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Struktura konfiguracyjna do zarządzania dystrybucją (Wzorzec: Parameter Object).
pub struct DistConfig<'a> {
    pub target_dir: &'a str,
    pub dist_dir: &'a str,
    /// Lista nazw binarek (bez rozszerzeń). Jeśli pusta - kopiuje wszystkie odnalezione binarki.
    pub binaries: Vec<&'a str>,
    pub clear_dist: bool,
    pub overwrite: bool,
    pub dry_run: bool,
}

impl<'a> Default for DistConfig<'a> {
    fn default() -> Self {
        Self {
            target_dir: "./target",
            dist_dir: "./dist",
            binaries: vec![],
            clear_dist: false,
            overwrite: true,
            dry_run: false,
        }
    }
}

/// Helper: Mapuje architekturę na przyjazne nazwy systemów.
fn parse_os_from_triple(triple: &str) -> String {
    let t = triple.to_lowercase();
    if t.contains("windows") {
        "windows".to_string()
    } else if t.contains("linux") {
        "linux".to_string()
    } else if t.contains("darwin") || t.contains("apple") {
        "macos".to_string()
    } else if t.contains("android") {
        "android".to_string()
    } else if t.contains("wasm") {
        "wasm".to_string()
    } else {
        "unknown".to_string()
    }
}

/// Helper: Prosta heurystyka odróżniająca prawdziwą binarkę od śmieci po kompilacji w systemach Unix/Windows.
fn is_likely_binary(path: &Path, os_name: &str) -> bool {
    if !path.is_file() {
        return false;
    }

    // Ignorujemy ukryte pliki (na wszelki wypadek)
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
    if file_name.starts_with('.') {
        return false;
    }

    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        // Odrzucamy techniczne pliki Rusta
        if ["d", "rlib", "rmeta", "pdb", "lib", "dll", "so", "dylib"].contains(&ext_str.as_str()) {
            return false;
        }
        if os_name == "windows" {
            return ext_str == "exe";
        }
        if os_name == "wasm" {
            return ext_str == "wasm";
        }
    } else {
        // Brak rozszerzenia to standard dla plików wykonywalnych na Linux/macOS
        if os_name == "windows" {
            return false;
        }
    }

    true
}

/// Przeszukuje katalog kompilacji i kopiuje pliki według konfiguracji `DistConfig`.
/// Zwraca listę przetworzonych plików: Vec<(Źródło, Cel)>
pub fn copy_dist(config: &DistConfig) -> io::Result<Vec<(PathBuf, PathBuf)>> {
    let target_path = Path::new(config.target_dir);
    let dist_path = Path::new(config.dist_dir);

    // Fail Fast
    if !target_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Katalog '{}' nie istnieje. Uruchom najpierw `cargo build`.",
                config.target_dir
            ),
        ));
    }

    // Opcja: Czyszczenie folderu dystrybucyjnego przed kopiowaniem
    if config.clear_dist && dist_path.exists() && !config.dry_run {
        // Używamy `let _` bo jeśli folder nie istnieje lub jest zablokowany, chcemy po prostu iść dalej
        let _ = fs::remove_dir_all(dist_path);
    }

    let mut found_files = Vec::new(); // Lista krotek (źródło, docelowy_folder, docelowy_plik)
    let profiles = ["debug", "release"];

    // Funkcja wewnętrzna: Przeszukuje folder (np. target/release) i dopasowuje reguły
    let mut scan_directory = |search_dir: &Path, os_name: &str, dest_base_dir: &Path| {
        if config.binaries.is_empty() {
            // TRYB 1: Kopiuj WSZYSTKIE odnalezione binarki
            if let Ok(entries) = fs::read_dir(search_dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if is_likely_binary(&path, os_name) {
                        let dest_file = dest_base_dir.join(path.file_name().unwrap());
                        found_files.push((path, dest_base_dir.to_path_buf(), dest_file));
                    }
                }
            }
        } else {
            // TRYB 2: Kopiuj KONKRETNE binarki
            for bin in &config.binaries {
                let suffix = if os_name == "windows" {
                    ".exe"
                } else if os_name == "wasm" {
                    ".wasm"
                } else {
                    ""
                };
                let full_name = format!("{}{}", bin, suffix);
                let path = search_dir.join(&full_name);
                if path.exists() {
                    let dest_file = dest_base_dir.join(&full_name);
                    found_files.push((path, dest_base_dir.to_path_buf(), dest_file));
                }
            }
        }
    };

    // =========================================================
    // KROK 1: Skanowanie kompilacji natywnej (Hosta)
    // =========================================================
    let host_os = std::env::consts::OS;
    for profile in &profiles {
        let search_dir = target_path.join(profile);
        let dest_base = dist_path.join(host_os).join(profile);
        if search_dir.exists() {
            scan_directory(&search_dir, host_os, &dest_base);
        }
    }

    // =========================================================
    // KROK 2: Skanowanie cross-kompilacji (Target Triples)
    // =========================================================
    if let Ok(entries) = fs::read_dir(target_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                if dir_name.contains('-') {
                    let os_name = parse_os_from_triple(&dir_name);
                    for profile in &profiles {
                        let search_dir = path.join(profile);
                        let dest_base = dist_path.join(&os_name).join(profile);
                        if search_dir.exists() {
                            scan_directory(&search_dir, &os_name, &dest_base);
                        }
                    }
                }
            }
        }
    }

    // =========================================================
    // KROK 3: Fizyczne operacje (z uwzględnieniem overwrite i dry_run)
    // =========================================================
    let mut processed_files = Vec::new();

    for (src, dest_dir, dest_file) in found_files {
        // Obsługa nadpisywania
        if dest_file.exists() && !config.overwrite {
            continue; // Pomijamy ten plik
        }

        if !config.dry_run {
            fs::create_dir_all(&dest_dir)?;
            fs::copy(&src, &dest_file)?;
        }

        processed_files.push((src, dest_file));
    }

    Ok(processed_files)
}
