# cargo-plot

**cargo-plot** (v0.1.1) to biblioteka napisana w języku Rust (edycja 2024), której autorem jest Gepiden (Jan Roman Cisowski).

Biblioteka `cargo-plot` oferuje bogate API, które pozwala na:

* **Przeszukiwanie plików** w systemie z wykorzystaniem elastycznych reguł uwzględniających i wykluczających.
* **Budowanie struktury drzewa** na podstawie odnalezionych ścieżek.
* **Wizualizowanie struktury katalogów i plików** w postaci sformatowanego czystego tekstu ASCII lub kolorowego wydruku bezpośrednio w konsoli.
* **Generowanie kompleksowych raportów Markdown**, które zawierają zautomatyzowany skan wybranych kodów źródłowych spiętych w jeden plik dokumentacji.

> **Założenia projektowe i dobre praktyki**
>
> Biblioteka została zaprojektowana z rygorystycznym naciskiem na jakość kodu, przejrzystość architektury oraz > użyteczność. Podczas jej tworzenia stosowano następujące zasady i dobre praktyki:
>
> * **DRY (Don't Repeat Yourself)** – eliminacja powielania logiki; reużywalność; pisanie z myślą o ponownym > użyciu.
> * **DDD (Domain-Driven Design)** – Projektowanie zorientowane na domenę; skupianie się na modelu problemu, a > nie na technologii.
> * **SRP (Single Responsibility Principle)** – każda struktura, moduł i funkcja ma jedną, jasno określoną > odpowiedzialność.
> * **LSP (Liskov Substitution Principle)** - Podtypy muszą być wymienialne z typami bazowymi bez łamania > kontraktu.
> * **ISP (Interface Segregation Principle)** - Interfejsy powinny być małe i wyspecjalizowane.
> * **DIP (Dependency Inversion Principle)** - Moduły wysokiego poziomu nie powinny zależeć od modułów niskiego > poziomu.
> * **SoC (Separation of Concerns)** – wyraźny podział odpowiedzialności pomiędzy warstwy i moduły.
> * **LoD (Law of Demeter) & Minimal Dependencies** – minimalizacja zależności między modułami i strukturami; > ograniczanie zewnętrznych zależności dla stabilności projektu.
> * **SSoT (Single Source of Truth)** – unikanie definiowania tego samego stanu lub logiki w wielu miejscach.
> * **Reexporty i dużo małych plików** niż ogromne kilkuset linijkowe!!!
> * **Fail Fast & Fail Early & Defensive Programming** – szybkie wykrywanie i raportowanie błędów.
> * **Error Handling with Result / Option** – jawne i idiomatyczne zarządzanie błędami.
> * **The Boy Scout Rule** – pozostawianie kodu w lepszym stanie, niż go zastało.
> * **Modular Design** – małe, niezależne moduły łatwiejsze w utrzymaniu i testowaniu.
> * **Composition over Inheritance** – preferowanie kompozycji zamiast dziedziczenia.
> * **Encapsulation / Information Hiding** – ukrywanie szczegółów implementacji i eksponowanie tylko niezbędnego > API.
> * **Immutability by Default** – zmienne są niemutowalne, dopiero gdy naprawdę potrzebujesz, użyj `mut`.
> * **Idiomatic Rust** – czytelne, jednoznaczne i zgodne z konwencją nazwy funkcji, struktur, modułów oraz > folderów. Tam gdzie możliwe to po polsku.
> * **Zero-cost Abstractions** – korzystanie z idiomatycznych abstrakcji bez wpływu na wydajność.

---
---

## 🌟 Używasz cargo-plot? Daj znać!

Jeśli wykorzystujesz to narzędzie w swoim projekcie, bardzo chętnie o tym usłyszę! 
To dla mnie największa motywacja do dalszego rozwoju.

* Wyślij mi wiadomość na GitHubie.
* Zostaw gwiazdkę ⭐ pod repozytorium.
* Dodaj swój projekt do listy "Użytkownicy" (otwórz Pull Request!).

## 🛠 Zarządzanie Projektem i Instalacja

Poniżej znajduje się zestawienie najczęściej używanych komend do budowania, testowania i instalacji narzędzia `cargo-plot`.

### 🚀 Szybki Start (Budowanie i Uruchamianie)

* `cargo build` – Kompilacja projektu w trybie debug.
* `cargo run -- <SUBKOMENDA>` – Uruchomienie narzędzia w trybie deweloperskim (np. `cargo run -- plot tree`).
* `cargo build --release` – Kompilacja zoptymalizowana do użytku produkcyjnego (wynik w `target/release/cargo-plot`).
* `cargo run --release -- <SUBKOMENDA>` – Uruchomienie zoptymalizowanej wersji binarnej.

### 📦 Instalacja jako rozszerzenie Cargo

Aby móc używać narzędzia z dowolnego miejsca w systemie za pomocą komendy `cargo plot`:

* `cargo install --path .` – Instalacja narzędzia lokalnie na Twoim komputerze.
* `cargo --list` – Sprawdzenie, czy `plot` widnieje na liście zainstalowanych rozszerzeń Cargo.
* `cargo plot --help` – Wyświetlenie pomocy zainstalowanego narzędzia.
* `cargo uninstall cargo-plot` – Całkowite usunięcie narzędzia z systemu.

**Co zyskujesz dzięki `cargo install`?**

Instalacja przenosi skompilowaną binarkę do folderu `~/.cargo/bin/`. Dzięki temu:

1. Możesz wywoływać `cargo plot` z dowolnego folderu w systemie (nie musisz być wewnątrz katalogu projektu).
2. Nie musisz używać `cargo run --`, co skraca komendy używane w codziennej pracy.

#### 1. Instalacja bezpośrednio z GitHub

Jeśli nie chcesz ręcznie klonować repozytorium, Cargo może pobrać i zainstalować binarkę jednym poleceniem:

* `cargo install --git https://github.com/j-Cis/cargo-plot` – Pobiera najnowszą wersję z gałęzi głównej i instaluje ją w systemie.

#### 2. Klonowanie i instalacja lokalna

Jeśli chcesz mieć dostęp do kodu źródłowego lub skryptów pomocniczych:

* `git clone https://github.com/j-Cis/cargo-plot.git` – Klonuje repozytorium na Twój dysk.
* `cd cargo-plot` – Wejście do katalogu projektu.
* `cargo install --path .` – Instalacja narzędzia z lokalnych plików źródłowych.

#### 3. Zarządzanie zainstalowanym narzędziem

Po instalacji `cargo-plot` staje się integralną częścią Twojego środowiska Rust:

* `cargo plot --help` – Sprawdzenie, czy instalacja przebiegła pomyślnie i wyświetlenie pomocy.
* `cargo --list` – Wyświetlenie listy wszystkich zainstalowanych rozszerzeń Cargo (na liście powinien widnieć `plot`).
* `cargo uninstall cargo-plot` – Całkowite usunięcie narzędzia z Twojego systemu.

### 🧪 Jakość Kodu i Dokumentacja

* `cargo check` – Błyskawiczne sprawdzenie poprawności kodu bez pełnej kompilacji.
* `cargo clippy` – Uruchomienie lintera w celu wykrycia potencjalnych problemów i optymalizacji.
* `cargo fmt` – Automatyczne formatowanie kodu zgodnie ze standardami Rust.
* `cargo doc --no-deps --open` – Wygenerowanie dokumentacji technicznej projektu i otwarcie jej w przeglądarce.

### 🌍 Kompilacja skrośna (Cross-compilation)

Przygotowanie binarek dla różnych systemów operacyjnych (wymaga zainstalowanych odpowiednich targetów przez `rustup`):

| System | Komenda kompilacji |
| --- | --- |
| **Windows 64-bit** | `cargo build --target x86_64-pc-windows-msvc --release` |
| **Windows 32-bit** | `cargo build --target i686-pc-windows-msvc --release` |
| **Linux 64-bit** | `cargo build --target x86_64-unknown-linux-gnu --release` |
| **Linux (musl)** | `cargo build --target x86_64-unknown-linux-musl --release` |
| **macOS (Intel)** | `cargo build --target x86_64-apple-darwin --release` |
| **macOS (M1/M2)** | `cargo build --target aarch64-apple-darwin --release` |

### 📂 Nawigacja deweloperska

* `code .` – Otwarcie całego projektu w Visual Studio Code.
* `ii .` – Otwarcie katalogu projektu w Eksploratorze plików (tylko Windows/PowerShell).

---
---

## 🏅 Stan Projektu i Technologie

[![Rust](https://img.shields.io/badge/Rust-v1.93.1%20%7C%202024_Edition-b8744a.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/cargo-plot.svg)](https://crates.io/crates/cargo-plot)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

### 🛠️ Główne Zależności

Używamy sprawdzonych i wydajnych bibliotek z ekosystemu Rust:

| Biblioteka | Odznaka | Rola w projekcie |
| --- | --- | --- |
| **Clap** | ![Clap](https://img.shields.io/badge/clap-v4.5.60-blue) | Obsługa argumentów wiersza poleceń (CLI). |
| **Inquire** | ![Inquire](https://img.shields.io/badge/inquire-v0.9.4-orange) | Interaktywne menu i zapytania w trybie TUI. |
| **Chrono** | ![Chrono](https://img.shields.io/badge/chrono-v0.4.44-green) | Precyzyjne generowanie unikalnych sygnatur czasowych. |
| **Regex** | ![Regex](https://img.shields.io/badge/regex-v1.12.3-red) | Zaawansowane filtrowanie plików za pomocą wzorców Glob. |
| **Colored** | ![Colored](https://img.shields.io/badge/colored-v3.1.1-yellow) | Kolorowanie drzewa plików w terminalu. |
| **Walkdir** | ![Walkdir](https://img.shields.io/badge/walkdir-v2.5.0-lightgrey) | Szybkie i bezpieczne skanowanie struktury katalogów. |

---
---

## CLI (Interfejs Wiersza Poleceń)

Narzędzie `cargo-plot` zostało zaprojektowane jako oficjalne rozszerzenie (subcommand) dla menedżera pakietów Cargo. Pozwala na błyskawiczne przeszukiwanie systemu plików, wizualizowanie struktury oraz generowanie kompleksowych raportów Markdown bezpośrednio z poziomu terminala.

Architektura CLI opiera się na podkomendach. Każdy poziom narzędzia i każda podkomenda posiada wbudowany system pomocy, który można wywołać dodając flagę `-h` lub `--help` (np. `cargo plot --help`, `cargo plot doc --help`).

### Główne wywołanie

* **`cargo plot`** (brak argumentów) – Uruchamia interaktywny kreator TUI (Text-based User Interface), który poprowadzi Cię krok po kroku przez proces konfiguracji zadania.
* **`cargo plot stamp [OPCJE]`** – Narzędzie pomocnicze. Generuje i wypisuje w konsoli unikalną, ujednoliconą sygnaturę czasową (np. do użycia w Twoich własnych skryptach bash/powershell).
* **`cargo plot tree [OPCJE]`** – Tryb wizualizacji. Buduje strukturę drzewa na podstawie odnalezionych ścieżek i wyrzuca kolorowy wydruk bezpośrednio na standardowe wyjście konsoli.
* **`cargo plot doc [OPCJE]`** – Tryb generatora. Automatycznie skanuje wybrane kody źródłowe i spina je w jeden plik dokumentacji Markdown w folderze `doc/`.
* **`cargo plot dist-copy [OPCJE]`** – Menedżer dystrybucji. Wyciąga skompilowane binarki (wskazane lub automatycznie wykryte wszystkie) z folderu `target/` i organizuje je w ustrukturyzowanym katalogu wydawniczym `dist/`.

---

### Podkomenda: `stamp` - OPCJE

Służy do szybkiego wygenerowania unikalnego znacznika czasu w standardzie biblioteki.

#### Opcje dla komendy `stamp`

* **`-d, --date <RRRR-MM-DD>`** – Zwraca stempel dla konkretnej daty (jeśli nie podano czasu, użyje domyślnie `00:00:00`).
* **`-t, --time <GG:MM:SS>`** – Zwraca stempel dla konkretnej godziny (wymaga podania flagi daty).
* **`-m, --millis <MMM>`** – Opcjonalne milisekundy (wartość od 0 do 999). Używane tylko w połączeniu z flagą `-t`. Domyślnie: `000`.
* *(Brak flag)* – Domyślne zachowanie: błyskawicznie zwraca znacznik dla obecnego, lokalnego czasu (`datestamp_now`).

---

### Podkomenda: `stamp` - Przykład: Generowanie sygnatury (Tryb `stamp`)

Poniższe komendy pokazują, jak uzyskać identyczny rezultat jak w Twoim kodzie źródłowym.

**1. Sygnatura dla konkretnej daty i czasu** (odpowiednik `datestamp(d, t)`):
Wymaga podania flagi `--date` oraz `--time`. Opcjonalnie możesz podać milisekund.

```powershell
# Wywołanie w PowerShell lub Bash
cargo run -- plot stamp --date 2026-03-09 --time 13:51:01 --millis 123

```

* **Wynik**: Otrzymasz sformatowany ciąg, np.: `2026Q1D068W11_Mon09Mar_135101123`.

**2. Sygnatura dla aktualnego czasu** (odpowiednik `datestamp_now()`):
Uruchomienie podkomendy bez żadnych parametrów natychmiast generuje stempel dla chwili obecnej.

```powershell
# Błyskawiczne wygenerowanie stempla "teraz"
cargo run -- plot stamp

```

* **Wynik**: Sygnatura oparta na Twoim lokalnym czasie systemowym.

---

### Podkomenda: `tree` - OPCJE

Służy do błyskawicznego podglądu struktury projektu. Pozwala na testowanie wzorców (Glob) przed wygenerowaniem właściwego raportu.

#### Opcje szybkiego zadania (Globalne flagi)

Te flagi służą do stworzenia pojedynczego, szybkiego zadania skanowania "w locie".

* **`-p, --path <ŚCIEŻKA>`** – Ścieżka bazowa do rozpoczęcia skanowania (Domyślnie: `.`).
* **`--no-default-excludes`** – Wyłącza domyślne ignorowanie folderów technicznych takich jak `.git/`, `target/`, `node_modules/`, `.vs/`, itp..
* **`-e, --exclude <WZORZEC>...`** – Wzorce Glob ignorujące ścieżki i foldery. Odrzucenie następuje na wczesnym etapie skanowania, co znacząco przyspiesza działanie. Można podawać wielokrotnie (np. `-e "./target/" -e "*.toml"`).
* **`-i, --include-only <WZORZEC>...`** – Rygorystyczna biała lista. Jeśli użyta, program pominie wszystko, co nie pasuje do podanych wzorców.
* **`-f, --filter-files <WZORZEC>...`** – Filtruje wyłącznie pliki do wyświetlenia (np. `-f "*.rs"`).
* **`-t, --type <TYP>`** – Tryb wyświetlania węzłów:
* `dirs` – Wyświetla wyłącznie foldery.
* `files` – Wyświetla odnalezione pliki i automatycznie podciąga ich nadrzędne foldery.
* `all` – (Domyślnie) Wyświetla wszystko.

#### Opcje zaawansowanego zarządzania zadaniami

Zastępują szybkie flagi, pozwalając na budowanie skomplikowanych macierzy skanowania.

* **`--task <KLUCZ=WARTOŚĆ>...`** – Tryb Inline Multi-Task. Definiuje pełne zadanie w jednym ciągu znaków. Można używać tej flagi wielokrotnie, a wyniki zostaną połączone. Dostępne klucze: `loc`, `exc`, `inc`, `fil`, `out` (np. `--task loc=.,inc=Cargo.toml,out=files`).
* **`--tasks <PLIK_TOML>`** – Tryb Zewnętrznej Konfiguracji. Wczytuje gotową definicję listy zadań `[[task]]` bezpośrednio z pliku `.toml`.

#### Opcje wyjściowe formatowania

* **`-s, --sort <METODA>`** – Sposób sortowania węzłów drzewa.
* `dirs-first` – Foldery wyświetlane są przed plikami.
* `files-first` – Pliki wyświetlane są przed folderami.
* `alpha` – (Domyślnie) Klasyczne sortowanie alfabetyczne.

---

### Podkomenda: `tree` -  Przykłady wywołań i niuanse terminali

Podczas pracy z wieloliniowymi komendami w środowisku deweloperskim (używając `cargo run --`), należy zwrócić uwagę na znaki kontynuacji linii specyficzne dla danego systemu operacyjnego.

> **Ważna uwaga deweloperska**: Separator `--` po komendzie `cargo run` informuje Cargo, aby nie interpretował kolejnych flag jako własnych parametrów, lecz przekazał je bezpośrednio do skompilowanej binarki `cargo-plot`.

#### Przykład: Złożone skanowanie wielozadaniowe (Multi-Task)

Poniższe komendy wykonują identyczne zadanie: skanują wybrane pliki główne projektu oraz wszystkie pliki `.rs` w folderze `lib`, a następnie wyświetlają je w formie drzewa z plikami na górze.

**1. Wywołanie uniwersalne (jednoliniowe)** – Działa w każdym terminalu (CMD, PowerShell, Bash):

```powershell
cargo run -- plot tree --task "loc=.,inc=./Cargo.toml,inc=./README.md,inc=./src/main.rs,inc=./src/cli.rs,out=files" --task "loc=./src/lib,fil=*.rs,out=files" --sort files-first

```

**2. Wywołanie wieloliniowe w PowerShell (Windows)** – Wymaga znaku grawisu (backtick: ```) na końcu każdej linii:

```powershell
cargo run -- plot tree `
  --task "loc=.,inc=./Cargo.toml,inc=./README.md,inc=./src/main.rs,inc=./src/cli.rs,out=files" `
  --task "loc=./src/lib,fil=*.rs,out=files" `
  --sort files-first

```

**3. Wywołanie wieloliniowe w Bash (Linux/macOS)** – Wymaga znaku backslash (`\`) na końcu każdej linii:

```bash
cargo run -- plot tree \
  --task "loc=.,inc=./Cargo.toml,inc=./README.md,inc=./src/main.rs,inc=./src/cli.rs,out=files" \
  --task "loc=./src/lib,fil=*.rs,out=files" \
  --sort files-first

```

---

### Podkomenda: `doc` - OPCJE

Główny orkiestrator biblioteki. Służy do zautomatyzowanego tworzenia gotowych raportów. **Podkomenda `doc` dziedziczy wszystkie opcje budowania zadań z podkomendy `tree**` (czyli `-p`, `--no-default-excludes`, `-e`, `-i`, `-f`, `-t`, `--task`, `--tasks`).

Oprócz nich posiada dedykowane opcje sterujące procesem zapisu plików i formatowania Markdown:

#### Opcje wejścia/wyjścia (I/O)

* **`--out-dir <ŚCIEŻKA>`** – Określa katalog, w którym zostanie zapisany wygenerowany raport (Domyślnie: `doc`). Program automatycznie utworzy ten folder, jeśli nie istnieje.
* **`-o, --out <NAZWA>`** – Bazowa nazwa pliku wyjściowego. Program automatycznie utworzy folder `doc/`, doklei do nazwy wygenerowaną sygnaturę czasową oraz rozszerzenie `.md` (Domyślnie: `code`).
* **`--dry-run`** (lub **`--simulate`**) – Tryb symulacji (Fail Fast). Przechodzi przez cały proces skanowania, formatowania drzewa oraz identyfikatorów, wypisując podsumowanie w terminalu, ale **blokuje fizyczny zapis na dysku**.

#### Opcje formatowania Markdown

* **`-I, --id-style <STYL>`** – Formatowanie zautomatyzowanych nagłówków sekcji (Identyfikatorów):
* `tag` – (Domyślnie) Pełne tagowanie opisowe (np. `## Plik-RustLibPub_01:`).
* `num` – Numeracja sekwencyjna (np. `## Plik-001:`).
* `none` – Minimalizm (samą ścieżkę, np. `## Plik: ./src/main.rs`).

* **`-T, --insert-tree <METODA>`** – Decyduje o spisie treści na początku dokumentu:
* `dirs-first` – Wkleja drzewo z folderami na górze.
* `files-first` – (Domyślnie) Wkleja drzewo z plikami na górze.
* `none` – Całkowicie wyłącza rysowanie spisu treści.

---

### Podkomenda: `doc` - Przykład: Generowanie raportu z wielu lokalizacji (Multi-Task)

Poniższa komenda zbiera dane z dwóch zadań (pliki główne oraz biblioteka), ustawia prefiks nazwy pliku na `doc`, włącza numerację sekwencyjną sekcji (`id-num`) i generuje spis treści w formacie `files-first`.

**1. Wywołanie wieloliniowe w PowerShell (Windows):**

```powershell
cargo run -- plot doc `
  --task "loc=.,inc=./Cargo.toml,inc=./README.md,inc=./src/main.rs,inc=./src/cli.rs,out=files" `
  --task "loc=./src/lib,fil=*.rs,out=files" `
  --out "doc" `
  --out-dir "doc" `
  --id-style num `
  --insert-tree files-first

```

**2. Wywołanie wieloliniowe w Bash (Linux/macOS):**

```bash
cargo run -- plot doc \
  --task "loc=.,inc=./Cargo.toml,inc=./README.md,inc=./src/main.rs,inc=./src/cli.rs,out=files" \
  --task "loc=./src/lib,fil=*.rs,out=files" \
  --out "doc" \
  --out-dir "doc" \
  --id-style num \
  --insert-tree files-first

```

**3. Wywołanie uniwersalne (jednoliniowe):**

```powershell
cargo run -- plot doc --task "loc=.,inc=./Cargo.toml,inc=./README.md,inc=./src/main.rs,inc=./src/cli.rs,out=files" --task "loc=./src/lib,fil=*.rs,out=files" --out "doc" --out-dir "doc" --id-style num --insert-tree files-first

```

---

### Podkomenda: `dist-copy` - OPCJE

Automatyzuje proces przygotowywania paczek do wydań (releases). Przeszukuje katalog kompilacji Rusta i kopiuje wskazane pliki wykonywalne (lub automatycznie wykrywa wszystkie prawdziwe binarki, sprytnie odsiewając pliki techniczne Rusta) do uporządkowanej struktury z podziałem na system operacyjny i profil (np. `dist/windows/release/`).

#### Opcje dla komendy `dist-copy`

* **`-b, --bin <NAZWA>...`** – Nazwa pliku binarnego (bez rozszerzenia `.exe`), którego program ma szukać. Flagę można podać wielokrotnie, aby skopiować kilka konkretnych plików (np. `-b server -b client`). **Jeśli nie podano tej flagi, program użyje wbudowanej heurystyki i skopiuje WSZYSTKIE odnalezione pliki wykonywalne**.
* **`--target-dir <ŚCIEŻKA>`** – Ścieżka do technicznego folderu kompilacji Rusta (Domyślnie: `./target`).
* **`--dist-dir <ŚCIEŻKA>`** – Ścieżka do docelowego folderu, w którym ma zostać zbudowana struktura dystrybucyjna (Domyślnie: `./dist`).
* **`--clear`** – Czyści (usuwa) cały folder dystrybucyjny (`dist-dir`) przed rozpoczęciem skanowania i kopiowania, gwarantując paczkę bez starych artefaktów.
* **`--no-overwrite`** – Zabezpiecza przed nadpisaniem istniejących plików w folderze docelowym (domyślnie program nadpisuje pliki).
* **`--dry-run`** (lub **`--simulate`**) – Tryb symulacji (Fail Fast). Przeszukuje folder `target`, rozpoznaje architektury i systemy operacyjne, wypisując w konsoli informację o tym, co *zostałoby* skopiowane, ale **nie wykonuje absolutnie żadnych zapisów ani usunięć na fizycznym dysku**.

---

### Podkomenda: `dist-copy` - Przykład: Przygotowanie czystej dystrybucji

Komenda ta wyczyści folder `./dist`, a następnie spróbuje skopiować binarkę `cargo-plot` z folderu `target`, dbając o to, by nie nadpisać istniejących plików, jeśli proces czyszczenia by je pominął.

**1. Wywołanie wieloliniowe w PowerShell (Windows):**

```powershell
cargo run -- plot dist-copy `
  --bin "cargo-plot" `
  --target-dir "./target" `
  --dist-dir "./dist" `
  --clear `
  --no-overwrite

```

**2. Wywołanie wieloliniowe w Bash (Linux/macOS):**

```bash
cargo run -- plot dist-copy \
  --bin "cargo-plot" \
  \--target-dir "./target" \
  \--dist-dir "./dist" \
  --clear \
  --no-overwrite

```

**3. Wywołanie uniwersalne (jednoliniowe):**

```powershell
cargo run -- plot dist-copy --bin "cargo-plot" --target-dir "./target" --dist-dir "./dist" --clear --no-overwrite

```

---
---

### Przykłady wywołań CLI

**1. Szybki podgląd drzewa w konsoli (tylko pliki Rust, wykluczenie katalogu target):**

```bash
cargo plot tree -p . -e "./target/" -f "*.rs" -t files --sort dirs-first

```

**2. Symulacja generowania raportu (Dry Run) z użyciem zewnętrznego pliku konfiguracyjnego:**

```bash
cargo plot doc --tasks ./config/plot_tasks.toml --dry-run

```

**3. Generowanie docelowego raportu przy użyciu Inline Multi-Task (wiele zadań naraz):**

```bash
cargo plot doc \
  --task loc=.,inc=Cargo.toml,inc=src/main.rs,out=files \
  --task loc=./src/lib,fil=*.rs,out=files \
  --out "raport_architektury" \
  --id-style num \
  --insert-tree files-first

```

**4. Wyświetlenie pełnego ekranu pomocy dla danej podkomendy:**

```bash
cargo plot doc --help

```

---
---

## TUI

---
---

## API -> BIBLIOTEKA

Biblioteka `cargo-plot` oferuje bogate API do przeszukiwania plików, budowania z nich struktury drzewa, wizualizowania jej w konsoli lub czystym tekście oraz generowania raportów Markdown zawierających skan wybranych kodów źródłowych.

### API

Oto pełna instrukcja opisująca możliwości użycia:

#### 1. Moduł `fn_datestamp` (Generowanie sygnatur czasowych)

Zapewnia funkcje do tworzenia spójnych sygnatur czasowych (np. `2026Q1D069W11_Tue10Mar_063950222`).

* **`pub use chrono::{NaiveDate, NaiveTime}`** – Wygodny reeksport typów. Użytkownik API nie musi dodawać zależności `chrono` do swojego `Cargo.toml`, aby ręcznie definiować czas.
* **`datestamp_now() -> String`** – Generuje sygnaturę dla obecnego, lokalnego czasu.
* **`datestamp(date: NaiveDate, time: NaiveTime) -> String`** – Generuje sygnaturę dla precyzyjnie wskazanej daty i czasu (używając udostępnionych przez moduł typów).

#### 2. Moduł `fn_path_utils` (Narzędzia ścieżek)

Służy do formatowania ścieżek w jednolity sposób niezależnie od systemu operacyjnego.

* **`standardize_path(path: &Path) -> String`** – Konwertuje ścieżkę do stringa, zamienia ukośniki na uniksowe (`/`) i usuwa windowsowe prefiksy rozszerzone (`//?/`).
* **`to_display_path(path: &Path, base_dir: &Path) -> String`** – Zwraca czystą, relatywną ścieżkę (np. `./src/main.rs`), jeśli podany `path` znajduje się wewnątrz `base_dir`. W przeciwnym razie zwraca po prostu ustandaryzowaną ścieżkę bazową.

#### 3. Moduł `fn_pathtype` (Typowanie plików i ikony)

Dostarcza jedno źródło prawdy (SSoT) dla ikon drzewa oraz oznaczania języków w Markdown.

* **Struktura `PathFileType`** – Przechowuje publiczne pola: `icon: &'static str` oraz `md_lang: &'static str`.
* **Stała `DIR_ICON: &str`** – Globalna ikona zdefiniowana dla folderów.
* **`get_file_type(ext: &str) -> PathFileType`** – Oczekuje rozszerzenia pliku (np. `"rs"`) i zwraca odpowiadający mu język Markdown oraz ikonę. Obsługiwane rozszerzenia: `rs`, `toml`, `slint`, `md`, `json`, `yaml`/`yml`, `html`, `css`, `js`, `ts` oraz domyślny fallback jako `text`.

#### 4. Moduł `fn_filespath` (Silnik wyszukiwania ścieżek)

Główne narzędzie API do filtrowania i pozyskiwania ścieżek z systemu plików.

* **Struktura `Task<'a>`** – Definiuje zadanie wyszukiwania. Posiada następujące publiczne pola:
  * `path_location: &'a str` – Ścieżka początkowa (domyślnie `"."`).
  * `path_exclude: Vec<&'a str>` – Wzorce Glob (np. `"folder/*"`) do ignorowania. Odrzucenie następuje już na etapie skanowania, oszczędzając czas.
  * `path_include_only: Vec<&'a str>` – Ścisłe wzorce uwzględniające (np. `"*.rs"`). Jeśli lista nie jest pusta, narzędzie pominie wszystko, co nie pasuje.
  * `filter_files: Vec<&'a str>` – Dodatkowe odfiltrowanie samych plików po etapie skanowania.
  * `output_type: &'a str` – Steruje wynikiem. Przyjmuje `"dirs"`, `"files"` lub `"dirs_and_files"` (domyślne). Posiada ukrytą logikę dla trybu `"files"`, która automatycznie podciąga nadrzędne foldery, aby drzewo się nie spłaszczyło.
* **Wdrożony Trait `Default` dla `Task`** – Możliwość tworzenia zadania przez `..Default::default()`.
* **`filespath(tasks: &[Task]) -> Vec<PathBuf>`** – Funkcja konsumująca referencję do listy zadań i zwracająca unikalny wektor znalezionych ścieżek.

#### 5. Moduł `fn_filestree` (Budowanie struktury drzewa)

Przetwarza płaską listę ścieżek na hierarchiczną strukturę danych.

* **Struktura `FileNode`** – Węzeł drzewa zawierający pola: `name: String`, `path: PathBuf`, `is_dir: bool`, `icon: String`, `children: Vec<FileNode>`.
* **`filestree(paths: Vec<PathBuf>, sort_method: &str) -> Vec<FileNode>`** – Buduje wektor węzłów ze ścieżek przypisując do nich automatycznie ikony. Obsługuje trzy sposoby sortowania argumentem `sort_method`:
  * `"files-first"` – Pliki wyświetlane są przed folderami.
  * `"dirs-first"` – Foldery wyświetlane są przed plikami.
  * Dowolna inna wartość – Sortuje domyślnie tylko alfabetycznie.

#### 6. Moduł `fn_plotfiles` (Wizualizacja drzewa)

Zamienia obiekt `FileNode` w wizualne drzewo ASCII.

* **Struktura `TreeStyle`** – Udostępnia pełną kontrolę nad znakami graficznymi (box-drawing) budującymi gałęzie. Posiada publiczne pola: `dir_last_with_children`, `dir_last_no_children`, `dir_mid_with_children`, `dir_mid_no_children`, `file_last`, `file_mid`, `indent_last`, `indent_mid`.
* **Wdrożony Trait `Default` dla `TreeStyle`** – Inicjalizuje klasyczne załamania typu `└──┬`, `├───` itp..
* **`plotfiles_txt(nodes: &[FileNode], indent: &str, style: Option<&TreeStyle>) -> String`** – Generuje strukturę w postaci czystego tekstu (bez kolorów), z podanym wcięciem początkowym. Styl jest opcjonalny (można użyć `None`).
* **`plotfiles_cli(nodes: &[FileNode], indent: &str, style: Option<&TreeStyle>) -> String`** – Generuje gotowy do wydruku (do konsoli CLI) pokolorowany ciąg znaków (zielone gałęzie, kolorowe ikony, różnicowane foldery i pliki) z wykorzystaniem biblioteki `colored`.

#### 7. Moduł `fn_doc_models` (Model zadania generatora dokumentacji)

Definiuje strukturę wejściową dla orkiestratora raportów.

* **Struktura `DocTask<'a>`** – Posiada publiczne pola konfigurujące raport Markdown:
  * `output_filename: &'a str` – Pierwszy człon docelowej nazwy pliku.
  * `insert_tree: &'a str` – Kontroluje rzutowanie drzewa struktury katalogów na początku dokumentu: `"dirs-first"`, `"files-first"` lub `"with-out"` (całkowicie wyłącza rysowanie drzewa).
  * `id_style: &'a str` – Odpowiada za format nagłówków: `"id-tag"`, `"id-num"`, `"id-non"`.
  * `tasks: Vec<Task<'a>>` – Tablica zadań przeszukiwania pochodząca z modułu `fn_filespath`.

#### 8. Moduł `fn_doc_id` (Generowanie etykiet identyfikacyjnych)

* **`generate_ids(paths: &[PathBuf]) -> HashMap<PathBuf, String>`** – Iteruje po wektorze ścieżek plików i nadaje każdemu specjalny ciąg identyfikacyjny według twardych i dynamicznych reguł (np. dla `"Cargo.toml"` nadaje ID `"TomlCargo"`, główny `mod.rs` w bibliotece otrzyma `"RustLibMod_00"`, a ogólne pliki dostaną np. `"FileMd"` lub `"RustBin_01"`). Zwraca mapowanie ułatwiające nadawanie nagłówków w raporcie.

#### 9. Moduł `fn_doc_write` (Fizyczny zapis raportu)

* **`write_md(out_path: &str, files: &[PathBuf], id_map: &HashMap<PathBuf, String>, tree_text: Option<String>, _stamp: &str, id_style: &str) -> io::Result<()>`** – Funkcja, która parsuje całe wejście: otwiera i zczytuje na bieżąco faktyczną treść plików źródłowych i opakowuje je w bloki kodowe Markdown używając języka wskazanego w module typowania. **Posiada wbudowany wyjątek dla plików `.md`** – zamiast w bloki kodowe, ich treść jest rzutowana linijka po linijce jako cytaty (znak `>`), co zapobiega uszkodzeniom struktury raportu przez zagnieżdżone znaczniki kodu włączanego pliku. Ostatecznie funkcja zapisuje cały złączony tekst do fizycznego pliku na dysku, transparentnie przekazując ewentualne błędy systemowe I/O poprzez `Result`.

#### 10. Moduł `fn_doc_gen` (Orkiestracja generowania dokumentacji)

Moduł, który łączy wszystkie powyższe silniki w jeden proces.

* **`generate_docs(doc_tasks: Vec<DocTask>, output_dir: &str) -> io::Result<()>`** – Główna i jedyna funkcja wysokiego poziomu w tym module. Iteruje po zadaniach typu `DocTask`, asynchronicznie (pod kątem działania, nie `async` Rusta) tworzy znacznik czasu, buduje pełną nazwę pliku, pobiera ścieżki (`filespath`), rysuje drzewo (`filestree` + `plotfiles_txt`), nakłada identyfikatory (`generate_ids`), wymusza utworzenie folderu wynikowego (o ile nie istnieje) i przekazuje to do zapisania na dysku (`write_md`). Wydrukuje w konsoli informację po wygenerowaniu każdego pliku. Zwraca transparentny `Result` do obsługi ewentualnych braków uprawnień odczytu/zapisu na dysku.

#### 11. Moduł `fn_copy_dist` (Zarządzanie paczkami dystrybucyjnymi)

Służy do zautomatyzowanego wyciągania skompilowanych plików wykonywalnych z technicznego folderu `target/` i organizowania ich w czystą, zorganizowaną strukturę gotową do dystrybucji (np. pod wydania (releases) na GitHubie).

* **Struktura `DistConfig<'a>`** – Wdraża wzorzec *Parameter Object*, udostępniając zaawansowaną kontrolę nad wdrażaniem (deploymentem). Posiada następujące pola:
  * `target_dir: &'a str` / `dist_dir: &'a str` – Wskazują katalog źródłowy i docelowy.
  * `binaries: Vec<&'a str>` – Lista poszukiwanych plików. **Potężna heurystyka**: Jeśli podasz pustą listę (`vec![]`), funkcja spróbuje automatycznie odnaleźć i przenieść **wszystkie** skompilowane pliki wykonywalne, sprytnie odrzucając "śmieci" kompilacyjne (jak pliki `.d`, `.rlib`, `.pdb` itp.).
  * `clear_dist: bool` – Bezpiecznie usuwa stary folder dystrybucji przed nowym kopiowaniem.
  * `overwrite: bool` – Pozwala zablokować nadpisywanie już istniejących plików.
  * `dry_run: bool` – Wbudowany tryb symulacji. Program przeszukuje foldery, mapuje ścieżki i zwraca wynik, ale **nie wykonuje absolutnie żadnych zapisów/usunięć na dysku** (Defensive Programming).
* **Wdrożony Trait `Default` dla `DistConfig`** – Pozwala na wywołanie domyślnej, bezpiecznej konfiguracji (bez czyszczenia, z domyślnymi ścieżkami).
* **`copy_dist(config: &DistConfig) -> io::Result<Vec<(PathBuf, PathBuf)>>`** – Funkcja przeszukująca katalog kompilacji.
  * Rozpoznaje profile (`release`, `debug`) oraz systemy natywne i krzyżowe (*Target Triples* np. `windows`, `linux`, `macos`, `android`, `wasm`).
  * Kopiuje pliki tworząc intuicyjną hierarchię według klucza `<system_operacyjny>/<profil>` (np. `dist/linux/release` lub `dist/windows/debug`).
  * Realizuje założenia *Fail Fast* i *SRP* – nie drukuje niczego samowolnie w terminalu. Jeśli katalog źródłowy nie istnieje, natychmiast rzuca błąd. Zwraca `Ok` zawierające wektor krotek ze ścieżkami `(Źródło, Cel)`, oddając pełną kontrolę nad prezentacją informacji (np. w konsoli) programiście.

#### 12. Moduł `fn_files_blacklist` (Weryfikacja typów danych)

Dostarcza mechanizmy ochronne (Defensive Programming), które zapobiegają wczytywaniu do pamięci RAM ogromnych lub nieczytelnych plików binarnych podczas generowania raportów tekstowych.

* **`is_blacklisted_extension(ext: &str) -> bool`** – Funkcja weryfikująca rozszerzenie pliku pod kątem przynależności do rozbudowanej "czarnej listy".
  * **Zakres ochrony**: Obejmuje grafikę (np. `.png`, `.psd`), binarki i artefakty kompilacji (np. `.exe`, `.rlib`, `.obj`), archiwa (np. `.zip`, `.iso`), bazy danych (np. `.sqlite`), dokumenty biurowe (np. `.docx`, `.pdf`), fonty oraz media (audio/wideo).
  * **Zastosowanie**: Wykorzystywana przez silnik zapisu (`write_md`) do filtrowania plików przed próbą ich odczytu. Jeśli rozszerzenie znajduje się na liście, generator pomija treść pliku, wstawiając jedynie stosowną informację w raporcie, co drastycznie redukuje zużycie zasobów systemowych i zapobiega błędom kodowania UTF-8.

### Użycie - Przykłady

Biblioteka `cargo-plot` została zaprojektowana w sposób modułowy. Poniżej znajdują się kompleksowe przykłady użycia pokazujące krok po kroku, jak w pełni wykorzystać możliwości naszego API w Twoich plikach binarnych.

#### 1. Generowanie precyzyjnych sygnatur czasowych

Moduł `fn_datestamp` jest idealny do oznaczania generowanych plików raportów jednolitym formatem. Możesz wygenerować znacznik dla aktualnego czasu lub wymusić własny:

```rust
use lib::fn_datestamp::{datestamp, datestamp_now, NaiveDate, NaiveTime};

fn main() {
    // Użycie z precyzyjnie podaną datą i czasem
    let d = NaiveDate::from_ymd_opt(2026, 3, 9).unwrap();
    let t = NaiveTime::from_hms_milli_opt(13, 51, 1, 123).unwrap();
    let s1 = datestamp(d, t);
    println!("Datestamp z podanym czasem: {}", s1);

    // Błyskawiczne użycie dla aktualnego, lokalnego czasu
    let s2 = datestamp_now();
    println!("Datestamp teraz: {}", s2);
}
```

#### 2. Skanowanie plików i wizualizacja struktury drzewa ASCII

Narzędzie pozwala zdefiniować złożone reguły wyszukiwania (wzorce ignorujące, wymuszające i filtrujące), a następnie przetworzyć wyniki w piękne drzewo plików, które można zapisać do czystego pliku Markdown lub wydrukować w kolorze w konsoli.

```rust
use lib::fn_filespath::{filespath, Task};
use lib::fn_filestree::{filestree, FileNode};
use lib::fn_plotfiles::{plotfiles_cli, plotfiles_txt};

fn main() {
    // 1. Definiowanie tablicy zadań skanowania systemu plików
    let tasks = vec![
        Task {
            output_type: "files",
            // Dzięki wdrożeniu cechy Default, nie musimy wypełniać pustych filtrów
            ..Default::default() 
        },
    ];

    // 2. Zebranie unikalnych ścieżek
    let paths = filespath(&tasks);

    // 3. Zbudowanie struktury węzłów drzewa z sortowaniem "pliki na górze"
    let tree_ff: Vec<FileNode> = filestree(paths.clone(), "files-first");
    
    // 4a. Generowanie czystego tekstu ASCII ze strukturą (np. do Markdowna)
    let txt = plotfiles_txt(&tree_ff, "", None);
    println!("MARKDOWN:\n{}", txt);

    // 3b. Zbudowanie struktury z sortowaniem "foldery na górze"
    let tree_df: Vec<FileNode> = filestree(paths.clone(), "dirs-first");
    
    // 4b. Wyświetlanie pokolorowanego, czytelnego drzewa w terminalu CLI
    let cli = plotfiles_cli(&tree_df, "", None);
    println!("CLI:\n{}", cli);
}
```

#### 3. Orkiestracja i generowanie pełnych raportów Markdown

Najpotężniejszą funkcją biblioteki jest `generate_docs`, która łączy wszystkie silniki: skanowanie, rysowanie drzewa, nadawanie identyfikatorów i zapisywanie plików kodowych bezpośrednio do fizycznego katalogu.

```rust
use lib::fn_filespath::Task;
use lib::fn_doc_models::DocTask;
use lib::fn_doc_gen::generate_docs;

fn main() {
    // Konfigurujemy jedno lub więcej niezależnych zadań generowania dokumentacji
    let doc_tasks = vec![
        DocTask {
            output_filename: "doc", // Przedrostek tworzonego pliku
            insert_tree: "files-first", // Jak umieścić wizualizację drzewa w pliku
            id_style: "id-num", // Metoda oznaczania nagłówków poszczególnych plików
            
            // Definiujemy, co fizycznie ma zostać wyciągnięte do raportu:
            tasks: vec![
                Task {
                    path_location: ".",
                    path_include_only: vec!["./Cargo.toml", "./src/main.rs"],
                    output_type: "files",
                    ..Default::default()
                },
                Task {
                    path_location: "./src/lib",
                    filter_files: vec!["*.rs"], // Chcemy tylko pliki Rust
                    output_type: "files",
                    ..Default::default()
                },
            ],
        },
    ];

    // Uruchamiamy orkiestrator. 
    // Funkcja tworzy katalog wyjściowy ("doc"), generuje datestamp i zapisuje raport.
    // Transparentnie zwraca Result, by móc obsłużyć potencjalne błędy zapisu I/O.
    if let Err(e) = generate_docs(doc_tasks, "doc") {
        eprintln!("[-] KRYTYCZNY BŁĄD podczas generowania dokumentacji: {}", e);
    } else {
        println!("[+] Zakończono generowanie wszystkich raportów!");
    }
}
```

#### 4. Ekstrakcja binarek i przygotowanie dystrybucji

Moduł `fn_copy_dist` automatyzuje żmudny proces wyciągania skompilowanych plików wykonywalnych z folderu `target/` po wykonaniu `cargo build`. Poniższy przykład pokazuje, jak wywołać funkcję kopiującą i samodzielnie obsłużyć wynik (zgodnie z zasadą SRP), drukując ładne podsumowanie operacji w terminalu.

```rust
use lib::fn_copy_dist::{copy_dist, DistConfig};

fn main() {
    println!("[*] Rozpoczynam przygotowanie paczek dystrybucyjnych...");

    // Pełna kontrola nad dystrybucją dzięki strukturze konfiguracyjnej
    let config = DistConfig {
        target_dir: "target",
        dist_dir: "dist",
        binaries: vec!["cargo-plot"], // Zostaw puste vec![], by pobrać wszystkie!
        clear_dist: true,             // Wyczyści stare pliki w ./dist
        overwrite: false,             // Nie nadpisze istniejących (choć tu clear_dist i tak usunie folder)
        dry_run: false,               // Zmień na true, by tylko zasymulować
    };

    match copy_dist(&config) {
        Ok(skopiowane) => {
            if skopiowane.is_empty() {
                println!(" [-] Brak plików. Upewnij się, że użyto `cargo build`.");
            } else {
                for (src, cel) in skopiowane {
                    println!(" [+] Skopiowano: {} -> {}", src.display(), cel.display());
                }
            }
        },
        Err(e) => eprintln!("[-] KRYTYCZNY BŁĄD: {}", e),
    }
}
```

---
---
