use std::env;

pub struct BySection;

impl BySection {
    #[must_use]
    pub fn generate(tag: &str) -> String {
        let args: Vec<String> = env::args().collect();
        let command = args.join(" ");

        let instructions = "\
**Krótka instrukcja flag:**
- `-d, --dir <PATH>` : Ścieżka wejściowa do skanowania (domyślnie: `.`)
- `-p, --pat <PATTERNS>...` : Wzorce dopasowań (wymagane)
- `-s, --sort <STRATEGY>` : Strategia sortowania (np. `az-file-merge`)
- `-v, --view <MODE>` : Widok wyników (`tree`, `list`, `grid`)
- `-m, --on-match` : Pokaż tylko dopasowane ścieżki
- `-x, --on-mismatch` : Pokaż tylko odrzucone ścieżki
- `-o, --out-paths [PATH]` : Zapisz ścieżki do pliku (AUTO: `./other/`)
- `-c, --out-cache [PATH]` : Zapisz kod do pliku (AUTO: `./other/`)
- `-i, --info` : Tryb gadatliwy w terminalu
- `-b, --by` : Dodaj sekcję informacyjną na końcu pliku
- `--ignore-case` : Ignoruj wielkość liter we wzorcach
- `--treeview-no-root` : Ukryj główny folder w widoku drzewa";

        let markdown = format!(
            "\n\n---\n\
---\n\n\
## Command\n\n\
**Wywołana komenda:**\n\n\
```bash\n\
{command}\n\
```\n\n\
{instructions}\n\n\
[📊 Sprawdź `cargo-plot` na crates.io](https://crates.io/crates/cargo-plot)\n\n\
**Wersja raportu:**  
{tag}\n\n\
---\n\
"
        );

        markdown
    }
}
