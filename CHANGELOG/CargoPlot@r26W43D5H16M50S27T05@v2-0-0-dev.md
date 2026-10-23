# CargoPlot@r26W43D5H16M50S27T05@V2-0-0-dev

## Structure

```plaintext
  ▣─┬ 🆗cargo-plot                                [120.4 KiB]                      A:/A-JAN/WIN-DOCS/REPO_OW 
    │                                                                             N_RUST/GIT_j-Cis/🆗cargo-p 
    │                                                                             lot/                      
 1  ├──• Cargo.toml                              [  1.8 KiB] [2026-43-4:02:35:09] ./Cargo.toml              
 2  ├──• .rustfmt.toml                           [  3.4 KiB] [2026-38-7:22:31:33] ./.rustfmt.toml           
    ├──┬ .cargo                                  [  3.0 KiB] [2026-43-5:09:53:04] ./.cargo/                 
 3  │  ├──• config.toml                          [    593 B] [2026-43-3:22:26:37] ./.cargo/config.toml      
 4  │  ├──• .x-do.toml                           [  1.2 KiB] [2026-43-5:16:50:14] ./.cargo/.x-do.toml       
 5  │  ├──• .x-do-history.txt                    [     67 B] [2026-43-2:23:39:30] ./.cargo/.x-do-history.tx 
    │  │                                                                          t                         
 6  │  └──• .x-do2.toml                          [  1.1 KiB] [2026-43-4:16:05:44] ./.cargo/.x-do2.toml      
    ├──┬ examples                                [  2.1 KiB] [2026-43-5:09:17:10] ./examples/               
 7  │  ├──• example_timer.rs                     [    295 B] [2026-43-5:09:27:39] ./examples/example_timer. 
    │  │                                                                          rs                        
 8  │  └──• example01.rs                         [  1.8 KiB] [2026-43-5:16:09:10] ./examples/example01.rs   
    ├──┬ src                                     [108.2 KiB] [2026-43-2:15:49:53] ./src/                    
 9  │  ├──• lib.rs                               [    322 B] [2026-43-5:09:18:30] ./src/lib.rs              
    │  └──┬ libs                                 [107.9 KiB] [2026-43-5:09:18:16] ./src/libs/               
10  │     ├──• querypath.rs                      [    421 B] [2026-43-5:14:28:49] ./src/libs/querypath.rs   
11  │     ├──• timer.rs                          [    145 B] [2026-43-5:06:45:48] ./src/libs/timer.rs       
    │     ├──┬ querypath                         [ 80.8 KiB] [2026-43-5:09:21:39] ./src/libs/querypath/     
12  │     │  ├──• controllers.rs                 [    374 B] [2026-43-5:09:20:15] ./src/libs/querypath/cont 
    │     │  │                                                                    rollers.rs                
13  │     │  ├──• core.rs                        [    360 B] [2026-43-5:09:22:56] ./src/libs/querypath/core 
    │     │  │                                                                    .rs                       
    │     │  ├──┬ controllers                    [ 19.3 KiB] [2026-43-5:09:26:44] ./src/libs/querypath/cont 
    │     │  │  │                                                                 rollers/                  
14  │     │  │  ├──• gather_claims.rs            [    463 B] [2026-43-5:14:28:36] ./src/libs/querypath/cont 
    │     │  │  │                                                                 rollers/gather_claims.rs  
15  │     │  │  ├──• prepar_setups.rs            [    833 B] [2026-43-5:16:26:08] ./src/libs/querypath/cont 
    │     │  │  │                                                                 rollers/prepar_setups.rs  
    │     │  │  └──┬ gather_claims               [ 18.1 KiB] [2026-43-5:13:06:51] ./src/libs/querypath/cont 
    │     │  │     │                                                              rollers/gather_claims/    
16  │     │  │     ├──• engine.rs                [ 12.5 KiB] [2026-43-5:16:25:13] ./src/libs/querypath/cont 
    │     │  │     │                                                              rollers/gather_claims/eng 
    │     │  │     │                                                              ine.rs                    
17  │     │  │     ├──• traits.rs                [  1.5 KiB] [2026-43-5:09:48:29] ./src/libs/querypath/cont 
    │     │  │     │                                                              rollers/gather_claims/tra 
    │     │  │     │                                                              its.rs                    
18  │     │  │     └──• types.rs                 [  4.1 KiB] [2026-43-5:16:25:13] ./src/libs/querypath/cont 
    │     │  │                                                                    rollers/gather_claims/typ 
    │     │  │                                                                    es.rs                     
    │     │  └──┬ core                           [ 60.7 KiB] [2026-43-5:09:21:39] ./src/libs/querypath/core 
    │     │     │                                                                 /                         
19  │     │     ├──• features.rs                 [    608 B] [2026-43-5:16:25:24] ./src/libs/querypath/core 
    │     │     │                                                                 /features.rs              
20  │     │     ├──• structs.rs                  [    791 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │                                                                 /structs.rs               
21  │     │     ├──• traits.rs                   [    320 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │                                                                 /traits.rs                
22  │     │     ├──• types.rs                    [    303 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │                                                                 /types.rs                 
23  │     │     ├──• utils.rs                    [    515 B] [2026-43-3:16:00:19] ./src/libs/querypath/core 
    │     │     │                                                                 /utils.rs                 
    │     │     ├──┬ features                    [ 31.6 KiB] [2026-43-5:16:18:36] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/                
24  │     │     │  ├──• handle_corpus.rs         [  4.9 KiB] [2026-43-5:16:23:47] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/handle_corpus.r 
    │     │     │  │                                                              s                         
25  │     │     │  ├──• handle_formatted_time.rs [  2.5 KiB] [2026-43-5:14:24:19] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/handle_formatte 
    │     │     │  │                                                              d_time.rs                 
26  │     │     │  ├──• handle_prepared_patterns.[  3.2 KiB] [2026-43-5:14:24:36] ./src/libs/querypath/core 
    │     │     │  │    rs                                                        /features/handle_prepared 
    │     │     │  │                                                              _patterns.rs              
27  │     │     │  ├──• handle_verified_dirs.rs  [  4.3 KiB] [2026-43-5:14:24:54] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/handle_verified 
    │     │     │  │                                                              _dirs.rs                  
28  │     │     │  ├──• handle_walk_scan.rs      [ 15.6 KiB] [2026-43-5:15:37:26] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/handle_walk_sca 
    │     │     │  │                                                              n.rs                      
29  │     │     │  ├──• step05_build_tree.rs     [    326 B] [2026-43-3:16:08:11] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/step05_build_tr 
    │     │     │  │                                                              ee.rs                     
30  │     │     │  └──• verbose_display.rs       [    781 B] [2026-43-5:16:25:41] ./src/libs/querypath/core 
    │     │     │                                                                 /features/verbose_display 
    │     │     │                                                                 .rs                       
    │     │     ├──┬ structs                     [ 20.4 KiB] [2026-43-3:12:27:41] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/                 
31  │     │     │  ├──• anchored_path.rs         [  2.6 KiB] [2026-43-5:11:06:12] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/anchored_path.rs 
32  │     │     │  ├──• fs_walk_stat.rs          [    597 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/fs_walk_stat.rs  
33  │     │     │  ├──• item_file_dir.rs         [    879 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/item_file_dir.rs 
34  │     │     │  ├──• path_context.rs          [  1.1 KiB] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/path_context.rs  
35  │     │     │  ├──• path_node.rs             [    761 B] [2026-43-5:11:06:21] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/path_node.rs     
36  │     │     │  ├──• pattern_compiled.rs      [  5.4 KiB] [2026-43-5:11:06:54] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/pattern_compiled 
    │     │     │  │                                                              .rs                       
37  │     │     │  ├──• patterns.rs              [  3.3 KiB] [2026-43-5:11:07:05] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/patterns.rs      
38  │     │     │  ├──• tree.rs                  [  1.5 KiB] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/tree.rs          
39  │     │     │  ├──• tree_bundle_item.rs      [    529 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/tree_bundle_item 
    │     │     │  │                                                              .rs                       
40  │     │     │  ├──• tree_bundle_result.rs    [  1.2 KiB] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/tree_bundle_resu 
    │     │     │  │                                                              lt.rs                     
41  │     │     │  ├──• tree_item.rs             [  1.1 KiB] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/tree_item.rs     
42  │     │     │  └──• walk_env_index.rs        [  1.6 KiB] [2026-43-5:11:07:48] ./src/libs/querypath/core 
    │     │     │                                                                 /structs/walk_env_index.r 
    │     │     │                                                                 s                         
    │     │     ├──┬ traits                      [  1.0 KiB] [2026-43-3:22:09:49] ./src/libs/querypath/core 
    │     │     │  │                                                              /traits/                  
43  │     │     │  ├──• apply_claim.rs           [    431 B] [2026-43-3:22:07:06] ./src/libs/querypath/core 
    │     │     │  │                                                              /traits/apply_claim.rs    
44  │     │     │  └──• path_env_index.rs        [    640 B] [2026-43-3:22:06:02] ./src/libs/querypath/core 
    │     │     │                                                                 /traits/path_env_index.rs 
    │     │     ├──┬ types                       [  2.3 KiB] [2026-43-2:16:31:45] ./src/libs/querypath/core 
    │     │     │  │                                                              /types/                   
45  │     │     │  ├──• consts.rs                [    777 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /types/consts.rs          
46  │     │     │  └──• enums.rs                 [  1.5 KiB] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │                                                                 /types/enums.rs           
    │     │     └──┬ utils                       [  2.9 KiB] [2026-43-3:11:05:26] ./src/libs/querypath/core 
    │     │        │                                                              /utils/                   
47  │     │        ├──• check_is_binary.rs       [    652 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │        │                                                              /utils/check_is_binary.rs 
48  │     │        ├──• diff_paths.rs            [    889 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │        │                                                              /utils/diff_paths.rs      
49  │     │        ├──• get_modified_time.rs     [    553 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │        │                                                              /utils/get_modified_time. 
    │     │        │                                                              rs                        
50  │     │        └──• normalize_path.rs        [    861 B] [2026-43-3:16:21:22] ./src/libs/querypath/core 
    │     │                                                                       /utils/normalize_path.rs  
    │     └──┬ timer                             [ 26.6 KiB] [2026-43-5:06:45:30] ./src/libs/timer/         
51  │        ├──• consts.rs                      [  2.0 KiB] [2026-43-5:08:37:10] ./src/libs/timer/consts.r 
    │        │                                                                    s                         
52  │        ├──• engine.rs                      [ 14.6 KiB] [2026-43-5:08:46:13] ./src/libs/timer/engine.r 
    │        │                                                                    s                         
53  │        ├──• enums.rs                       [  2.3 KiB] [2026-43-4:15:56:07] ./src/libs/timer/enums.rs 
54  │        ├──• functions.rs                   [  5.1 KiB] [2026-43-4:15:55:37] ./src/libs/timer/function 
    │        │                                                                    s.rs                      
55  │        └──• structs.rs                     [  2.7 KiB] [2026-43-5:07:18:12] ./src/libs/timer/structs. 
    │                                                                             rs                        
    └──┬ .zed                                    [  1.9 KiB] [2026-43-3:20:32:00] ./.zed/                   
56     └──• settings.json                        [  1.9 KiB] [2026-43-3:21:52:40] ./.zed/settings.json      

```

## Source Code Content

### [1] `./Cargo.toml`

```toml
[package]
edition = "2024"
rust-version = "1.99.0"
name = "cargo-plot"
version = "1.0.0"
authors = ["Jan Roman Cisowski „j-Cis”"]
include = [
    "src/**/*",
    "examples/**/*",
    "Cargo.toml",
    "Cargo.lock",
    "README.md",
    "LICENSE-APACHE",
    "LICENSE-MIT",
]
description = "A powerful Cargo plugin for blazing-fast filesystem scanning, structural tree visualization, and automated Markdown snapshot generation."
readme = "README.md"
keywords = [
    "cargo",
    "tree",
    "markdown",
    "filesystem",
    "documentation",
]
categories = [
    "development-tools::cargo-plugins",
    "command-line-utilities",
    "command-line-interface",
    "text-processing",
]
license = "MIT OR Apache-2.0"
repository = "https://github.com/j-Cis/cargo-plot"
resolver = "3"

[package.metadata.cargo]
edition = "2024"

[package.metadata.docs.rs]
targets = [
    "x86_64-pc-windows-msvc",
    "i686-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "i686-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
]

[dependencies]
jiff = "0.2.38"

# [[bin]]
# name = "cargo-plot"
# path = "src/main.rs"

[dependencies.anyhow]
version = "1.0.104"

[dependencies.crossterm]
version = "0.29.0"

[dependencies.inquire]
version = "0.9.4"

[dependencies.regex]
version = "1.13.1"

[dependencies.clap]
version = "4.6.7"
features = [
    "derive",
    "cargo",
    "unicode",
    "wrap_help",
    "string",
    "env",
    "unstable-doc",
    "unstable-markdown",
    "unstable-styles",
]

[dependencies.serde]
version = "1.0.229"
features = [
    "derive"
]

[dependencies.toml]
version = "1.1.8"

[dependencies.walkdir]
version = "2.5.0"

[lints.rust]
dead_code = "allow"
unsafe_code = "forbid"

[lints.clippy]
bool_comparison = "allow"
too_many_arguments = "allow"

```

### [2] `./.rustfmt.toml`

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

# 27【F03】 Automatyczne zawijanie komentarzy - UWAGA TRUE NISZCZY KOMPOZYCJE
wrap_comments = false

# 28【F04】 Normalizacja stylu komentarzy - UWAGA TRUE NISZCZY KOMPOZYCJE
normalize_comments = false


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

### [3] `./.cargo/config.toml`

```toml
# .cargo/config.toml

[alias]
x = "x-do"
xtask = "x-do"


# Wymuszenie statycznego linkowania bibliotek C (CRT) dla Windows.
# Dzięki temu plik .exe będzie działał na czystym Windows 7/8/10/11
# bez konieczności instalowania "Visual C++ Redistributable".

# Konfiguracja dla standardowych systemów 64-bitowych (Większość dzisiejszych PC)
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]

# Konfiguracja dla starych systemów 32-bitowych (Stare laptopy, Windows XP/7 32-bit)
[target.i686-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```

### [4] `./.cargo/.x-do.toml`

```toml
[snapshot]
title = "CargoPlot v2.0.0-dev."
title_sufix_date = true
title_sufix_date_fmt = "WYYY-WWW-D::hh:mm:ss:tt:qq"
name = "CargoPlot-2-0-0-dev-"
name_sufix_date = true
name_sufix_date_fmt = "WYWWWDhh-mmssttqq"
output_dir = "./CHANGELOG"
max_single_file_size = 524288
add_footer_summary_setups = true
add_header_customize_text = "\nGenerated via cargo-x-do\n"
add_fotter_customize_text = "\n---\n*Generated automatically via querypath & querypath-snapshot*\n"

[snapshot.scan]
match_patterns = [
	"./{src|examples|tests|sandobx|.cargo|.zed}/**",
	"./{Cargo|.rustfmt}.toml",
	"!./Cargo.lock",
	"!./{target|.git|dist|dump}/**"
]
keep_parent = true

[snapshot.sort]
same_name_priority = "FileFirst"
no_ext_priority = "Above"
ignore_leading_dot = true
case_precedence = "Insensitive"

[snapshot.incl]
numerate_dirs = false
numerate_binaries = false
numerate_skip = [
	"*.svg"
]
content_skip = [
	"./assets/**",
	"*.svg"
]
numerate_start_from = 1

[snapshot.tree]
name_width = 25
path_width = 25
column_order_left = []
column_order_right = ["Weight", "Temporal", "Path"]

[snapshot.tree.weight]
stats_weight = true
unit_system_binary = true
dir_display = "MatchedOnly"
precision = "0.1"

[snapshot.tree.temporal]
stats_temporal = true
pattern = "WYYY-WW-D:hh:mm:ss"

```

### [5] `./.cargo/.x-do-history.txt`

```plaintext
git log --oneline --graph --decorate -n 10
cargo +nightly fmt --all
```

### [6] `./.cargo/.x-do2.toml`

```toml
[snapshot]
title = "HELP ME"
title_sufix_date = false
title_sufix_date_fmt = "WYYY-WW-D::hh:mm:ss:tt:qq"
name = "HELP_ME"
name_sufix_date = false
name_sufix_date_fmt = "WYYYWW-Dhhmm-ssttqq"
output_dir = "./dump"
max_single_file_size = 524288
add_footer_summary_setups = true
add_header_customize_text = "---"
add_fotter_customize_text = "---"

[snapshot.scan]
match_patterns = [
	"./src/lib.rs",
	"./src/libs/timer.rs",
	"./src/libs/timer/**",
	"./examples/example2.rs",
	"./Cargo.toml",
]
keep_parent = false

[snapshot.sort]
same_name_priority = "FileFirst"
no_ext_priority = "Above"
ignore_leading_dot = true
case_precedence = "Insensitive"

[snapshot.incl]
numerate_dirs = false
numerate_binaries = false
numerate_skip = [
	"*.svg"
]
content_skip = [
	"./assets/**",
	"*.svg"
]
numerate_start_from = 1

[snapshot.tree]
name_width = 25
path_width = 55
column_order_left = []
column_order_right = ["Weight", "Temporal", "Path"]

[snapshot.tree.weight]
stats_weight = true
unit_system_binary = true
dir_display = "MatchedOnly"
precision = "0.1"

[snapshot.tree.temporal]
stats_temporal = true
pattern = "WYYY-WWW-D:hh:mm:ss"

```

### [7] `./examples/example_timer.rs`

```rust
use anyhow::Result;

use cargo_plot::timer::Timer;
fn main() -> Result<()> {
	let a = Timer::now();
	// println!("now {a:?}");
	// println!("now {:?}", a.fmt(WeekFirstDay::Mon));
	println!("Sformatowany string: {}", a.format("[YW]-W[WW]-[D][DDD] [hh]:[mm]:[ss]:[ssmil]"));

	Ok(())
}

```

### [8] `./examples/example01.rs`

```rust
//! Path: `examples/example01.rs`
//! Last significant modified: 2026-W43-3 22:13:37 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use anyhow::Result;
use cargo_plot::querypath::{ClaimPatterns, DebugVerboses, GatherClaims, ClaimRefining};

fn main() -> Result<()> {
	println!("🚀 Start ETAPU-1: Konfiguracja i Walidacja\n");

	// 1. Zbieranie deklaracji (Roszczeń)
	// Używamy krotki (Tuple), aby nadpisać tylko wzorce i logowanie.
	// Reszta ustawień (Ordering, Creating) załaduje się domyślnie.
	let claims = GatherClaims::new((
		ClaimPatterns::new()
			.patterns_all_matched(["**/querypath/**"])
			//.patterns_no_copypast(["*.svg"])
			.entry_dirs(["./{src|examples}/"])
			.case_sensitive(false),
		ClaimRefining::new().skip_nodoted_withext_files_text(true),
		DebugVerboses::new()
			.step1a_patterns(false)
			.step1b_entrydir(false)
			.step1c_walk(true)
			.step1d_execute(true),
	));

	// 2. Twarda walidacja (Pipeline)
	// Każdy etap weryfikuje dane i natychmiast wyrzuca błąd (?) w razie
	// pomyłki.
	let _prepared_setup = claims
		.step1a_valid_patterns()?
		.step1b_valid_entry_paths()?
		.step1c_valid_scan_walk()?
		.step1d_valid_execute_query()?
		.step99_if_all_valid_corect_then_gather_out_to_prepared_setup()?;

	//println!("\n✅ ETAP-1 Zakończony. Otrzymano twardy `PreparedSetup`!");

	// ========================================================================
	// ETAP-2: Z tego miejsca zwalidowany setup zasili logikę rysowania drzewa
	// i generowania raportów.
	// ========================================================================

	// let a = prepared_setup.prepare_get_paths();
	// println!("Zeskanowano poprawnie i przygotowano pakiety konfiguracyjne.");

	Ok(())
}

```

### [9] `./src/lib.rs`

```rust
//! Path: `src/lib.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

mod libs {
	pub mod querypath;
	pub mod timer;
}

pub use libs::*;

```

### [10] `./src/libs/querypath.rs`

```rust
//! Path: `src/libs/querypath.rs`
//! Last significant modified: 2026-W43-5 11:20 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

pub mod controllers;
pub mod core;


pub use controllers::{DebugVerboses,ClaimCreating,ClaimDefaults,ClaimOrdering,ClaimPatterns,GatherClaims, ClaimRefining};
```

### [11] `./src/libs/timer.rs`

```rust
pub mod enums;
pub use enums::WeekFirstDay;
pub mod structs;
pub use structs::Timer;

pub mod consts;
pub mod engine;
pub mod functions;

```

### [12] `./src/libs/querypath/controllers.rs`

```rust
//! Path: `src/libs/querypath/controllers.rs`
//! Last significant modified: 2026-W43-3 01:13 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

pub mod gather_claims;
pub mod prepar_setups;

pub use gather_claims::*;
pub use prepar_setups::*;

```

### [13] `./src/libs/querypath/core.rs`

```rust
//! Path: `src/libs/querypath/core.rs`
//! Last significant modified: 2026-W43-5 11:20 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

pub mod features;
pub mod structs;
pub mod traits;
pub mod types;
pub mod utils;

```

### [14] `./src/libs/querypath/controllers/gather_claims.rs`

```rust
//! Path: `src/libs/querypath/controllers/gather_claims.rs`
//! Last significant modified: 2026-W43-3 22:13:11 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

pub mod traits;
pub mod types;
pub mod engine;
pub use types::{DebugVerboses,ClaimCreating,ClaimDefaults,ClaimOrdering,ClaimPatterns,GatherClaims, ClaimRefining};
```

### [15] `./src/libs/querypath/controllers/prepar_setups.rs`

```rust
//! Path: `src/libs/querypath/controllers/prepar_setups.rs`
//! Last significant modified: 2026-W43-3 19:12:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::gather_claims::types::{ClaimCreating, ClaimOrdering, DebugVerboses};
use crate::libs::querypath::core::features::{
	HandlePreparedPatterns, HandleVerifiedDirs, HandleWalkScan, HandleCorpus,
};

/// Twardy, zwalidowany stan, z którego korzystać będą potoki tworzenia i
/// drukowania.
pub struct PreparedSetup {
	pub entry: HandleVerifiedDirs,
	pub patts: HandlePreparedPatterns,
	pub walk: HandleWalkScan,
	pub execute: HandleCorpus,
	pub ordering: ClaimOrdering,
	pub creating: ClaimCreating,
	pub verboses: DebugVerboses,
}

```

### [16] `./src/libs/querypath/controllers/gather_claims/engine.rs`

```rust
use anyhow::{Context, Result};

// use std::collections::HashMap;

use crate::libs::querypath::{
	controllers::{
		traits::ApplyClaim,
		types::{
			ClaimCreating, /*ClaimDefaults,*/ ClaimOrdering, ClaimPatterns, ClaimRefining, DebugVerboses,
			/*Gather,*/ GatherClaims,
		},
	},
	core::{
		features::{HandlePreparedPatterns, HandleVerifiedDirs, HandleWalkScan},
		types::enums::{
			EnumCasePrecedence, EnumCharClass, EnumColumn, EnumDirWeightDisplay, EnumGroupStrategy, EnumNoExtPriority,
			EnumSameNamePriority, EnumUnitSystem, EnumWeightPrecision,
		},
	},
};

// ============================================================================
// ============================================================================

impl DebugVerboses {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn step1b_entrydir(mut self, enable: bool) -> Self {
		self.step1b_entrydir = enable;
		self
	}

	#[must_use]
	pub fn step1a_patterns(mut self, enable: bool) -> Self {
		self.step1a_patterns = enable;
		self
	}

	#[must_use]
	pub fn step1c_walk(mut self, enable: bool) -> Self {
		self.step1c_walk = enable;
		self
	}

	#[must_use]
	pub fn step1d_execute(mut self, enable: bool) -> Self {
		self.step1d_execute = enable;
		self
	}
}

impl ApplyClaim for DebugVerboses {
	fn apply(self, claims: &mut GatherClaims) {
		claims.verboses = self;
	}
}

// ============================================================================
// 3. GŁÓWNY ZBIERACZ (GatherClaims)
// ============================================================================

impl GatherClaims {
	/// Tworzy nowy GatherClaims, odpakowując z użyciem Traita dostarczone
	/// konfiguracje.
	#[must_use]
	pub fn new<T>(configs: T) -> Self
	where
		T: ApplyClaim,
	{
		let mut instance = Self {
			patterns: ClaimPatterns::default(),
			refining: ClaimRefining::default(),
			ordering: ClaimOrdering::default(),
			creating: ClaimCreating::default(),
			verboses: DebugVerboses::default(),
			step1a_val_patts: None,
			step1b_val_entry: None,
			step1c_val_walk: None,
			step1d_val_execute: None,
		};
		// Metoda apply z Traita ApplyClaim automatycznie podmieni tylko
		// dostarczone pola!
		configs.apply(&mut instance);
		instance
	}

	pub fn step1a_valid_patterns(mut self) -> Result<Self> {
		let patts = HandlePreparedPatterns::build(
			&self.patterns.patterns_all_matched,
			&self.patterns.patterns_no_numerate,
			&self.patterns.patterns_no_copypast,
			self.patterns.case_sensitive,
			self.verboses.step1a_patterns,
		)
		.context("ETAP-1 [Błąd kroku 01]: Nieudana kompilacja wzorców")?;

		self.step1a_val_patts = Some(patts);
		Ok(self)
	}

	pub fn step1b_valid_entry_paths(mut self) -> Result<Self> {
		let entry = HandleVerifiedDirs::build(&self.patterns.entry_dirs, self.verboses.step1b_entrydir)
			.context("ETAP-1 [Błąd kroku 00]: Niepoprawne ścieżki wejściowe (entry_dirs)")?;

		self.step1b_val_entry = Some(entry);
		Ok(self)
	}

	pub fn step1c_valid_scan_walk(mut self) -> Result<Self> {
		let entry = self
			.step1b_val_entry
			.as_ref()
			.context("ETAP-1 [Błąd sekwencji]: Próba uruchomienia skanu przed walidacją wejścia!")?;

		// Skanowanie dysku z bezpośrednim odcięciem szufladek wg ClaimRefining
		let walk = HandleWalkScan::scan(entry, &self.refining, self.verboses.step1c_walk)
			.context("ETAP-1 [Błąd kroku 02]: Skanowanie dysku (HandleWalkScan) zakończone niepowodzeniem")?;

		self.step1c_val_walk = Some(walk);
		Ok(self)
	}

	pub fn step1d_valid_execute_query(mut self) -> Result<Self> {
		let patts =
			self.step1a_val_patts.as_ref().context("ETAP-1 [Błąd sekwencji]: Brak wzorców przed filtrowaniem!")?;

		let walk = self
			.step1c_val_walk
			.as_ref()
			.context("ETAP-1 [Błąd sekwencji]: Brak drzewa katalogów przed filtrowaniem!")?;

		let execute = crate::querypath::core::features::HandleCorpus::apply(
			&patts.all_matched,
			walk,
			self.creating.keep_parent,
			self.verboses.step1d_execute,
		)
		.context("ETAP-1 [Błąd kroku 03]: Filtracja wyników nie powiodła się!")?;

		self.step1d_val_execute = Some(execute);
		Ok(self)
	}
	/// Gwarantuje zebranie w pełni zwalidowanych obiektów i przekazanie ich do
	/// zablokowanego stanu (Read-Only Engine).
	pub fn step99_if_all_valid_corect_then_gather_out_to_prepared_setup(
		mut self,
	) -> Result<crate::libs::querypath::controllers::prepar_setups::PreparedSetup> {
		let entry = self.step1b_val_entry.take().context("Pominięto step1b_valid_entry_paths w łańcuchu walidacji!")?;
		let patts = self.step1a_val_patts.take().context("Pominięto step1a_valid_patterns w łańcuchu walidacji!")?;
		let walk = self.step1c_val_walk.take().context("Pominięto step1c_valid_scan_walk w łańcuchu walidacji!")?;
		let execute =
			self.step1d_val_execute.take().context("Pominięto step1d_valid_execute_query w łańcuchu walidacji!")?;

		Ok(crate::libs::querypath::controllers::prepar_setups::PreparedSetup {
			entry,
			patts,
			walk,
			execute,
			ordering: self.ordering,
			creating: self.creating,
			verboses: self.verboses,
		})
	}
}

// ============================================================================
// 2. STRUKTURY DANYCH I BUDOWNICZOWIE (Ukrycie Default::default)
// ============================================================================

impl ClaimPatterns {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn entry_dirs<I, S>(mut self, dirs: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.entry_dirs = dirs.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn patterns_all_matched<I, S>(mut self, patterns: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.patterns_all_matched = patterns.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn patterns_no_numerate<I, S>(mut self, patterns: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.patterns_no_numerate = patterns.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn patterns_no_copypast<I, S>(mut self, patterns: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.patterns_no_copypast = patterns.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn case_sensitive(mut self, sensitive: bool) -> Self {
		self.case_sensitive = sensitive;
		self
	}
}

impl ApplyClaim for ClaimPatterns {
	fn apply(self, claims: &mut GatherClaims) {
		claims.patterns = self;
	}
}

// ============================================================================
// ============================================================================

impl Default for ClaimRefining {
	fn default() -> Self {
		Self {
			skip_doted_dirs_empty: false,
			skip_nodot_dirs_empty: false,
			skip_doted_dirs_laden: false,
			skip_nodot_dirs_laden: false,
			skip_doted_noext_files_empty: false,
			skip_doted_withext_files_empty: false,
			skip_nodoted_noext_files_empty: false,
			skip_nodoted_withext_files_empty: false,
			skip_doted_noext_files_binary: false,
			skip_doted_withext_files_binary: false,
			skip_nodoted_noext_files_binary: false,
			skip_nodoted_withext_files_binary: false,
			skip_doted_noext_files_text: false,
			skip_doted_withext_files_text: false,
			skip_nodoted_noext_files_text: false,
			skip_nodoted_withext_files_text: false,
		}
	}
}

impl ClaimRefining {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn skip_doted_dirs_empty(mut self, val: bool) -> Self {
		self.skip_doted_dirs_empty = val;
		self
	}

	#[must_use]
	pub fn skip_nodot_dirs_empty(mut self, val: bool) -> Self {
		self.skip_nodot_dirs_empty = val;
		self
	}

	#[must_use]
	pub fn skip_doted_dirs_laden(mut self, val: bool) -> Self {
		self.skip_doted_dirs_laden = val;
		self
	}

	#[must_use]
	pub fn skip_nodot_dirs_laden(mut self, val: bool) -> Self {
		self.skip_nodot_dirs_laden = val;
		self
	}

	#[must_use]
	pub fn skip_doted_noext_files_empty(mut self, val: bool) -> Self {
		self.skip_doted_noext_files_empty = val;
		self
	}

	#[must_use]
	pub fn skip_doted_withext_files_empty(mut self, val: bool) -> Self {
		self.skip_doted_withext_files_empty = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_noext_files_empty(mut self, val: bool) -> Self {
		self.skip_nodoted_noext_files_empty = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_withext_files_empty(mut self, val: bool) -> Self {
		self.skip_nodoted_withext_files_empty = val;
		self
	}

	#[must_use]
	pub fn skip_doted_noext_files_binary(mut self, val: bool) -> Self {
		self.skip_doted_noext_files_binary = val;
		self
	}

	#[must_use]
	pub fn skip_doted_withext_files_binary(mut self, val: bool) -> Self {
		self.skip_doted_withext_files_binary = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_noext_files_binary(mut self, val: bool) -> Self {
		self.skip_nodoted_noext_files_binary = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_withext_files_binary(mut self, val: bool) -> Self {
		self.skip_nodoted_withext_files_binary = val;
		self
	}

	#[must_use]
	pub fn skip_doted_noext_files_text(mut self, val: bool) -> Self {
		self.skip_doted_noext_files_text = val;
		self
	}

	#[must_use]
	pub fn skip_doted_withext_files_text(mut self, val: bool) -> Self {
		self.skip_doted_withext_files_text = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_noext_files_text(mut self, val: bool) -> Self {
		self.skip_nodoted_noext_files_text = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_withext_files_text(mut self, val: bool) -> Self {
		self.skip_nodoted_withext_files_text = val;
		self
	}
}

impl ApplyClaim for ClaimRefining {
	fn apply(self, claims: &mut GatherClaims) {
		claims.refining = self;
	}
}

// ============================================================================
// ============================================================================

// Ręczna implementacja Default rozwiązuje błąd E0277 dla enumów
impl Default for ClaimOrdering {
	fn default() -> Self {
		Self {
			order_group_strategy: EnumGroupStrategy::Mixed,
			order_same_name_priority: EnumSameNamePriority::DirectoryFirst,
			order_ignore_leading_dot: false,
			order_split_extension: true,
			order_no_ext_priority: EnumNoExtPriority::Above,
			order_char_class_order: [EnumCharClass::Special, EnumCharClass::Digit, EnumCharClass::Letter],
			order_case_precedence: EnumCasePrecedence::Insensitive,
			order_reverse: false,
			numerate_dirs: true,
			numerate_binaries: true,
			numerate_start_from: 1,
		}
	}
}

impl ClaimOrdering {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn order_reverse(mut self, reverse: bool) -> Self {
		self.order_reverse = reverse;
		self
	}

	#[must_use]
	pub fn numerate_start_from(mut self, start: usize) -> Self {
		self.numerate_start_from = start;
		self
	}
}

impl ApplyClaim for ClaimOrdering {
	fn apply(self, claims: &mut GatherClaims) {
		claims.ordering = self;
	}
}

// ============================================================================
// ============================================================================

// Ręczna implementacja Default rozwiązuje błąd E0277
impl Default for ClaimCreating {
	fn default() -> Self {
		Self {
			wraper_name_width: 25,
			wraper_path_width: 35,
			column_order_left: vec![EnumColumn::Weight, EnumColumn::Temporal],
			column_order_right: vec![EnumColumn::Path],
			weight_binary_base: EnumUnitSystem::Binary,
			weight_dir_display: EnumDirWeightDisplay::MatchedOnly,
			weight_precision: EnumWeightPrecision::Tenths,
			keep_parent: true,
			include_files: true,
			include_binaries_in_matched: true,
			title: "CODE SNAPSHOT",
			title_sufix_timestamp: false,
			saveas: "snapshot_",
			saveas_sufix_timestamp: true,
			output_dir: "./dump".to_string(),
			limit_max_single_file_size: 524288,
			add_in_header_autoreklama: false,
			add_in_footer_autoreklama: true,
			add_in_header_table_setup: false,
			add_in_footer_table_setup: true,
			add_in_header_table_stats: false,
			add_in_footer_table_stats: false,
			add_in_header_paragraph: "",
			add_in_footer_paragraph: "",
		}
	}
}

impl ClaimCreating {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}
}

impl ApplyClaim for ClaimCreating {
	fn apply(self, claims: &mut GatherClaims) {
		claims.creating = self;
	}
}

```

### [17] `./src/libs/querypath/controllers/gather_claims/traits.rs`

```rust
//! Path: `src/libs/querypath/controllers/gather_claims/traits.rs`
//! Last significant modified: 2026-W43-5 11:46 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792
//!
use crate::libs::querypath::controllers::types::{ClaimDefaults, GatherClaims};
// ============================================================================
// 1. TRAIT KONFIGURACYJNY I KROTKI (Pozwala pominąć Enum Gather)
// ============================================================================

/// Trait aplikujący konfiguracje do głównego zbiornika roszczeń.
pub trait ApplyClaim {
	fn apply(self, claims: &mut GatherClaims);
}

impl ApplyClaim for ClaimDefaults {
	fn apply(self, _claims: &mut GatherClaims) {}
}

// Obsługa krotek dla jednoczesnego wstrzykiwania wielu różnych konfiguracji.
impl<A: ApplyClaim, B: ApplyClaim> ApplyClaim for (A, B) {
	fn apply(self, claims: &mut GatherClaims) {
		self.0.apply(claims);
		self.1.apply(claims);
	}
}

impl<A: ApplyClaim, B: ApplyClaim, C: ApplyClaim> ApplyClaim for (A, B, C) {
	fn apply(self, claims: &mut GatherClaims) {
		self.0.apply(claims);
		self.1.apply(claims);
		self.2.apply(claims);
	}
}

impl<A: ApplyClaim, B: ApplyClaim, C: ApplyClaim, D: ApplyClaim> ApplyClaim for (A, B, C, D) {
	fn apply(self, claims: &mut GatherClaims) {
		self.0.apply(claims);
		self.1.apply(claims);
		self.2.apply(claims);
		self.3.apply(claims);
	}
}

```

### [18] `./src/libs/querypath/controllers/gather_claims/types.rs`

```rust
//! Path: `src/libs/querypath/controllers/gather_claims/structs.rs`
//! Last significant modified: 2026-W43-5 11:39 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

// use std::collections::HashMap;
use crate::libs::querypath::core::{
	features::{HandlePreparedPatterns, HandleVerifiedDirs, HandleWalkScan},
	types::enums::{
		EnumCasePrecedence, EnumCharClass, EnumColumn, EnumDirWeightDisplay, EnumGroupStrategy, EnumNoExtPriority,
		EnumSameNamePriority, EnumUnitSystem, EnumWeightPrecision,
	},
};

#[derive(Debug, Clone, Default)]
pub struct DebugVerboses {
	pub step1a_patterns: bool,
	pub step1b_entrydir: bool,
	pub step1c_walk: bool,
	pub step1d_execute: bool,
}

/// Znacznik dla całkowicie domyślnych ustawień (szybki start).
pub struct ClaimDefaults;

// --- STRUKTURY DANYCH (CLAIMS) ---
#[derive(Debug, Clone, Default)]
pub struct ClaimPatterns {
	pub entry_dirs: Vec<String>,
	pub patterns_all_matched: Vec<String>,
	pub patterns_no_numerate: Vec<String>,
	pub patterns_no_copypast: Vec<String>,
	pub case_sensitive: bool,
}

#[derive(Debug, Clone)]
pub struct ClaimRefining {
	pub skip_doted_dirs_empty: bool,
	pub skip_nodot_dirs_empty: bool,
	pub skip_doted_dirs_laden: bool,
	pub skip_nodot_dirs_laden: bool,
	pub skip_doted_noext_files_empty: bool,
	pub skip_doted_withext_files_empty: bool,
	pub skip_nodoted_noext_files_empty: bool,
	pub skip_nodoted_withext_files_empty: bool,
	pub skip_doted_noext_files_binary: bool,
	pub skip_doted_withext_files_binary: bool,
	pub skip_nodoted_noext_files_binary: bool,
	pub skip_nodoted_withext_files_binary: bool,
	pub skip_doted_noext_files_text: bool,
	pub skip_doted_withext_files_text: bool,
	pub skip_nodoted_noext_files_text: bool,
	pub skip_nodoted_withext_files_text: bool,
}

#[derive(Debug, Clone)]
pub struct ClaimOrdering {
	pub order_group_strategy: EnumGroupStrategy,
	pub order_same_name_priority: EnumSameNamePriority,
	pub order_ignore_leading_dot: bool,
	pub order_split_extension: bool,
	pub order_no_ext_priority: EnumNoExtPriority,
	pub order_char_class_order: [EnumCharClass; 3],
	pub order_case_precedence: EnumCasePrecedence,
	pub order_reverse: bool,
	pub numerate_dirs: bool,
	pub numerate_binaries: bool,
	pub numerate_start_from: usize,
}

#[derive(Debug, Clone)]
pub struct ClaimCreating {
	pub wraper_name_width: usize,
	pub wraper_path_width: usize,
	pub column_order_left: Vec<EnumColumn>,
	pub column_order_right: Vec<EnumColumn>,
	pub weight_binary_base: EnumUnitSystem,
	pub weight_dir_display: EnumDirWeightDisplay,
	pub weight_precision: EnumWeightPrecision,
	pub keep_parent: bool,
	pub include_files: bool,
	pub include_binaries_in_matched: bool,
	pub title: &'static str,
	pub title_sufix_timestamp: bool,
	pub saveas: &'static str,
	pub saveas_sufix_timestamp: bool,
	pub output_dir: String,
	pub limit_max_single_file_size: u32,
	pub add_in_header_autoreklama: bool,
	pub add_in_footer_autoreklama: bool,
	pub add_in_header_table_setup: bool,
	pub add_in_footer_table_setup: bool,
	pub add_in_header_table_stats: bool,
	pub add_in_footer_table_stats: bool,
	pub add_in_header_paragraph: &'static str,
	pub add_in_footer_paragraph: &'static str,
}

pub enum Gather {
	Defaults,
	Patterns(ClaimPatterns),
	Ordering(ClaimOrdering),
	Creating(ClaimCreating),
	Verboses(DebugVerboses),
}

pub struct GatherClaims {
	pub patterns: ClaimPatterns,
	pub refining: ClaimRefining,
	pub ordering: ClaimOrdering,
	pub creating: ClaimCreating,
	pub verboses: DebugVerboses,

	// Używamy zaktualizowanych ścieżek z katalogu features
	pub step1b_val_entry: Option<HandleVerifiedDirs>,
	pub step1a_val_patts: Option<HandlePreparedPatterns>,
	pub step1c_val_walk: Option<HandleWalkScan>,
	pub step1d_val_execute: Option<crate::querypath::core::features::HandleCorpus>,
	// step03_val_sorts
	// step04_val_filtr
	// step05_val_add_index
	// step06_val_get_stats
	// step07_val_result_paths
	// i wiele innych
}

```

### [19] `./src/libs/querypath/core/features.rs`

```rust
//! Path: `src/libs/querypath/features.rs`
//! Last significant modified: 2026-W43-3 15:03:30 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

mod handle_formatted_time;
mod handle_prepared_patterns;
mod handle_verified_dirs;
mod handle_walk_scan;
mod handle_corpus;
mod verbose_display;

pub use verbose_display::*;
pub use handle_formatted_time::*;
pub use handle_prepared_patterns::*;
pub use handle_verified_dirs::*;
pub use handle_walk_scan::*;
pub use handle_corpus::*;

```

### [20] `./src/libs/querypath/core/structs.rs`

```rust
//! Path: `src/libs/querypath/structs.rs`
//! Last significant modified: 2026-W43-3 14:28:29 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

mod anchored_path;
mod fs_walk_stat;
mod item_file_dir;
mod path_context;
mod path_node;
mod pattern_compiled;
mod patterns;
mod tree;
mod tree_bundle_item;
mod tree_bundle_result;
mod tree_item;
mod walk_env_index;

pub use anchored_path::*;
pub use fs_walk_stat::*;
pub use item_file_dir::*;
pub use path_context::*;
pub use path_node::*;
pub use pattern_compiled::*;
pub use patterns::*;
pub use tree::*;
pub use tree_bundle_item::*;
pub use tree_bundle_result::*;
pub use tree_item::*;
pub use walk_env_index::*;

```

### [21] `./src/libs/querypath/core/traits.rs`

```rust
//! Path: `src/libs/querypath/traits.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

mod path_env_index;
pub use path_env_index::*;

```

### [22] `./src/libs/querypath/core/types.rs`

```rust
//! Path: `src/libs/querypath/types.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

pub mod consts;
pub mod enums;

```

### [23] `./src/libs/querypath/core/utils.rs`

```rust
//! Path: `src/libs/querypath/utils.rs`
//! Last significant modified: 2026-W43-3 13:06:02 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

mod normalize_path;
pub use normalize_path::normalize_path;

mod check_is_binary;
pub use check_is_binary::check_is_binary;

mod get_modified_time;
pub use get_modified_time::get_modified_time;

mod diff_paths;
pub use diff_paths::diff_paths;

```

### [24] `./src/libs/querypath/core/features/handle_corpus.rs`

```rust
//! Path: `src/libs/querypath/features/handle_corpus.rs`
//! Last significant modified: 2026-W43-3 19:07:00 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::{
	borrow::Borrow,
	collections::{HashMap, HashSet},
	fmt, fs,
};

use crate::querypath::core::{
	features::{HandleWalkScan, VerboseDisplay},
	structs::{DirItem, FileItem, PathsPatterns, WalkEnvIndex},
	traits::PattEnvIndex,
	utils::{check_is_binary, get_modified_time},
};
use anyhow::Result;

// ============================================================================
// GŁÓWNY EGZEKUTOR
// ============================================================================

pub struct HandleCorpus {
	pub matched_files: Vec<FileItem>,
	pub matched_dirs: Vec<DirItem>,
	pub scanned_files: usize,
	pub scanned_dirs: usize,
}

impl HandleCorpus {
	pub fn apply<P, W>(patterns: P, walk: W, keep_parent: bool, debug: bool) -> Result<Self>
	where
		P: Borrow<PathsPatterns>,
		W: Borrow<HandleWalkScan>,
	{
		// Odpakowanie referencji z generyków
		let patterns_ref = patterns.borrow();
		let walk_ref = walk.borrow();

		// 0. Zbieranie rozmiarów fizycznych wszystkich zeskanowanych plików
		let mut file_sizes: HashMap<String, u64> = HashMap::new();
		for file in walk_ref.iter_all_files() {
			let meta_size = fs::metadata(&file.str).map(|m| m.len()).unwrap_or(0);
			file_sizes.insert(file.str.clone(), meta_size);
		}

		// 1. env to pamięciowy indeks (cache) plików i katalogów
		//    przeskanowanych przez HandleWalkScan.
		let env = WalkEnvIndex::from_walk(walk_ref);

		// 2. Filtrowanie plików wzorcem oraz budowa FileItem
		let mut matched_files: Vec<FileItem> = Vec::new();
		for file in walk_ref.iter_all_files() {
			if patterns_ref.is_match(&file.str, &env) {
				let size = *file_sizes.get(&file.str).unwrap_or(&0);
				let is_binary = check_is_binary(&file.str);
				let modified_at = get_modified_time(&file.str);

				matched_files.push(FileItem { path: file.str.clone(), size, is_binary, modified_at });
			}
		}

		// 3. Budowa zbioru katalogów i ich wyliczanie
		let mut matched_dirs_set: HashSet<String> = walk_ref
			.iter_all_dirs()
			.map(|d| d.str.as_str())
			.filter(|d| patterns_ref.is_match(d, &env))
			.map(String::from)
			.collect();

		if keep_parent {
			for file_item in &matched_files {
				let mut current: &str = file_item.path.as_str();
				while let Some((parent, _)) = current.rsplit_once('/') {
					if parent.is_empty() {
						break;
					}
					let dir_candidate = format!("{parent}/");
					if env.has_dir(&dir_candidate) {
						matched_dirs_set.insert(dir_candidate);
					}
					current = parent;
				}
			}
		}

		// 4. Kalkulacja real_size oraz matched_size dla każdego katalogu
		let matched_files_map: HashMap<&str, u64> = matched_files.iter().map(|f| (f.path.as_str(), f.size)).collect();

		let mut matched_dir_items: Vec<DirItem> = Vec::new();

		for dir_path in matched_dirs_set {
			let mut real_size: u64 = 0;
			let mut matched_size: u64 = 0;

			for file in walk_ref.iter_all_files() {
				if file.str.starts_with(&dir_path) {
					let f_size = *file_sizes.get(&file.str).unwrap_or(&0);
					real_size += f_size;

					if matched_files_map.contains_key(file.str.as_str()) {
						matched_size += f_size;
					}
				}
			}

			let modified_at = get_modified_time(&dir_path);

			matched_dir_items.push(DirItem { path: dir_path, real_size, matched_size, modified_at });
		}

		matched_dir_items.sort_by(|a, b| a.path.cmp(&b.path));

		// Kompilujemy ostateczną strukturę
		let instance = Self {
			matched_files,
			matched_dirs: matched_dir_items,
			scanned_files: walk_ref.count_all_files(),
			scanned_dirs: walk_ref.count_all_dirs(),
		};

		// 1e📕 Self-Logowanie procesu egzekucji
		if debug {
			println!("{}", instance.display_debug());
		}

		Ok(instance)
	}
}

impl VerboseDisplay for HandleCorpus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		//writeln!(f, "1e📕 [ structs: HandleCorpus, FileItem, DirItem ] 🔰")?;

		//writeln!(f, "📄📄📄📄📄📄📄")?;
		for file in &self.matched_files {
			//writeln!(
			//	f,
			//	"📄  {:?} (size: {} B, bin: {}, mod: {:?})",
			//	file.path, file.size, file.is_binary, file.modified_at
			//)?;
			writeln!(f, "📄  {:?} ", file)?;
		}

		//writeln!(f, "📁📁📁📁📁📁📁")?;
		for dir in &self.matched_dirs {
			//writeln!(
			//	f,
			//	"📁  {:?} (matched: {} B, real: {} B, mod: {:?})",
			//	dir.path, dir.matched_size, dir.real_size, dir.modified_at
			//)?;
			writeln!(f, "📁  {:?} ", dir)?;
		}

		//writeln!(f, "📊📊📊📊📊📊📊")?;
		//writeln!(f, "📊 count matched: 📄 = {};  📁 = {};", self.matched_files.len(), self.matched_dirs.len())?;
		//writeln!(f, "📊 count scanned: 📄 = {};  📁 = {};", self.scanned_files, self.scanned_dirs)?;

		//writeln!(f, "1e📕 [ structs: HandleCorpus, FileItem, DirItem ] 🏁")?;

		Ok(())
	}
}

```

### [25] `./src/libs/querypath/core/features/handle_formatted_time.rs`

```rust
//! Path: `src/libs/querypath/features/handle_formatted_time.rs`

use crate::libs::querypath::core::features::VerboseDisplay;
use crate::libs::timer::Timer;
use anyhow::{Result, bail};
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandleFormattedTime {
	pub pattern_tree: &'static str,
	pub pattern_text: &'static str,
	pub pattern_name: &'static str,
}

impl Default for HandleFormattedTime {
	fn default() -> Self {
		Self {
			pattern_tree: "[YW]-W[WW]-[D][DDD] [hh]:[mm]:[ss]",
			pattern_text: "[YW]-W[WW]-[D][DDD] [hh]:[mm]:[ss]",
			pattern_name: "[YW]W[WW]-[D][DDD][hh][mm]-[ss][tt]",
		}
	}
}

impl HandleFormattedTime {
	pub fn build(
		pattern_tree: &'static str,
		pattern_text: &'static str,
		pattern_name: &'static str,
		debug: bool,
	) -> Result<Self> {
		if pattern_tree.trim().is_empty() {
			bail!("Wzorzec czasu dla drzewa (pattern_tree) nie może być pusty.");
		}
		if pattern_text.trim().is_empty() {
			bail!("Wzorzec czasu dla tekstu (pattern_text) nie może być pusty.");
		}
		if pattern_name.trim().is_empty() {
			bail!("Wzorzec czasu dla nazwy pliku (pattern_name) nie może być pusty.");
		}

		let instance = Self { pattern_tree, pattern_text, pattern_name };

		if debug {
			println!("{}", instance.display_debug());
		}

		Ok(instance)
	}

	#[inline]
	pub fn format_tree(&self, timestamp_secs: u64) -> String {
		Timer::from_secs(timestamp_secs).format(self.pattern_tree)
	}

	#[inline]
	pub fn format_text(&self, timestamp_secs: u64) -> String {
		Timer::from_secs(timestamp_secs).format(self.pattern_text)
	}

	#[inline]
	pub fn format_name(&self, timestamp_secs: u64) -> String {
		Timer::from_secs(timestamp_secs).format(self.pattern_name)
	}

	#[inline]
	pub fn format_system_time_tree(&self, st: SystemTime) -> String {
		Timer::from_system_time(st).format(self.pattern_tree)
	}

	#[inline]
	pub fn format_system_time_name(&self, st: SystemTime) -> String {
		Timer::from_system_time(st).format(self.pattern_name)
	}
}

impl VerboseDisplay for HandleFormattedTime {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "99z📕 [ structs: HandleFormattedTime ] 🔰")?;
		writeln!(f, "  🌲 pattern_tree 🕒: {:?}", self.pattern_tree)?;
		writeln!(f, "  📄 pattern_text 🕒: {:?}", self.pattern_text)?;
		writeln!(f, "  🏷️  pattern_name 🕒: {:?}", self.pattern_name)?;
		writeln!(f, "99z📕 [ structs: HandleFormattedTime ] 🏁")?;
		Ok(())
	}
}

```

### [26] `./src/libs/querypath/core/features/handle_prepared_patterns.rs`

```rust
//! Path: `src/libs/querypath/features/handle_prepared_patterns.rs`
//! Last significant modified: 2026-W43-3 22:13:17 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use crate::querypath::core::{features::VerboseDisplay, structs::PathsPatterns};
use anyhow::{Context, Result};
use std::fmt;

/// Komplet wzorców jednego zapytania, podzielony na grupy.
#[derive(Debug, Clone)]
pub struct HandlePreparedPatterns {
	pub all_matched: PathsPatterns,
	pub no_numerate: PathsPatterns,
	pub no_copypast: PathsPatterns,
}

impl HandlePreparedPatterns {
	pub fn build(
		all_matched: &[String],
		no_numerate: &[String],
		no_copypast: &[String],
		case_sensitive: bool,
		debug: bool,
	) -> Result<Self> {
		let instance = Self {
			all_matched: PathsPatterns::build(all_matched, case_sensitive)
				.context("Grupa 'all_matched': nieudana kompilacja wzorców")?,
			no_numerate: PathsPatterns::build(no_numerate, case_sensitive)
				.context("Grupa 'no_numerate': nieudana kompilacja wzorców")?,
			no_copypast: PathsPatterns::build(no_copypast, case_sensitive)
				.context("Grupa 'no_copypast': nieudana kompilacja wzorców")?,
		};

		if debug {
			println!("{}", instance.display_debug());
		}

		Ok(instance)
	}

	fn fmt_group(f: &mut fmt::Formatter<'_>, name: &str, group: &PathsPatterns) -> fmt::Result {
		if group.rules_count() == 0 {
			return Ok(());
		}
		writeln!(f, "── {name} ({} reguł, case_sensitive: {}) ──", group.rules_count(), group.case_sensitive)?;

		let mut cursor = 0;
		for (r_idx, raw) in group.raw.0.iter().enumerate() {
			let r_num = format!("{:02}", r_idx + 1);
			if r_idx > 0 {
				writeln!(f)?;
			}
			writeln!(f, "{r_num} raw 🔎: {raw:?}")?;

			for (e_idx, exp) in PathsPatterns::expand_braces(raw).iter().enumerate() {
				let e_num = format!("{:02}", e_idx + 1);
				if let Some(rule) = group.compiled.get(cursor) {
					writeln!(
						f,
						"{r_num}:{e_num}  🪄 : 【{} 】    🔎: {:?}    {}",
						rule.regex,
						exp,
						format!(
							"⚗️ : base name: {:?}, is negated: {} , dir only: {} , requires sibling: {} , requires orphan: {} ,",
							rule.base_name,
							if rule.is_negated { "✔️" } else { "❌" },
							if rule.targets_dir_only { "✔️" } else { "❌" },
							if rule.requires_sibling { "✔️" } else { "❌" },
							if rule.requires_orphan { "✔️" } else { "❌" }
						)
					)?;
				}
				cursor += 1;
			}
		}
		Ok(())
	}
}

impl VerboseDisplay for HandlePreparedPatterns {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(
			f,
			"1a📕 [ structs: HandlePreparedPatterns, PathsPatterns, PattRaw, PattExp, PatternCompiled, Regex ] 🔰"
		)?;

		let mut has_printed = false;
		for (name, group) in [
			("all_matched", &self.all_matched),
			("no_numerate", &self.no_numerate),
			("no_copypast", &self.no_copypast),
		] {
			if group.rules_count() > 0 {
				if has_printed {
					writeln!(f)?;
				}
				Self::fmt_group(f, name, group)?;
				has_printed = true;
			}
		}

		writeln!(
			f,
			"1a📕 [ structs: HandlePreparedPatterns, PathsPatterns, PattRaw, PattExp, PatternCompiled, Regex ] 🏁"
		)?;
		Ok(())
	}
}

```

### [27] `./src/libs/querypath/core/features/handle_verified_dirs.rs`

```rust
//! Path: `src/libs/querypath/features/handle_verified_dirs.rs`
//! Last significant modified: 2026-W43-3 14:55:46 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use crate::querypath::core::{features::VerboseDisplay, structs::PathAnchored, utils::normalize_path};
use anyhow::{Context, Result, bail};
use std::{env, fmt, fs, path::PathBuf};

/// Menadżer zarządzający wszystkimi lokalizacjami startowymi skanowania.
/// Rozwija alternatywy `{a|b}` oraz weryfikuje ich istnienie.
#[derive(Debug, Clone)]
pub struct HandleVerifiedDirs {
	/// Napisy podane przez użytkownika (po `trim`, puste zamienione na `./`).
	pub raw: Vec<String>,
	pub handles: Vec<PathAnchored>,
	pub cwd_buf: PathBuf,
	pub cwd_str: String,
}

impl HandleVerifiedDirs {
	/// Inicjuje listę celów skanowania na podstawie podanych przez użytkownika
	/// stringów. Jeśli lista jest pusta lub zawiera puste napisy (`""`),
	/// domyślnie używa katalogu roboczego (`./`).
	pub fn build<I, S>(inputs: I, debug: bool) -> Result<Self>
	where
		I: IntoIterator<Item = S>,
		S: AsRef<str>,
	{
		// 1. Ustalenie kanonicznego CWD (również przepuszczonego przez
		//    fs::canonicalize)
		let raw_cwd = env::current_dir().context("Nie można odczytać katalogu roboczego (CWD)")?;
		let cwd_buf = fs::canonicalize(&raw_cwd).unwrap_or(raw_cwd);
		let cwd_str = normalize_path(&cwd_buf);

		let mut raw_inputs: Vec<String> = Vec::new();
		for input in inputs {
			let s = input.as_ref().trim();
			if s.is_empty() {
				raw_inputs.push("./".to_string());
			} else {
				raw_inputs.push(s.to_string());
			}
		}

		// 2. Jeśli po przefiltrowaniu lista jest pusta - domyślnie badamy CWD
		if raw_inputs.is_empty() {
			raw_inputs.push("./".to_string());
		}

		let raw = raw_inputs.clone();

		let mut expanded_paths = Vec::new();
		for input in raw_inputs {
			expanded_paths.extend(Self::expand_alternatives(&input));
		}

		// 3. Budowanie punktów zakotwiczenia.
		let mut handles: Vec<PathAnchored> = Vec::new();
		for expanded in expanded_paths {
			if let Ok(anchored) = PathAnchored::resolve(&expanded, &cwd_buf)
				&& !handles.iter().any(|handle| handle.buf == anchored.buf)
			{
				handles.push(anchored);
			}
		}

		if handles.is_empty() {
			bail!("Żadna z podanych lub rozwiniętych ścieżek wejściowych nie istnieje na dysku.");
		}

		let instance = Self { raw, handles, cwd_buf, cwd_str };

		if debug {
			println!("{}", instance.display_debug()); // używasz swojego traitu
		}

		Ok(instance)
	}

	/// Rekurencyjnie rozwija alternatywne bloki zapisane w klamrach,
	/// np.: `./{abc/ao/|def/}` -> `./abc/ao/` oraz `./def/`
	fn expand_alternatives(input: &str) -> Vec<String> {
		let Some(end) = input.find('}') else {
			return vec![input.to_string()];
		};

		let Some(sub) = input.get(..end) else {
			return vec![input.to_string()];
		};

		let Some(start) = sub.rfind('{') else {
			return vec![input.to_string()];
		};

		let (Some(prefix), Some(suffix), Some(options)) =
			(input.get(..start), input.get(end + 1..), input.get(start + 1..end))
		else {
			return vec![input.to_string()];
		};

		let mut expanded = Vec::new();

		for opt in options.split('|') {
			let merged = format!("{prefix}{opt}{suffix}");
			expanded.extend(Self::expand_alternatives(&merged));
		}

		expanded
	}
}

// ============================================================================
// FORMATOWANIE WYNIKOWE
// ============================================================================

impl VerboseDisplay for HandleVerifiedDirs {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "1b📕 [ structs: HandleVerifiedDirs, PathAnchored ] 🔰")?;

		writeln!(f, "cwd str 👀: {:?}", self.cwd_str)?;
		writeln!(f, "cwd buf 👀: {:?}", self.cwd_buf.to_string_lossy())?;

		let raw = self.raw.join(", ");
		writeln!(f, "raw paths 💡: {:?}", raw)?;
		for (i, handle) in self.handles.iter().enumerate() {
			let num = format!("{:>2}", i + 1);
			writeln!(f, "path {num} rel 🎯: {:?}", handle.rel)?;
			writeln!(f, "path {num} str 🎯: {:?}", handle.str)?;
			writeln!(f, "path {num} buf 🎯: {:?}", handle.buf.to_string_lossy())?;
		}
		writeln!(f, "1b📕 [ structs: HandleVerifiedDirs, PathAnchored ] 🏁")?;

		Ok(())
	}
}

```

### [28] `./src/libs/querypath/core/features/handle_walk_scan.rs`

```rust
//! Path: `src/libs/querypath/features/handle_walk_scan.rs`
//! Last significant modified: 2026-W43-3 19:12:17 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use anyhow::Result;
use std::{
	fmt,
	path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::querypath::{
	controllers::types::ClaimRefining,
	core::{
		features::{HandleVerifiedDirs, VerboseDisplay},
		structs::{PathAnchored, PathNode},
		utils::{check_is_binary, normalize_path},
	},
};

/// Surowy rezultat przejścia po drzewie katalogów.
#[derive(Debug, Clone, Default)]
pub struct HandleWalkScan {
	// katalogi puste
	pub dirs_empty: Vec<PathNode>,
	pub dotdirs_empty: Vec<PathNode>,
	pub count_dirs_empty: usize,
	pub count_dotdirs_empty: usize,

	// katalogi nie puste
	pub dirs_no_empty: Vec<PathNode>,
	pub dotdirs_no_empty: Vec<PathNode>,
	pub count_dirs_no_empty: usize,
	pub count_dotdirs_no_empty: usize,

	// pliki puste
	pub files_empty_no_ext: Vec<PathNode>,
	pub dotfiles_empty_no_ext: Vec<PathNode>,
	pub files_empty_has_ext: Vec<PathNode>,
	pub dotfiles_empty_has_ext: Vec<PathNode>,
	pub count_files_empty_no_ext: usize,
	pub count_dotfiles_empty_no_ext: usize,
	pub count_files_empty_has_ext: usize,
	pub count_dotfiles_empty_has_ext: usize,

	// pliki nie textowe - czyli binarne
	pub files_binary_no_ext: Vec<PathNode>,
	pub dotfiles_binary_no_ext: Vec<PathNode>,
	pub files_binary_has_ext: Vec<PathNode>,
	pub dotfiles_binary_has_ext: Vec<PathNode>,
	pub count_files_binary_no_ext: usize,
	pub count_dotfiles_binary_no_ext: usize,
	pub count_files_binary_has_ext: usize,
	pub count_dotfiles_binary_has_ext: usize,

	// pliki nie binarne - czyli prawdopodobnie tekstowe
	pub files_text_no_empty_no_ext: Vec<PathNode>,
	pub dotfiles_text_no_empty_no_ext: Vec<PathNode>,
	pub files_text_no_empty_has_ext: Vec<PathNode>,
	pub dotfiles_text_no_empty_has_ext: Vec<PathNode>,
	pub count_files_text_no_empty_no_ext: usize,
	pub count_dotfiles_text_no_empty_no_ext: usize,
	pub count_files_text_no_empty_has_ext: usize,
	pub count_dotfiles_text_no_empty_has_ext: usize,
}

impl HandleWalkScan {
	pub fn count_all_dirs(&self) -> usize {
		self.count_dirs_empty + self.count_dotdirs_empty + self.count_dirs_no_empty + self.count_dotdirs_no_empty
	}

	pub fn count_all_files(&self) -> usize {
		self.count_files_empty_no_ext
			+ self.count_dotfiles_empty_no_ext
			+ self.count_files_empty_has_ext
			+ self.count_dotfiles_empty_has_ext
			+ self.count_files_binary_no_ext
			+ self.count_dotfiles_binary_no_ext
			+ self.count_files_binary_has_ext
			+ self.count_dotfiles_binary_has_ext
			+ self.count_files_text_no_empty_no_ext
			+ self.count_dotfiles_text_no_empty_no_ext
			+ self.count_files_text_no_empty_has_ext
			+ self.count_dotfiles_text_no_empty_has_ext
	}

	/// Iterator scalający wszystkie 4 kategorie katalogów (Zero-Copy)
	pub fn iter_all_dirs(&self) -> impl Iterator<Item = &PathNode> {
		self.dirs_empty
			.iter()
			.chain(self.dotdirs_empty.iter())
			.chain(self.dirs_no_empty.iter())
			.chain(self.dotdirs_no_empty.iter())
	}

	/// Iterator scalający wszystkie 12 kategorii plików (Zero-Copy)
	pub fn iter_all_files(&self) -> impl Iterator<Item = &PathNode> {
		self.files_empty_no_ext
			.iter()
			.chain(self.dotfiles_empty_no_ext.iter())
			.chain(self.files_empty_has_ext.iter())
			.chain(self.dotfiles_empty_has_ext.iter())
			.chain(self.files_binary_no_ext.iter())
			.chain(self.dotfiles_binary_no_ext.iter())
			.chain(self.files_binary_has_ext.iter())
			.chain(self.dotfiles_binary_has_ext.iter())
			.chain(self.files_text_no_empty_no_ext.iter())
			.chain(self.dotfiles_text_no_empty_no_ext.iter())
			.chain(self.files_text_no_empty_has_ext.iter())
			.chain(self.dotfiles_text_no_empty_has_ext.iter())
	}

	/// Czyści konkretne szufladki na podstawie flag ClaimRefining
	pub fn refine(&mut self, config: &ClaimRefining, debug: bool) {
		let mut active = 0;
		let mut log_out = String::new();

		macro_rules! cut {
			($flag:expr, $list:expr, $msg:expr) => {
				if $flag {
					$list.clear();
					if debug {
						log_out.push_str(&format!("  ✂️ CUT: {}\n", $msg));
						active += 1;
					}
				}
			};
		}

		// Katalogi
		cut!(config.skip_doted_dirs_empty, self.dotdirs_empty, "skip_doted_dirs_empty");
		cut!(config.skip_nodot_dirs_empty, self.dirs_empty, "skip_nodot_dirs_empty");
		cut!(config.skip_doted_dirs_laden, self.dotdirs_no_empty, "skip_doted_dirs_laden");
		cut!(config.skip_nodot_dirs_laden, self.dirs_no_empty, "skip_nodot_dirs_laden");

		// Pliki puste
		cut!(config.skip_doted_noext_files_empty, self.dotfiles_empty_no_ext, "skip_doted_noext_files_empty");
		cut!(config.skip_doted_withext_files_empty, self.dotfiles_empty_has_ext, "skip_doted_withext_files_empty");
		cut!(config.skip_nodoted_noext_files_empty, self.files_empty_no_ext, "skip_nodoted_noext_files_empty");
		cut!(
			config.skip_nodoted_withext_files_empty,
			self.files_empty_has_ext,
			"skip_nodoted_withext_files_empty"
		);

		// Pliki binarne
		cut!(config.skip_doted_noext_files_binary, self.dotfiles_binary_no_ext, "skip_doted_noext_files_binary");
		cut!(
			config.skip_doted_withext_files_binary,
			self.dotfiles_binary_has_ext,
			"skip_doted_withext_files_binary"
		);
		cut!(config.skip_nodoted_noext_files_binary, self.files_binary_no_ext, "skip_nodoted_noext_files_binary");
		cut!(
			config.skip_nodoted_withext_files_binary,
			self.files_binary_has_ext,
			"skip_nodoted_withext_files_binary"
		);

		// Pliki tekstowe
		cut!(
			config.skip_doted_noext_files_text,
			self.dotfiles_text_no_empty_no_ext,
			"skip_doted_noext_files_text"
		);
		cut!(
			config.skip_doted_withext_files_text,
			self.dotfiles_text_no_empty_has_ext,
			"skip_doted_withext_files_text"
		);
		cut!(
			config.skip_nodoted_noext_files_text,
			self.files_text_no_empty_no_ext,
			"skip_nodoted_noext_files_text"
		);
		cut!(
			config.skip_nodoted_withext_files_text,
			self.files_text_no_empty_has_ext,
			"skip_nodoted_withext_files_text"
		);

		self.update_counts();

		if debug {
			println!("1c📕 [ structs: ClaimRefining applied to HandleWalkScan ] 🔰");
			if active == 0 {
				println!("  (Brak aktywnych filtrów ClaimRefining - nic nie odcięto)");
			} else {
				print!("{}", log_out);
			}
			println!("1c📕 [ structs: ClaimRefining applied to HandleWalkScan ] 🏁\n");
		}
	}

	/// Wykonuje surowe skanowanie dla wszystkich celów w `HandleVerifiedDirs`.
	pub fn scan(entry: &HandleVerifiedDirs, refining: &ClaimRefining, debug: bool) -> Result<Self> {
		let mut instance = Self::default();

		for handle in &entry.handles {
			instance.scan_target(handle, &entry.cwd_buf)?;
		}

		instance.sort_and_dedup_all();
		
		// Bezpośrednie wyłączenie szufladek wewnątrz skanera
		instance.refine(refining, debug);

		instance.update_counts();

		if debug {
			println!("{}", instance.display_debug());
		}

		Ok(instance)
	}

	fn scan_target(&mut self, handle: &PathAnchored, cwd: &Path) -> Result<()> {
		let root = &handle.buf;

		for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
			if entry.depth() == 0 {
				continue;
			}
			if entry.path_is_symlink() {
				continue;
			}

			let Ok(rel_prefix) = entry.path().strip_prefix(cwd) else {
				continue;
			};

			let norm_rel = normalize_path(rel_prefix);
			let clean_rel = norm_rel.trim_start_matches("./");

			let file_name = entry.file_name().to_string_lossy();
			let is_dot = file_name.starts_with('.');

			if entry.file_type().is_dir() {
				let is_empty = entry.path().read_dir().map(|mut r| r.next().is_none()).unwrap_or(false);
				let dir_str = format!("./{}/", clean_rel.trim_end_matches('/'));
				let node = PathNode::new(PathBuf::from(dir_str));

				match (is_empty, is_dot) {
					(true, false) => self.dirs_empty.push(node),
					(true, true) => self.dotdirs_empty.push(node),
					(false, false) => self.dirs_no_empty.push(node),
					(false, true) => self.dotdirs_no_empty.push(node),
				}
			} else {
				let has_ext = entry.path().extension().is_some();
				let meta = entry.metadata().ok();
				let is_empty = meta.as_ref().map(|m| m.len() == 0).unwrap_or(false);

				let is_bin = if !is_empty {
					if let Some(path_str) = entry.path().to_str() { check_is_binary(path_str) } else { false }
				} else {
					false
				};

				let file_str = format!("./{}", clean_rel);
				let node = PathNode::new(PathBuf::from(file_str));

				match (is_empty, is_bin, is_dot, has_ext) {
					(true, _, false, false) => self.files_empty_no_ext.push(node),
					(true, _, false, true) => self.files_empty_has_ext.push(node),
					(true, _, true, false) => self.dotfiles_empty_no_ext.push(node),
					(true, _, true, true) => self.dotfiles_empty_has_ext.push(node),

					(false, true, false, false) => self.files_binary_no_ext.push(node),
					(false, true, false, true) => self.files_binary_has_ext.push(node),
					(false, true, true, false) => self.dotfiles_binary_no_ext.push(node),
					(false, true, true, true) => self.dotfiles_binary_has_ext.push(node),

					(false, false, false, false) => self.files_text_no_empty_no_ext.push(node),
					(false, false, false, true) => self.files_text_no_empty_has_ext.push(node),
					(false, false, true, false) => self.dotfiles_text_no_empty_no_ext.push(node),
					(false, false, true, true) => self.dotfiles_text_no_empty_has_ext.push(node),
				}
			}
		}

		Ok(())
	}

	fn sort_and_dedup_all(&mut self) {
		let process_vec = |v: &mut Vec<PathNode>| {
			v.sort_unstable_by(|a, b| a.str.cmp(&b.str));
			v.dedup_by(|a, b| a.str == b.str);
		};

		process_vec(&mut self.dirs_empty);
		process_vec(&mut self.dotdirs_empty);
		process_vec(&mut self.dirs_no_empty);
		process_vec(&mut self.dotdirs_no_empty);

		process_vec(&mut self.files_empty_no_ext);
		process_vec(&mut self.dotfiles_empty_no_ext);
		process_vec(&mut self.files_empty_has_ext);
		process_vec(&mut self.dotfiles_empty_has_ext);

		process_vec(&mut self.files_binary_no_ext);
		process_vec(&mut self.dotfiles_binary_no_ext);
		process_vec(&mut self.files_binary_has_ext);
		process_vec(&mut self.dotfiles_binary_has_ext);

		process_vec(&mut self.files_text_no_empty_no_ext);
		process_vec(&mut self.dotfiles_text_no_empty_no_ext);
		process_vec(&mut self.files_text_no_empty_has_ext);
		process_vec(&mut self.dotfiles_text_no_empty_has_ext);
	}

	fn update_counts(&mut self) {
		self.count_dirs_empty = self.dirs_empty.len();
		self.count_dotdirs_empty = self.dotdirs_empty.len();
		self.count_dirs_no_empty = self.dirs_no_empty.len();
		self.count_dotdirs_no_empty = self.dotdirs_no_empty.len();

		self.count_files_empty_no_ext = self.files_empty_no_ext.len();
		self.count_dotfiles_empty_no_ext = self.dotfiles_empty_no_ext.len();
		self.count_files_empty_has_ext = self.files_empty_has_ext.len();
		self.count_dotfiles_empty_has_ext = self.dotfiles_empty_has_ext.len();

		self.count_files_binary_no_ext = self.files_binary_no_ext.len();
		self.count_dotfiles_binary_no_ext = self.dotfiles_binary_no_ext.len();
		self.count_files_binary_has_ext = self.files_binary_has_ext.len();
		self.count_dotfiles_binary_has_ext = self.dotfiles_binary_has_ext.len();

		self.count_files_text_no_empty_no_ext = self.files_text_no_empty_no_ext.len();
		self.count_dotfiles_text_no_empty_no_ext = self.dotfiles_text_no_empty_no_ext.len();
		self.count_files_text_no_empty_has_ext = self.files_text_no_empty_has_ext.len();
		self.count_dotfiles_text_no_empty_has_ext = self.dotfiles_text_no_empty_has_ext.len();
	}
}

impl VerboseDisplay for HandleWalkScan {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "1c📕 [ structs: HandleWalkScan ] 🔰\n")?;

		let total_dirs =
			self.count_dirs_empty + self.count_dotdirs_empty + self.count_dirs_no_empty + self.count_dotdirs_no_empty;

		let total_files = self.count_files_empty_no_ext
			+ self.count_dotfiles_empty_no_ext
			+ self.count_files_empty_has_ext
			+ self.count_dotfiles_empty_has_ext
			+ self.count_files_binary_no_ext
			+ self.count_dotfiles_binary_no_ext
			+ self.count_files_binary_has_ext
			+ self.count_dotfiles_binary_has_ext
			+ self.count_files_text_no_empty_no_ext
			+ self.count_dotfiles_text_no_empty_no_ext
			+ self.count_files_text_no_empty_has_ext
			+ self.count_dotfiles_text_no_empty_has_ext;

		// --- PODSUMOWANIE KATALOGÓW ---
		writeln!(f, "📁 TOTAL {} DIRECTORIES SCANNED", total_dirs)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, NOT HIDDEN", self.count_dirs_empty)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, HIDDEN (DOT)", self.count_dotdirs_empty)?;
		writeln!(f, "  ▪️ INCLUDING {} NOT EMPTY, NOT HIDDEN", self.count_dirs_no_empty)?;
		writeln!(f, "  ▪️ INCLUDING {} NOT EMPTY, HIDDEN (DOT)", self.count_dotdirs_no_empty)?;
		writeln!(f)?;

		// --- PODSUMOWANIE PLIKÓW ---
		writeln!(f, "📄 TOTAL {} FILES SCANNED", total_files)?;
		writeln!(f)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, NO EXTENSION", self.count_files_empty_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, HIDDEN (DOT), NO EXTENSION", self.count_dotfiles_empty_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, WITH EXTENSION", self.count_files_empty_has_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} EMPTY, HIDDEN (DOT), WITH EXTENSION", self.count_dotfiles_empty_has_ext)?;
		writeln!(f)?;
		writeln!(f, "  ▪️ INCLUDING {} BINARY, NO EXTENSION", self.count_files_binary_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} BINARY, HIDDEN (DOT), NO EXTENSION", self.count_dotfiles_binary_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} BINARY, WITH EXTENSION", self.count_files_binary_has_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} BINARY, HIDDEN (DOT), WITH EXTENSION", self.count_dotfiles_binary_has_ext)?;
		writeln!(f)?;
		writeln!(f, "  ▪️ INCLUDING {} TEXT, NO EXTENSION", self.count_files_text_no_empty_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} TEXT, HIDDEN (DOT), NO EXTENSION", self.count_dotfiles_text_no_empty_no_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} TEXT, WITH EXTENSION", self.count_files_text_no_empty_has_ext)?;
		writeln!(f, "  ▪️ INCLUDING {} TEXT, HIDDEN (DOT), WITH EXTENSION", self.count_dotfiles_text_no_empty_has_ext)?;
		writeln!(f)?;

		// --- SZCZEGÓŁOWE WYKAZY ---
		macro_rules! print_list {
			($title:expr, $list:expr) => {
				if !$list.is_empty() {
					writeln!(f, "🗃️  LIST OF {}", $title)?;
					for item in &$list {
						writeln!(f, "  {}", item.str)?;
					}
					writeln!(f)?;
				}
			};
		}

		print_list!("📁 EMPTY DIRECTORIES, NOT HIDDEN", self.dirs_empty);
		print_list!("📁 EMPTY DIRECTORIES, HIDDEN (DOT)", self.dotdirs_empty);
		print_list!("📁 NOT EMPTY DIRECTORIES, NOT HIDDEN", self.dirs_no_empty);
		print_list!("📁 NOT EMPTY DIRECTORIES, HIDDEN (DOT)", self.dotdirs_no_empty);

		print_list!("📄 EMPTY FILES, NO EXTENSION", self.files_empty_no_ext);
		print_list!("📄 EMPTY FILES, HIDDEN (DOT), NO EXTENSION", self.dotfiles_empty_no_ext);
		print_list!("📄 EMPTY FILES, WITH EXTENSION", self.files_empty_has_ext);
		print_list!("📄 EMPTY FILES, HIDDEN (DOT), WITH EXTENSION", self.dotfiles_empty_has_ext);

		print_list!("📄 BINARY FILES, NO EXTENSION", self.files_binary_no_ext);
		print_list!("📄 BINARY FILES, HIDDEN (DOT), NO EXTENSION", self.dotfiles_binary_no_ext);
		print_list!("📄 BINARY FILES, WITH EXTENSION", self.files_binary_has_ext);
		print_list!("📄 BINARY FILES, HIDDEN (DOT), WITH EXTENSION", self.dotfiles_binary_has_ext);

		print_list!("📄 TEXT FILES, NO EXTENSION", self.files_text_no_empty_no_ext);
		print_list!("📄 TEXT FILES, HIDDEN (DOT), NO EXTENSION", self.dotfiles_text_no_empty_no_ext);
		print_list!("📄 TEXT FILES, WITH EXTENSION", self.files_text_no_empty_has_ext);
		print_list!("📄 TEXT FILES, HIDDEN (DOT), WITH EXTENSION", self.dotfiles_text_no_empty_has_ext);

		writeln!(f, "1c📕 [ structs: HandleWalkScan ] 🏁")?;
		Ok(())
	}
}

```

### [29] `./src/libs/querypath/core/features/step05_build_tree.rs`

```rust
//! Path: `src/libs/querypath/features/step05_build_tree.rs`
//! Last significant modified: 2026-W43-3 00:54:59 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792


use anyhow::Result;



```

### [30] `./src/libs/querypath/core/features/verbose_display.rs`

```rust
//! Path: `src/libs/querypath/features/verbose_display.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::fmt;

pub trait VerboseDisplay {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

	/// Adapter pozwalający na wyświetlenie w `println!("{}",
	/// x.display_debug())`
	fn display_debug(&self) -> DebugViewer<'_, Self> {
		DebugViewer(self)
	}
}

pub struct DebugViewer<'a, T: ?Sized>(pub &'a T);

impl<'a, T: VerboseDisplay + ?Sized> fmt::Display for DebugViewer<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		VerboseDisplay::fmt(self.0, f)
	}
}

```

### [31] `./src/libs/querypath/core/structs/anchored_path.rs`

```rust
//! Path: `src/libs/querypath/structs/anchored_path.rs`
//! Last significant modified: 2026-W43-3 13:57:35 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use anyhow::{Context, Result};
use std::{
	fs,
	path::{Path, PathBuf},
};

use crate::querypath::core::utils::{diff_paths, normalize_path};
// ============================================================================
// RELACJE I ZAKOTWICZENIE (ANCHORING)
// ============================================================================

/// Odwzorowanie relacji pomiędzy miejscem odpalenia programu (CWD),
/// a fizycznym celem skanowania, wyliczające zawsze bezpieczną ścieżkę
/// relatywną.
#[derive(Debug, Clone)]
pub struct PathAnchored {
	pub buf: PathBuf, // kanoniczna ścieżka absolutna
	pub str: String,  // normalize_path(&buf)
	pub rel: String,  // Ścieżka pomostowa względem CWD
}

impl PathAnchored {
	/// Tworzy nowy punkt zakotwiczenia. Zwraca błąd, jeśli ścieżka docelowa
	/// nie istnieje na dysku (wczesna walidacja z kontekstem).
	pub fn resolve<P: AsRef<Path>>(input: P, cwd: &Path) -> Result<Self> {
		let input_ref = input.as_ref();

		// Kanonizacja rozwija m.in. symlinki, '..' i weryfikuje istnienie na
		// dysku.
		let buf = fs::canonicalize(input_ref)
			.with_context(|| format!("Ścieżka docelowa nie istnieje lub jest niedostępna: {:?}", input_ref))?;
		let str = normalize_path(&buf);
		let is_dir = buf.is_dir();

		// Wyliczamy relację (jak dostać się z CWD do Targetu).
		/*
		 * let rel = match buf.strip_prefix(cwd) {
		 * 	Ok(rel) => {
		 * 		let clean_rel = normalize_path(rel).trim_matches('/').to_string();
		 * 		if clean_rel.is_empty() {
		 * 			"./".to_string()
		 * 		} else if is_dir {
		 * 			format!("./{clean_rel}/")
		 * 		} else {
		 * 			format!("./{clean_rel}")
		 * 		}
		 * 	}
		 * 	Err(_) => {
		 * 		let mut s = normalize_path(&buf);
		 * 		if is_dir && !s.ends_with('/') {
		 * 			s.push('/');
		 * 		}
		 * 		s
		 * 	}
		 * };
		 * Ok(Self { buf, str, rel })
		 *
		 * */
		let rel = match diff_paths(&buf, cwd) {
			Some(rel_path) => {
				let clean_rel = normalize_path(&rel_path).trim_matches('/').to_string();
				let prefix = if clean_rel == ".." || clean_rel.starts_with("../") { "" } else { "./" };

				if clean_rel.is_empty() {
					"./".to_string()
				} else if is_dir {
					format!("{prefix}{clean_rel}/")
				} else {
					format!("{prefix}{clean_rel}")
				}
			}
			None => {
				let mut s = str.clone();
				if is_dir && !s.ends_with('/') {
					s.push('/');
				}
				s
			}
		};

		Ok(Self { buf, str, rel })
	}
}

```

### [32] `./src/libs/querypath/core/structs/fs_walk_stat.rs`

```rust
//! Path: `src/libs/querypath/structs/fs_walk_stat.rs`
//! Last significant modified: 2026-W43-3 14:54:13 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Statystyki surowego skanowania dysku (przed filtrowaniem wzorcami i
/// opcjami).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FsWalkStat {
	pub count_dirs_all: usize,
	pub count_dirs_empty: usize,
	pub count_files_all: usize,
	pub count_files_empty: usize,
	pub count_files_binary: usize,
}

```

### [33] `./src/libs/querypath/core/structs/item_file_dir.rs`

```rust
//! Path: `src/libs/querypath/structs/item_file_dir.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Metadane dopasowanego pliku.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FileItem {
	pub path: String,
	pub size: u64,
	pub is_binary: bool,
	pub modified_at: Option<u64>,
}

/// Metadane dopasowanego katalogu z podwójnym rozliczeniem rozmiaru.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DirItem {
	pub path: String,
	/// Rzeczywisty, pełny rozmiar fizyczny wszystkich plików wewnątrz katalogu.
	pub real_size: u64,
	/// Sumaryczny rozmiar tylko tych plików, które spełniły kryteria wzorca.
	pub matched_size: u64,
	pub modified_at: Option<u64>,
}

```

### [34] `./src/libs/querypath/core/structs/path_context.rs`

```rust
//! Path: `src/libs/querypath/structs/path_context.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Analiza relacji wewnątrz ścieżki tekstowej bez ani jednej alokacji na
/// stercie. Działa w 100% na pożyczonych wycinkach referencji (`&str`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathContext<'a> {
	pub parent: &'a str,
	pub file: &'a str,
}

impl<'a> PathContext<'a> {
	/// Dzieli ścieżkę na katalog nadrzędny i plik/katalog docelowy.
	#[must_use]
	pub fn from_path(path: &'a str) -> Self {
		let clean_path = path.trim_start_matches("./");

		let (parent, file) = match clean_path.rsplit_once('/') {
			Some((p, f)) => (p, f),
			std::option::Option::None => ("", clean_path),
		};

		Self { parent, file }
	}

	pub fn name(&self) -> &'a str {
		self.file
	}

	pub fn parent(&self) -> &'a str {
		self.parent
	}

	pub fn is_root_level(&self) -> bool {
		self.parent.is_empty()
	}
}

```

### [35] `./src/libs/querypath/core/structs/path_node.rs`

```rust
//! Path: `src/libs/querypath/structs/path_node.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use crate::querypath::core::utils::normalize_path;
use std::path::PathBuf;

/// Reprezentacja pojedynczego węzła ścieżki przechowująca zarówno
/// oryginalny obiekt systemowy (`PathBuf`), jak i szybką, znormalizowaną
/// postać tekstową (`String`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathNode {
	pub buf: PathBuf,
	pub str: String,
}

impl PathNode {
	pub fn new(buf: PathBuf) -> Self {
		let str = normalize_path(&buf);
		Self { buf, str }
	}
}

```

### [36] `./src/libs/querypath/core/structs/pattern_compiled.rs`

```rust
//! Path: `src/libs/querypath/structs/pattern_compiled.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use crate::querypath::core::traits::PattEnvIndex;
use anyhow::Result;
use regex::Regex;

// ============================================================================
// TYPY DOMENOWE
// ============================================================================

/// Silnie typowana lista surowych wzorców wejściowych.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PattRaw(pub Vec<String>);

/// Silnie typowana lista wzorców rozwiniętych po klamrach `{a|b}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PattExp(pub Vec<String>);

// ============================================================================
// SKOMPILOWANA REGUŁA (PATTERN COMPILED)
// ============================================================================

#[derive(Debug, Clone)]
pub struct PatternCompiled {
	pub regex: Regex,
	pub targets_dir_only: bool,
	pub requires_sibling: bool,
	pub requires_orphan: bool,
	pub base_name: String,
	pub is_negated: bool,
}

// ============================================================================
// KOMPILACJA I DOPASOWANIE REGUŁY
// ============================================================================

impl PatternCompiled {
	pub fn compile(pattern: &str, ignore_case: bool) -> Result<Self, regex::Error> {
		let is_negated: bool = pattern.starts_with('!');
		let mut p: &str = if is_negated { pattern.get(1..).unwrap_or("") } else { pattern };

		let mut requires_sibling: bool = false;
		let mut requires_orphan: bool = false;

		if p.starts_with('@') {
			requires_sibling = true;
			p = p.get(1..).unwrap_or("");
		} else if p.starts_with('$') {
			requires_orphan = true;
			p = p.get(1..).unwrap_or("");
		}

		let is_direct_regex: bool = p.starts_with("re:");
		if is_direct_regex {
			let raw_re: &str = p.get(3..).unwrap_or("");
			let re_str: String = if ignore_case { format!("(?i){raw_re}") } else { raw_re.to_string() };
			let regex = Regex::new(&re_str)?;
			return Ok(Self {
				regex,
				targets_dir_only: pattern.ends_with('/'),
				requires_sibling,
				requires_orphan,
				base_name: String::new(),
				is_negated,
			});
		}

		// Tylko jawnny slash '/' na końcu oznacza wzorzec wyłącznie dla
		// katalogów
		let targets_dir_only: bool = p.ends_with('/');

		let base_name: String =
			p.trim_end_matches('/').split('/').next_back().unwrap_or("").split('.').next().unwrap_or("").to_string();

		let mut re = String::new();
		if ignore_case {
			re.push_str("(?i)");
		}

		let mut anchored: bool = false;

		if p.starts_with("./") {
			anchored = true;
			p = p.get(2..).unwrap_or("");
		} else if p.starts_with("**/") {
			// Wzorzec **/ oznacza wyszukiwanie od dowolnego poziomu
			anchored = false;
			p = p.get(3..).unwrap_or("");
		}

		if anchored {
			re.push('^');
		} else {
			re.push_str("(?:^|/)");
		}

		let chars: Vec<char> = p.chars().collect();
		let mut i: usize = 0;

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
						// ** dopasowuje dowolny ciąg znaków (włącznie z /)
						re.push_str(".*");
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
				c => re.push_str(&regex::escape(&c.to_string())),
			}
			i += 1;
		}

		re.push('$');

		Ok(
			Self {
				regex: Regex::new(&re)?,
				targets_dir_only,
				requires_sibling,
				requires_orphan,
				base_name,
				is_negated,
			},
		)
	}

	#[allow(clippy::bool_comparison)]
	pub fn is_match<E: PattEnvIndex>(&self, path: &str, env: &E) -> bool {
		let is_dir: bool = path.ends_with('/');
		let clean: &str = path.strip_prefix("./").unwrap_or(path);

		if self.targets_dir_only && is_dir == false {
			return false;
		}

		if self.regex.is_match(clean) == false {
			return false;
		}

		// Relacje rodzeństwa/sieroty dla plików
		if (self.requires_sibling || self.requires_orphan) && is_dir == false {
			let parent: &str = match clean.rsplit_once('/') {
				Some((p, _)) => p,
				std::option::Option::None => "",
			};

			let mut expected_folder = String::new();
			if !parent.is_empty() {
				expected_folder.push_str(parent);
				expected_folder.push('/');
			}
			expected_folder.push_str(&self.base_name);
			expected_folder.push('/');

			let exists: bool = env.has_dir(&expected_folder);

			if self.requires_sibling && exists == false {
				return false;
			}
			if self.requires_orphan && exists {
				return false;
			}
		}

		// Relacje rodzeństwa/sieroty dla katalogów
		if (self.requires_sibling || self.requires_orphan) && is_dir {
			let dir_no_slash: &str = path.trim_end_matches('/');
			let mut search_prefix = String::from(dir_no_slash);
			search_prefix.push('.');

			let has_file_sibling: bool = env.has_file_with_prefix(&search_prefix);

			if self.requires_sibling && has_file_sibling == false {
				return false;
			}
			if self.requires_orphan && has_file_sibling {
				return false;
			}
		}

		true
	}
}

```

### [37] `./src/libs/querypath/core/structs/patterns.rs`

```rust
//! Path: `src/libs/querypath/structs/patterns.rs`
//! Last significant modified: 2026-W43-3 21:30:59 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use crate::querypath::core::{
	structs::{PattExp, PattRaw, PatternCompiled},
	traits::PattEnvIndex,
};
use anyhow::{Context, Result};

/// Główna struktura zarządzająca dopasowywaniem wzorców do ścieżek.
#[derive(Debug, Clone)]
pub struct PathsPatterns {
	pub raw: PattRaw,
	pub expanded: PattExp,
	pub compiled: Vec<PatternCompiled>,
	pub case_sensitive: bool,
}

// ============================================================================
// IMPLEMENTACJA PATHS PATTERNS
// ============================================================================

impl PathsPatterns {
	/// Tworzy i kompiluje zbiór wzorców.
	/// Rozwija klamry `{a|b}` oraz przelicza reguły na wyrażenia regularne.
	pub fn build<I, S>(patterns: I, case_sensitive: bool) -> Result<Self>
	where
		I: IntoIterator<Item = S>,
		S: AsRef<str>,
	{
		let mut raw_vec = Vec::new();
		let mut expanded_vec = Vec::new();

		for p in patterns {
			let s: &str = p.as_ref();
			if s.trim().is_empty() {
				continue;
			}
			raw_vec.push(s.to_string());
			expanded_vec.extend(Self::expand_braces(s));
		}

		let mut compiled = Vec::with_capacity(expanded_vec.len());
		let ignore_case = !case_sensitive;
		for p in &expanded_vec {
			let rule =
				PatternCompiled::compile(p, ignore_case).with_context(|| format!("Błąd kompilacji wzorca: '{p}'"))?;
			compiled.push(rule);
		}

		Ok(Self { raw: PattRaw(raw_vec), expanded: PattExp(expanded_vec), compiled, case_sensitive })
	}

	/// Sprawdza, czy podana ścieżka spełnia zasady narzucone przez zbiór
	/// wzorców. Wzorce zanegowane (`!`) działają jako bezwarunkowe weto.
	#[must_use]
	pub fn is_match<E: PattEnvIndex>(&self, path: &str, env: &E) -> bool {
		if self.compiled.is_empty() {
			return true;
		}

		let mut has_positive: bool = false;
		let mut matched_positive: bool = false;

		for c in &self.compiled {
			if c.is_negated {
				if c.is_match(path, env) {
					return false;
				}
			} else {
				has_positive = true;
				if !matched_positive && c.is_match(path, env) {
					matched_positive = true;
				}
			}
		}

		if has_positive { matched_positive } else { true }
	}

	#[must_use]
	pub fn rules_count(&self) -> usize {
		self.compiled.len()
	}

	/// Rekurencyjnie rozwija alternatywy zaimplementowane w klamrach `{a|b}`.
	pub fn expand_braces(input: &str) -> Vec<String> {
		let Some(end) = input.find('}') else {
			return vec![input.to_string()];
		};

		let Some(sub) = input.get(..end) else {
			return vec![input.to_string()];
		};

		let Some(start) = sub.rfind('{') else {
			return vec![input.to_string()];
		};

		let (Some(prefix), Some(suffix), Some(options)) =
			(input.get(..start), input.get(end + 1..), input.get(start + 1..end))
		else {
			return vec![input.to_string()];
		};

		let mut result = Vec::new();
		for opt in options.split('|') {
			let merged = format!("{prefix}{opt}{suffix}");
			result.extend(Self::expand_braces(&merged));
		}
		result
	}
}

```

### [38] `./src/libs/querypath/core/structs/tree.rs`

```rust
//! Path: `src/libs/querypath/structs/tree.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::{super::types::consts::TREE_SYMBOLS, TreeItem};

impl Default for Tree {
	fn default() -> Self {
		Self::new()
	}
}

pub struct Tree {
	pub name_width: usize,
}

impl Tree {
	pub fn new() -> Self {
		Self { name_width: 25 } // <-- Domyślnie 25
	}

	pub fn name_width(mut self, width: usize) -> Self {
		self.name_width = width.max(15); // <-- Minimum 15
		self
	}

	pub fn calculate_max_prefix_len_for_item(&self, item: &TreeItem) -> usize {
		let mut max_len = 0;
		self.walk_prefix_len(item, "", &mut max_len);
		max_len
	}

	fn walk_prefix_len(&self, item: &TreeItem, prefix: &str, max_len: &mut usize) {
		match item {
			TreeItem::Spacer => {
				*max_len = (*max_len).max(prefix.chars().count());
			}
			TreeItem::Root { children, .. } => {
				*max_len = (*max_len).max(3); // "▣─┬"
				for child in children {
					self.walk_prefix_len(child, "  ", max_len);
				}
			}
			TreeItem::Node { children, .. } => {
				let symbol_len = 4; // np. "├──┬"
				let line_prefix_len = prefix.chars().count() + symbol_len;
				*max_len = (*max_len).max(line_prefix_len);

				let child_prefix = format!("{}{}", prefix, TREE_SYMBOLS[7]);
				for child in children {
					self.walk_prefix_len(child, &child_prefix, max_len);
				}
			}
		}
	}
}

```

### [39] `./src/libs/querypath/core/structs/tree_bundle_item.rs`

```rust
//! Path: `src/libs/querypath/structs/tree_bundle_item.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Pojedynczy plik/folder z nadanym numerem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleItem {
	pub number: usize,
	pub path: String,
	pub is_dir: bool,
	pub is_binary: bool,
	pub size: u64,
	pub modified_at: Option<u64>,
}

```

### [40] `./src/libs/querypath/core/structs/tree_bundle_result.rs`

```rust
//! Path: `src/libs/querypath/structs/tree_bundle_result.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::{BundleItem, TreeItem};
use std::collections::HashMap;

/// Gotowy wynik: posortowane drzewo z przypisanymi numerami oraz lista
/// elementów.
#[derive(Debug, Clone)]
pub struct BundleResult {
	pub tree: TreeItem,
	pub items: Vec<BundleItem>,
	pub indices: HashMap<String, usize>,
	pub max_num: usize,
}

impl BundleResult {
	pub fn get(&self, path: &str) -> Option<usize> {
		self.indices.get(path).copied()
	}

	/// Iterator po plikach tekstowych w kolejności ułożenia w drzewie.
	pub fn text_files(&self) -> impl Iterator<Item = &BundleItem> {
		self.items.iter().filter(|i| !i.is_dir && !i.is_binary)
	}

	pub fn files(&self) -> impl Iterator<Item = &BundleItem> {
		self.items.iter().filter(|i| !i.is_dir)
	}

	pub fn dirs(&self) -> impl Iterator<Item = &BundleItem> {
		self.items.iter().filter(|i| i.is_dir)
	}

	pub fn len(&self) -> usize {
		self.items.len()
	}

	pub fn is_empty(&self) -> bool {
		self.items.is_empty()
	}
}

```

### [41] `./src/libs/querypath/core/structs/tree_item.rs`

```rust
//! Path: `src/libs/querypath/structs/tree_item.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

#[derive(Debug, Clone)]
pub enum TreeItem {
	Root {
		label: String,
		path: String,
		matched_size: u64,
		real_size: u64,
		modified_at: Option<u64>,
		children: Vec<TreeItem>,
	},
	Node {
		label: String,
		is_dir: bool,
		is_binary: bool,
		path: String,
		number: Option<usize>,
		size: u64,
		matched_size: u64,
		real_size: u64,
		modified_at: Option<u64>,
		children: Vec<TreeItem>,
	},
	Spacer,
}

// Akcesory używane przez silnik sortujący
impl TreeItem {
	pub fn is_dir(&self) -> bool {
		match self {
			TreeItem::Root { .. } => true,
			TreeItem::Node { is_dir, .. } => *is_dir,
			TreeItem::Spacer => false,
		}
	}

	pub fn label(&self) -> &str {
		match self {
			TreeItem::Root { label, .. } => label,
			TreeItem::Node { label, .. } => label,
			TreeItem::Spacer => "",
		}
	}
}

```

### [42] `./src/libs/querypath/core/structs/walk_env_index.rs`

```rust
//! Path: `src/libs/querypath/structs/walk_env_index.rs`
//! Last significant modified: 2026-W43-3 19:11:46 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use crate::querypath::core::{features::HandleWalkScan, traits::PattEnvIndex};
use std::collections::HashSet;

// ============================================================================
// INDEX I POMOCNICZE
// ============================================================================

pub struct WalkEnvIndex<'a> {
	dirs: HashSet<&'a str>,
	files: Vec<&'a str>,
}

impl<'a> WalkEnvIndex<'a> {
	pub fn from_walk(walk: &'a HandleWalkScan) -> Self {
		// Używamy nowych, płaskich iteratorów generujących się na żywo
		let dirs = walk.iter_all_dirs().map(|d| d.str.as_str()).collect();
		let mut files: Vec<&str> = walk.iter_all_files().map(|f| f.str.as_str()).collect();
		files.sort_unstable();
		Self { dirs, files }
	}
}

impl<'a> PattEnvIndex for WalkEnvIndex<'a> {
	fn has_dir(&self, dir: &str) -> bool {
		self.dirs.contains(dir)
	}

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

```

### [43] `./src/libs/querypath/core/traits/apply_claim.rs`

```rust
//! Path: `src/libs/querypath/traits/apply_claim.rs`
//! Last significant modified: 2026-W43-4 00:04 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Trait aplikujący konfiguracje do głównego zbiornika roszczeń.
pub trait ApplyClaim {
	fn apply(self, claims: &mut GatherClaims);
}

```

### [44] `./src/libs/querypath/core/traits/path_env_index.rs`

```rust
//! Path: `src/libs/querypath/traits/path_env_index.rs`
//! Last significant modified: 2026-W43-3 01:13 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Trait umożliwiający bez-dyskowe (zero-I/O) sprawdzanie obecności
/// plików i katalogów w pamięci podczas weryfikacji relacji `@` oraz `$`.
pub trait PattEnvIndex {
	fn has_dir(&self, dir: &str) -> bool;
	fn has_file_with_prefix(&self, prefix: &str) -> bool;
	fn any_file_in_dir(&self, dir: &str, check: &mut dyn FnMut(&str) -> bool) -> bool;
}

```

### [45] `./src/libs/querypath/core/types/consts.rs`

```rust
//! Path: `src/libs/querypath/types/consts.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

pub const DEFAULT_COL_NAME_SIZE: usize = 25;
pub const MINIMUM_COL_NAME_SIZE: usize = 15;
pub const DEFAULT_COL_PATH_SIZE: usize = 35;
pub const MINIMUM_COL_PATH_SIZE: usize = 25;

pub const TREE_SYMBOLS: [&str; 17] = [
	"└───",
	"└──┬",
	"└──•",
	"   ",
	"├───",
	"├──┬",
	"├──•",
	"│  ",
	"  ├───",
	"  └───",
	"    •",
	"  ├──•",
	"  ┌──•",
	"  └──•",
	"   ──•",
	"▣─┬",
	"▣───",
];

```

### [46] `./src/libs/querypath/core/types/enums.rs`

```rust
//! Path: `src/libs/querypath/types/enums.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumNodeGroup {
	Directory,
	TextFile,
	BinaryFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumGroupStrategy {
	Mixed,
	Custom(Vec<EnumNodeGroup>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumSameNamePriority {
	DirectoryFirst,
	FileFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumNoExtPriority {
	Above,
	Below,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumCharClass {
	Special,
	Digit,
	Letter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumCasePrecedence {
	Insensitive,
	UpperFirst,
	LowerFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumColumn {
	Weight,
	Temporal,
	Path,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumUnitSystem {
	Binary,  // IEC: B, KiB, MiB, GiB, TiB (dzielnik 1024)
	Decimal, // SI:  B, KB, MB, GB, TB   (dzielnik 1000)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumDirWeightDisplay {
	None,
	MatchedOnly,
	RealOnly,
	Both, // np. "12.5 MiB / 140.2 MiB"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumWeightPrecision {
	Tenths,     // Dokładność do części dziesiętnych (np. 12.5 MiB)
	Hundredths, // Dokładność do części setnych (np. 12.54 MiB)
}

```

### [47] `./src/libs/querypath/core/utils/check_is_binary.rs`

```rust
//! Path: `src/libs/querypath/utils/check_is_binary.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::{fs, io::Read};

/// Badanie pierwszego kilobajta pliku pod kątem bajtu zerowego (`0x00`).
pub fn check_is_binary(path_str: &str) -> bool {
	let Ok(mut file) = fs::File::open(path_str) else {
		return false;
	};
	let mut buffer = [0u8; 1024];
	let Ok(n) = file.read(&mut buffer) else {
		return false;
	};
	buffer.iter().take(n).any(|&byte| byte == 0)
}

```

### [48] `./src/libs/querypath/core/utils/diff_paths.rs`

```rust
//! Path: `src/libs/querypath/utils/diff_paths.rs`
//! Last significant modified: 2026-W43-3 13:06:18 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::path::{Component, Path, PathBuf};

/// Ścieżka `target` względem `base`, z `..` gdy trzeba.
/// `None`, gdy nie ma wspólnego korzenia (np. inny dysk na Windows).
pub fn diff_paths(target: &Path, base: &Path) -> Option<PathBuf> {
	let t: Vec<Component> = target.components().collect();
	let b: Vec<Component> = base.components().collect();

	let common = t.iter().zip(&b).take_while(|(x, y)| x == y).count();
	if common == 0 {
		return None;
	}

	let mut out = PathBuf::new();
	for _ in common..b.len() {
		out.push("..");
	}
	for c in &t[common..] {
		out.push(c.as_os_str());
	}
	Some(out)
}

```

### [49] `./src/libs/querypath/core/utils/get_modified_time.rs`

```rust
//! Path: `src/libs/querypath/utils/get_modified_time.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::{fs, time::UNIX_EPOCH};

/// Odczyt czasu modyfikacji z metadanych w formacie UNIX timestamp.
pub fn get_modified_time(path_str: &str) -> Option<u64> {
	fs::metadata(path_str).ok()?.modified().ok()?.duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs())
}

```

### [50] `./src/libs/querypath/core/utils/normalize_path.rs`

```rust
//! Path: `src/libs/querypath/utils/normalize_path.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::path::Path;
// ============================================================================
// NORMALIZACJA ŚCIEŻEK
// ============================================================================

/// Normalizuje ścieżkę do ujednoliconego formatu tekstowego (POSIX-like).
/// - Zmienia `\` na `/`
/// - Usuwa Windowsowe prefiksy UNC (`\\?\`)
/// - Redukuje zduplikowane sekwencje `/./` do `/`
#[must_use]
pub fn normalize_path<P: AsRef<Path>>(p: P) -> String {
	p.as_ref().to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/").replace("/./", "/")
}

```

### [51] `./src/libs/timer/consts.rs`

```rust
use super::enums::WeekFirstDay;
/// Globalne, tymczasowo zahardkodowane ustawienie: tydzień zaczyna się w poniedziałek.
pub const WEEK_FIRST_DAY_IS_MON_HARDCODED: WeekFirstDay = WeekFirstDay::Mon;
// Współczynniki JDE0 (Meeus, tabele 27.a i 27.b): [stała, Y, Y^2, Y^3, Y^4],
// kolejno: równonoc marcowa, przesilenie czerwcowe, równonoc wrześniowa, przesilenie grudniowe.
// Tabela 27.a: lata -1000..1000, Y = rok / 1000.
pub const SOLAR_EVENTS_Y0: [[f64; 5]; 4] = [
	[1_721_139.291_89, 365_242.137_40, 0.061_34, 0.001_11, -0.000_71],
	[1_721_233.254_01, 365_241.725_62, -0.053_23, 0.009_07, 0.000_25],
	[1_721_325.704_55, 365_242.495_58, -0.116_77, -0.002_97, 0.000_74],
	[1_721_414.399_87, 365_242.882_57, -0.007_69, -0.009_33, -0.000_06],
];
// Tabela 27.b: lata 1000..3000, Y = (rok - 2000) / 1000.
pub const SOLAR_EVENTS_Y2000: [[f64; 5]; 4] = [
	[2_451_623.809_84, 365_242.374_04, 0.051_69, -0.004_11, -0.000_57],
	[2_451_716.567_67, 365_241.626_03, 0.003_25, 0.008_88, -0.000_30],
	[2_451_810.217_15, 365_242.017_67, -0.115_75, 0.003_37, 0.000_78],
	[2_451_900.059_52, 365_242.740_49, -0.062_23, -0.008_23, 0.000_32],
];
// Wyrazy okresowe (Meeus, tabela 27.c): (A, B, C), składnik = A * cos(B + C * T) [stopnie].
pub const SOLAR_PERIODIC: [(f64, f64, f64); 24] = [
	(485.0, 324.96, 1_934.136),
	(203.0, 337.23, 32_964.467),
	(199.0, 342.08, 20.186),
	(182.0, 27.85, 445_267.112),
	(156.0, 73.14, 45_036.886),
	(136.0, 171.52, 22_518.443),
	(77.0, 222.54, 65_928.934),
	(74.0, 296.72, 3_034.906),
	(70.0, 243.58, 9_037.513),
	(58.0, 119.81, 33_718.147),
	(52.0, 297.17, 150.678),
	(50.0, 21.02, 2_281.226),
	(45.0, 247.54, 29_929.562),
	(44.0, 325.15, 31_555.956),
	(29.0, 60.93, 4_443.417),
	(18.0, 155.12, 67_555.328),
	(17.0, 288.79, 4_562.452),
	(16.0, 198.04, 62_894.029),
	(14.0, 199.76, 31_436.921),
	(12.0, 95.39, 14_577.848),
	(12.0, 287.11, 31_931.756),
	(12.0, 320.81, 34_777.259),
	(9.0, 227.73, 1_222.114),
	(8.0, 15.45, 16_859.074),
];

```

### [52] `./src/libs/timer/engine.rs`

```rust
use super::{
	consts::WEEK_FIRST_DAY_IS_MON_HARDCODED,
	enums::{MonthDay, Months, Season, Week, WeekDay, WeekFirstDay, YearDay},
	functions::{
		civil_from_days, days_from_civil, days_in_month, is_leap, iso_weekday, month_day, months_of, nz, quater_of,
		season, week_of,
	},
	structs::{
		Cycle, Day, Month, MonthWith28Day, MonthWith29Day, MonthWith30Day, MonthWith31Day, Range0To23, Range0To59,
		Range0To999, Range1To12, T, Time, Timer, Year, YearWith365Day, YearWith366Day,
	},
};
use std::{
	num::NonZeroU16,
	// ops::{Add, Sub},
	time::{/*Duration,*/ SystemTime, UNIX_EPOCH},
};

impl Timer {
	pub fn now() -> Self {
		let n = jiff::Zoned::now();
		let total_secs_utc = (n.timestamp().as_nanosecond() / 1_000_000_000) as i64;
		let subsec_nanos = (n.timestamp().as_nanosecond() % 1_000_000_000) as u32;

		// Pobranie offsetu i zapisanie jako oddzielna zmienna
		let offset_secs = n.offset().seconds() as i32;
		let base_secs = total_secs_utc + i64::from(offset_secs);
		// let base_secs = total_secs_utc + (n.offset().seconds() as i64);

		// Używamy dzielenia euklidesowego dla bezpieczeństwa dat ujemnych (przed 1970)
		let days = base_secs.div_euclid(86_400);
		let rem_secs = base_secs.rem_euclid(86_400) as u32;

		Self(days, rem_secs, subsec_nanos, offset_secs)
	}
	pub fn format_without_format(self /*week_start: WeekFirstDay*/) -> T {
		T::new(self /*week_start*/)
	}
	pub fn format(&self, /*week_start: WeekFirstDay,*/ pattern: &str) -> String {
		// Przekazujemy self (kopia Timera) i start tygodnia do budowy danych kalendarzowych
		let t = T::new(*self /*week_start*/);
		// Zgodnie ze strukturą Timer(pub i64, pub u32, pub u32, pub i32), offset jest pod indeksem 3
		t.format_string(pattern, self.3 /*week_start*/)
	}
	/// Tworzy strukturę `Timer` z systemowego czasu modyfikacji pliku (`std::time::SystemTime`).
	pub fn from_system_time(st: SystemTime) -> Self {
		let duration = st.duration_since(UNIX_EPOCH).unwrap_or_default();
		Self::from_secs_and_nanos(duration.as_secs(), duration.subsec_nanos())
	}

	/// Tworzy strukturę `Timer` ze znacznika czasu w sekundach UNIX (np. pobranego z metadanych).
	pub fn from_secs(timestamp_secs: u64) -> Self {
		Self::from_secs_and_nanos(timestamp_secs, 0)
	}

	/// Tworzy strukturę `Timer` ze znacznika czasu w milisekundach UNIX.
	pub fn from_millis(timestamp_millis: u64) -> Self {
		let secs = timestamp_millis / 1_000;
		let nanos = (timestamp_millis % 1_000) as u32 * 1_000_000;
		Self::from_secs_and_nanos(secs, nanos)
	}

	/// Prywatna funkcja pomocnicza przeliczająca całkowity czas UTC i aplikująca lokalny offset.
	fn from_secs_and_nanos(total_secs_utc: u64, subsec_nanos: u32) -> Self {
		let offset_secs = jiff::Zoned::now().offset().seconds() as i32;
		let base_secs = (total_secs_utc as i64) + i64::from(offset_secs);

		let days = base_secs.div_euclid(86_400);
		let rem_secs = base_secs.rem_euclid(86_400) as u32;

		Self(days, rem_secs, subsec_nanos, offset_secs)
	}
	/// Tworzy `Timer` ze znacznika czasu w mikrosekundach UNIX.
	pub fn from_micros(timestamp_micros: u64) -> Self {
		let secs = timestamp_micros / 1_000_000;
		let nanos = (timestamp_micros % 1_000_000) as u32 * 1_000;
		Self::from_secs_and_nanos(secs, nanos)
	}

	/// Tworzy `Timer` ze znacznika czasu w nanosekundach UNIX.
	pub fn from_nanos(timestamp_nanos: u128) -> Self {
		let secs = (timestamp_nanos / 1_000_000_000) as u64;
		let nanos = (timestamp_nanos % 1_000_000_000) as u32;
		Self::from_secs_and_nanos(secs, nanos)
	}
	pub fn to_unix_secs(&self) -> i64 {
		self.0 * 86_400 + i64::from(self.1) - i64::from(self.3)
	}

	pub fn to_unix_nanos(&self) -> i128 {
		(self.to_unix_secs() as i128) * 1_000_000_000 + i128::from(self.2)
	}
}

// Implementacja Traitu TryFrom dla SystemTime -> Timer
impl TryFrom<SystemTime> for Timer {
	type Error = std::time::SystemTimeError;

	fn try_from(st: SystemTime) -> Result<Self, Self::Error> {
		let duration = st.duration_since(UNIX_EPOCH)?;
		Ok(Self::from_secs_and_nanos(duration.as_secs(), duration.subsec_nanos()))
	}
}

// ------------------------------------------------------------ KONSTRUKCJA T  Timer
impl T {
	/// Buduje komplet danych kalendarzowych i czasu z `Timer`
	/// (dni od 1970-01-01 w czasie lokalnym, sekundy doby, nanosekundy).
	pub fn new(timer: Timer /* week_start: WeekFirstDay*/) -> Self {
		let Timer(days, rem_secs, subsec_nanos, offset_secs) = timer;
		// Chwila w UTC (sekundy uniksowe), potrzebna do porównania z równonocami i przesileniami.

		// Zmienna pomocnicza do funkcji week_of()
		/* let sun0 = week_start == WeekFirstDay::Sun;*/
		let sun0 = WEEK_FIRST_DAY_IS_MON_HARDCODED == WeekFirstDay::Sun;

		let now_unix =
			(days * 86_400 + i64::from(rem_secs) - i64::from(offset_secs)) as f64 + f64::from(subsec_nanos) / 1e9;

		// ---- data
		let (y, m, d) = civil_from_days(days);
		let month_len = days_in_month(y, m);
		let leap = is_leap(y);
		let jan1 = days_from_civil(y, 1, 1);
		let dec31 = days_from_civil(y, 12, 31);
		let day_of_year = (days - jan1 + 1) as u16;
		let iso_wd = iso_weekday(days);

		let day = Day {
			in_year: if leap {
				YearDay::Y366D(YearWith366Day(nz(day_of_year)))
			} else {
				YearDay::Y365D(YearWith365Day(nz(day_of_year)))
			},
			in_month: month_day(month_len, d as u16),
			in_week: WeekDay::from_iso(iso_wd),
		};

		// Tutaj używamy sun0, aby obliczyć tylko jeden zestaw danych:
		let year = Year {
			week_year: week_of(days, sun0).1 as i16,
			year: y as i16,
			season: season(y, now_unix),
			quater: quater_of(m),
			is_leap: leap,
			cycle: Cycle { century: ((y - 1).div_euclid(100) + 1) as i16, cycl400: y.rem_euclid(400) as u16 },
			infirst_day: WeekDay::from_iso(iso_weekday(jan1)),
			infinal_day: WeekDay::from_iso(iso_weekday(dec31)),
			infirst_weeknum: Week::from_number(week_of(jan1, sun0).0),
			infinal_weeknum: Week::from_number(week_of(dec31, sun0).0),
		};

		let month = Month { month: months_of(m), has_days: month_day(month_len, month_len as u16) };

		let week = Week::from_number(week_of(days, sun0).0);

		// ---- czas
		let hour24 = (rem_secs / 3_600) as u16;
		let minute = (rem_secs % 3_600 / 60) as u16;
		let second = (rem_secs % 60) as u16;

		// Ułamek sekundy dzielony kolejno przez 60 (tercja, kwadra, kwinta, seksta, septyma).
		let mut rest = u64::from(subsec_nanos);
		let mut next_sexagesimal = || {
			rest *= 60;
			let digit = (rest / 1_000_000_000) as u16;
			rest %= 1_000_000_000;
			digit
		};
		let tierce = next_sexagesimal();
		let quadra = next_sexagesimal();
		let quinta = next_sexagesimal();
		let sextus = next_sexagesimal();
		let septim = next_sexagesimal();

		let time = Time {
			hour24: Range0To23(hour24),
			hour12: Range1To12(nz((hour24 + 11) % 12 + 1)),
			hour_is_pm: hour24 >= 12,
			minute_in_hou: Range0To59(minute),
			second_in_min: Range0To59(second),
			tierce_in_sec: Range0To59(tierce),
			quadra_in_tie: Range0To59(quadra),
			quinta_in_qua: Range0To59(quinta),
			sextus_in_qui: Range0To59(sextus),
			septim_in_sex: Range0To59(septim),
			ms_in_second: Range0To999((subsec_nanos / 1_000_000) as u16),
			us_in_millis: Range0To999((subsec_nanos / 1_000 % 1_000) as u16),
			ns_in_micros: Range0To999((subsec_nanos % 1_000) as u16),
		};

		Self { day, year, week, month, time }
	}

	/// Formatuje strukturę `T` na podstawie podanego wzorca (jednoprzebiegowy parser).
	pub fn format_string(&self, pattern: &str, offset_seconds: i32 /*week_start: WeekFirstDay*/) -> String {
		let mut out = String::with_capacity(pattern.len() + 16);
		let mut remaining = pattern;

		let day_num = match self.day.in_month {
			MonthDay::M31D(d) => d.0.get(),
			MonthDay::M30D(d) => d.0.get(),
			MonthDay::M29D(d) => d.0.get(),
			MonthDay::M28D(d) => d.0.get(),
		};

		let month_num = self.month.month as u32;

		// Baza to ISO (Poniedziałek = 1 ... Niedziela = 7)
		let iso_weekday_num = match self.day.in_week {
			WeekDay::Mon => 1,
			WeekDay::Tue => 2,
			WeekDay::Wed => 3,
			WeekDay::Thu => 4,
			WeekDay::Fri => 5,
			WeekDay::Sat => 6,
			WeekDay::Sun => 7,
		};

		// Dopasowanie do ogólnoświatowych standardów:
		// Jeśli startujemy w poniedziałek (ISO): 1-7
		// Jeśli startujemy w niedzielę (Cron/POSIX): Niedziela to 0, Poniedziałek to 1...
		let weekday_num = match /*week_start*/ WEEK_FIRST_DAY_IS_MON_HARDCODED {
			WeekFirstDay::Mon => iso_weekday_num,
			WeekFirstDay::Sun => {
				if iso_weekday_num == 7 {
					0
				} else {
					iso_weekday_num
				}
			}
		};
		let day_in_year_num = match self.day.in_year {
			YearDay::Y365D(d) => d.0.get(),
			YearDay::Y366D(d) => d.0.get(),
		};

		while !remaining.is_empty() {
			// 1. Podwojone nawiasy traktujemy jako ucieczkę do znaku dosłownego
			if let Some(rest) = remaining.strip_prefix("[[") {
				out.push('[');
				remaining = rest;
				continue;
			}
			if let Some(rest) = remaining.strip_prefix("]]") {
				out.push(']');
				remaining = rest;
				continue;
			}

			// 2. Blok tokenu [TOKEN]
			if let Some(rest) = remaining.strip_prefix('[') {
				if let Some(end_idx) = rest.find(']') {
					let token = &rest[..end_idx];
					remaining = &rest[end_idx + 1..];

					match token {
						// Strefa
						"ZONE" => {
							let sign = if offset_seconds >= 0 { '+' } else { '-' };
							let abs_offset = offset_seconds.abs();
							out.push_str(&format!(
								"{}{:02}{:02}UTC",
								sign,
								abs_offset / 3600,
								(abs_offset % 3600) / 60
							));
						}

						// Rok
						"YYYW" => out.push_str(&format!("{:04}", self.year.week_year)),
						"YYYY" => out.push_str(&format!("{:04}", self.year.year)),
						"YW" => out.push_str(&format!("{:02}", (self.year.week_year % 100).abs())),
						"YY" => out.push_str(&format!("{:02}", (self.year.year % 100).abs())),
						// "OOO" => out.push_str(&format!("{:03}", self.year.cycle.cycl400)),

						// Miesiąc
						"MM" => out.push_str(&format!("{:02}", month_num)),
						"MMM" => out.push_str(match self.month.month {
							Months::Jan => "JAN",
							Months::Feb => "FEB",
							Months::Mar => "MAR",
							Months::Apr => "APR",
							Months::May => "MAY",
							Months::Jun => "JUN",
							Months::Jul => "JUL",
							Months::Aug => "AUG",
							Months::Sep => "SEP",
							Months::Oct => "OCT",
							Months::Nov => "NOV",
							Months::Dec => "DEC",
						}),

						// Dzień
						"D" => out.push_str(&weekday_num.to_string()),
						"DM" => out.push_str(&format!("{:02}", day_num)),
						"DDY" => out.push_str(&format!("{:03}", day_in_year_num)),
						"DDD" => out.push_str(match self.day.in_week {
							WeekDay::Mon => "MON",
							WeekDay::Tue => "TUE",
							WeekDay::Wed => "WED",
							WeekDay::Thu => "THU",
							WeekDay::Fri => "FRI",
							WeekDay::Sat => "SAT",
							WeekDay::Sun => "SUN",
						}),

						// Tydzień
						"WW" => out.push_str(&format!("{:02}", self.week as u32)),

						// Pora roko oraz kwartał
						"SSS" => out.push_str(match self.year.season {
							Season::Aut => "AUT",
							Season::Win => "WIN",
							Season::Spr => "SPR",
							Season::Sum => "SUM",
						}),
						"S" => out.push_str(&(self.year.season as u32).to_string()),
						"Q" => out.push_str(&(self.year.quater as u32).to_string()),

						// Czas
						"hh" => out.push_str(&format!("{:02}", self.time.hour24.0)),
						"hhhh" => {
							let h12 = self.time.hour12.0.get();
							let ampm = if self.time.hour_is_pm { "pm" } else { "am" };
							out.push_str(&format!("{}{:02}", ampm, h12));
						}
						// 1/60
						"mm" => out.push_str(&format!("{:02}", self.time.minute_in_hou.0)),
						"ss" => out.push_str(&format!("{:02}", self.time.second_in_min.0)),
						"tt" => out.push_str(&format!("{:02}", self.time.tierce_in_sec.0)),
						"qa" => out.push_str(&format!("{:02}", self.time.quadra_in_tie.0)),
						"qi" => out.push_str(&format!("{:02}", self.time.quinta_in_qua.0)),
						"sx" => out.push_str(&format!("{:02}", self.time.sextus_in_qui.0)),
						"sp" => out.push_str(&format!("{:02}", self.time.septim_in_sex.0)),
						// 1/1000
						"ssmil" => out.push_str(&format!("{:03}ms", self.time.ms_in_second.0)),
						"ssmic" => out.push_str(&format!("{:03}µs", self.time.us_in_millis.0)),
						"ssnan" => out.push_str(&format!("{:03}ns", self.time.ns_in_micros.0)),

						// Nieznany token wewnątrz [...] – przepisujemy go dosłownie wraz z nawiasami
						unknown => {
							out.push('[');
							out.push_str(unknown);
							out.push(']');
						}
					}
					continue;
				}
			}

			// Przepisz zwykły znak
			if let Some(c) = remaining.chars().next() {
				out.push(c);
				remaining = &remaining[c.len_utf8()..];
			}
		}

		out
	}
}

impl WeekDay {
	/// `iso`: poniedziałek = 1 ... niedziela = 7.
	fn from_iso(iso: i64) -> Self {
		match iso {
			1 => Self::Mon,
			2 => Self::Tue,
			3 => Self::Wed,
			4 => Self::Thu,
			5 => Self::Fri,
			6 => Self::Sat,
			_ => Self::Sun,
		}
	}
}

impl Week {
	const ALL: [Week; 53] = [
		Week::W01,
		Week::W02,
		Week::W03,
		Week::W04,
		Week::W05,
		Week::W06,
		Week::W07,
		Week::W08,
		Week::W09,
		Week::W10,
		Week::W11,
		Week::W12,
		Week::W13,
		Week::W14,
		Week::W15,
		Week::W16,
		Week::W17,
		Week::W18,
		Week::W19,
		Week::W20,
		Week::W21,
		Week::W22,
		Week::W23,
		Week::W24,
		Week::W25,
		Week::W26,
		Week::W27,
		Week::W28,
		Week::W29,
		Week::W30,
		Week::W31,
		Week::W32,
		Week::W33,
		Week::W34,
		Week::W35,
		Week::W36,
		Week::W37,
		Week::W38,
		Week::W39,
		Week::W40,
		Week::W41,
		Week::W42,
		Week::W43,
		Week::W44,
		Week::W45,
		Week::W46,
		Week::W47,
		Week::W48,
		Week::W49,
		Week::W50,
		Week::W51,
		Week::W52,
		Week::W53,
	];

	/// `n` w zakresie 1..=53; poza zakresem zwraca `W01`.
	fn from_number(n: i64) -> Self {
		usize::try_from(n - 1).ok().and_then(|i| Self::ALL.get(i)).copied().unwrap_or(Self::W01)
	}
}

// ------------------------------------------------------------ DNI W MIESIĄCU I ROKU - MAKRO
macro_rules! impl_try_from_day {
	($type:ty, $max:expr) => {
		impl TryFrom<u16> for $type {
			type Error = &'static str;

			fn try_from(day: u16) -> Result<Self, Self::Error> {
				if (1..=$max).contains(&day) {
					Ok(Self(NonZeroU16::new(day).unwrap()))
				} else {
					Err(concat!("Day must be in range 1..=", $max))
				}
			}
		}
	};
}

impl_try_from_day!(YearWith365Day, 365);
impl_try_from_day!(YearWith366Day, 366);
impl_try_from_day!(MonthWith31Day, 31);
impl_try_from_day!(MonthWith30Day, 30);
impl_try_from_day!(MonthWith29Day, 29);
impl_try_from_day!(MonthWith28Day, 28);

```

### [53] `./src/libs/timer/enums.rs`

```rust

use super::structs::{
	MonthWith28Day, MonthWith29Day, MonthWith30Day, MonthWith31Day, 
	
	YearWith365Day, YearWith366Day,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekFirstDay {
	Mon,
	Sun,
}


// ------------------------------------------------------------ TYDZIEŃ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Week {
	W01 = 1,
	W02 = 2,
	W03 = 3,
	W04 = 4,
	W05 = 5,
	W06 = 6,
	W07 = 7,
	W08 = 8,
	W09 = 9,
	W10 = 10,
	W11 = 11,
	W12 = 12,
	W13 = 13,
	W14 = 14,
	W15 = 15,
	W16 = 16,
	W17 = 17,
	W18 = 18,
	W19 = 19,
	W20 = 20,
	W21 = 21,
	W22 = 22,
	W23 = 23,
	W24 = 24,
	W25 = 25,
	W26 = 26,
	W27 = 27,
	W28 = 28,
	W29 = 29,
	W30 = 30,
	W31 = 31,
	W32 = 32,
	W33 = 33,
	W34 = 34,
	W35 = 35,
	W36 = 36,
	W37 = 37,
	W38 = 38,
	W39 = 39,
	W40 = 40,
	W41 = 41,
	W42 = 42,
	W43 = 43,
	W44 = 44,
	W45 = 45,
	W46 = 46,
	W47 = 47,
	W48 = 48,
	W49 = 49,
	W50 = 50,
	W51 = 51,
	W52 = 52,
	W53 = 53,
}

// ------------------------------------------------------------ MIESIĄC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Months {
	Jan = 1,
	Feb = 2,
	Mar = 3,
	Apr = 4,
	May = 5,
	Jun = 6,
	Jul = 7,
	Aug = 8,
	Sep = 9,
	Oct = 10,
	Nov = 11,
	Dec = 12,
}

// ------------------------------------------------------------ DZIEŃ W TYGODNIU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekDay {
	Mon,
	Tue,
	Wed,
	Thu,
	Fri,
	Sat,
	Sun,
}

// ------------------------------------------------------------ DZIEŃ W ROKU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearDay {
	Y365D(YearWith365Day),
	Y366D(YearWith366Day),
}


// ------------------------------------------------------------ DZIEŃ W MIESIĄCU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthDay {
	M31D(MonthWith31Day),
	M30D(MonthWith30Day),
	M29D(MonthWith29Day),
	M28D(MonthWith28Day),
}

// ------------------------------------------------------------ PORA ROKU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season {
	Aut = 1,
	Win = 2,
	Spr = 3,
	Sum = 4,
}
// ------------------------------------------------------------ ĆWIARTKA ROKU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quater {
	JanFebMar = 1,
	AprMayJun = 2,
	JulAugSep = 3,
	OctNovDec = 4,
}

```

### [54] `./src/libs/timer/functions.rs`

```rust
use super::consts::{SOLAR_EVENTS_Y0, SOLAR_EVENTS_Y2000, SOLAR_PERIODIC};

use super::enums::{
	MonthDay, Months, Quater,
	Season,
};


use super::structs::{
	MonthWith28Day, MonthWith29Day, MonthWith30Day, MonthWith31Day, 
	
};
use std::num::NonZeroU16;
// ------------------------------------------------------------ FUNKCJE POMOCNICZE
/// `NonZeroU16` z wartości, która z konstrukcji algorytmów jest zawsze >= 1.
pub fn nz(v: u16) -> NonZeroU16 {
	NonZeroU16::new(v).unwrap_or(NonZeroU16::MIN)
}

/// Dni od 1970-01-01 -> (rok, miesiąc 1..=12, dzień 1..=31).
pub fn civil_from_days(days: i64) -> (i64, i64, i64) {
	let z = days + 719_468;
	let era = z.div_euclid(146_097);
	let doe = z.rem_euclid(146_097);
	let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
	let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
	let mp = (5 * doy + 2) / 153;
	let d = doy - (153 * mp + 2) / 5 + 1;
	let m = if mp < 10 { mp + 3 } else { mp - 9 };
	let y = yoe + era * 400 + i64::from(m <= 2);
	(y, m, d)
}

/// (rok, miesiąc, dzień) -> dni od 1970-01-01.
pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
	let y = y - i64::from(m <= 2);
	let era = y.div_euclid(400);
	let yoe = y.rem_euclid(400);
	let mp = if m > 2 { m - 3 } else { m + 9 };
	let doy = (153 * mp + 2) / 5 + d - 1;
	let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
	era * 146_097 + doe - 719_468
}

pub fn is_leap(y: i64) -> bool {
	(y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

pub fn days_in_month(y: i64, m: i64) -> i64 {
	match m {
		1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
		4 | 6 | 9 | 11 => 30,
		_ if is_leap(y) => 29,
		_ => 28,
	}
}

/// Dzień tygodnia ISO: poniedziałek = 1 ... niedziela = 7.
pub fn iso_weekday(days: i64) -> i64 {
	(days + 3).rem_euclid(7) + 1
}

/// (numer tygodnia, rok tygodnia). Tydzień 1 to pierwszy tydzień z co najmniej
/// 4 dniami w nowym roku. `sun0 = false`: tydzień zaczyna się w poniedziałek
/// (ISO 8601), `sun0 = true`: w niedzielę.
pub fn week_of(days: i64, sun0: bool) -> (i64, i64) {
	let iso = iso_weekday(days);
	let pos = if sun0 { iso % 7 } else { iso - 1 }; // 0..=6 od początku tygodnia
	let pivot = days + (3 - pos); // czwarty dzień tygodnia decyduje o roku tygodnia
	let (py, _, _) = civil_from_days(pivot);
	let pivot_doy = pivot - days_from_civil(py, 1, 1) + 1;
	((pivot_doy - 1) / 7 + 1, py)
}

pub fn month_day(len: i64, d: u16) -> MonthDay {
	match len {
		31 => MonthDay::M31D(MonthWith31Day(nz(d))),
		30 => MonthDay::M30D(MonthWith30Day(nz(d))),
		29 => MonthDay::M29D(MonthWith29Day(nz(d))),
		_ => MonthDay::M28D(MonthWith28Day(nz(d))),
	}
}

pub fn months_of(m: i64) -> Months {
	match m {
		1 => Months::Jan,
		2 => Months::Feb,
		3 => Months::Mar,
		4 => Months::Apr,
		5 => Months::May,
		6 => Months::Jun,
		7 => Months::Jul,
		8 => Months::Aug,
		9 => Months::Sep,
		10 => Months::Oct,
		11 => Months::Nov,
		_ => Months::Dec,
	}
}

pub fn quater_of(m: i64) -> Quater {
	match (m - 1) / 3 {
		0 => Quater::JanFebMar,
		1 => Quater::AprMayJun,
		2 => Quater::JulAugSep,
		_ => Quater::OctNovDec,
	}
}

/// Pora roku astronomiczna, półkula północna: wiosna od równonocy marcowej, lato od przesilenia
/// czerwcowego, jesień od równonocy wrześniowej, zima od przesilenia grudniowego.
/// Chwile zdarzeń liczone są dla roku `y` (Meeus, "Astronomical Algorithms", rozdz. 27)
/// i porównywane z chwilą `now_unix` (UTC), więc granica przechodzi w dokładnej minucie.
pub fn season(y: i64, now_unix: f64) -> Season {
	let [mar, jun, sep, dec] = solar_events_unix(y);
	if now_unix < mar {
		Season::Win
	} else if now_unix < jun {
		Season::Spr
	} else if now_unix < sep {
		Season::Sum
	} else if now_unix < dec {
		Season::Aut
	} else {
		Season::Win
	}
}

/// Chwile [równonoc marcowa, przesilenie czerwcowe, równonoc wrześniowa,
/// przesilenie grudniowe] w roku `y`, jako sekundy uniksowe UTC.
/// Algorytm jest opisany dla lat -1000..3000.
pub fn solar_events_unix(y: i64) -> [f64; 4] {
	let (table, yy) = if y < 1_000 {
		(SOLAR_EVENTS_Y0, y as f64 / 1_000.0)
	} else {
		(SOLAR_EVENTS_Y2000, (y - 2_000) as f64 / 1_000.0)
	};
	let delta_t = delta_t_seconds(y);

	table.map(|c| {
		let jde0 = c[0] + yy * (c[1] + yy * (c[2] + yy * (c[3] + yy * c[4])));
		let t = (jde0 - 2_451_545.0) / 36_525.0;
		let w = (35_999.373 * t - 2.47).to_radians();
		let dl = 1.0 + 0.0334 * w.cos() + 0.0007 * (2.0 * w).cos();
		let s: f64 = SOLAR_PERIODIC.iter().map(|&(a, b, k)| a * (b + k * t).to_radians().cos()).sum();
		let jde = jde0 + 0.00001 * s / dl; // czas dynamiczny (TT)
		(jde - 2_440_587.5) * 86_400.0 - delta_t // -> UTC, sekundy uniksowe
	})
}

/// Przybliżenie różnicy TT - UTC w sekundach (Espenak/Meeus).
/// Dla lat 2005..2050 wielomian, poza tym parabola długoterminowa (mniej dokładna).
pub fn delta_t_seconds(y: i64) -> f64 {
	let yf = y as f64 + 0.5;
	if (2_005..2_050).contains(&y) {
		let t = yf - 2_000.0;
		62.92 + 0.322_17 * t + 0.005_589 * t * t
	} else {
		let u = (yf - 1_820.0) / 100.0;
		-20.0 + 32.0 * u * u
	}
}

```

### [55] `./src/libs/timer/structs.rs`

```rust
use super::enums::{MonthDay, Months, Quater, Season, Week, WeekDay, YearDay};
use std::num::NonZeroU16;

#[derive(Debug, Clone, Copy)]
pub struct Timer(pub i64, pub u32, pub u32, pub i32);

#[derive(Debug)]
pub struct T {
	pub day: Day,
	pub year: Year,
	pub week: Week,
	pub month: Month,
	pub time: Time,
}

#[derive(Debug)]
pub struct Day {
	pub in_year: YearDay,
	pub in_month: MonthDay,
	pub in_week: WeekDay,
}

#[derive(Debug)]
pub struct Year {
	pub week_year: i16,
	pub year: i16,
	pub season: Season,
	pub quater: Quater,
	pub is_leap: bool,
	pub cycle: Cycle,
	pub infirst_day: WeekDay,
	pub infinal_day: WeekDay,
	pub infirst_weeknum: Week, // Skondensowane z infirst_sun7_weeknum i infirst_sun0_weeknum
	pub infinal_weeknum: Week, // Skondensowane z infinal_sun7_weeknum i infinal_sun0_weeknum
}

#[derive(Debug)]
pub struct Month {
	pub month: Months,
	pub has_days: MonthDay,
}

#[derive(Debug)]
pub struct Cycle {
	pub century: i16,
	pub cycl400: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearWith365Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearWith366Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith31Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith30Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith29Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith28Day(pub NonZeroU16);

//-------------------------------------------------------- CZAS

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range1To12(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range0To23(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range0To59(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range0To999(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
	pub hour24: Range0To23,
	pub hour12: Range1To12,
	pub hour_is_pm: bool,
	pub minute_in_hou: Range0To59,
	pub second_in_min: Range0To59,
	pub tierce_in_sec: Range0To59,
	pub quadra_in_tie: Range0To59,

	pub quinta_in_qua: Range0To59,
	pub sextus_in_qui: Range0To59,
	pub septim_in_sex: Range0To59,

	// 1 tercja ma  (50/3)    milisekund  (około  16.67  ms)
	// 1 kwadra ma  (2500/9)  mikrosekund (około 277.8   µs)
	// 1 kwinta ma  (125/27)  mikrosekund (około   4.63  µs)
	// 1 seksta ma  (6250/81) nanosekundy (około  77.16  ns)
	// 1 septyma ma (625/486) nanosekund  (około   1.286 ns)
	pub ms_in_second: Range0To999,
	pub us_in_millis: Range0To999,
	pub ns_in_micros: Range0To999,
}

```

### [56] `./.zed/settings.json`

```json
{

  // --- TABULATORY I FORMATOWANIE ---
  "tab_size": 4,
  "hard_tabs": true,

  // --- STRUKTURA I PROWADNICE EDYTORA ---
  "preferred_line_length": 100,
  "show_whitespaces": "boundary",
  "indent_guides": {
    "background_coloring": "indent_aware",
    "enabled": true
  },

  // --- WYKLUCZENIA ZE SKANOWANIA I WYSZUKIWARKI ---
  "file_scan_exclusions": [
    "**/.git",
    "**/.prompt",
    "**/target",
    "**/*.lock",
    "**/Thumbs.db",
    "./dump/",
    "./.vscode/"
  ],

  // --- PODPOWIEDZI W KODZIE (INLAY HINTS) ---
  "inlay_hints": {
    "enabled": true,
    "show_type_hints": true,
    "show_parameter_hints": true,
    "show_other_hints": true
  },

  // --- USTAWIENIA DLA KONKRETNYCH JĘZYKÓW ---
  "languages": {
    "Rust": {
      "format_on_save": "on",
      "tab_size": 4,
      "hard_tabs": true
    },
    "Markdown": {
      "preferred_line_length": 100
    }
  },

  // --- RUST-ANALYZER (LSP) ---
  "lsp": {
    "rust-analyzer": {
    	"enable_lsp_tasks": true,
	    "fetch": {
	    	"pre_release": true
	    },
      	"initialization_options": {
       			// get more cargo-less diagnostics from rust-analyzer,
                // which might include false-positives (those can be turned off by their names)
                "diagnostics": {
	                "experimental": {
	                    "enable": true
	                }
               },
	      	"inlayHints": {
		      	"maxLength": null,
		        // "lifetimeElisionHints": {
			    //     "enable": "skip_trivial",
			    //     "useParameterNames": true
		        // },
		        "chainingHints": { "enable": true },
		        "closureReturnTypeHints": { "enable": "always" },
		        "parameterHints": { "enable": true },
		        "typeHints": { "enable": true }
	     },
        "checkOnSave": true,
        "check": {
          "command": "check"
        },
      }
    }
  }
}

```

## End Structure Summary

```plaintext
  ▣─┬ 🆗cargo-plot                                [120.4 KiB]                      A:/A-JAN/WIN-DOCS/REPO_OW 
    │                                                                             N_RUST/GIT_j-Cis/🆗cargo-p 
    │                                                                             lot/                      
 1  ├──• Cargo.toml                              [  1.8 KiB] [2026-43-4:02:35:09] ./Cargo.toml              
 2  ├──• .rustfmt.toml                           [  3.4 KiB] [2026-38-7:22:31:33] ./.rustfmt.toml           
    ├──┬ .cargo                                  [  3.0 KiB] [2026-43-5:09:53:04] ./.cargo/                 
 3  │  ├──• config.toml                          [    593 B] [2026-43-3:22:26:37] ./.cargo/config.toml      
 4  │  ├──• .x-do.toml                           [  1.2 KiB] [2026-43-5:16:50:14] ./.cargo/.x-do.toml       
 5  │  ├──• .x-do-history.txt                    [     67 B] [2026-43-2:23:39:30] ./.cargo/.x-do-history.tx 
    │  │                                                                          t                         
 6  │  └──• .x-do2.toml                          [  1.1 KiB] [2026-43-4:16:05:44] ./.cargo/.x-do2.toml      
    ├──┬ examples                                [  2.1 KiB] [2026-43-5:09:17:10] ./examples/               
 7  │  ├──• example_timer.rs                     [    295 B] [2026-43-5:09:27:39] ./examples/example_timer. 
    │  │                                                                          rs                        
 8  │  └──• example01.rs                         [  1.8 KiB] [2026-43-5:16:09:10] ./examples/example01.rs   
    ├──┬ src                                     [108.2 KiB] [2026-43-2:15:49:53] ./src/                    
 9  │  ├──• lib.rs                               [    322 B] [2026-43-5:09:18:30] ./src/lib.rs              
    │  └──┬ libs                                 [107.9 KiB] [2026-43-5:09:18:16] ./src/libs/               
10  │     ├──• querypath.rs                      [    421 B] [2026-43-5:14:28:49] ./src/libs/querypath.rs   
11  │     ├──• timer.rs                          [    145 B] [2026-43-5:06:45:48] ./src/libs/timer.rs       
    │     ├──┬ querypath                         [ 80.8 KiB] [2026-43-5:09:21:39] ./src/libs/querypath/     
12  │     │  ├──• controllers.rs                 [    374 B] [2026-43-5:09:20:15] ./src/libs/querypath/cont 
    │     │  │                                                                    rollers.rs                
13  │     │  ├──• core.rs                        [    360 B] [2026-43-5:09:22:56] ./src/libs/querypath/core 
    │     │  │                                                                    .rs                       
    │     │  ├──┬ controllers                    [ 19.3 KiB] [2026-43-5:09:26:44] ./src/libs/querypath/cont 
    │     │  │  │                                                                 rollers/                  
14  │     │  │  ├──• gather_claims.rs            [    463 B] [2026-43-5:14:28:36] ./src/libs/querypath/cont 
    │     │  │  │                                                                 rollers/gather_claims.rs  
15  │     │  │  ├──• prepar_setups.rs            [    833 B] [2026-43-5:16:26:08] ./src/libs/querypath/cont 
    │     │  │  │                                                                 rollers/prepar_setups.rs  
    │     │  │  └──┬ gather_claims               [ 18.1 KiB] [2026-43-5:13:06:51] ./src/libs/querypath/cont 
    │     │  │     │                                                              rollers/gather_claims/    
16  │     │  │     ├──• engine.rs                [ 12.5 KiB] [2026-43-5:16:25:13] ./src/libs/querypath/cont 
    │     │  │     │                                                              rollers/gather_claims/eng 
    │     │  │     │                                                              ine.rs                    
17  │     │  │     ├──• traits.rs                [  1.5 KiB] [2026-43-5:09:48:29] ./src/libs/querypath/cont 
    │     │  │     │                                                              rollers/gather_claims/tra 
    │     │  │     │                                                              its.rs                    
18  │     │  │     └──• types.rs                 [  4.1 KiB] [2026-43-5:16:25:13] ./src/libs/querypath/cont 
    │     │  │                                                                    rollers/gather_claims/typ 
    │     │  │                                                                    es.rs                     
    │     │  └──┬ core                           [ 60.7 KiB] [2026-43-5:09:21:39] ./src/libs/querypath/core 
    │     │     │                                                                 /                         
19  │     │     ├──• features.rs                 [    608 B] [2026-43-5:16:25:24] ./src/libs/querypath/core 
    │     │     │                                                                 /features.rs              
20  │     │     ├──• structs.rs                  [    791 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │                                                                 /structs.rs               
21  │     │     ├──• traits.rs                   [    320 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │                                                                 /traits.rs                
22  │     │     ├──• types.rs                    [    303 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │                                                                 /types.rs                 
23  │     │     ├──• utils.rs                    [    515 B] [2026-43-3:16:00:19] ./src/libs/querypath/core 
    │     │     │                                                                 /utils.rs                 
    │     │     ├──┬ features                    [ 31.6 KiB] [2026-43-5:16:18:36] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/                
24  │     │     │  ├──• handle_corpus.rs         [  4.9 KiB] [2026-43-5:16:23:47] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/handle_corpus.r 
    │     │     │  │                                                              s                         
25  │     │     │  ├──• handle_formatted_time.rs [  2.5 KiB] [2026-43-5:14:24:19] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/handle_formatte 
    │     │     │  │                                                              d_time.rs                 
26  │     │     │  ├──• handle_prepared_patterns.[  3.2 KiB] [2026-43-5:14:24:36] ./src/libs/querypath/core 
    │     │     │  │    rs                                                        /features/handle_prepared 
    │     │     │  │                                                              _patterns.rs              
27  │     │     │  ├──• handle_verified_dirs.rs  [  4.3 KiB] [2026-43-5:14:24:54] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/handle_verified 
    │     │     │  │                                                              _dirs.rs                  
28  │     │     │  ├──• handle_walk_scan.rs      [ 15.6 KiB] [2026-43-5:15:37:26] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/handle_walk_sca 
    │     │     │  │                                                              n.rs                      
29  │     │     │  ├──• step05_build_tree.rs     [    326 B] [2026-43-3:16:08:11] ./src/libs/querypath/core 
    │     │     │  │                                                              /features/step05_build_tr 
    │     │     │  │                                                              ee.rs                     
30  │     │     │  └──• verbose_display.rs       [    781 B] [2026-43-5:16:25:41] ./src/libs/querypath/core 
    │     │     │                                                                 /features/verbose_display 
    │     │     │                                                                 .rs                       
    │     │     ├──┬ structs                     [ 20.4 KiB] [2026-43-3:12:27:41] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/                 
31  │     │     │  ├──• anchored_path.rs         [  2.6 KiB] [2026-43-5:11:06:12] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/anchored_path.rs 
32  │     │     │  ├──• fs_walk_stat.rs          [    597 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/fs_walk_stat.rs  
33  │     │     │  ├──• item_file_dir.rs         [    879 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/item_file_dir.rs 
34  │     │     │  ├──• path_context.rs          [  1.1 KiB] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/path_context.rs  
35  │     │     │  ├──• path_node.rs             [    761 B] [2026-43-5:11:06:21] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/path_node.rs     
36  │     │     │  ├──• pattern_compiled.rs      [  5.4 KiB] [2026-43-5:11:06:54] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/pattern_compiled 
    │     │     │  │                                                              .rs                       
37  │     │     │  ├──• patterns.rs              [  3.3 KiB] [2026-43-5:11:07:05] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/patterns.rs      
38  │     │     │  ├──• tree.rs                  [  1.5 KiB] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/tree.rs          
39  │     │     │  ├──• tree_bundle_item.rs      [    529 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/tree_bundle_item 
    │     │     │  │                                                              .rs                       
40  │     │     │  ├──• tree_bundle_result.rs    [  1.2 KiB] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/tree_bundle_resu 
    │     │     │  │                                                              lt.rs                     
41  │     │     │  ├──• tree_item.rs             [  1.1 KiB] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /structs/tree_item.rs     
42  │     │     │  └──• walk_env_index.rs        [  1.6 KiB] [2026-43-5:11:07:48] ./src/libs/querypath/core 
    │     │     │                                                                 /structs/walk_env_index.r 
    │     │     │                                                                 s                         
    │     │     ├──┬ traits                      [  1.0 KiB] [2026-43-3:22:09:49] ./src/libs/querypath/core 
    │     │     │  │                                                              /traits/                  
43  │     │     │  ├──• apply_claim.rs           [    431 B] [2026-43-3:22:07:06] ./src/libs/querypath/core 
    │     │     │  │                                                              /traits/apply_claim.rs    
44  │     │     │  └──• path_env_index.rs        [    640 B] [2026-43-3:22:06:02] ./src/libs/querypath/core 
    │     │     │                                                                 /traits/path_env_index.rs 
    │     │     ├──┬ types                       [  2.3 KiB] [2026-43-2:16:31:45] ./src/libs/querypath/core 
    │     │     │  │                                                              /types/                   
45  │     │     │  ├──• consts.rs                [    777 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │  │                                                              /types/consts.rs          
46  │     │     │  └──• enums.rs                 [  1.5 KiB] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │     │                                                                 /types/enums.rs           
    │     │     └──┬ utils                       [  2.9 KiB] [2026-43-3:11:05:26] ./src/libs/querypath/core 
    │     │        │                                                              /utils/                   
47  │     │        ├──• check_is_binary.rs       [    652 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │        │                                                              /utils/check_is_binary.rs 
48  │     │        ├──• diff_paths.rs            [    889 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │        │                                                              /utils/diff_paths.rs      
49  │     │        ├──• get_modified_time.rs     [    553 B] [2026-43-3:20:53:57] ./src/libs/querypath/core 
    │     │        │                                                              /utils/get_modified_time. 
    │     │        │                                                              rs                        
50  │     │        └──• normalize_path.rs        [    861 B] [2026-43-3:16:21:22] ./src/libs/querypath/core 
    │     │                                                                       /utils/normalize_path.rs  
    │     └──┬ timer                             [ 26.6 KiB] [2026-43-5:06:45:30] ./src/libs/timer/         
51  │        ├──• consts.rs                      [  2.0 KiB] [2026-43-5:08:37:10] ./src/libs/timer/consts.r 
    │        │                                                                    s                         
52  │        ├──• engine.rs                      [ 14.6 KiB] [2026-43-5:08:46:13] ./src/libs/timer/engine.r 
    │        │                                                                    s                         
53  │        ├──• enums.rs                       [  2.3 KiB] [2026-43-4:15:56:07] ./src/libs/timer/enums.rs 
54  │        ├──• functions.rs                   [  5.1 KiB] [2026-43-4:15:55:37] ./src/libs/timer/function 
    │        │                                                                    s.rs                      
55  │        └──• structs.rs                     [  2.7 KiB] [2026-43-5:07:18:12] ./src/libs/timer/structs. 
    │                                                                             rs                        
    └──┬ .zed                                    [  1.9 KiB] [2026-43-3:20:32:00] ./.zed/                   
56     └──• settings.json                        [  1.9 KiB] [2026-43-3:21:52:40] ./.zed/settings.json      

```

## Configuration & Setups Summary

| Setup Parameter | Configuration Value |
| :--- | :--- |
| **Execution Dir (CWD)** | `A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT_j-Cis/🆗cargo-plot` |
| **Scan Targets** | `["./"]` |
| **Match Patterns** | `["./{src|examples|tests|sandobx|.cargo|.zed}/**", "./{Cargo|.rustfmt}.toml", "!./Cargo.lock", "!./{target|.git|dist|dump}/**"]` |
| **Content Skip Patterns** | `["./assets/**", "*.svg"]` |
| **Max Single File Size** | `524288 B` |
| **Scanned Objects** | `2440 files, 431 dirs` |
| **Execution Time** | `136 ms` |

---


