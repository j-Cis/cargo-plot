use std::env;
use std::fs;
use std::path::Path;

/// [POL]: Kontekst ścieżki roboczej - oblicza relacje między terminalem a celem skanowania.
/// [ENG]: Working path context - calculates relations between terminal and scan target.
#[derive(Debug)]
pub struct PathContext {
    pub base_absolute: String,
    pub entry_absolute: String,
    pub entry_relative: String,
}

impl PathContext {
    pub fn resolve<P: AsRef<Path>>(entered_path: P) -> Result<Self, String> {
        let path_ref = entered_path.as_ref();

        // 1. BASE ABSOLUTE: Gdzie fizycznie odpalono program?
        let cwd = env::current_dir().map_err(|e| format!("Błąd odczytu CWD: {}", e))?;
        let base_abs = cwd.to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/");

        // 2. ENTRY ABSOLUTE: Pełna ścieżka do folderu, który skanujemy
        let abs_path = fs::canonicalize(path_ref)
            .map_err(|e| format!("Nie można ustalić ścieżki '{:?}': {}", path_ref, e))?;
        let entry_abs = abs_path.to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/");

        // 3. ENTRY RELATIVE: Ścieżka od terminala do skanowanego folderu
        let entry_rel = match abs_path.strip_prefix(&cwd) {
            Ok(rel) => {
                let rel_str = rel.to_string_lossy().replace('\\', "/");
                if rel_str.is_empty() {
                    "./".to_string() // Cel to ten sam folder co terminal
                } else {
                    format!("./{}/", rel_str)
                }
            }
            Err(_) => {
                // Jeśli cel jest na innym dysku (np. C:\ a terminal na D:\) 
                // lub całkiem poza strukturą CWD, relatywna nie istnieje. 
                // Wracamy wtedy do tego, co wpisał użytkownik, lub dajemy absolutną.
                path_ref.to_string_lossy().replace('\\', "/")
            }
        };

        Ok(Self {
            base_absolute: base_abs,
            entry_absolute: entry_abs,
            entry_relative: entry_rel,
        })
    }
}