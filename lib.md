
### Specyfikacja Wzorców Dopasowań (Pattern Matching Specification)

---

### 1. Standardowe modyfikatory dopasowań (Globbing & Wildcards)

Warstwa parsowania przekształcająca znaki tekstowe w reguły wyrażeń regularnych.

| Wzorzec | Nazwa techniczna | Zachowanie silnika |
| :--- | :--- | :--- |
| `*` | Single-level Wildcard | **[POL]:** Dopasowuje zero lub więcej znaków, ale nie przekracza granicy katalogu (nie dopasowuje `/`).<br><br>**[ENG]:** Matches zero or more characters but does not cross the directory boundary (does not match `/`). |
| `**` | Multi-level Wildcard | **[POL]:** Dopasowuje dowolną liczbę znaków łącznie z separatorami katalogów `/` (rekurencja wielopoziomowa).<br><br>**[ENG]:** Matches any number of characters including directory separators `/` (multi-level recursion). |
| `?` | Single Character | **[POL]:** Dopasowuje dokładnie jeden dowolny znak, z wyłączeniem separatora `/`.<br><br>**[ENG]:** Matches exactly one arbitrary character, excluding the `/` separator. |
| `{a,b}` | Brace Expansion | **[POL]:** Middleware. Klonuje i rozwija wzorzec na oddzielne ścieżki przed kompilacją. Z `src/{a,b}.rs` generuje `src/a.rs` oraz `src/b.rs`. Obsługuje rekurencję.<br><br>**[ENG]:** Middleware. Clones and expands the pattern into separate paths before compilation. From `src/{a,b}.rs` it generates `src/a.rs` and `src/b.rs`. Supports recursion. |
| `[a-z]` | Character Class | **[POL]:** Dopasowuje dokładnie jeden znak z podanego zakresu lub zbioru.<br><br>**[ENG]:** Matches exactly one character from the specified range or set. |
| `[!a-z]` | Negated Class | **[POL]:** Dopasowuje jeden znak, który nie należy do podanego zbioru.<br><br>**[ENG]:** Matches one character that does not belong to the specified set. |
| `\` | Escape Character | **[POL]:** Traktuje następny znak dosłownie (np. `\.` dopasowuje kropkę, a nie dowolny znak).<br><br>**[ENG]:** Treats the next character literally (e.g. `\.` matches a dot, not an arbitrary character). |

---

### 2. Kotwiczenie i Typowanie (Target Typing)

Bariery logiczne analizujące surowy wzorzec w celu precyzyjnego ustalenia docelowych obiektów w systemie plików.

| Wzorzec | Nazwa techniczna | Zachowanie silnika |
| :--- | :--- | :--- |
| `./...` | Root Anchor | **[POL]:** Wymusza szukanie ścieżki dokładnie od korzenia skanowanego środowiska.<br><br>**[ENG]:** Enforces path searching exactly from the root of the scanned environment. |
| `.../` | Directory Target | **[POL]:** Wzorzec kończący się ukośnikiem. Natychmiast wyklucza pliki. Dopasowuje wyłącznie katalogi.<br><br>**[ENG]:** Pattern ending with a slash. Instantly excludes files. Matches directories exclusively. |
| *(brak)* | File Target | **[POL]:** Bariera logiczna. Jeśli znormalizowany wzorzec nie kończy się na `/` ani na `**`, silnik traktuje go jako wzorzec plikowy i odrzuca badane katalogi.<br><br>**[ENG]:** Logical barrier. If the normalised pattern does not end with `/` or `**`, the engine treats it as a file pattern and rejects examined directories. |

---

### 3. Flagi Relacji Strukturalnych (Rdzeń Logiki Biznesowej)

Autorskie modyfikatory zachowania bazujące na stanie globalnego środowiska plików (`env`), weryfikujące istnienie zależności w strukturze na dysku.

| Flaga | Nazwa techniczna | Zasada działania weryfikatora kontekstowego |
| :--- | :--- | :--- |
| **`@`** | Sibling Requirement<br>*(Relacja Obustronna)* | **[POL]:** Dla plików: Wymaga istnienia katalogu o tej samej nazwie rdzennej (np. `A.rs` wymaga `A/`). Dla katalogów: Wymaga istnienia pliku o tej samej nazwie rdzennej obok (np. `A/` wymaga `A.rs` lub `.A.rs`).<br><br>**[ENG]:** For files: Requires a directory with the same core name (e.g. `A.rs` requires `A/`). For directories: Requires a file with the same core name (e.g. `A/` requires `A.rs` or `.A.rs`). |
| **`$`** | Orphan Requirement<br>*(Relacja Jednostronna)* | **[POL]:** Działa tylko na wzorce plikowe. Wymaga, aby dopasowany plik posiadał odpowiadający mu katalog (podobnie jak `@`). Nie nakłada żadnych restrykcji na same katalogi.<br><br>**[ENG]:** Acts on file patterns only. Requires the matched file to have a corresponding directory (similar to `@`). Does not impose any restrictions on directories themselves. |
| **`+`** | Deep Root Authorization<br>*(Rekurencja Autoryzowana)* | **[POL]:** Działa w symbiozie z `@`. Weryfikuje, czy korzeń modułu zdefiniowany we wzorcu posiada autoryzowaną relację `@` (plik + katalog). Jeśli korzeń jest poprawny, silnik akceptuje wszystko w jego poddrzewie.<br><br>**[ENG]:** Works in symbiosis with `@`. Verifies if the module root defined in the pattern possesses an authorised `@` relation (file + directory). If the root is valid, the engine accepts everything within its subtree. |


### API Wewnętrzne (Core API)

---

### 1. Skanowanie i Normalizacja (`core::path_getter`)

Moduł generujący bazowy zbiór znormalizowanych ścieżek wejściowych.

| Sygnatura | Opis Techniczny |
| :--- | :--- |
| `get_paths<P: AsRef<Path>>(dir_path: P) -> Vec<String>` | **[POL]:** Wykonuje rekurencyjny odczyt drzewa katalogów (pomijając głębokość `0` oraz dowiązania symboliczne). Normalizuje ścieżki: wymusza prefiks `./`, unifikuje separatory na `/` i dokleja `/` na końcu katalogów.<br><br>**[ENG]:** Performs a recursive read of the directory tree (ignoring depth `0` and symbolic links). Normalises paths: enforces `./` prefix, unifies separators to `/`, and appends `/` to directories. |

---

### 2. Klasyfikacja Wizualna (`core::path_class`)

Moduł przypisujący graficzne identyfikatory do znormalizowanych ścieżek.

| Sygnatura | Opis Techniczny |
| :--- | :--- |
| `get_icon_for_path(path: &str) -> &'static str` | **[POL]:** Rozpoznaje typ obiektu na podstawie końcowego `/` (katalog) oraz prefiksu `.` w nazwie rdzennej (ukryty). Zwraca statyczne emoji: `📁` (folder), `🗃️` (ukryty folder), `📄` (plik), `⚙️` (ukryty plik).<br><br>**[ENG]:** Identifies the object type based on the trailing `/` (directory) and the `.` prefix in the core name (hidden). Returns static emojis: `📁` (folder), `🗃️` (hidden folder), `📄` (file), `⚙️` (hidden file). |

---

### 3. Narzędzia Dopasowywania i Sortowania (`core::path_matcher::matcher_utils`)

Narzędzia transformujące wzorce oraz porządkujące kolekcje ścieżek wejściowych.

| Sygnatura / Typ | Opis Techniczny |
| :--- | :--- |
| `expand_braces(pattern: &str) -> Vec<String>` | **[POL]:** Middleware rekurencyjnie rozwijający klamry (np. `{a,b}`) względem przecinków na niezależne wzorce tekstowe.<br><br>**[ENG]:** Middleware recursively expanding braces (e.g. `{a,b}`) separated by commas into independent text patterns. |
| `SortStrategy` (Enum) | **[POL]:** 11 wariantów określających algorytm sortowania: `None`, `Az`, `Za`, `AzFileFirst`, `ZaFileFirst`, `AzDirFirst`, `ZaDirFirst` oraz 4 strategie typu `Merge` (`AzFileFirstMerge`, itd.) grupujące pary plik-katalog w bloki logiczne.<br><br>**[ENG]:** 11 variants defining the sorting algorithm: `None`, `Az`, `Za`, `AzFileFirst`, `ZaFileFirst`, `AzDirFirst`, `ZaDirFirst` and 4 `Merge` strategies (`AzFileFirstMerge`, etc.) grouping file-directory pairs into logical blocks. |
| `get_merge_key(path: &str) -> &str` | **[POL]:** Funkcja wewnętrzna ekstrahująca klucz do sortowania logicznego `Merge`. Obcina ukośniki i rozszerzenia plików, chroniąc ukryte pliki konfiguracyjne przed błędnym ucięciem nazwy.<br><br>**[ENG]:** Internal function extracting the key for `Merge` logical sorting. Trims slashes and file extensions while protecting hidden config files from erroneous name truncation. |
| `sort_paths<S: AsRef<str>>(paths: &mut Vec<S>, strategy: SortStrategy)` | **[POL]:** Wykonuje sortowanie struktury `Vec` w miejscu (in-place) przy użyciu zdefiniowanej strategii porównawczej.<br><br>**[ENG]:** Performs an in-place sort of a `Vec` structure using the defined comparative strategy. |

---

### 4. Niskopoziomowy Silnik Dopasowujący (`core::path_matcher::matcher`)

Kompiluje i ewaluuje pojedyncze wzorce tekstowe, wymuszając reguły strukturalne.

| Sygnatura Metody (`PathMatcher`) | Opis Techniczny |
| :--- | :--- |
| `new(pattern: &str, case_sensitive: bool) -> Result<Self, regex::Error>` | **[POL]:** Konstruktor. Wyciąga flagi kontrolne: `@` (para plik-folder), `$` (sierota), `+` (rekurencyjne zacienianie). Oblicza `base_name`. Ustanawia barierę `targets_file` na wyczyszczonym ze znaków kontrolnych wzorcu `p`, weryfikując brak ukośnika lub `**` na końcu. Kompiluje ostateczny Regex.<br><br>**[ENG]:** Constructor. Extracts control flags: `@` (file-directory pair), `$` (orphan), `+` (recursive shadowing). Calculates `base_name`. Establishes the `targets_file` barrier on the cleaned pattern `p`, verifying the absence of a trailing slash or `**`. Compiles the final Regex. |
| `is_match(&self, path: &str, env: &HashSet<&str>) -> bool` | **[POL]:** Ewaluator logiczny. Odrzuca katalogi, jeśli aktywna jest bariera `targets_file`. Po weryfikacji wyrażeniem regularnym wymusza zależności względem globalnego zbioru `env` (sprawdza wymogi `@` i `$`).<br><br>**[ENG]:** Logical evaluator. Rejects directories if the `targets_file` barrier is active. Post-regex validation enforces dependencies against the global `env` set (validating `@` and `$` requirements). |
| `evaluate<I, S, OnMatch, OnMismatch>(&self, paths: I, env: &HashSet<&str>, strategy: SortStrategy, on_match: OnMatch, on_mismatch: OnMismatch)` | **[POL]:** Kolektuje ścieżki na wektory `matched` i `mismatched`, poddaje je sortowaniu zgodnie z parametrem `strategy`, a następnie w pętli wywołuje wstrzyknięte domknięcia dla każdego rekordu.<br><br>**[ENG]:** Collects paths into `matched` and `mismatched` vectors, sorts them according to the `strategy` parameter, and then loops to invoke injected closures for each record. |
| `check_authorized_root(&self, path: &str, env: &HashSet<&str>) -> bool` | **[POL]:** Metoda wewnętrzna trybu `deep` (`+`). Waliduje, czy w zadanej strukturze nadrzędnej istnieje poprawny rdzeń modułu (katalog i przypisany plik), bazując na wyliczonym `base_name`.<br><br>**[ENG]:** Internal method for `deep` mode (`+`). Validates whether a valid module root (directory and assigned file) exists in the parent structure, based on the computed `base_name`. |

---

### 5. Koordynator Dopasowań (`core::path_matcher::matchers`)

Zarządza kolekcją pojedynczych silników, agregując ich zachowanie w model logiczny OR.

| Sygnatura Metody (`PathMatchers`) | Opis Techniczny |
| :--- | :--- |
| `new<I, S>(patterns: I, case_sensitive: bool) -> Result<Self, regex::Error>` | **[POL]:** Przepuszcza każdy wejściowy ciąg znaków przez warstwę middleware `expand_braces`, po czym kompiluje rozszerzone wzorce do obiektów `PathMatcher`.<br><br>**[ENG]:** Passes each input string through the `expand_braces` middleware layer, then compiles the expanded patterns into `PathMatcher` objects. |
| `is_match(&self, path: &str, env: &HashSet<&str>) -> bool` | **[POL]:** Iteruje po zainicjalizowanych instancjach `PathMatcher`. Zwraca `true`, gdy zidentyfikuje pierwsze poprawne dopasowanie we wzorcu logicznym OR.<br><br>**[ENG]:** Iterates over initialised `PathMatcher` instances. Returns `true` upon identifying the first valid match in a logical OR pattern. |
| `evaluate<I, S, OnMatch, OnMismatch>(...)` | **[POL]:** Analogiczna logika segregacji i sortowania wyników jak w `PathMatcher::evaluate`, lecz operująca na całym kontenerze wzorców równolegle.<br><br>**[ENG]:** Identical segregation and result sorting logic as in `PathMatcher::evaluate`, but operating concurrently across the entire pattern container. |
