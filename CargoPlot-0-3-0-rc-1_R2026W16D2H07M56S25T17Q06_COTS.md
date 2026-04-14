# (CargoPlot v0.3.0-rc.1) CONTENT OF THE STRUCTURE v:R2026W16D2H07M56S25T17Q06

```plaintext
| 2026 W16 Tue 07:47:53.509 | [   63.50 kB ] ├──┬ 📂  1. ./src/
| 2026 W16 Tue 07:47:53.487 | [   58.36 kB ] │  └──┬ 📂  2. ./src/lib/
| 2026 W16 Tue 07:47:53.506 | [   38.20 kB ] │     ├──┬ 📂  3. ./src/lib/logic/
| 2026 W16 Mon 16:22:45.387 | [    1.12 kB ] │     │  ├──• 📝  4. ./src/lib/logic/tag_time.rs
| 2026 W16 Tue 05:58:18.820 | [    1.73 kB ] │     │  ├──• 📝  5. ./src/lib/logic/table_spec.rs
| 2026 W16 Tue 05:56:04.067 | [    5.30 kB ] │     │  ├──• 📝  6. ./src/lib/logic/table_data.rs
| 2026 W16 Tue 05:55:34.193 | [    4.41 kB ] │     │  ├──• 📝  7. ./src/lib/logic/paths_result.rs
| 2026 W16 Mon 16:22:45.381 | [    8.55 kB ] │     │  ├──• 📝  8. ./src/lib/logic/paths_patterns.rs
| 2026 W16 Mon 16:22:45.376 | [    2.00 kB ] │     │  ├──• 📝  9. ./src/lib/logic/path_scan.rs
| 2026 W16 Mon 16:22:45.374 | [    1.45 kB ] │     │  ├──• 📝 10. ./src/lib/logic/path_context.rs
| 2026 W16 Mon 16:22:45.374 | [    1.37 kB ] │     │  ├──• 📝 11. ./src/lib/logic/path_canonical_ctx.rs
| 2026 W16 Mon 22:51:03.860 | [    1.52 kB ] │     │  ├──• 📝 12. ./src/lib/logic/lang_mapper.rs
| 2026 W16 Tue 02:55:09.559 | [    4.43 kB ] │     │  ├──• 📝 13. ./src/lib/logic/doc_markdown.rs
| 2026 W16 Tue 05:56:28.122 | [    6.32 kB ] │     │  └──• 📝 14. ./src/lib/logic/doc_engine.rs
| 2026 W16 Tue 07:47:53.485 | [   12.14 kB ] │     ├──┬ 📂 15. ./src/lib/display/
| 2026 W16 Tue 06:03:52.198 | [    7.90 kB ] │     │  ├──• 📝 16. ./src/lib/display/table_data.rs
| 2026 W15 Sun 14:23:52.040 | [      781 B ] │     │  ├──• 📝 17. ./src/lib/display/paths_result.rs
| 2026 W15 Sun 14:23:52.040 | [    1.58 kB ] │     │  ├──• 📝 18. ./src/lib/display/paths_patterns.rs
| 2026 W15 Sun 13:37:27.889 | [      717 B ] │     │  ├──• 📝 19. ./src/lib/display/path_scan.rs
| 2026 W16 Mon 16:22:26.064 | [    1.20 kB ] │     │  └──• 📝 20. ./src/lib/display/path_canonical_ctx.rs
| 2026 W16 Tue 07:47:53.473 | [    8.03 kB ] │     └──┬ 📂 21. ./src/lib/command/
| 2026 W16 Tue 06:01:02.680 | [    1.27 kB ] │        ├──• 📝 22. ./src/lib/command/table.rs
| 2026 W16 Tue 05:41:44.423 | [        0 B ] │        ├──• 📝 23. ./src/lib/command/pattern_syntax_and_semantics.rs
| 2026 W16 Tue 07:43:08.223 | [    6.75 kB ] │        └──• 📝 24. ./src/lib/command/args.rs
| 2026 W16 Tue 07:52:21.934 | [    1.62 kB ] ├──• 📝 25. ./Cargo.toml
| 2026 W15 Sun 12:23:16.722 | [    3.48 kB ] └──• 📝 26. ./.rustfmt.toml
📚 | [Crates.io](https://crates.io/crates/cargo-plot) |
📚 | [GitHub](https://github.com/j-Cis/cargo-plot/releases) |
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

```

## 004: `./src/lib/logic/tag_time.rs`

```rust
use chrono::{Datelike, Local, Timelike};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagTime(pub String);

/// Generuje tag czasowy w postaci R<RRRR>W<WW>D<D>H<HH>M<MM>S<SS>T<TT>Q<QQ>
/// Gdzie T to Tercja (1/60 sekundy), a Q to Qwadra/Kwarta (1/60 tercji).
pub fn tag_time() -> TagTime {
    let now = Local::now();

    let r = now.year(); // R: Rok
    let w = now.iso_week().week(); // W: Tydzień ISO
    let d = now.weekday().number_from_monday(); // D: Dzień tygodnia (1-7)
    let h = now.hour(); // H: Godzina
    let m = now.minute(); // M: Minuta
    let s = now.second(); // S: Sekunda

    // Pobieramy nanosekundy jako bazę do obliczeń ułamków sekundy (od 0 do
    // 999_999_999)
    let nanos = now.nanosecond() as u64;

    // T: Tercja (1/60 sekundy). Zasięg 0-59.
    let t = (nanos * 60 / 1_000_000_000) as u32;

    // Q: Qwadra (1/60 tercji). Zasięg 0-59.
    // Najpierw wyliczamy, w której z 3600 kwart ułamka sekundy jesteśmy,
    // a potem bierzemy modulo 60, by uzyskać kwartę w obrębie bieżącej tercji.
    let q = ((nanos * 3600 / 1_000_000_000) % 60) as u32;

    TagTime(format!("R{r:04}W{w:02}D{d}H{h:02}M{m:02}S{s:02}T{t:02}Q{q:02}"))
}

```

## 005: `./src/lib/logic/table_spec.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabColumn {
    TreeList, // Symbole drzewa (  ├──• ) // Wcięcia wg głębokości ( │  │  ├──• )
    Number,   // Numeracja pozycji ( 1. )
    Icon,     // Ikona typu ( 📂 / 📝 )
    Size,     // Rozmiar w nawiasach ( [ 1.20 kB] )
    Date,     // Data: | 2026 W14 Sun |
    Time,     // Czas: | 11:08:06.298 PM |
    Path,     // Ścieżka pliku
}

#[derive(Debug, Clone, Copy)]
pub enum TabSortBy {
    Name,
    Size,
    Date,
    Kind,
    FileFirst,
    DirFirst,
    FileFirstMerge,
    DirFirstMerge,
}

#[derive(Debug, Clone, Copy)]
pub enum TabSortOrder {
    Asc,
    Desc,
}

/// Zero-cost konfiguracja widoku tabeli.
#[derive(Debug, Clone)]
pub struct TableSpec {
    pub sort_by: TabSortBy,
    pub sort_order: TabSortOrder,
    pub is_tree: bool,
    pub columns: Vec<TabColumn>,
    pub limit: Option<usize>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub extended_icons: bool,
}

impl Default for TableSpec {
    fn default() -> Self {
        Self {
            sort_by: TabSortBy::Name,
            sort_order: TabSortOrder::Asc,
            is_tree: false,
            columns: vec![],
            limit: None,
            page: None,
            page_size: None,
            extended_icons: false,
        }
    }
}

impl TableSpec {
    pub fn sort(mut self, by: TabSortBy, order: TabSortOrder, is_tree: bool) -> Self {
        self.sort_by = by;
        self.sort_order = order;
        self.is_tree = is_tree;
        self
    }

    pub fn columns(mut self, cols: &[TabColumn]) -> Self {
        self.columns = cols.to_vec();
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn paginate(mut self, page: usize, size: usize) -> Self {
        self.page = Some(page);
        self.page_size = Some(size);
        self
    }

    pub fn extended_icons(mut self, enabled: bool) -> Self {
        self.extended_icons = enabled;
        self
    }
}

```

## 006: `./src/lib/logic/table_data.rs`

```rust
use std::{fs, io::Read};

use chrono::{DateTime, Local};

use super::{
    path_canonical_ctx::PathCanonicalCtx,
    paths_result::{FilterList, MatchLabel},
    table_spec::{TabColumn, TabSortBy, TabSortOrder, TableSpec},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileKind {
    Dir,
    Text,
    Binary,
    Other,
}

/// Idiomatyczny wiersz danych
#[derive(Debug, Clone)]
pub struct TableRow {
    pub path: String,
    pub size: u64,
    pub modified: DateTime<Local>,
    pub kind: FileKind,
}

/// Idiomatyczny kontener zebranych danych
#[derive(Debug, Clone)]
pub struct TableData {
    pub rows: Vec<TableRow>,
    pub is_tree: bool,
}

/// Ostateczny wynik materializacji
pub struct TableOutput {
    pub data: TableData,
    pub limit: Option<usize>,
    pub columns: Vec<TabColumn>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub extended_icons: bool,
}

fn is_binary(path: &std::path::Path) -> std::io::Result<bool> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0u8; 1024];
    let n = file.read(&mut buffer)?;
    Ok(buffer[..n].contains(&0))
}

fn get_dir_size(path: &std::path::Path) -> u64 {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

impl TableData {
    pub fn gather<L: MatchLabel>(list: &FilterList<L>) -> Self {
        let rows = list.paths.iter().filter_map(|p| Self::inspect(p, &list.entry).ok()).collect();
        Self { rows, is_tree: false }
    }

    fn inspect(rel_path: &str, relation: &PathCanonicalCtx) -> anyhow::Result<TableRow> {
        let clean_rel = rel_path.strip_prefix("./").unwrap_or(rel_path);
        let absolute_path = relation.select_dir.buf.join(clean_rel);

        let metadata = fs::metadata(&absolute_path)?;
        let modified = DateTime::from(metadata.modified()?);

        let (kind, size) = if metadata.is_dir() {
            (FileKind::Dir, get_dir_size(&absolute_path))
        } else {
            let k = if is_binary(&absolute_path)? { FileKind::Binary } else { FileKind::Text };
            (k, metadata.len())
        };

        Ok(TableRow { path: rel_path.to_string(), size, modified, kind })
    }

    pub fn sort(mut self, by: TabSortBy, order: TabSortOrder, is_tree: bool) -> Self {
        self.is_tree = is_tree;

        fn get_merge_key(path: &str) -> &str {
            let trimmed = path.trim_end_matches('/');
            if let Some(idx) = trimmed.rfind('.')
                && idx > 0 && trimmed.as_bytes()[idx - 1] != b'/'
            {
                return &trimmed[..idx];
            }
            trimmed
        }

        let compare = |a: &TableRow, b: &TableRow| {
            let a_is_dir = a.kind == FileKind::Dir;
            let b_is_dir = b.kind == FileKind::Dir;
            let a_merge = get_merge_key(&a.path);
            let b_merge = get_merge_key(&b.path);

            let mut cmp = match by {
                TabSortBy::Name => a.path.cmp(&b.path),
                TabSortBy::Size => a.size.cmp(&b.size),
                TabSortBy::Date => a.modified.cmp(&b.modified),
                TabSortBy::Kind => (a.kind.clone() as u8).cmp(&(b.kind.clone() as u8)),
                TabSortBy::FileFirst => (a_is_dir, &a.path).cmp(&(b_is_dir, &b.path)),
                TabSortBy::DirFirst => (!a_is_dir, &a.path).cmp(&(!b_is_dir, &b.path)),
                TabSortBy::FileFirstMerge => (a_merge, a_is_dir, &a.path).cmp(&(b_merge, b_is_dir, &b.path)),
                TabSortBy::DirFirstMerge => (a_merge, !a_is_dir, &a.path).cmp(&(b_merge, !b_is_dir, &b.path)),
            };
            if matches!(order, TabSortOrder::Desc) {
                cmp = cmp.reverse();
            }
            cmp
        };

        if is_tree {
            use std::{
                collections::BTreeMap,
                path::{Path, PathBuf},
            };

            let clean_paths: Vec<PathBuf> =
                self.rows.iter().map(|r| PathBuf::from(r.path.trim_end_matches('/'))).collect();
            let mut tree_map: BTreeMap<PathBuf, Vec<usize>> = BTreeMap::new();

            for (i, p) in clean_paths.iter().enumerate() {
                let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
                tree_map.entry(parent).or_default().push(i);
            }

            for indices in tree_map.values_mut() {
                indices.sort_by(|&a, &b| compare(&self.rows[a], &self.rows[b]));
            }

            let mut root_indices = Vec::new();
            for (i, p) in clean_paths.iter().enumerate() {
                let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
                let is_root = parent == Path::new(".") || parent == Path::new("") || !clean_paths.contains(&parent);
                if is_root {
                    root_indices.push(i);
                }
            }
            root_indices.sort_by(|&a, &b| compare(&self.rows[a], &self.rows[b]));

            let mut flat_indices = Vec::with_capacity(self.rows.len());
            fn flatten(
                indices: &[usize],
                tree_map: &BTreeMap<PathBuf, Vec<usize>>,
                clean_paths: &[PathBuf],
                out: &mut Vec<usize>,
            ) {
                for &idx in indices {
                    out.push(idx);
                    if let Some(children) = tree_map.get(&clean_paths[idx]) {
                        flatten(children, tree_map, clean_paths, out);
                    }
                }
            }

            flatten(&root_indices, &tree_map, &clean_paths, &mut flat_indices);

            let old_rows = std::mem::take(&mut self.rows);
            let mut temp_rows: Vec<Option<TableRow>> = old_rows.into_iter().map(Some).collect();
            let mut new_rows = Vec::with_capacity(temp_rows.len());

            for idx in flat_indices {
                new_rows.push(temp_rows[idx].take().unwrap());
            }
            self.rows = new_rows;
        } else {
            self.rows.sort_by(compare);
        }

        self
    }

    pub fn into_output(self, spec: &TableSpec) -> TableOutput {
        TableOutput {
            data: self,
            limit: spec.limit,
            columns: spec.columns.clone(),
            page: spec.page,
            page_size: spec.page_size,
            extended_icons: spec.extended_icons,
        }
    }
}

```

## 007: `./src/lib/logic/paths_result.rs`

```rust
use std::collections::HashSet;

// Wciągamy czyste, domenowe nazwy
use super::table_data::{TableData, TableOutput};
use super::{
    PathCanonicalCtx,
    PathScan,
    PathsPatterns,
    PattEnvIndex,
    table_spec::{TabColumn, TabSortBy, TabSortOrder, TableSpec},
};

// ============================================================================
// SEMANTYKA TRUE/FALSE
// ============================================================================

pub trait MatchLabel {
    fn label() -> &'static str;
}

#[derive(Debug, Clone)]
pub struct Matched;
#[derive(Debug, Clone)]
pub struct Mismatched;

impl MatchLabel for Matched {
    fn label() -> &'static str { "✔" } // ✔️
}

impl MatchLabel for Mismatched {
    fn label() -> &'static str { "✖" } // ✖️
}

/// ============================================================================
/// FILTER LIST
/// ============================================================================

#[derive(Debug, Clone)]
pub struct FilterList<L: MatchLabel> {
    pub paths: Vec<String>,
    pub label: &'static str,
    pub entry: PathCanonicalCtx,
    pub _marker: std::marker::PhantomData<L>,
}

// ============================================================================
// ENV INDEX IMPLEMENTATION (Zero-copy, Binary Search)
// ============================================================================

struct EnvIndex<'a> {
    pub dirs: HashSet<&'a str>,
    pub files: Vec<&'a str>,
}

impl<'a> PattEnvIndex for EnvIndex<'a> {
    fn has_dir(&self, dir: &str) -> bool { self.dirs.contains(dir) }

    fn has_file_with_prefix(&self, prefix: &str) -> bool {
        let start = self.files.partition_point(|&f| f < prefix);
        start < self.files.len() && self.files[start].starts_with(prefix)
    }

    fn any_file_in_dir(&self, dir: &str, check: &mut dyn FnMut(&str) -> bool) -> bool {
        let start = self.files.partition_point(|&f| f < dir);
        for &f in &self.files[start..] {
            if !f.starts_with(dir) {
                break;
            }
            if check(f) {
                return true;
            }
        }
        false
    }
}

// ============================================================================
// RESULT SCAN PATTERNS
// ============================================================================

#[derive(Debug, Clone)]
pub struct ResultScanPatterns {
    pub scanner: PathScan,
    pub patterns: PathsPatterns,

    pub m: FilterList<Matched>,
    pub x: FilterList<Mismatched>,

    // Przechowuje recepturę jak zbudować wyjściową tabelę
    pub spec: TableSpec,
}

impl ResultScanPatterns {
    pub fn new(scanner: PathScan, patterns: PathsPatterns) -> Self {
        let env_index = EnvIndex {
            dirs: scanner.dirs.iter().map(|n| n.str.as_str()).collect(),
            files: scanner.files.iter().map(|n| n.str.as_str()).collect(),
        };

        let mut m_vec = Vec::new();
        let mut x_vec = Vec::new();

        let all_paths = scanner.files.iter().chain(scanner.dirs.iter());

        for node in all_paths {
            let p = node.str.as_str();
            if patterns.is_match(p, &env_index) {
                m_vec.push(node.str.clone());
            } else {
                x_vec.push(node.str.clone());
            }
        }

        let entry = scanner.stat.relation.clone();

        Self {
            scanner,
            patterns,
            m: FilterList {
                paths: m_vec,
                label: Matched::label(),
                entry: entry.clone(),
                _marker: std::marker::PhantomData,
            },
            x: FilterList { paths: x_vec, label: Mismatched::label(), entry, _marker: std::marker::PhantomData },
            spec: TableSpec::default(),
        }
    }

    // ============================================================================
    // BUILDER API (Konfiguracja specyfikacji w miejscu)
    // ============================================================================

    pub fn sort(mut self, by: TabSortBy, order: TabSortOrder, is_tree: bool) -> Self {
        self.spec = self.spec.sort(by, order, is_tree);
        self
    }

    pub fn columns(mut self, cols: &[TabColumn]) -> Self {
        self.spec = self.spec.columns(cols);
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.spec = self.spec.limit(n);
        self
    }

    // ============================================================================
    // LENIWA MATERIALIZACJA (Fizyczny odczyt FS na wybranej grupie)
    // ============================================================================

    pub fn build_matched(&self) -> TableOutput {
        TableData::gather(&self.m)
            .sort(self.spec.sort_by, self.spec.sort_order, self.spec.is_tree)
            .into_output(&self.spec)
    }

    pub fn build_mismatched(&self) -> TableOutput {
        TableData::gather(&self.x)
            .sort(self.spec.sort_by, self.spec.sort_order, self.spec.is_tree)
            .into_output(&self.spec)
    }
}

```

## 008: `./src/lib/logic/paths_patterns.rs`

```rust
use regex::Regex;

use super::PathContext;

// ============================================================================
// ENV ABSTRAKCJA (ODCIĘCIE OD FS / STRUKTUR KOLEKCJI)
// ============================================================================

pub trait PattEnvIndex {
    fn has_dir(&self, dir: &str) -> bool;
    fn has_file_with_prefix(&self, prefix: &str) -> bool;
    fn any_file_in_dir(&self, dir: &str, check: &mut dyn FnMut(&str) -> bool) -> bool;
}

// ============================================================================
// PATTERNS (PUBLIC API)
// ============================================================================

/// [DOMENA]: Silnie typowana lista surowych wzorców
#[derive(Debug, Clone)]
pub struct PattRaw(pub Vec<String>);

/// [DOMENA]: Silnie typowana lista rozwiniętych wzorców
#[derive(Debug, Clone)]
pub struct PattExp(pub Vec<String>);

#[derive(Debug, Clone)]
pub struct PathsPatterns {
    pub patterns: PattRaw,
    pub expanded: PattExp,
    compiled: Vec<PatternCompiled>,
}

// ============================================================================
// COMPILED RULE
// ============================================================================

#[derive(Debug, Clone)]
struct PatternCompiled {
    regex: Regex,
    targets_file: bool,
    requires_sibling: bool,
    requires_orphan: bool,
    is_deep: bool,
    include_parents: bool,
    base_name: String,
    pub is_negated: bool,
    #[allow(dead_code)]
    pub original: String,
}

// ============================================================================
// PATTERNS IMPLEMENTATION
// ============================================================================

impl PathsPatterns {
    pub fn new<I, S>(patterns: I, ignore_case_sensitive: bool) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>, {
        let mut patterns_vec = Vec::new();
        let mut expanded_vec = Vec::new();

        for p in patterns {
            let s = p.as_ref();
            patterns_vec.push(s.to_string());
            expanded_vec.extend(Self::expand_braces(s));
        }

        let compiled = expanded_vec.iter().map(|p| PatternCompiled::new(p, ignore_case_sensitive).unwrap()).collect();

        Self {
            // Pakujemy wektory w nasze nowe typy domenowe!
            patterns: PattRaw(patterns_vec),
            expanded: PattExp(expanded_vec),
            compiled,
        }
    }

    /// public API
    pub fn is_match<E: PattEnvIndex>(&self, path: &str, env: &E) -> bool {
        let mut has_positive = false;
        let mut matched = false;

        for c in &self.compiled {
            if c.is_negated {
                if c.is_match(path, env) {
                    return false;
                }
            } else {
                has_positive = true;
                if !matched && c.is_match(path, env) {
                    matched = true;
                }
            }
        }

        if has_positive { matched } else { true }
    }

    pub fn rules_count(&self) -> usize { self.compiled.len() }

    // =========================================================================
    // BRACE EXPANSION (DRY, REKURENCYJNE {{a,b},c} - znajdź najgłębsze)
    // =========================================================================

    fn expand_braces(input: &str) -> Vec<String> {
        // Szukamy pierwszej zamykającej klamry
        if let Some(end) = input.find('}') {
            // Szukamy otwierającej klamry najbliżej tej zamykającej (najgłębszy poziom)
            if let Some(start) = input[..end].rfind('{') {
                let prefix = &input[..start];
                let suffix = &input[end + 1..];
                let options = &input[start + 1..end];

                let mut result = Vec::new();
                for opt in options.split(',') {
                    let merged = format!("{}{}{}", prefix, opt, suffix);
                    // Rekurencja dla kolejnych klamer
                    result.extend(Self::expand_braces(&merged));
                }
                return result;
            }
        }
        vec![input.to_string()]
    }
}

// ============================================================================
// COMPILATION & MATCHING LOGIC
// ============================================================================

impl PatternCompiled {
    pub fn new(pattern: &str, ignore_case_sensitive: bool) -> Result<Self, regex::Error> {
        let is_negated = pattern.starts_with('!');
        let p = if is_negated { &pattern[1..] } else { pattern };

        let is_deep = p.ends_with('+');
        let include_parents = p.ends_with("&/") || p.ends_with("&\\");
        let requires_sibling = p.contains('@');
        let requires_orphan = p.contains('$');

        let mut clean = p.replace(['@', '$', '+'], "");

        if include_parents {
            clean = clean.strip_suffix("&/").or_else(|| clean.strip_suffix("&\\")).unwrap_or(&clean).to_string();
        }

        let base_name = clean
            .trim_end_matches('/')
            .split('/')
            .next_back()
            .unwrap_or("")
            .split('.')
            .next()
            .unwrap_or("")
            .to_string();

        let mut re = String::new();

        if ignore_case_sensitive {
            re.push_str("(?i)");
        }

        let mut p_str = clean.as_str();
        let mut anchored = false;

        let targets_file = !p_str.ends_with('/') && !p_str.ends_with("**");

        if p_str.starts_with("./") {
            anchored = true;
            p_str = &p_str[2..];
        } else if p_str.starts_with("**/") {
            anchored = true;
        }

        if anchored {
            re.push('^');
        } else {
            re.push_str("(?:^|/)");
        }

        let chars: Vec<char> = p_str.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '\\' => {
                    i += 1;
                    if i < chars.len() {
                        re.push_str(&regex::escape(&chars[i].to_string()));
                    }
                }
                '.' => re.push_str("\\."),
                '/' => re.push('/'),
                '*' => {
                    if i + 1 < chars.len() && chars[i + 1] == '*' {
                        re.push_str(".+");
                        i += 1;
                    } else {
                        re.push_str("[^/]*");
                    }
                }
                '?' => re.push_str("[^/]"),
                '[' => {
                    re.push('[');
                    if i + 1 < chars.len() && chars[i + 1] == '!' {
                        re.push('^');
                        i += 1;
                    }
                }
                ']' | '-' | '^' => re.push(chars[i]),
                // Klamry są już obsłużone przez expand_braces!
                _ => re.push_str(&regex::escape(&chars[i].to_string())),
            }
            i += 1;
        }

        if is_deep {
            re.push_str("(?:/.*)?$");
        } else {
            re.push('$');
        }

        Ok(Self {
            regex: Regex::new(&re)?,
            targets_file,
            requires_sibling,
            requires_orphan,
            is_deep,
            include_parents,
            base_name,
            is_negated,
            original: pattern.to_string(),
        })
    }

    pub fn is_match<E: PattEnvIndex>(&self, path: &str, env: &E) -> bool {
        let is_dir = path.ends_with('/');
        let clean = path.strip_prefix("./").unwrap_or(path);

        // 1. Ochrona rodziców (Flaga &/)
        if self.targets_file && is_dir {
            if self.include_parents {
                return env.any_file_in_dir(path, &mut |p| self.regex.is_match(p.strip_prefix("./").unwrap_or(p)));
            }
            return false;
        }

        if !self.regex.is_match(clean) {
            if is_dir && self.include_parents {
                return env.any_file_in_dir(path, &mut |p| self.regex.is_match(p.strip_prefix("./").unwrap_or(p)));
            }
            return false;
        }

        // 2. Relacje dla plików (@ i $)
        if (self.requires_sibling || self.requires_orphan) && !is_dir {
            if self.is_deep && self.requires_sibling {
                if !self.check_authorized_root(path, env) {
                    return false;
                }
                return true;
            }

            let ctx = PathContext::from(path);
            let expected_folder = if ctx.parent.is_empty() {
                format!("{}/", self.base_name)
            } else {
                format!("{}/{}/", ctx.parent, self.base_name)
            };

            let exists = env.has_dir(&expected_folder);

            if self.requires_sibling && !exists {
                return false;
            }
            if self.requires_orphan && exists {
                return false;
            }
        }

        // 3. Relacje dla folderów (@ i $)
        if (self.requires_sibling || self.requires_orphan) && is_dir {
            if self.is_deep && self.requires_sibling {
                if !self.check_authorized_root(path, env) {
                    return false;
                }
            } else {
                let dir_no_slash = path.trim_end_matches('/');
                let search_prefix = format!("{}.", dir_no_slash);
                let has_file_sibling = env.has_file_with_prefix(&search_prefix);

                if self.requires_sibling && !has_file_sibling {
                    return false;
                }
                if self.requires_orphan && has_file_sibling {
                    return false;
                }
            }
        }

        true
    }

    fn check_authorized_root<E: PattEnvIndex>(&self, path: &str, env: &E) -> bool {
        let clean = path.strip_prefix("./").unwrap_or(path);
        let components: Vec<&str> = clean.split('/').collect();

        for i in 0..components.len() {
            let comp_core = components[i].split('.').next().unwrap_or("");

            if comp_core == self.base_name {
                let base_dir = if i == 0 {
                    self.base_name.clone()
                } else {
                    format!("{}/{}", components[0..i].join("/"), self.base_name)
                };

                let full_base_dir = if path.starts_with("./") { format!("./{}", base_dir) } else { base_dir };
                let dir_path = format!("{}/", full_base_dir);

                let has_dir = env.has_dir(&dir_path);
                let search_prefix = format!("{}.", full_base_dir);
                let has_file = env.has_file_with_prefix(&search_prefix);

                if has_dir && has_file {
                    return true;
                }
            }
        }
        false
    }
}

```

## 009: `./src/lib/logic/path_scan.rs`

```rust
use walkdir::WalkDir;

use super::{PathCanonicalCtx, PathNode};

/// Statystyki skanowania systemu plików
#[derive(Debug, Clone)]
pub struct PathScanStat {
    pub count_files: usize,
    pub count_folder: usize,
    pub count_empty: usize,
    pub relation: PathCanonicalCtx,
}

/// Skaner systemu plików (warstwa IO)
#[derive(Debug, Clone)]
pub struct PathScan {
    pub files: Vec<PathNode>,
    pub dirs: Vec<PathNode>,
    pub stat: PathScanStat,
}

impl PathScan {
    pub fn scan(relation: &PathCanonicalCtx) -> Self {
        let mut files = Vec::new();
        let mut dirs = Vec::new();

        let mut count_files = 0;
        let mut count_folder = 0;
        let mut count_empty = 0;

        let root = &relation.select_dir.buf;

        for e in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if e.depth() == 0 {
                continue;
            }

            if e.path_is_symlink() {
                continue;
            }

            let Ok(rel_path) = e.path().strip_prefix(root) else {
                continue;
            };

            let mut path = rel_path.to_string_lossy().replace('\\', "/");

            if e.file_type().is_dir() {
                path.push('/');
                count_folder += 1;

                let is_empty = e.metadata().map(|m| m.is_dir()).unwrap_or(false)
                    && e.path().read_dir().map(|mut r| r.next().is_none()).unwrap_or(false);

                if is_empty {
                    count_empty += 1;
                }

                dirs.push(PathNode::new(format!("./{}", path).into()));
            } else {
                count_files += 1;

                files.push(PathNode::new(format!("./{}", path).into()));
            }
        }

        files.sort_unstable_by(|a, b| a.str.cmp(&b.str));
        dirs.sort_unstable_by(|a, b| a.str.cmp(&b.str));

        let stat = PathScanStat { count_files, count_folder, count_empty, relation: relation.clone() };

        Self { files, dirs, stat }
    }

    pub fn files(&self) -> &[PathNode] { &self.files }

    pub fn dirs(&self) -> &[PathNode] { &self.dirs }

    fn to_strs(v: &[PathNode]) -> impl Iterator<Item = &str> + '_ { v.iter().map(|p| p.str.as_str()) }

    pub fn file_strs(&self) -> impl Iterator<Item = &str> + '_ { Self::to_strs(&self.files) }

    pub fn dir_strs(&self) -> impl Iterator<Item = &str> + '_ { Self::to_strs(&self.dirs) }
}

```

## 010: `./src/lib/logic/path_context.rs`

```rust
/// ============================================================================
/// PATH CONTEXT (ZERO-COPY / STACK-BASED)
/// ============================================================================
///
/// Reprezentuje semantyczny podział ścieżki na część katalogową (parent)
/// oraz nazwę pliku (file). Działa w 100% na pożyczonych referencjach (zero
/// alokacji na stercie).

#[derive(Debug, Clone)]
pub struct PathContext<'a> {
    pub parent: &'a str,
    pub file: &'a str,
}

impl<'a> PathContext<'a> {
    /// Tworzy kontekst z surowej ścieżki tekstowej bez kopiowania pamięci.
    pub fn from(path: &'a str) -> Self {
        let clean_path = path.trim_start_matches("./");

        // Zamiast tworzyć wektory i łączyć stringi, robimy matematykę na indeksach
        // (super szybkie!)
        let (parent, file) = match clean_path.rfind('/') {
            Some(idx) => {
                // Odcinamy wszystko do ostatniego ukośnika (to jest parent)
                // i wszystko po nim (to jest file)
                (&clean_path[..idx], &clean_path[idx + 1..])
            }
            None => {
                // Brak ukośnika - plik jest w katalogu głównym
                ("", clean_path)
            }
        };

        Self { parent, file }
    }

    pub fn name(&self) -> &'a str { self.file }

    pub fn parent(&self) -> &'a str { self.parent }

    pub fn is_root_level(&self) -> bool { self.parent.is_empty() }

    // Zauważ: usunąłem funkcję `full()`, bo wymuszałaby znowu łączenie stringów
    // (alokację). Jeśli silnik DSL jej nie używa, wywalamy ją dla bezpieczeństwa.
}

```

## 011: `./src/lib/logic/path_canonical_ctx.rs`

```rust
use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

/// Reprezentacja pojedynczego węzła ścieżki (buf + string)
#[derive(Debug, Clone)]
pub struct PathNode {
    pub buf: PathBuf,
    pub str: String,
}

fn normalize_path<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/")
}

impl PathNode {
    pub fn new(buf: PathBuf) -> Self {
        let str = normalize_path(&buf);
        Self { buf, str }
    }
}

/// Relacja między katalogiem wykonania a katalogiem docelowym
#[derive(Debug, Clone)]
pub struct PathCanonicalCtx {
    pub execut_dir: PathNode,
    pub select_dir: PathNode,
    pub relat_path: String,
}

impl PathCanonicalCtx {
    pub fn new<P: AsRef<Path>>(input: P) -> Result<Self> {
        let input = input.as_ref();

        let execut_dir_buf = env::current_dir().context("Nie można odczytać katalogu roboczego (CWD)")?;

        let select_dir_buf = fs::canonicalize(input)
            .with_context(|| format!("Nie można ustalić ścieżki '{}'", input.to_string_lossy()))?;

        let relat_path = match select_dir_buf.strip_prefix(&execut_dir_buf) {
            Ok(rel) => {
                let s = rel.to_string_lossy().replace('\\', "/");
                if s.is_empty() { "./".to_string() } else { format!("./{}/", s) }
            }
            Err(_) => normalize_path(input),
        };

        Ok(Self { execut_dir: PathNode::new(execut_dir_buf), select_dir: PathNode::new(select_dir_buf), relat_path })
    }
}

```

## 012: `./src/lib/logic/lang_mapper.rs`

```rust
pub struct LangMapper;

impl LangMapper {
    /// [POL]: Czarna lista rozszerzeń zabezpieczająca przed próbą odczytu plików binarnych.
    pub fn is_blacklisted(ext: &str) -> bool {
        let e = ext.to_lowercase();
        matches!(
            e.as_str(),
            // GRAFIKA
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "svg" | "webp" | "tiff" | "tif" | "heic" | "psd" | "ai" | 
            // BINARKI
            "exe" | "dll" | "so" | "dylib" | "bin" | "wasm" | "pdb" | "rlib" | "rmeta" | "lib" | "o" | "a" | "obj" | "pch" | "ilk" | "exp" | 
            "jar" | "class" | "war" | "ear" | 
            "pyc" | "pyd" | "pyo" | "whl" | 
            // ARCHIWA
            "zip" | "tar" | "gz" | "tgz" | "7z" | "rar" | "bz2" | "xz" | "iso" | "dmg" | "pkg" | "apk" | 
            // BAZY / DOKUMENTY / FONTY
            "sqlite" | "sqlite3" | "db" | "db3" | "mdf" | "ldf" | "rdb" | 
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "odt" | "ods" | "odp" | 
            "woff" | "woff2" | "ttf" | "eot" | "otf" | 
            // MEDIA
            "mp3" | "mp4" | "avi" | "mkv" | "wav" | "flac" | "ogg" | "m4a" | "mov" | "wmv" | "flv"
        )
    }

    /// [POL]: Zwraca identyfikator języka dla bloków kodu w DocMarkdown.
    pub fn get_md_lang(ext: &str) -> &'static str {
        match ext.to_lowercase().as_str() {
            "rs" => "rust",
            "toml" => "toml",
            "slint" => "slint",
            "md" => "markdown",
            "json" => "json",
            "yaml" | "yml" => "yaml",
            "html" => "html",
            "css" => "css",
            "js" => "javascript",
            "ts" => "typescript",
            _ => "text",
        }
    }
}

```

## 013: `./src/lib/logic/doc_markdown.rs`

```rust
use std::{
    fs,
    path::{Path, PathBuf},
};

use console::strip_ansi_codes;

#[allow(clippy::upper_case_acronyms)]
enum ScOrCs {
    SOTC,
    COTS,
}

use super::{LangMapper, PathNode, TableOutput, TagTime};

pub struct DocMarkdown {
    cwd: PathNode,
    wrk: PathNode,
    table: TableOutput,
    timetag: TagTime,
    content: String,
}

impl DocMarkdown {
    pub fn new(content: impl Into<String>, table: TableOutput, cwd: PathNode, wrk: PathNode, timetag: TagTime) -> Self {
        let raw = content.into();

        let clean_content = strip_ansi_codes(&raw).into_owned().replace('\t', "    ");

        Self { cwd, wrk, table, timetag, content: clean_content }
    }

    // ============================================================================
    // BUILD & RENDER
    // ============================================================================

    fn structure_of_the_content_render(&self) -> String {
        format!("# STRUCTURE OF THE CONTENT v:{}\n\n```plaintext\n{}\n```\n", self.timetag.0, self.content)
    }

    fn content_of_the_structure_render(&self, code_blocks: &str) -> String {
        format!(
            "# CONTENT OF THE STRUCTURE v:{}\n\n```plaintext\n{}\n```\n\n{}",
            self.timetag.0, self.content, code_blocks
        )
    }

    pub fn content_of_the_structure_build(&self, target_dir: &Path) -> String {
        let mut content = String::new();

        let (rows_to_show, index_offset) = if let (Some(page), Some(size)) = (self.table.page, self.table.page_size) {
            let start = page.saturating_sub(1) * size;
            (self.table.data.rows.iter().skip(start).take(size).collect::<Vec<_>>(), start)
        } else if let Some(n) = self.table.limit {
            (self.table.data.rows.iter().take(n).collect::<Vec<_>>(), 0)
        } else {
            (self.table.data.rows.iter().collect::<Vec<_>>(), 0)
        };

        for (i, row) in rows_to_show.iter().enumerate() {
            let actual_idx = index_offset + i + 1;
            let p_str = &row.path;

            if p_str.ends_with('/') {
                continue;
            }

            let clean_rel = p_str.strip_prefix("./").unwrap_or(p_str);
            let absolute_path = target_dir.join(clean_rel);

            let ext = absolute_path.extension().unwrap_or_default().to_string_lossy().to_string();

            let lang = LangMapper::get_md_lang(&ext);

            if LangMapper::is_blacklisted(&ext) {
                content.push_str(&format!(
                    "## {:03}: `{}`\n\n> *(Plik binarny/graficzny - pominięto zawartość)*\n\n",
                    actual_idx, p_str
                ));
                continue;
            }

            match fs::read_to_string(&absolute_path) {
                Ok(file_content) => {
                    let safe_content = file_content.replace('\t', "    ");
                    content.push_str(&format!(
                        "## {:03}: `{}`\n\n```{}\n{}\n```\n\n",
                        actual_idx, p_str, lang, safe_content
                    ));
                }
                Err(_) => {
                    content.push_str(&format!(
                        "## {:03}: `{}`\n\n> *(Błąd odczytu / plik nie jest UTF-8)*\n\n",
                        actual_idx, p_str
                    ));
                }
            }
        }

        content
    }

    // ============================================================================
    // SAVE AS
    // ============================================================================

    pub fn structure_of_the_content_save_as(&self, relpath: &str) -> std::io::Result<()> {
        let file_path = self.path_build(relpath, ScOrCs::SOTC)?;
        let content = self.structure_of_the_content_render();

        Self::fs_write(&file_path, content)?;
        Ok(())
    }

    pub fn content_of_the_structure_save_as(&self, relpath: &str) -> std::io::Result<()> {
        let file_path = self.path_build(relpath, ScOrCs::COTS)?;
        let code_blocks = self.content_of_the_structure_build(&self.wrk.buf);
        let content = self.content_of_the_structure_render(&code_blocks);

        Self::fs_write(&file_path, content)?;
        Ok(())
    }

    // ============================================================================
    // UTILS "DRY"
    // ============================================================================

    fn fs_write(file_path: &PathBuf, content: String) -> std::io::Result<()> {
        fs::write(file_path, content)?;
        println!("📦 Zapisano archiwum kodu do: {}", file_path.display());
        Ok(())
    }

    fn path_build(&self, relpath: &str, suffix: ScOrCs) -> std::io::Result<PathBuf> {
        let base_dir_path: PathBuf = self.cwd.buf.join(relpath);

        let stem = base_dir_path.file_stem().unwrap_or_else(|| std::ffi::OsStr::new("output")).to_string_lossy();

        let suffix_str = match suffix {
            ScOrCs::COTS => "COTS",
            ScOrCs::SOTC => "SOTC",
        };

        let file_name = format!("{}_{}_{}.md", stem, self.timetag.0, suffix_str);

        let file_path = base_dir_path.with_file_name(file_name);

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(file_path)
    }
}

```

## 014: `./src/lib/logic/doc_engine.rs`

```rust
use super::{
    TabColumn,
    DocMarkdown,
    PathCanonicalCtx,
    PathScan,
    PathsPatterns,
    ResultScanPatterns,
    TabSortBy,
    TabSortOrder,
    TableOutput,
    TagTime,
    tag_time,
};

#[derive(Clone, Copy)]
pub enum MX {
    M,
    X,
    Matched,
    Mismatched,
}

#[derive(Clone, Copy)]
pub struct RenderFlags {
    pub hide_stats: bool,
    pub hide_promo: bool,
    pub mode: MX,
}

/// Główny silnik spinający skanowanie, filtrowanie i konfigurację widoku.
pub struct DocEngine {
    pub path: PathCanonicalCtx,
    pub tagtime: TagTime,
    pub result: ResultScanPatterns,
    pub last_render: Option<RenderFlags>,
}

impl DocEngine {
    pub fn new(
        target_to_scan: &str,
        paths_patterns: Vec<&str>,
        ignore_case_sensitive: bool,
        view_row: (TabSortBy, TabSortOrder, bool),
        view_col: &[TabColumn],
    ) -> Self {
        let dir = PathCanonicalCtx::new(target_to_scan).unwrap_or_else(|x| {
            eprintln!("❌ {}", x);
            std::process::exit(1);
        });

        let ctx = PathScan::scan(&dir);
        let cfg = PathsPatterns::new(paths_patterns, ignore_case_sensitive);
        let tab = ResultScanPatterns::new(ctx, cfg).sort(view_row.0, view_row.1, view_row.2).columns(view_col);

        Self { path: dir, tagtime: tag_time(), result: tab, last_render: None }
    }

    // ============================================================================
    // Wewnętrzny silnik generujący
    // ============================================================================

    fn section_header(&self, border: String) -> String {
        let pats = format!("{:?}", self.result.patterns.patterns.0);
        let m_len = self.result.m.paths.len();
        let x_len = self.result.x.paths.len();

        let stat = &self.result.scanner.stat;
        let dir_path = format!("\"{}\"", stat.relation.select_dir.buf.display());

        let mut header = String::new();
        header.push_str(&format!("{}\n", border));
        header.push_str(&format!(
            "📊 | 📝 {} | 📂 {} | ⭕ {} | ✔️  {} | ✖️  {} |\n",
            stat.count_files, stat.count_folder, stat.count_empty, m_len, x_len
        ));
        header.push_str(&format!("🔎 {}\n", pats));
        header.push_str(&format!("🗃️  {}\n", dir_path));
        header.push_str(&border);
        header
    }

    fn section_footer(&self, border: String) -> String {
        let mut footer = String::new();
        footer.push_str("📚 | [Crates.io](https://crates.io/crates/cargo-plot) |\n");
        footer.push_str("📚 | [GitHub](https://github.com/j-Cis/cargo-plot/releases) |\n");
        // footer.push_str(&format!("Wersja: {}\n", self.version));
        footer.push_str(&border);
        footer
    }

    /// Składa w całość ostateczny ciąg znaków
    fn build_structure_of_the_content(&self, flags: &RenderFlags) -> String {
        let tab_out = match flags.mode {
            MX::M | MX::Matched => self.result.build_matched(),
            MX::X | MX::Mismatched => self.result.build_mismatched(),
        };

        let mut parts = Vec::with_capacity(3);

        if !flags.hide_stats {
            parts.push(self.section_header("░".repeat(80)));
            parts.push(format!("{}", tab_out));
            parts.push("░".repeat(80));
        } else {
            parts.push(format!("{}", tab_out));
        }

        if !flags.hide_promo {
            parts.push(self.section_footer("░".repeat(80)));
        }

        // Łączymy wszystko znakami nowej linii i dodajemy na końcu
        parts.join("\n") + "\n"
    }

    // ============================================================================
    // KONFIGURACJA WIDOKU (Builder API)
    // ============================================================================

    #[inline]
    fn finalize_view_structure_of_the_content(&mut self, flags: RenderFlags) {
        self.last_render = Some(flags);
        print!("{}", self.build_structure_of_the_content(&flags));
    }

    /// REZULTAT PEŁNY
    pub fn view(mut self, tab_mode: MX, hide_stats: bool, hide_promo: bool) -> Self {
        self.finalize_view_structure_of_the_content(RenderFlags { hide_stats, hide_promo, mode: tab_mode });
        self
    }

    /// REZULTAT OGRANICZONY PRZEZ LIMIT
    pub fn view_limit(mut self, tab_mode: MX, limit: usize, hide_stats: bool, hide_promo: bool) -> Self {
        self.result = self.result.limit(limit);
        self.finalize_view_structure_of_the_content(RenderFlags { hide_stats, hide_promo, mode: tab_mode });
        self
    }

    /// REZULTAT OGRANICZONY PRZEZ PAGINACJE
    pub fn view_pages(
        mut self,
        tab_mode: MX,
        page: usize,
        page_size: usize,
        hide_stats: bool,
        hide_promo: bool,
    ) -> Self {
        self.result.spec = self.result.spec.paginate(page, page_size);
        self.finalize_view_structure_of_the_content(RenderFlags { hide_stats, hide_promo, mode: tab_mode });
        self
    }

    // ============================================================================
    // UTILS "DRY"
    // ============================================================================

    /// Pobiera aktualne flagi renderowania lub zwraca domyślne
    fn current_flags(&self) -> RenderFlags {
        self.last_render.unwrap_or(RenderFlags { hide_stats: false, hide_promo: false, mode: MX::M })
    }

    /// Generuje surowy obiekt TableOutput na podstawie trybu z flag
    fn generate_table(&self, mode: MX) -> TableOutput {
        match mode {
            MX::M | MX::Matched => self.result.build_matched(),
            MX::X | MX::Mismatched => self.result.build_mismatched(),
        }
    }

    /// Składa w jedną całość inicjalizację obiektu Markdown (redukcja powtórzeń
    /// w metodach save)
    fn init_markdown(&self, content: String, table: TableOutput) -> DocMarkdown {
        DocMarkdown::new(
            content,
            table,
            self.path.execut_dir.clone(),
            self.path.select_dir.clone(),
            self.tagtime.clone(),
        )
    }

    // ============================================================================
    // ZAPIS DO PLIKU
    // ============================================================================

    pub fn save_structure_of_the_content(self, rel_path: &str) -> Self {
        let flags = self.current_flags();
        let table_output = self.generate_table(flags.mode);
        let raw_out_str = self.build_structure_of_the_content(&flags);
        let md = self.init_markdown(raw_out_str, table_output);

        if let Err(e) = md.structure_of_the_content_save_as(rel_path) {
            eprintln!("❌ Błąd zapisu SOTC (Struktura Zawartości): {}", e);
        }
        self
    }

    pub fn save_content_of_the_structure(self, rel_path: &str) -> Self {
        let flags = self.current_flags();
        let table_output = self.generate_table(flags.mode);
        let raw_out_str = self.build_structure_of_the_content(&flags);
        let md = self.init_markdown(raw_out_str, table_output);

        if let Err(e) = md.content_of_the_structure_save_as(rel_path) {
            eprintln!("❌ Błąd zapisu COTS (Zawartość Struktury): {}", e);
        }
        self
    }
}

```

## 016: `./src/lib/display/table_data.rs`

```rust
use colored::Colorize;

use super::{Color, DrawTree, Icon};
use crate::lib::logic::{TabColumn, FileKind, TableData, TableOutput, TableRow};

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} kB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

impl std::fmt::Display for TableData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total = self.rows.len();
        if total == 0 {
            return write!(f, "{} {} Pusta tabela", DrawTree::ITEM_LAST, Icon::EMPTY);
        }

        let mut lines = Vec::new();

        for (i, row) in self.rows.iter().enumerate() {
            let prefix = DrawTree::list(i, total);

            let kind_icon = match row.kind {
                FileKind::Dir => Icon::FOLDER,
                FileKind::Binary => "💾",
                FileKind::Text => Icon::FILE,
                FileKind::Other => "❓",
            };

            lines.push(format!(
                "{} {} [{:>10}] | {} | {}",
                prefix,
                kind_icon,
                format_size(row.size),
                row.modified.format("%Y-%m-%d %H:%M"),
                row.path
            ));
        }

        write!(f, "{}", lines.join("\n"))
    }
}

impl std::fmt::Display for TableOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_rows = self.data.rows.len();

        let (rows_to_show, index_offset) = if let (Some(page), Some(size)) = (self.page, self.page_size) {
            let start = page.saturating_sub(1) * size;
            (self.data.rows.iter().skip(start).take(size).collect::<Vec<_>>(), start)
        } else if let Some(n) = self.limit {
            (self.data.rows.iter().take(n).collect::<Vec<_>>(), 0)
        } else {
            (self.data.rows.iter().collect::<Vec<_>>(), 0)
        };

        let current_view_count = rows_to_show.len();
        let num_width = total_rows.to_string().len();

        let mut prefix_map = std::collections::HashMap::new();

        if self.data.is_tree {
            use std::{
                collections::BTreeMap,
                path::{Path, PathBuf},
            };

            let clean_paths: Vec<PathBuf> =
                self.data.rows.iter().map(|r| PathBuf::from(r.path.trim_end_matches('/'))).collect();

            let mut tree_map: BTreeMap<PathBuf, Vec<&TableRow>> = BTreeMap::new();

            for (i, p) in clean_paths.iter().enumerate() {
                let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
                tree_map.entry(parent).or_default().push(&self.data.rows[i]);
            }

            let mut roots = Vec::new();
            for (i, p) in clean_paths.iter().enumerate() {
                let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
                let is_root = parent == Path::new(".") || parent == Path::new("") || !clean_paths.contains(&parent);
                if is_root {
                    roots.push(&self.data.rows[i]);
                }
            }

            fn build_prefixes(
                nodes: &[&TableRow],
                tree_map: &BTreeMap<PathBuf, Vec<&TableRow>>,
                indent: &str,
                out_map: &mut std::collections::HashMap<String, String>,
            ) {
                for (i, node) in nodes.iter().enumerate() {
                    let clean_p = PathBuf::from(node.path.trim_end_matches('/'));
                    let is_last = i == nodes.len() - 1;
                    let children = tree_map.get(&clean_p);
                    let has_children = children.is_some() && !children.unwrap().is_empty();
                    let is_dir = node.kind == FileKind::Dir;

                    let (branch, next_indent) = DrawTree::tree(is_dir, is_last, has_children);

                    let prefix = format!("{}{}", indent, branch);
                    out_map.insert(node.path.clone(), prefix);

                    if let Some(child_nodes) = children {
                        let new_indent = format!("{}{}", indent, next_indent);
                        build_prefixes(child_nodes, tree_map, &new_indent, out_map);
                    }
                }
            }

            build_prefixes(&roots, &tree_map, "", &mut prefix_map);
        }

        let mut output_lines = Vec::new();

        for (i, row) in rows_to_show.iter().enumerate() {
            let actual_idx = index_offset + i + 1;

            let tree_list_prefix_str = if self.data.is_tree {
                prefix_map.get(&row.path).map(|s| s.as_str()).unwrap_or("")
            } else {
                DrawTree::list(i, current_view_count)
            };
            let tree_list_prefix = Color::tree(tree_list_prefix_str);

            let icon_str = if self.extended_icons {
                let is_dir = row.kind == FileKind::Dir;
                let file_name = row.path.trim_end_matches('/').split('/').next_back().unwrap_or("");
                let is_hidden = file_name.starts_with('.');

                if is_dir {
                    if is_hidden { Icon::FOLDER2_HIDDEN } else { Icon::FOLDER2 }
                } else if is_hidden {
                    Icon::FILE2_HIDDEN
                } else {
                    let path_obj = std::path::Path::new(&row.path);
                    if let Some(ext) = path_obj.extension().and_then(|e| e.to_str()) {
                        match ext.to_lowercase().as_str() {
                            "rs" => Icon::LANG_RUST,
                            "toml" => "⚙️",
                            "slint" => "🎨",
                            "md" => "📝",
                            "json" => "🔣",
                            "yaml" | "yml" => "🛠️",
                            "html" => "📖",
                            "css" => "🖌️",
                            "js" => "📜",
                            "ts" => "📘",
                            _ => {
                                if row.kind == FileKind::Binary {
                                    "💾"
                                } else {
                                    Icon::FILE2
                                }
                            }
                        }
                    } else {
                        if row.kind == FileKind::Binary { "💾" } else { Icon::FILE2 }
                    }
                }
            } else {
                match row.kind {
                    FileKind::Dir => Icon::FOLDER,
                    FileKind::Binary => "💾",
                    FileKind::Text => Icon::FILE,
                    FileKind::Other => "❓",
                }
            };

            let path_styled = match row.kind {
                FileKind::Dir => Color::folder(&row.path),
                FileKind::Binary => Color::binary(&row.path),
                FileKind::Text => Color::file(&row.path),
                FileKind::Other => row.path.dimmed(),
            };

            let icon = match row.kind {
                FileKind::Dir => icon_str.yellow(),
                FileKind::Binary => icon_str.magenta(),
                FileKind::Text => icon_str.normal(),
                FileKind::Other => icon_str.red(),
            };

            let size_str = Color::size(&format!("{:>10}", format_size(row.size)));
            let pos_num = Color::num(&format!("{:>width$}.", actual_idx, width = num_width));
            let date_str = Color::date(&row.modified.format("%Y W%V %a").to_string());
            let time_str = Color::time(&row.modified.format("%H:%M:%S.%3f").to_string());

            let mut line_parts = Vec::new();
            for (col_idx, col) in self.columns.iter().enumerate() {
                let part = match col {
                    TabColumn::TreeList => tree_list_prefix.to_string(),
                    TabColumn::Number => pos_num.to_string(),
                    TabColumn::Icon => icon.to_string(),
                    TabColumn::Size => {
                        format!("{}{}{}", Color::border("[ "), size_str, Color::border(" ]"))
                    }
                    TabColumn::Date => {
                        let pipe = Color::border("|");
                        let next_is_time = matches!(self.columns.get(col_idx + 1), Some(TabColumn::Time));
                        let prev_is_time = col_idx > 0 && self.columns[col_idx - 1] == TabColumn::Time;
                        match (prev_is_time, next_is_time) {
                            (false, true) => format!("{} {}", pipe, date_str),
                            (true, false) => format!("{} {}", date_str, pipe),
                            (true, true) => format!("{}", date_str),
                            (false, false) => format!("{} {} {}", pipe, date_str, pipe),
                        }
                    }
                    TabColumn::Time => {
                        let pipe = Color::border("|");
                        let next_is_date = matches!(self.columns.get(col_idx + 1), Some(TabColumn::Date));
                        let prev_is_date = col_idx > 0 && self.columns[col_idx - 1] == TabColumn::Date;
                        match (prev_is_date, next_is_date) {
                            (false, true) => format!("{} {}", pipe, time_str),
                            (true, false) => format!("{} {}", time_str, pipe),
                            (true, true) => format!("{}", time_str),
                            (false, false) => format!("{} {} {}", pipe, time_str, pipe),
                        }
                    }
                    TabColumn::Path => path_styled.to_string(),
                };
                line_parts.push(part);
            }

            output_lines.push(line_parts.join(" "));
        }

        if let (Some(page), Some(size)) = (self.page, self.page_size) {
            let total_pages = total_rows.saturating_add(size - 1) / size;
            if total_pages > 1 {
                output_lines.push(format!(
                    "          {}",
                    format!("... Strona {} z {} (łącznie {} pozycji)", page, total_pages, total_rows).italic().dimmed()
                ));
            }
        } else if let Some(n) = self.limit
            && total_rows > n
        {
            output_lines
                .push(format!("          {}", format!("... i {} innych pozycji", total_rows - n).italic().dimmed()));
        }

        write!(f, "{}", output_lines.join("\n"))
    }
}

```

## 017: `./src/lib/display/paths_result.rs`

```rust
use super::{DrawTree, Icon};
use crate::lib::logic::{FilterList, MatchLabel, ResultScanPatterns};

impl<L: MatchLabel> std::fmt::Display for FilterList<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.label)?;
        for path in &self.paths {
            writeln!(f, "{} {}", DrawTree::ITEM, path)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for ResultScanPatterns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pierwsza linia z enterem
        writeln!(f, "{} {} Matched (m): {}", DrawTree::list(0, 3), Icon::BOOL_TRUE, self.m.paths.len())?;
        // Ostatnia linia BEZ entera i bez żadnych wiszących spacji (ident)
        write!(f, "{} {} Mismatched (x): {}", DrawTree::list(2, 3), Icon::BOOL_FALSE, self.x.paths.len())
    }
}

```

## 018: `./src/lib/display/paths_patterns.rs`

```rust
use super::{BoolExt, DrawTree, Icon};
use crate::lib::logic::{PathsPatterns, PattExp, PattRaw};

impl std::fmt::Display for PattRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Zwróć uwagę na .0, którym dobieramy się do wewnętrznego Vec<String>
        let raw_len = self.0.len();
        if raw_len == 0 {
            return write!(f, "{} {} Brak reguł wejściowych", DrawTree::ITEM_LAST, Icon::EMPTY);
        }

        let mut lines = Vec::new();
        for (i, pat) in self.0.iter().enumerate() {
            let prefix = DrawTree::list(i, raw_len);
            lines.push(format!("{} {}", prefix, pat));
        }
        write!(f, "{}", lines.join("\n"))
    }
}

impl std::fmt::Display for PattExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let matchers_len = self.0.len();
        if matchers_len == 0 {
            return write!(f, "{} {} Brak aktywnych reguł", DrawTree::ITEM_LAST, Icon::EMPTY);
        }

        let mut lines = Vec::new();
        for (i, pattern_text) in self.0.iter().enumerate() {
            let prefix = DrawTree::list(i, matchers_len);
            let is_negated = pattern_text.starts_with('!');
            let rule_icon = (!is_negated).as_symbol();
            lines.push(format!("{} [{}] {}", prefix, rule_icon, pattern_text));
        }
        write!(f, "{}", lines.join("\n"))
    }
}

impl std::fmt::Display for PathsPatterns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();

        // Magia! Delegate Display do wewnętrznych typów:
        lines.push(format!("{}", self.patterns));
        lines.push(format!("   {}", Icon::EXPAND));
        lines.push(format!("{}", self.expanded));

        write!(f, "{}", lines.join("\n"))
    }
}

```

## 019: `./src/lib/display/path_scan.rs`

```rust
use super::Icon;
use crate::lib::logic::{PathScan, PathScanStat}; // Zwróć uwagę, że usunąłem DrawTree, bo nie jest tu już potrzebne

impl std::fmt::Display for PathScanStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} | {} {} | {} {} | {} {}",
            Icon::ENTRY,
            self.relation.select_dir.buf.display(),
            Icon::FILE,
            self.count_files,
            Icon::FOLDER,
            self.count_folder,
            Icon::EMPTY,
            self.count_empty
        )
    }
}

impl std::fmt::Display for PathScan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Domyślny format dla całego skanera to po prostu czytelne statystyki!
        write!(f, "📊 Stats -> {}", self.stat)
    }
}

```

## 020: `./src/lib/display/path_canonical_ctx.rs`

```rust
use super::DrawTree;
use crate::lib::logic::{PathCanonicalCtx, PathNode};

impl std::fmt::Display for PathNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Domyślnie pokazujemy znormalizowany ciąg
        write!(f, "{}", self.str)
    }
}

impl std::fmt::Display for PathCanonicalCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();

        // 1. Sekcja CWD
        lines.push("🅰️ 📍 execut_dir (CWD)".to_string());
        lines.push(format!("{} buf: {}", DrawTree::ENTRY_BRANCH, self.execut_dir.buf.display()));
        lines.push(format!("{} str: {}", DrawTree::ENTRY_TERMINAL, self.execut_dir.str));

        // 2. Sekcja TARGET
        lines.push("🅱️ 🎯 select_dir (TARGET)".to_string());
        lines.push(format!("{} buf: {}", DrawTree::ENTRY_BRANCH, self.select_dir.buf.display()));
        lines.push(format!("{} str: {}", DrawTree::ENTRY_TERMINAL, self.select_dir.str));

        // 3. Sekcja RELATIVE
        lines.push("🆎🔗 relat_path (RELATIVE, BETWEEN CWD & TARGET)".to_string());
        lines.push(format!("{} str: {}", DrawTree::ENTRY_TERMINAL, self.relat_path));

        // Sklejamy wszystko znakiem nowej linii i wypisujemy zwykłym write!
        write!(f, "{}", lines.join("\n"))
    }
}

```

## 022: `./src/lib/command/table.rs`

```rust
use crate::lib::logic::{TabColumn, TabSortBy};

pub fn parse_column(s: &str) -> Result<TabColumn, String> {
    match s.trim().to_lowercase().as_str() {
        "treelist" | "tree" | "list" => Ok(TabColumn::TreeList),
        "number" => Ok(TabColumn::Number),
        "icon" => Ok(TabColumn::Icon),
        "size" => Ok(TabColumn::Size),
        "date" => Ok(TabColumn::Date),
        "time" => Ok(TabColumn::Time),
        "path" => Ok(TabColumn::Path),
        _ => Err(format!(
            "Nieprawidłowa nazwa kolumny: '{}'. Dostępne: list, tree, number, icon, size, date, time, path",
            s.trim()
        )),
    }
}

pub fn parse_sort(s: &str) -> Result<TabSortBy, String> {
    match s.trim().to_lowercase().as_str() {
        "name" => Ok(TabSortBy::Name),
        "size" => Ok(TabSortBy::Size),
        "date" => Ok(TabSortBy::Date),
        "kind" => Ok(TabSortBy::Kind),
        "file-first" => Ok(TabSortBy::FileFirst),
        "dir-first" => Ok(TabSortBy::DirFirst),
        "file-merge" => Ok(TabSortBy::FileFirstMerge),
        "dir-merge" => Ok(TabSortBy::DirFirstMerge),
        _ => Err(format!(
            "Nieprawidłowa wartość sortowania: '{}'. Dostępne: name, size, date, kind, file-first, dir-first, file-merge, dir-merge",
            s.trim()
        )),
    }
}
```

## 023: `./src/lib/command/pattern_syntax_and_semantics.rs`

```rust

```

## 024: `./src/lib/command/args.rs`

```rust
use clap::{ArgGroup, Parser};
use super::table::{parse_column, parse_sort};
use crate::lib::logic::{TabColumn, TabSortBy};

#[derive(Parser, Debug)]
#[command(name = "x-do", author, version, about = "Advanced File Scanner & Code Archiver", long_about = None, arg_required_else_help = true)]
// ⚡ TWORZYMY GRUPĘ: Grupuje flagi trybu, aby móc ich wymagać jako "jedno z dwóch"
#[command(group(
    ArgGroup::new("mode_flags")
        .args(["matched", "mismatched"])
))]
pub struct ArgsCommand {
    // ============================================================================
    // 0. INFORMACJE I POMOC
    // ============================================================================

    /// Wyświetla szczegółową pomoc dotyczącą składni i semantyki wzorców (Patterns)
    #[arg(
        short = 'P', 
        long = "pattern-help", 
        visible_aliases = ["syntax", "pat-help"], 
        exclusive = true, // ⚡ KLUCZOWE: Pozwala odpalić tę flagę bez podawania `-p` (ignoruje required=true)
        help_heading = "Information"
    )]
    pub pattern_help: bool,

    // ============================================================================
    // 1. WEJŚCIE I SKANOWANIE (TARGET & PATTERNS)
    // ============================================================================

    /// Ścieżka katalogu do skanowania
    #[arg(
        short = 'w', 
        long = "work-path", 
        short_alias = 'j',
        visible_aliases = ["entry", "read", "job-path"], 
        required = true,
        help_heading = "Input Options"
    )]
    pub work_path: String,

    /// Wzorce wyszukiwania (glob, rozszerzenia)
    #[arg(
        short = 'p', 
        long = "pattern", 
        visible_aliases = ["pat", "patterns"], 
        required = true,
        help_heading = "Input Options"
    )]
    pub patterns: Vec<String>,

    /// Ignoruj wielkość liter przy dopasowywaniu wzorców
    #[arg(
        short = 'i', 
        long = "ignore-case", 
        help_heading = "Input Options"
    )]
    pub ignore_case: bool,

    
    // ============================================================================
    // 2. KOLEJNOŚĆ i SPOSÓB PREZENCJI STUKTÓRY ZAWARTOŚCI
    // ============================================================================

    /// Aktywuje widok drzewa (zamiast płaskiej listy)
    #[arg(
        short = 't', 
        long = "tree", 
        help_heading = "Layout V Options"
    )]
    pub tree: bool,

    /// Kryterium sortowania
    #[arg(
        long = "sort", 
        value_parser = parse_sort, 
        default_value = "kind",
        help_heading = "Layout V Options"
    )]
    pub sort: TabSortBy,
    
    /// Odwraca kierunek sortowania (Descending)
    #[arg(
        short = 'r', 
        long = "reverse", 
        help_heading = "Layout V Options"
    )]
    pub reverse: bool,

     /// Kolumny do wyświetlenia w tabeli
    #[arg(
        short = 'v', 
        long = "view-columns",
        value_parser = parse_column,
        value_delimiter = ',',
        default_value = "time,number,size,date,time,treelist,icon,path",
        help_heading = "Layout H Options"
    )]
    pub columns: Vec<TabColumn>,   

    /// Włącza rozszerzony zestaw ikon (np. dla konkretnych języków programowania)
    #[arg(
        short = 'e', 
        long = "ext-icons", 
        help_heading = "Layout H Options"
    )]
    pub ext_icons: bool,

    // ============================================================================
    // 3. TRYB PRACY (MATCHED vs MISMATCHED) - GENEROWANIE STRUKTURY ZAWARTOŚCI
    // ============================================================================

    /// Wyświetl pliki odrzucone (Mismatched) zamiast dopasowanych
    #[arg(
        short = 'x', 
        long = "mismatched", 
        short_alias = 'X',
        conflicts_with = "matched",
        help_heading = "Mode Options"
    )]
    pub mismatched: bool,

    /// Wyświetl pliki dopasowane (Matched) - domyślne
    #[arg(
        short = 'm', 
        long = "matched", 
        short_alias = 'M',
        conflicts_with = "mismatched",
        help_heading = "Mode Options"
    )]
    pub matched: bool,

    
    /// Twardy limit wyświetlanych/zapisywanych pozycji
    #[arg(
        short = 'l', 
        long = "size-limit", 
        conflicts_with = "page",
        help_heading = "Limits & Pagination"
    )]
    pub limit: Option<usize>,

    /// Wybierz konkretną stronę wyników
    #[arg(
        long = "page", 
        conflicts_with = "limit",
        help_heading = "Limits & Pagination"
    )]
    pub page: Option<usize>,

    /// Liczba wyników na stronę (wymaga --page)
    #[arg(
        long = "size-page", 
        default_value = "20",
        help_heading = "Limits & Pagination"
    )]
    pub page_size: usize,

    // ============================================================================
    // 4. ZAPIS NA DYSK (SOTC & COTS)
    // ============================================================================

    /// Zapisuje STRUKTURĘ zawartości (tylko tabela i statystyki)
    #[arg(
        short = 's', 
        long = "save-sotc-at", 
        visible_alias = "save-structure-of-the-content-at",
        value_name = "OUT_DIR",
        requires = "mode_flags",
        num_args = 0..=1, // ⚡ Pozwala wywołać flagę -s bez podawania ścieżki
        default_missing_value = "./target/.cargo-plot/", // ⚡ Wartość, gdy podano samo -s
        help_heading = "Export Options"
    )]
    pub save_sotc_at: Option<String>,

    /// Zapisuje ZAWARTOŚĆ struktury (tabela + pełne kody źródłowe)
    #[arg(
        short = 'c', 
        long = "save-cots-at", 
        visible_alias = "save-content-of-the-structure-at",
        value_name = "OUT_DIR",
        requires = "mode_flags",
        num_args = 0..=1, // ⚡ Pozwala wywołać flagę -c bez podawania ścieżki
        default_missing_value = "./target/.cargo-plot/", // ⚡ Wartość, gdy podano samo -c
        help_heading = "Export Options"
    )]
    pub save_cots_at: Option<String>,

    // ============================================================================
    // 5. MODYFIKATORY RENDEROWANIA (RENDER FLAGS)
    // ============================================================================

    /// Ukrywa nagłówek ze statystykami skanowania
    #[arg(
        short = 'a', 
        long = "hide-stats", 
        help_heading = "Render Flags"
    )]
    pub hide_stats: bool,

    /// Ukrywa stopkę promocyjną (info o narzędziu)
    #[arg(
        short = 'b', 
        long = "hide-promo", 
        help_heading = "Render Flags"
    )]
    pub hide_promo: bool,
    
    /// Ukrywa wyjście w terminalu (Cichy tryb, przydatny jeśli zależy nam tylko na plikach)
    #[arg(
        short = 'q',
        long = "quiet",
        help_heading = "Render Flags"
    )]
    pub quiet: bool,
}
```

## 025: `./Cargo.toml`

```toml
[package]
name = "cargo-plot"
version = "0.2.0"
authors = ["Jan Roman Cisowski „j-Cis”"]
edition = "2024"
rust-version = "1.94.0"
description = "Szwajcarski scyzoryk do wizualizacji struktury projektu i generowania dokumentacji bezpośrednio z poziomu Cargo."
license = "MIT OR Apache-2.0"
readme = "README.md"
repository = "https://github.com/j-Cis/cargo-plot"

keywords = [ "cargo",  "tree",  "markdown",  "filesystem", "documentation"]
categories = [ "development-tools::cargo-plugins",  "command-line-utilities", "command-line-interface", "text-processing",]
resolver = "3"

[package.metadata.cargo]
edition = "2024"


[lib]
name = "plot"        
path = "src/api.rs" 


[dependencies]
anyhow = "1.0.102"
chrono = "0.4.44"
clap = { version = "4.6.0", features = ["derive", "cargo", "unicode", "wrap_help", "string", "env", "unstable-doc", "unstable-markdown", "unstable-styles"] }
colored = "3.1.1"
console = "0.16.3"
regex = "1.12.3"
walkdir = "2.5.0"

# cliclack = "0.5.0"
# ctrlc = "3.5.2"
# shlex = "1.3.0"
# eframe = "0.34.1"
# rfd = "0.17.2"


# ==========================================
# Globalna konfiguracja lintów (Analiza kodu)
# ==========================================
[lints.rust]
# Kategorycznie zabraniamy używania bloków `unsafe` w całym projekcie
unsafe_code = "forbid"
# Ostrzegamy o nieużywanych importach, zmiennych i funkcjach
# unused = "warn"
#
[lints.clippy]
# Włączamy surowsze reguły, ale jako ostrzeżenia (nie zepsują kompilacji)
# pedantic = "warn"
# Możemy tu też wyciszyć globalnie to, co nas irytuje (opcjonalnie):
too_many_arguments = "allow"

```

## 026: `./.rustfmt.toml`

```toml
# =========================
# 🧠 RUST EDITION / CORE
# =========================

# 01【A01】 Wersja edycji Rust (wpływa na składnię i reguły języka)
edition = "2024"

# 02【A02】 Włącza niestabilne funkcje rustfmt (eksperymentalne opcje formatowania)
unstable_features = true


# =========================
# 📏 FORMATOWANIE OGÓLNE (LINIE / UKŁAD)
# =========================

# 03【B01】 Maksymalna szerokość linii kodu
max_width = 120

# 04【B02】 Styl końca linii (Auto = zachowuje LF/CRLF)
newline_style = "Auto"

# 05【B03】 Heurystyka formatowania (Max = minimalne łamanie linii)
use_small_heuristics = "Max"

# 06【B04】 Styl wcięć (blokowy)
indent_style = "Block"

# 07【B05】 Używa TABów zamiast spacji
hard_tabs = true

# 08【B06】 Maksymalna liczba pustych linii
blank_lines_upper_bound = 1


# =========================
# 🔧 STRUKTURA KODU (BLOCKS / CONTROL FLOW)
# =========================

# 09【C01】 Preferuje klamry w tej samej linii co nagłówek
brace_style = "PreferSameLine"

# 10【C02】 Klamry w if/for/while zawsze w tej samej linii
control_brace_style = "AlwaysSameLine"

# 11【C03】 Where clause może być w jednej linii
where_single_line = true

# 12【C04】 Funkcje mogą być jednoliniowe
fn_single_line = true

# 13【C05】 Max długość if/else w jednej linii
single_line_if_else_max_width = 100

# 14【C06】 Elastyczne formatowanie wyrażeń delimitowanych
overflow_delimited_expr = true

# 15【C07】 Łączenie wyrażeń kontrolnych w bardziej kompaktową formę
combine_control_expr = true


# =========================
# 🧩 FUNKCJE / WYWOŁANIA / STRUKTURY
# =========================

# 16【D01】 Skrócona inicjalizacja structów
use_field_init_shorthand = true

# 17【D02】 Struct literal w jednej linii jeśli się mieści
struct_lit_single_line = true

# 18【D03】 Maksymalna szerokość wywołań funkcji
fn_call_width = 100

# 19【D04】 Maksymalna szerokość tablic
array_width = 100

# 20【D05】 Maksymalna szerokość atrybutów typu derive
attr_fn_like_width = 100


# =========================
# 📦 IMPORTY I MODUŁY
# =========================

# 21【E01】 Automatyczne sortowanie importów
reorder_imports = true

# 22【E02】 Granularność importów (Crate-level grouping)
imports_granularity = "Crate"

# 23【E03】 Grupowanie importów: std → external → local
group_imports = "StdExternalCrate"

# 24【E04】 Styl układu importów (horizontal / vertical fallback)
imports_layout = "HorizontalVertical"


# =========================
# 🧾 DOKUMENTACJA I KOMENTARZE
# =========================

# 25【F01】 Formatowanie kodu w doc comments
format_code_in_doc_comments = true

# 26【F02】 Normalizacja atrybutów dokumentacji
normalize_doc_attributes = true

# 27【F03】 Automatyczne zawijanie komentarzy
wrap_comments = true

# 28【F04】 Normalizacja stylu komentarzy
normalize_comments = true


# =========================
# 🧠 MACRA / META / ABSTRAKCJE
# =========================

# 29【G01】 Formatowanie matcherów w macro_rules!
format_macro_matchers = true

# 30【G02】 Sortowanie elementów w impl
# reorder_impl_items = true # IRYTUJĄCE


# =========================
# 🧮 LITERALS / STYLE DETALE
# =========================

# 31【H01】 HEX literały w uppercase
hex_literal_case = "Upper"

# 32【H02】 Skrócony try operator (? zamiast pełnych form)
use_try_shorthand = true
```

