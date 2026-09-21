# cargo-plot

[![Crates.io](https://img.shields.io/crates/v/cargo-plot.svg)](https://crates.io/crates/cargo-plot)
[![Docs.rs](https://docs.rs/cargo-plot/badge.svg)](https://docs.rs/cargo-plot)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Unlicense-blue.svg)](LICENSE)

**cargo-plot** (v1.0.1) to potężna wtyczka Cargo do błyskawicznego skanowania systemu plików, wizualizacji strukturalnej drzewa projektu oraz automatycznego generowania migawek (snapshots) w formacie Markdown. Wersja 1.0.0 całkowicie porzuca narzut plików TOML na rzecz w 100% sterowanego z konsoli, modularnego ekosystemu querypath.

**cargo-plot** (v1.0.1) is a powerful Cargo plugin for blazing-fast filesystem scanning, structural tree visualization, and automated Markdown snapshot generation. Version 1.0.0 drops TOML configuration overhead in favor of a 100% CLI-driven, modular querypath ecosystem.

🔗 **Crates.io**: [crates.io/crates/cargo-plot](https://crates.io/crates/cargo-plot)
🔗 **GitHub**: [github.com/j-Cis/cargo-plot](https://github.com/j-Cis/cargo-plot)

```text
[KiB 149.7] └──┬ 📂 cargo-plot                                                 ./cargo-plot/
[KiB 2.238]    ├──• ⚙️ Cargo.toml                                             ./Cargo.toml
[KiB 8.519]    ├──• 📝 README.md                                               ./README.md
[KiB 136.6]    └──┬ 📂 src                                                     ./src/
[KiB 3.262]       ├──• 🦀 cli.rs                                               ./src/cli.rs
[KiB 2.388]       └──• 🦀 main.rs                                              ./src/main.rs

```

---

* install `PS> cargo install cargo-plot`

**UŻYCIE / USAGE**

```powershell
  PS > cargo run -- -d ./ -p "./{examples|tests|src}/{*.rs|**/*.rs}" -p "./{Cargo.toml|README.md}" -c "H W D P" --dir-weight matched -Y "WYYY-WW-D hh:mm:ss" --ignore-leading-dot -o "./CHANGELOG" --max-file-size 262144 --file-name "CargoPlot-1-0-0_{VERSION}.md" --version-pattern "WYYYWWDSSShhmmssttqq"
```

**OR / LUB** 

```powershell
  PS > cargo plot -d ./ -p "./{examples|tests|src}/{*.rs|**/*.rs}" -p "./{Cargo.toml|README.md}" -c "H W D P" --dir-weight matched -Y "WYYY-WW-D hh:mm:ss" --ignore-leading-dot -o "./CHANGELOG" --max-file-size 262144 --file-name "CargoPlot-1-0-0_{VERSION}.md" --version-pattern "WYYYWWDSSShhmmssttqq"
```

---

## 🚀 Główne Funkcje / Key Features

* **Silnik Wzorców (powered by querypath)**: Zoptymalizowany system dopasowywania wzorców w pamięci operacyjnej. Obsługuje zaawansowany globbing, rozwijanie klamer separatorem pipe `{a|b}`, prefiksy relacji `@` (rodzeństwo) i `$` (sierota) oraz bezpośrednie injekcje regex (`re:`).
* **Pattern Engine**: In-memory pattern matching system. Supports advanced globbing, pipe-separated brace expansion `{a|b}`, `@` (sibling) and `$` (orphan) relation prefixes, and raw regex injection (`re:`).


* **Zachowywanie Drzewa (keep_parent)**: Kontrolowana z poziomu opcji opcja zachowywania katalogów nadrzędnych dla dopasowanych plików, gwarantująca pełną spójność wizualną w strukturze drzewa.
* **Tree Retention (keep_parent)**: Option-level setting to retain parent directories for matched files, ensuring full visual consistency in the tree structure.


* **100% CLI Driven**: Pozbawiony narzutu konfiguracji TOML i interfejsów graficznych. Pełna kontrola nad kolumnami, priorytetami sortowania i widokiem z poziomu flag (np. `-c "H W D P"`).
* **Pure CLI Workflow**: Stripped of TOML overhead. Full control over columns, sorting priorities, and views via CLI flags (e.g. `-c "H W D P"`).


* **Zrzuty Kodu i Dokumentacja (querypath-snapshot)**: Automatyczne generowanie raportów Markdown z pełnym kodem źródłowym, ignorujące pliki binarne oraz pliki przekraczające dozwolony limit bajtów.

---

## 🔍 Składnia Wzorców / Pattern Syntax

Architektura `cargo-plot` wspiera zaawansowany system prefiksów i modyfikatorów zgodny ze specyfikacją `querypath`.

| Wzorzec | Opis (PL) | Description (ENG) |
| --- | --- | --- |
| `*` / `**` | Pojedynczy / Wielopoziomowy Wildcard | Single / Multi-level Wildcard |
| `src/{lib \| bin}` | Robijanie klamer (**wyłącznie separator `\|`**) |
| `!*test*` | Twarde Weto (Negacja na początku) | Hard Veto (Negation at start) |
| `@core` | Rodzeństwo (prefiks `@` na początku wymaga pary plik + katalog) | Sibling (`@` prefix requiring file + dir pair) |
| `$core` | Sierota (prefiks `$` na początku wymaga braku pary) | Orphan (`$` prefix requiring missing pair) |
| `re:^v\d+` | Bezpośrednie wyrażenie regularne (prefiks `re:`) | Direct Raw Regex evaluation (`re:` prefix) |
| `./src/` | Zakotwiczenie do korzenia projektu | Anchoring to project root |

---

## 🛠 Użycie / Usage

**Uruchomienie przez Cargo (Zalecane) / Run via Cargo (Recommended):**

```powershell
cargo plot -d ./ -p "./{examples|tests|src}/{*.rs|**/*.rs}" -p "./{Cargo.toml|README.md}" -c "H W D P" --dir-weight matched -Y "WYYY-WW-D hh:mm:ss" --ignore-leading-dot -o "./CHANGELOG" --max-file-size 262144 --file-name "CargoPlot-1-0-0_{VERSION}.md" --version-pattern "WYYYWWDSSShhmmssttqq"

```

**Budowanie deweloperskie / Development build:**

```powershell
cargo run -- -d ./ -p "./{examples|tests|src}/{*.rs|**/*.rs}" -p "./{Cargo.toml|README.md}" -c "H W D P" --dir-weight matched -Y "WYYY-WW-D hh:mm:ss" --ignore-leading-dot -o "./CHANGELOG" --max-file-size 262144 --file-name "CargoPlot-1-0-0_{VERSION}.md" --version-pattern "WYYYWWDSSShhmmssttqq"

```

---
---
---

## Specyfikacja Wzorców Dopasowań (Pattern Matching Specification)

### 1. Standardowe modyfikatory dopasowań (Globbing & Wildcards)

Warstwa parsowania przekształcająca znaki tekstowe w reguły wyrażeń regularnych, wspierająca opcjonalną wrażliwość na wielkość liter.

| Wzorzec | Nazwa techniczna | Zachowanie silnika |
| :--- | :--- | :--- |
| `*` | Single-level Wildcard | **[POL]:** Dopasowuje zero lub więcej znaków w obrębie jednego poziomu (nie dopasowuje `/`).<br>**[ENG]:** Matches zero or more characters within a single level (does not match `/`). |
| `**` | Multi-level Wildcard | **[POL]:** Dopasowuje dowolną liczbę znaków łącznie z separatorami `/` (rekurencja wielopoziomowa).<br>**[ENG]:** Matches any number of characters including `/` separators (multi-level recursion). |
| `?` | Single Character | **[POL]:** Dopasowuje dokładnie jeden dowolny znak, z wyłączeniem separatora `/`.<br>**[ENG]:** Matches exactly one arbitrary character, excluding the `/` separator. |
| `{a\|b}` | Brace Expansion | **[POL]:** Rozwija wzorzec na oddzielne warianty logiczne przed kompilacją z użyciem separatora `\|`.<br>**[ENG]:** Expands the pattern into separate logical variants before compilation using `\|` separator. |
| `[a-z]` | Character Class | **[POL]:** Dopasowuje jeden znak z podanego zakresu lub zbioru.<br>**[ENG]:** Matches one character from the specified range or set. |
| `\` | Escape Character | **[POL]:** Traktuje następny znak dosłownie (np. `\.` dopasowuje kropkę).<br>**[ENG]:** Treats the next character literally (e.g. `\.` matches a dot). |

-----

### 2. Prefiksy Specjalne i Kotwiczenie

Bariery logiczne analizujące surowy wzorzec na podstawie prefiksów wejściowych w celu precyzyjnego ustalenia docelowych obiektów.

| Prefiks / Wzorzec | Nazwa techniczna | Zasada działania |
| :--- | :--- | :--- |
| `!` | Hard Veto | **[POL]:** Umieszczony na początku wzorca. Dopasowanie negatywne bezwzględnie odrzuca ścieżkę.<br>**[ENG]:** Placed at the start of a pattern. A negative match unconditionally rejects the path. |
| `@` | Sibling Requirement | **[POL]:** Umieszczony na początku wzorca (np. `@core`). Wymaga istnienia pary plik + katalog o tej samej nazwie rdzennej.<br>**[ENG]:** Placed at the start of a pattern (e.g. `@core`). Requires a file + directory pair of the same core name. |
| `$` | Orphan Requirement | **[POL]:** Umieszczony na początku wzorca (np. `$core`). Przeciwieństwo `@`. Dopasowuje element tylko wtedy, gdy w środowisku brakuje odpowiadającej mu pary.<br>**[ENG]:** Placed at the start of a pattern (e.g. `$core`). Opposite of `@`. Matches an element only if its corresponding pair is missing. |
| `re:` | Direct Regex | **[POL]:** Umieszczony na początku wzorca (np. `re:^src/.*\.rs$`). Przekazuje czyste wyrażenie regularne bezpośrednio do silnika.<br>**[ENG]:** Placed at the start of a pattern (e.g. `re:^src/.*\.rs$`). Passes raw regex straight to the engine. |
| `./...` | Root Anchor | **[POL]:** Wymusza szukanie ścieżki dokładnie od korzenia skanowanego środowiska.<br>**[ENG]:** Enforces path searching exactly from the root of the scanned environment. |
| `.../` | Directory Target | **[POL]:** Wzorzec kończący się ukośnikiem. Dopasowuje wyłącznie katalogi.<br>**[ENG]:** Pattern ending with a slash. Matches directories exclusively. |

-----

### 3. Opcje Skanowania (PathsOptions)

Zachowanie strukturalne drzewa jest kontrolowane na poziomie konfiguracji skanera (`PathsOptions`), a nie w samej składni wzorców.

| Opcja | Metoda API | Opis |
| :--- | :--- | :--- |
| `keep_parent` | `.keep_parent(bool)` | **[POL]:** Zachowuje katalogi nadrzędne dla dopasowanych elementów, zapobiegając powstawaniu osieroconych węzłów w drzewie.<br>**[ENG]:** Retains parent directories for matched elements, preventing orphan nodes in the tree structure. |
| `ignore_case` | `.ignore_case(bool)` | **[POL]:** Włącza ignorowanie wielkości liter we wszystkich skompilowanych wzorcach.<br>**[ENG]:** Enables case-insensitive matching across all compiled patterns. |

-----

---

## Features

- 🔍 **Advanced Pattern Matching**: Full glob support with pattern negations (`!`), brace expansion (`{a|b}`), and wildcard safeguards.
- ⚡ **High Performance Engine**: Fast directory traversal built on `walkdir` with in-memory target indexing.
- 📊 **Rich Metadata**: Retrieves physical sizes, modification timestamps, and automated binary file detection via null-byte checking.
- 📁 **Dual-Size Directory Aggregation**: Evaluates both total physical directory size (`real_size`) and pattern-matched content size (`matched_size`).
- ⏱️ **Execution Metrics**: Accurate tracking of query lifecycle timestamps (`started_at_ms`, `finished_at_ms`) and duration in milliseconds.
- 🛠️ **Cross-Platform Normalization**: Automatic handling of UNC/POSIX paths and root resolution.

---

## Installation

Add `querypath` to your `Cargo.toml`:

```toml
[dependencies]
querypath = "1.0.0"

```

Or run:

```bash
cargo add querypath

```

---

## Quickstart

```rust
use anyhow::Result;
use querypath::QueryPath;

fn main() -> Result<()> {
    let results = QueryPath::new()
        .scan_at(["./"])
        .match_pattern(["*.rs", "!**/{tests|.git|target}/?**"])
        .keep_parent(true)
        .ignore_case(true)
        .run()?;

    println!("Scan finished in {} ms", results.duration_ms);
    println!("Scanned {} files, {} dirs", results.scanned_files, results.scanned_dirs);

    for dir in &results.dirs {
        println!("📁 {} (matched: {} B, total: {} B)", dir.path, dir.matched_size, dir.real_size);
    }

    for file in &results.files {
        println!("📄 {} (size: {} B, binary: {})", file.path, file.size, file.is_binary);
    }

    Ok(())
}

```

## Core API & Data Structures

### `QueryPath` Builder

| Method | Type Signature | Description |
| --- | --- | --- |
| `new()` | `() -> Self` | Initializes a default query instance. |
| `scan_at(paths)` | `(IntoIterator<Item = S>) -> Self` | Sets target directories or root entry paths. |
| `match_pattern(patterns)` | `(IntoIterator<Item = S>) -> Self` | Defines glob matching engine rules and negations. |
| `keep_parent(bool)` | `(bool) -> Self` | Retains structural parent directories of matched files. |
| `ignore_case(bool)` | `(bool) -> Self` | Toggles case insensitivity for glob patterns. |
| `run()` | `() -> Result<QueryResults>` | Executes the directory scan and returns structured metrics. |

### `QueryResults` Structure

```rust
pub struct QueryResults {
    pub execution_dir: String,
    pub scanned_paths: Vec<String>,
    pub patterns: Vec<String>,
    pub scanned_files: usize,
    pub scanned_dirs: usize,
    pub started_at_ms: u64,
    pub finished_at_ms: u64,
    pub duration_ms: u64,
    pub files: Vec<FileItem>,
    pub dirs: Vec<DirItem>,
}

```

-----

> [\!NOTE]
> [POL]: Niniejsza specyfikacja stanowi oficjalną i ostateczną dokumentację technologiczną zunifikowanego ekosystemu querypath i `cargo-plot` w wersji 1.0.0.
>
> [ENG]: This specification constitutes the official and final technological documentation of the unified querypath ecosystem and `cargo-plot` version 1.0.0.

🫟
