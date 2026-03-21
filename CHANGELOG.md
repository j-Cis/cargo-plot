Oto profesjonalny, szczegółowy i dwujęzyczny plik `CHANGELOG.md`, dokumentujący ewolucję projektu **cargo-plot** z wersji **0.1.5** do przełomowej wersji **0.2.0**.

---

# CHANGELOG - cargo-plot

All notable changes to this project will be documented in this file.
Wszystkie istotne zmiany w tym projekcie będą dokumentowane w tym pliku.

---

## [0.2.0] — 2026-03-21

### "The Architecture Leap" / "Skok Architektoniczny"

#### 1. Total Architectural Refactoring / Całkowity Refaktoring Architektury

* **[ENG]** Transitioned from a flat library structure to a robust "Core + Interfaces" model. Business logic has been moved to `src/core/`, separating data processing from presentation.
* **[POL]** Przejście z płaskiej struktury biblioteki na solidny model „Core + Interfejsy”. Logika biznesowa została przeniesiona do `src/core/`, oddzielając przetwarzanie danych od prezentacji.
* **[ENG]** Introduced internal modularization: `path_matcher` for logic, `path_store` for data, and `path_view` for rendering.
* **[POL]** Wprowadzono wewnętrzną modularyzację: `path_matcher` dla logiki, `path_store` dla danych oraz `path_view` dla renderowania.

#### 2. Introduction of Graphical User Interface (GUI) / Wprowadzenie Interfejsu Graficznego (GUI)

* **[ENG]** Added a fully functional GUI powered by `eframe` and `egui`. Users can now manage patterns, preview trees, and generate reports in a dedicated window.
* **[POL]** Dodano w pełni funkcjonalny interfejs graficzny (GUI) oparty na `eframe` i `egui`. Użytkownicy mogą teraz zarządzać wzorcami, podglądać drzewa i generować raporty w dedykowanym oknie.
* **[ENG]** Features include a live "Zoom" scale, interactive file browsing using `rfd`, and real-time statistics.
* **[POL]** Funkcje obejmują skalowanie „Zoom” na żywo, interaktywne przeglądanie folderów za pomocą `rfd` oraz statystyki w czasie rzeczywistym.

#### 3. Full Internationalization (i18n) / Pełna Internacjonalizacja (i18n)

* **[ENG]** Implemented a comprehensive i18n system supporting English and Polish across all interfaces (CLI, TUI, GUI).
* **[POL]** Zaimplementowano kompleksowy system i18n wspierający język angielski i polski we wszystkich interfejsach (CLI, TUI, GUI).
* **[ENG]** Automatic language detection based on environment variables with an option for manual override via `--lang`.
* **[POL]** Automatyczna detekcja języka na podstawie zmiennych środowiskowych z opcją ręcznego wymuszenia przez `--lang`.

#### 4. Advanced Sorting: "Merge" Strategies / Zaawansowane Sortowanie: Strategie „Merge”

* **[ENG]** Introduced new sorting algorithms: `AzFileFirstMerge` and `ZaFileFirstMerge`. These group logical file-directory pairs (e.g., `mod.rs` and a directory of the same name) together.
* **[POL]** Wprowadzono nowe algorytmy sortowania: `AzFileFirstMerge` oraz `ZaFileFirstMerge`. Grupują one logiczne pary plik-katalog (np. `mod.rs` i katalog o tej samej nazwie) obok siebie.
* **[ENG]** Improved directory/file priority logic to ensure cleaner visual output in large Rust projects.
* **[POL]** Ulepszono logikę priorytetów katalogów/plików, aby zapewnić czystszy wynik wizualny w dużych projektach Rust.

#### 5. Brace Expansion in Patterns / Rozwijanie Klamer we Wzorcach

* **[ENG]** The pattern engine now supports brace expansion, e.g., `src/{lib,bin}/*.rs` expands automatically to multiple search patterns.
* **[POL]** Silnik wzorców obsługuje teraz rozwijanie klamer, np. `src/{lib,bin}/*.rs` automatycznie rozwija się do wielu wzorców wyszukiwania.
* **[ENG]** Added support for recursive expansion, allowing complex path filtering with single-line inputs.
* **[POL]** Dodano wsparcie dla rekurencyjnego rozwijania, co pozwala na złożone filtrowanie ścieżek za pomocą pojedynczej linii tekstu.

#### 6. Enhanced Weight and Size Calculation / Ulepszone Obliczanie Wagi i Rozmiaru

* **[ENG]** Added a toggle between Binary (IEC: KiB, MiB) and Decimal (SI: kB, MB) unit systems via the `-u` flag.
* **[POL]** Dodano przełącznik między binarnym (IEC: KiB, MiB) a dziesiętnym (SI: kB, MB) systemem jednostek za pomocą flagi `-u`.
* **[ENG]** Introduced the `-a` / `--all` flag to calculate the "Physical" size of directories (including hidden/ignored files) versus the "Filtered" sum.
* **[POL]** Wprowadzono flagę `-a` / `--all`, aby obliczać „Fizyczny” rozmiar katalogów (wliczając ukryte/ignorowane pliki) w przeciwieństwie do sumy „Przefiltrowanej”.

#### 7. Grid View Mode / Nowy Tryb Widoku: Siatka (Grid)

* **[ENG]** Added `ViewMode::Grid`, a new visualization style that aligns file names and their relative paths into a clean, readable table-like structure in the terminal.
* **[POL]** Dodano `ViewMode::Grid` – nowy styl wizualizacji, który wyrównuje nazwy plików i ich ścieżki relatywne w czystą, czytelną strukturę przypominającą tabelę w terminalu.
* **[ENG]** Dynamic width calculation ensures the grid adapts to the longest path in the result set.
* **[POL]** Dynamiczne obliczanie szerokości zapewnia dopasowanie siatki do najdłuższej ścieżki w zestawie wyników.

#### 8. Professional Markdown Footers / Profesjonalne Stopki Markdown

* **[ENG]** Reports now include a detailed metadata table at the end (when using the `-b` flag), containing the tool version, input path, TimeTag, and the exact CLI command used.
* **[POL]** Raporty zawierają teraz szczegółową tabelę metadanych na końcu (przy użyciu flagi `-b`), zawierającą wersję narzędzia, ścieżkę wejściową, TimeTag oraz dokładną użytą komendę CLI.
* **[ENG]** Improved report aesthetics with better use of blockquotes and horizontal rules.
* **[POL]** Ulepszono estetykę raportów dzięki lepszemu wykorzystaniu cytatów (blockquotes) i linii poziomych.

#### 9. TUI "Cockpit" Evolution / Ewolucja „Kokpitu” TUI

* **[ENG]** The Terminal User Interface has been rewritten to act as an interactive builder. It now features dynamic labels showing current configuration status before execution.
* **[POL]** Interfejs TUI został przepisany, aby działać jako interaktywny kreator. Posiada teraz dynamiczne etykiety pokazujące aktualny stan konfiguracji przed uruchomieniem.
* **[ENG]** Added a "CLI Mode" in TUI, allowing users to paste raw CLI arguments to instantly configure the interactive session.
* **[POL]** Dodano „Tryb CLI” w TUI, pozwalający użytkownikom na wklejanie surowych argumentów CLI w celu natychmiastowej konfiguracji sesji interaktywnej.

#### 10. Security and Performance / Bezpieczeństwo i Wydajność

* **[ENG]** Global implementation of `#![forbid(unsafe_code)]` to guarantee memory safety.
* **[POL]** Globalna implementacja `#![forbid(unsafe_code)]`, aby zagwarantować bezpieczeństwo pamięci.
* **[ENG]** Replaced standard manual path parsing with `walkdir` and `shlex` for better reliability across different operating systems.
* **[POL]** Zastąpiono standardowe ręczne parsowanie ścieżek bibliotekami `walkdir` i `shlex` dla lepszej niezawodności w różnych systemach operacyjnych.

#### 11. Context-Aware Path Resolution / Inteligentne Rozwiązywanie Ścieżek

* **[ENG]** New `PathContext` logic correctly identifies the relationship between the terminal's working directory and the scan target, ensuring correct `./relative/` path rendering.
* **[POL]** Nowa logika `PathContext` poprawnie identyfikuje relację między katalogiem roboczym terminala a celem skanowania, zapewniając poprawne renderowanie ścieżek `./relatywnych/`.
* **[ENG]** Improved handling of Windows extended-length paths (`\\?\`).
* **[POL]** Ulepszona obsługa długich ścieżek systemu Windows (`\\?\`).

#### 12. Refined Pattern Logic Flags / Doprecyzowane Flagi Logiki Wzorców

* **[ENG]** Clearly separated `@` (sibling), `$` (orphan), and `+` (deep/recursive) flags to give users surgical control over what is included in the documentation.
* **[POL]** Wyraźnie rozdzielono flagi `@` (rodzeństwo), `$` (sierota) oraz `+` (głęboki/rekurencyjny), aby dać użytkownikom chirurgiczną kontrolę nad tym, co znajdzie się w dokumentacji.
* **[ENG]** Negation (`!`) now acts as a "Hard Veto", overriding any positive matches for a path.
* **[POL]** Negacja (`!`) działa teraz jako „Twarde Weto”, unieważniając wszelkie pozytywne dopasowania dla danej ścieżki.

---

### [0.1.5] — 2026-03-11

#### "The Foundation" / "Fundament"

* **[ENG]** Initial stable release of the new generation documentation tool.
* **[POL]** Pierwsze stabilne wydanie nowej generacji narzędzia dokumentacyjnego.
* **[ENG]** Basic `Tree` and `Doc` commands implemented.
* **[POL]** Zaimplementowano podstawowe komendy `Tree` oraz `Doc`.
* **[ENG]** Support for automatic file identification and icons (Rust, TOML, Markdown).
* **[POL]** Wsparcie dla automatycznej identyfikacji plików i ikon (Rust, TOML, Markdown).
* **[ENG]** Simple TUI based on `cliclack`.
* **[POL]** Proste TUI oparte na `cliclack`.
* **[ENG]** Binary file blacklist to prevent Markdown corruption.
* **[POL]** Czarna lista plików binarnych zapobiegająca uszkodzeniu plików Markdown.

---

### Statistics / Statystyki (v0.2.0)

* **Lines of Code / Linii kodu:** ~4500+
* **New Modules / Nowych modułów:** 22
* **Supported Languages / Wspierane języki:** 2 (PL, EN)
* **Interface Modes / Tryby interfejsu:** 3 (CLI, TUI, GUI)

---

> 🚀 **cargo-plot** | Generated by cargo-plot v0.2.0 | [GitHub](https://github.com/j-Cis/cargo-plot)
> 🚀 **cargo-plot** | Wygenerowano przez cargo-plot v0.2.0 | [GitHub](https://github.com/j-Cis/cargo-plot)
