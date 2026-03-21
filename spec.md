
# Specyfikacja Wzorców Dopasowań (Pattern Matching Specification)

-----

### 1\. Standardowe modyfikatory dopasowań (Globbing & Wildcards)

Warstwa parsowania przekształcająca znaki tekstowe w reguły wyrażeń regularnych, wspierająca opcjonalną wrażliwość na wielkość liter.

| Wzorzec | Nazwa techniczna | Zachowanie silnika |
| :--- | :--- | :--- |
| `*` | Single-level Wildcard | **[POL]:** Dopasowuje zero lub więcej znaków w obrębie jednego poziomu (nie dopasowuje `/`).<br>**[ENG]:** Matches zero or more characters within a single level (does not match `/`). |
| `**` | Multi-level Wildcard | **[POL]:** Dopasowuje dowolną liczbę znaków łącznie z separatorami `/` (rekurencja wielopoziomowa).<br>**[ENG]:** Matches any number of characters including `/` separators (multi-level recursion). |
| `?` | Single Character | **[POL]:** Dopasowuje dokładnie jeden dowolny znak, z wyłączeniem separatora `/`.<br>**[ENG]:** Matches exactly one arbitrary character, excluding the `/` separator. |
| `{a,b}` | Brace Expansion | **[POL]:** Middleware rozwijający wzorzec na oddzielne ścieżki przed kompilacją (obsługuje rekurencję).<br>**[ENG]:** Middleware expanding the pattern into separate paths before compilation (supports recursion). |
| `[a-z]` | Character Class | **[POL]:** Dopasowuje jeden znak z podanego zakresu lub zbioru.<br>**[ENG]:** Matches one character from the specified range or set. |
| `\` | Escape Character | **[POL]:** Traktuje następny znak dosłownie (np. `\.` dopasowuje kropkę).<br>**[ENG]:** Treats the next character literally (e.g. `\.` matches a dot). |

-----

### 2\. Kotwiczenie i Typowanie (Target Typing)

Bariery logiczne analizujące surowy wzorzec w celu precyzyjnego ustalenia docelowych obiektów.

| Wzorzec | Nazwa techniczna | Zachowanie silnika |
| :--- | :--- | :--- |
| `./...` | Root Anchor | **[POL]:** Wymusza szukanie ścieżki dokładnie od korzenia skanowanego środowiska.<br>**[ENG]:** Enforces path searching exactly from the root of the scanned environment. |
| `.../` | Directory Target | **[POL]:** Wzorzec kończący się ukośnikiem. Dopasowuje wyłącznie katalogi.<br>**[ENG]:** Pattern ending with a slash. Matches directories exclusively. |
| `!` | Hard Veto | **[POL]:** Flaga negacji. Dopasowanie negatywne bezwzględnie odrzuca ścieżkę, unieważniając inne reguły.<br>**[ENG]:** Negation flag. A negative match unconditionally rejects the path, overriding other rules. |

-----

### 3\. Flagi Relacji Strukturalnych (Rdzeń Logiki 0.2.0)

Autorskie modyfikatory weryfikujące zależności w strukturze plików na podstawie globalnego indeksu środowiska (`env`).

| Flaga | Nazwa techniczna | Zasada działania weryfikatora kontekstowego |
| :--- | :--- | :--- |
| **`@`** | Sibling Requirement | **[POL]:** Wymaga istnienia pary plik + katalog o tej samej nazwie rdzennej (np. `core.rs` wymaga `core/`).<br>**[ENG]:** Requires a file + directory pair of the same core name (e.g. `core.rs` requires `core/`). |
| **`$`** | Orphan Requirement | **[POL]:** Przeciwieństwo `@`. Dopasowuje element tylko wtedy, gdy w środowisku brakuje odpowiadającej mu pary.<br>**[ENG]:** Opposite of `@`. Matches an element only if its corresponding pair is missing from the environment. |
| **`+`** | Deep Authorization | **[POL]:** Tryb głęboki. Jeśli korzeń modułu spełnia warunek `@`, silnik akceptuje całą zawartość jego poddrzewa.<br>**[ENG]:** Deep mode. If the module root satisfies the `@` condition, the engine accepts the entire contents of its subtree. |

-----

### API Wewnętrzne v0.2.0 (Core API)

-----

### 1\. Skanowanie i Magazyn Danych (`core::path_store`)

Moduł odpowiedzialny za pozyskiwanie i indeksowanie ścieżek z dysku.

| Sygnatura Metody | Opis Techniczny |
| :--- | :--- |
| `PathStore::scan(dir_path) -> Self` | **[POL]:** Wykonuje rekurencyjny odczyt drzewa (pomijając głębokość 0 i symlinki). Normalizuje separatory na `/`.<br>**[ENG]:** Performs a recursive tree read (ignoring depth 0 and symlinks). Normalises separators to `/`. |
| `PathStore::get_index(&self) -> HashSet<&str>` | **[POL]:** Tworzy szybki indeks referencyjny (`env`) niezbędny dla flag relacyjnych `@` i `$`.<br>**[ENG]:** Creates a fast reference index (`env`) necessary for `@` and `$` relational flags. |

-----

### 2\. Klasyfikacja Wizualna (`theme::for_path_list`)

Moduł definiujący tożsamość wizualną obiektów.

| Funkcja | Opis Techniczny |
| :--- | :--- |
| `get_icon_for_path(path) -> &'static str` | **[POL]:** Przypisuje emoji (`📁`, `🗃️`, `📄`, `⚙️`) na podstawie typu obiektu i statusu ukrycia.<br>**[ENG]:** Assigns emojis (`📁`, `🗃️`, `📄`, `⚙️`) based on object type and hidden status. |

-----

### 3\. Silnik Dopasowujący i Sortujący (`core::path_matcher`)

Zunifikowana warstwa kompilacji wzorców i porządkowania wyników.

| Komponent | Opis Techniczny |
| :--- | :--- |
| `PatternContext` | **[POL]:** Middleware zarządzający rozwijaniem klamer `{a,b}` i przechowywaniem wzorców surowych (`raw`) oraz przetworzonych (`tok`). |
| `SortStrategy` | **[POL]:** Definiuje 11 strategii sortowania, w tym zaawansowane tryby `Merge` grupujące logiczne moduły Rusta. |
| `PathMatchers` | **[POL]:** Koordynator zarządzający kolekcją silników. Implementuje model logiczny OR dla wzorców pozytywnych i twarde weto dla negatywnych. |

-----

### 4\. Orkiestracja Wykonania (`execute`)

| Funkcja | Opis Techniczny |
| :--- | :--- |
| `execute(...) -> MatchStats` | **[POL]:** Główny procesor łączący skanowanie, dopasowywanie i budowanie widoków (`Tree`, `List`, `Grid`) w jedną atomową operację.<br>**[ENG]:** Primary processor combining scanning, matching, and building views (`Tree`, `List`, `Grid`) into a single atomic operation. |

-----

> [\!NOTE]
> [POL]: Niniejsza specyfikacja stanowi oficjalną dokumentację technologiczną wersji 0.2.0 i jest nadrzędna względem komentarzy w kodzie źródłowym.
>
> [ENG]: This specification constitutes the official technological documentation for version 0.2.0 and takes precedence over comments in the source code.
