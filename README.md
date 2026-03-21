# cargo-plot

**cargo-plot** (v0.2.0) to wszechstronny „szwajcarski scyzoryk” dewelopera napisany w języku Rust (edycja 2024). Służy do zaawansowanej wizualizacji struktur projektów, audytu zajętości miejsca oraz automatycznego generowania dokumentacji technicznej bezpośrednio z poziomu Cargo.

**cargo-plot** (v0.2.0) is a versatile developer's "Swiss Army knife" written in Rust (2024 edition). It is used for advanced project structure visualization, disk space auditing, and automatic technical documentation generation directly from Cargo.

🔗 **Crates.io**: [crates.io/crates/cargo-plot](https://crates.io/crates/cargo-plot)
🔗 **GitHub**: [github.com/j-Cis/cargo-plot](https://github.com/j-Cis/cargo-plot)

```text
[KiB 174.4] └──┬ 📂 cargo-plot-2                   ./cargo-plot-2/
[KiB 1.380]    ├──• ⚙️ Cargo.toml                 ./Cargo.toml
[KiB 173.0]    └──┬ 📂 src                         ./src/
[  B 70.00]       ├──• 🦀 addon.rs                 ./src/addon.rs
[KiB 2.431]       ├──┬ 📂 addon                    ./src/addon/
[KiB 2.431]       │  └──• 🦀 time_tag.rs           ./src/addon/time_tag.rs
[  B 120.0]       ├──• 🦀 core.rs                  ./src/core.rs
[KiB 65.85]       ├──┬ 📂 core                     ./src/core/
[KiB 1.117]       │  ├──• 🦀 file_stats.rs         ./src/core/file_stats.rs
[KiB 3.177]       │  ├──┬ 📂 file_stats            ./src/core/file_stats/
[KiB 3.177]       │  │  └──• 🦀 weight.rs          ./src/core/file_stats/weight.rs
[  B 285.0]       │  ├──• 🦀 path_matcher.rs       ./src/core/path_matcher.rs
[KiB 23.00]       │  ├──┬ 📂 path_matcher          ./src/core/path_matcher/
[KiB 14.65]       │  │  ├──• 🦀 matcher.rs         ./src/core/path_matcher/matcher.rs
[KiB 4.501]       │  │  ├──• 🦀 sort.rs            ./src/core/path_matcher/sort.rs
[KiB 3.845]       │  │  └──• 🦀 stats.rs           ./src/core/path_matcher/stats.rs
```

---

## 🚀 Główne Funkcje / Key Features

* **Silnik Wzorców 2.0 / Pattern Engine 2.0**: Zaawansowane filtrowanie strukturalne z użyciem flag relacyjnych: `@` (rodzeństwo), `$` (sierota) oraz `+` (głęboki skan). Wspiera również rozwijanie klamer `{a,b}`.
  * **Structural Flags**: Advanced filtering using relational flags: `@` (sibling), `$` (orphan), and `+` (deep scan). Also supports brace expansion `{a,b}`.
* **Audyt Miejsca / Disk Audit**: Obliczanie wag w systemach binarnym (IEC) i dziesiętnym (SI). Flaga `-a` pozwala na odczyt rzeczywistego fizycznego rozmiaru folderów z dysku.
  * **Weight Systems**: Calculate sizes in Binary (IEC) and Decimal (SI) systems. The `-a` flag enables reading the actual physical directory size from the disk.
* **Trzy Interfejsy / Triple Interface**: Pełna swoboda pracy dzięki natywnej aplikacji GUI (`egui`), interaktywnemu TUI (`cliclack 0.5.0`) oraz klasycznemu CLI.
  * **Multi-Modal**: Full workflow flexibility with a native GUI (`egui`), interactive TUI (`cliclack 0.5.0`), and classic CLI.
* **Automatyczna Dokumentacja / Auto-Doc**: Generowanie raportów Markdown i pełnych archiwów kodu źródłowego z profesjonalną tabelaryczną stopką metadanych.
  * **Technical Reporting**: Generate Markdown reports and full source code archives with professional tabular metadata footers.

---

## 🔍 Składnia Wzorców / Pattern Syntax

| Symbol | Opis (PL) | Description (ENG) |
| :--- | :--- | :--- |
| `src/{lib,bin}` | Rozwijanie klamer | Brace expansion |
| `!*test*` | Twarde Weto (Negacja) | Hard Veto (Negation) |
| `src/+` | Tryb głęboki (rekurencja) | Deep mode (recursive) |
| `@tui` | Rodzeństwo (wymaga plik+dir) | Sibling (requires file+dir) |
| `$core` | Sierota (tylko brak pary) | Orphan (only if pair is missing) |

---

## 🛠 Instalacja / Installation

**Jako rozszerzenie Cargo (Zalecane) / As Cargo extension (Recommended):**

```bash
cargo install cargo-plot
```

**Budowanie deweloperskie / Development build:**

```bash
git clone https://github.com/j-Cis/cargo-plot.git
cd cargo-plot
cargo build --release
```

---

## Zestawienie Różnic / Comparison Table

| Cecha / Feature | Wersja / Version 0.1.5 | Wersja / Version 0.2.0 |
| :--- | :--- | :--- |
| **Architektura**<br>**Architecture** | **[PL]** Płaska struktura biblioteki (`src/lib/*.rs`).<br>**[ENG]** Flat library structure (`src/lib/*.rs`). | **[PL]** Modularna struktura „Core + Interfaces” (Porty i Adaptery).<br>**[ENG]** Modular "Core + Interfaces" structure (Ports & Adapters). |
| **Interfejsy**<br>**Interfaces** | **[PL]** Klasyczne CLI oraz uproszczone TUI.<br>**[ENG]** Classic CLI and simplified TUI. | **[PL]** Trio: CLI, TUI (v0.5.0) oraz natywne GUI (egui).<br>**[ENG]** Triple: CLI, TUI (v0.5.0), and native GUI (egui). |
| **Silnik Wzorców**<br>**Pattern Engine** | **[PL]** Proste filtrowanie oparte na maskach Glob.<br>**[ENG]** Simple filtering based on Glob masks. | **[PL]** Regex + flagi relacyjne: `@` (rodzeństwo), `$` (sierota), `+` (głęboki skan).<br>**[ENG]** Regex + relational flags: `@` (sibling), `$` (orphan), `+` (deep scan). |
| **Statystyki**<br>**Statistics** | **[PL]** Brak lub tylko sumaryczna waga projektu.<br>**[ENG]** None or only total project weight. | **[PL]** Live Update: podział na pliki tekstowe (Txt), binarne (Bin) i błędy (Err).<br>**[ENG]** Live Update: split into Text (Txt), Binary (Bin), and Errors (Err). |
| **Raportowanie**<br>**Reporting** | **[PL]** Rozbudowana, opisowa stopka tekstowa.<br>**[ENG]** Long, descriptive text footer. | **[PL]** Profesjonalna tabela metadanych w bloku Markdown.<br>**[ENG]** Professional metadata table in a Markdown block. |
| **System Wag**<br>**Weight System** | **[PL]** Podstawowe obliczenia (SI/IEC).<br>**[ENG]** Basic calculations (SI/IEC). | **[PL]** SI/IEC + flaga `-a` (fizyczny rozmiar folderów z dysku).<br>**[ENG]** SI/IEC + `-a` flag (actual physical folder size from disk). |
| **Bezpieczeństwo**<br>**Safety** | **[PL]** Standardowe mechanizmy Rusta.<br>**[ENG]** Standard Rust mechanisms. | **[PL]** Rygorystyczny zakaz używania bloków `unsafe`.<br>**[ENG]** Strict prohibition of `unsafe` blocks. |
| **Logika Widoku**<br>**View Logic** | **[PL]** Zduplikowana w plikach `tree.rs` i `grid.rs`.<br>**[ENG]** Duplicated in `tree.rs` and `grid.rs` files. | **[PL]** Zunifikowane budowanie struktury w `shared.rs` (DRY).<br>**[ENG]** Unified structure building in `shared.rs` (DRY). |
| **Internacjonalizacja**<br>**i18n** | **[PL]** Tylko twardo zakodowane teksty.<br>**[ENG]** Hardcoded texts only. | **[PL]** Pełne wsparcie PL/EN we wszystkich modułach i interfejsach.<br>**[ENG]** Full PL/EN support across all modules and interfaces. |

---

## Kluczowe usprawnienia techniczne / Key Technical Enhancements

* **[PL] Precyzja wagi korzenia:** W wersji 0.2.0 wyeliminowano błąd wyświetlania wagi `0` dla głównego folderu; teraz korzeń dumnie reprezentuje sumę całego skanowania.
* **[ENG] Root weight precision:** Version 0.2.0 eliminates the bug displaying `0` weight for the main folder; now the root proudly represents the sum of the entire scan.
* **[PL] Izolacja procesów GUI:** Dzięki nowej architekturze, generowanie podglądu kodu w GUI odbywa się tylko dla żądanej sekcji, co drastycznie optymalizuje zużycie pamięci.
* **[ENG] GUI process isolation:** Thanks to the new architecture, code preview generation in the GUI occurs only for the requested section, drastically optimizing memory usage.
* **[PL] Modernizacja TUI:** Pełna przesiadka na `cliclack 0.5.0` oraz integracja z `shlex` zapewniają bezbłędne parsowanie złożonych komend CLI wewnątrz interfejsu interaktywnego.
* **[ENG] TUI Modernization:** Full transition to `cliclack 0.5.0` and `shlex` integration ensures flawless parsing of complex CLI commands within the interactive interface.

---

## 🌍 Wspierane Systemy / Supported Systems

| System | Target Triple |
| :--- | :--- |
| **Windows 64-bit** | `x86_64-pc-windows-msvc` |
| **Linux 64-bit** | `x86_64-unknown-linux-gnu` |
| **macOS (Intel/M1)** | `x86_64-apple-darwin` / `aarch64-apple-darwin` |

---

> 🚀 **cargo-plot** | Wygenerowano przez cargo-plot v0.2.0 | [GitHub](https://github.com/j-Cis/cargo-plot)
> 🚀 **cargo-plot** | Generated by cargo-plot v0.2.0 | [GitHub](https://github.com/j-Cis/cargo-plot)

---
