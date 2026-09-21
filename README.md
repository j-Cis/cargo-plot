# cargo-plot

[![Crates.io](https://img.shields.io/crates/v/cargo-plot.svg)](https://crates.io/crates/cargo-plot)
[![Docs.rs](https://docs.rs/cargo-plot/badge.svg)](https://docs.rs/cargo-plot)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

**cargo-plot** (v1.0.0) to potężna wtyczka Cargo do błyskawicznego skanowania systemu plików, wizualizacji strukturalnej drzewa projektu oraz automatycznego generowania migawek (snapshots) w formacie Markdown. Wersja 1.0.0 całkowicie porzuca narzut plików TOML na rzecz w 100% sterowanego z konsoli, modularnego ekosystemu.

**cargo-plot** (v1.0.0) is a powerful Cargo plugin for blazing-fast filesystem scanning, structural tree visualization, and automated Markdown snapshot generation. Version 1.0.0 drops TOML configuration overhead in favor of a 100% CLI-driven, modular ecosystem.

🔗 **Crates.io**: [crates.io/crates/cargo-plot](https://crates.io/crates/cargo-plot)
🔗 **GitHub**: [github.com/j-Cis/cargo-plot](https://github.com/j-Cis/cargo-plot)

```text
[KiB 149.7] └──┬ 📂 cargo-plot                                                 ./cargo-plot/
[KiB 2.238]    ├──• ⚙️ Cargo.toml                                              ./Cargo.toml
[KiB 8.519]    ├──• 📝 README.md                                               ./README.md
[KiB 136.6]    └──┬ 📂 src                                                     ./src/
[KiB 3.262]       ├──• 🦀 cli.rs                                               ./src/cli.rs
[KiB 2.388]       └──• 🦀 main.rs                                              ./src/main.rs

```

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

* **Silnik Wzorców 2.0 (querypath)**: Zero-alokacyjny system dopasowywania wzorców w pamięci. Obsługuje zaawansowane globbingi, rozwijanie klamer separatorami pipe `{a|b}`, wstrzykiwanie surowych wyrażeń regularnych (`re:`) oraz modyfikatory relacyjne (sierota/rodzeństwo).
* **Pattern Engine**: Zero-allocation in-memory pattern matching. Supports advanced globbing, pipe-separated brace expansion `{a|b}`, raw regex injection (`re:`), and relational modifiers (orphan/sibling).


* **Inteligentny Audyt Wagi (querypath-fmt)**: Katalogi otrzymują podwójne rozliczenie rozmiaru — `real_size` (fizyczna waga całego katalogu na dysku) oraz `matched_size` (waga wyłącznie plików spełniających kryteria wyszukiwania).
* **Smart Weight Audit**: Directories receive dual size accounting — `real_size` (total physical disk weight) and `matched_size` (weight of only the files matching search criteria).


* **100% CLI Driven**: Pozbawiony narzutu konfiguracji TOML i interfejsów graficznych. Pełna kontrola nad kolumnami, priorytetami sortowania i widokiem z poziomu flag (np. `-c "W H D P"`).
* **Pure CLI Workflow**: Stripped of TOML overhead. Full control over columns, sorting priorities, and views via flags (e.g. `-c "W H D P"`).


* **Zrzuty Kodu i Dokumentacja (querypath-snapshot)**: Automatyczne generowanie raportów Markdown z pełnym kodem źródłowym, odrzucające pliki binarne oraz pliki przekraczające dozwolony limit bajtów. Raporty tagowane są autorskim silnikiem `temporal-fmt`.
* **Code Snapshots**: Automated generation of Markdown reports containing full source code, ignoring binary files and oversized payloads.



---

## 🔍 Składnia Wzorców / Pattern Syntax

Architektura `cargo-plot` wspiera zaawansowany system prefiksów i modyfikatorów, które aplikowane są *przed* kompilacją wzorca.

| Symbol | Opis (PL) | Description (ENG) |
| --- | --- | --- |
| `*` / `**` | Pojedynczy / Wielopoziomowy Wildcard | Single / Multi-level Wildcard |
| `src/{lib | bin}` | Rozwijanie klamer (separator `|`) |
| `!*test*` | Twarde Weto (Negacja) | Hard Veto (Negation) |
| `@core` | Rodzeństwo (wymaga pary plik + katalog) | Sibling (requires file + dir pair) |
| `$core` | Sierota (tylko brak odpowiadającej pary) | Orphan (only if pair is missing) |
| `re:^v\d+` | Bezpośrednie wyrażenie regularne | Direct Raw Regex evaluation |
| `./src/` | Kotwiczenie do korzenia projektu | Anchoring to project root |
| `.../` | Wymuszenie dopasowania tylko do katalogów | Forces matching directories only |

---

## 🛠 Instalacja / Installation

**Jako rozszerzenie Cargo (Zalecane) / As Cargo extension (Recommended):**

```bash
cargo install cargo-plot

```

**Budowanie deweloperskie / Development build:**

```bash
git clone [https://github.com/j-Cis/cargo-plot.git](https://github.com/j-Cis/cargo-plot.git)
cd cargo-plot
cargo build --release

```

---

## Zestawienie Różnic / Comparison Table

| Cecha / Feature | Wersja / Version 0.2.0 | Wersja / Version 1.0.0 |
| --- | --- | --- |
| **Architektura** | **[PL]** Monolityczna struktura `src/lib`. | **[PL]** Ekosystem 4 niezależnych mikro-bibliotek (`querypath`). |
| **Architecture** | **[ENG]** Monolithic `src/lib` structure.  | **[ENG]** Ecosystem of 4 independent micro-crates. |
| **Interfejsy**   | **[PL]** Wiele interfejsów: CLI, TUI, GUI (egui).    | **[PL]** W 100% oparte na czystym i elastycznym CLI. |
| **Interfaces**   | **[ENG]** Multiple interfaces: CLI, TUI, GUI (egui). | **[ENG]** 100% Pure & flexible CLI driven. |
| **Konfiguracja**  | **[PL]** Złożone schematy plików konfiguracyjnych TOML. | **[PL]** Sterowanie argumentami `clap` (np. `--group-order`). |
| **Configuration** | **[ENG]** Complex TOML configuration schemas.           |**[ENG]** Controlled by `clap` arguments (e.g. `--group-order`). |
| **Pamięć / I/O** | **[PL]** Częste odpytywanie dysku przy analizie relacji.  | **[PL]** Zero-I/O weryfikacja oparta na indeksie `WalkEnvIndex`.|
| **Memory / I/O** | **[ENG]** Frequent disk polling during relation analysis. | **[ENG]** Zero-I/O verification based on `WalkEnvIndex`. |

---