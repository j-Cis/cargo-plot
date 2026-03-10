use std::collections::HashMap;
use std::path::PathBuf;

pub fn generate_ids(paths: &[PathBuf]) -> HashMap<PathBuf, String> {
    let mut map = HashMap::new();
    let mut counters: HashMap<String, usize> = HashMap::new();

    // Klonujemy i sortujemy ścieżki, żeby ID były nadawane powtarzalnie
    let mut sorted_paths = paths.to_vec();
    sorted_paths.sort();

    for path in sorted_paths {
      // Ignorujemy foldery, przypisujemy ID tylko plikom
        if path.is_dir() {
            continue;
        }
        // DODAJEMY .to_string() NA KOŃCU, ABY ZROBIĆ NIEZALEŻNĄ KOPIĘ
        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        
        // Tutaj .replace() i tak zwraca już własnego Stringa, więc jest bezpiecznie
        let path_str = path.to_string_lossy().replace('\\', "/");

        // 1. Twarde reguły dla znanych plików
        if file_name == "Cargo.toml" { map.insert(path.clone(), "TomlCargo".to_string()); continue; }
        if file_name == "Makefile.toml" { map.insert(path.clone(), "TomlMakefile".to_string()); continue; }
        if file_name == "build.rs" { map.insert(path.clone(), "RustBuild".to_string()); continue; }
        if path_str.contains("src/ui/index.slint") { map.insert(path.clone(), "SlintIndex".to_string()); continue; }

        // 2. Dynamiczne ID na podstawie ścieżki
        let prefix = if path_str.contains("src/lib") {
            if file_name == "mod.rs" { "RustLibMod".to_string() } else { "RustLibPub".to_string() }
        } else if path_str.contains("src/bin") || path_str.contains("src/main.rs") {
            "RustBin".to_string()
        } else if path_str.contains("src/ui") {
            "Slint".to_string()
        } else {
            let ext = path.extension().unwrap_or_default().to_string_lossy();
            format!("File{}", capitalize(&ext))
        };

        // Licznik dla danej kategorii
        let count = counters.entry(prefix.clone()).or_insert(1);
        
        let id = if file_name == "mod.rs" && prefix == "RustLibMod" {
            format!("{}_00", prefix) // mod.rs zawsze jako 00
        } else {
            format!("{}_{:02}", prefix, count)
        };

        map.insert(path, id);
        if !(file_name == "mod.rs" && prefix == "RustLibMod") {
            *count += 1;
        }
    }

    map
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}