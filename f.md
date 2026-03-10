# Struktura Projektu 2026Q1D070W11_Wed11Mar_001706338

**Wywołana komenda:**
```bash
target\debug\cargo-plot.exe plot tree -s files-first --no-default-excludes -e ./f.md -e ./d.md -e ./target/ -e ./.git/ -e ./test/ -e ./.gitignore -e ./u.md -e ./Cargo.lock -e ./LICENSE-APACHE -e ./LICENSE-MIT -e ./.github/ -e ./.cargo/ -e ./doc/ -e ./README.md -w binary --weight-precision 5 --no-dir-weight --out-file f.md --print-console --watermark last --print-command --title-file Struktura Projektu
```

```text
[KiB 1.689] ├──• ⚙️ Cargo.toml
            └──┬ 📂 src
[  B 671.0]    ├──• 🦀 main.rs
               ├──┬ 📂 cli
[KiB 7.231]    │  ├──• 🦀 args.rs
[  B 724.0]    │  ├──• 🦀 dist.rs
[KiB 1.791]    │  ├──• 🦀 doc.rs
[  B 408.0]    │  ├──• 🦀 mod.rs
[  B 577.0]    │  ├──• 🦀 stamp.rs
[KiB 3.690]    │  ├──• 🦀 tree.rs
[KiB 2.486]    │  └──• 🦀 utils.rs
               ├──┬ 📂 lib
[KiB 6.758]    │  ├──• 🦀 fn_copy_dist.rs
[KiB 1.702]    │  ├──• 🦀 fn_datestamp.rs
[KiB 1.913]    │  ├──• 🦀 fn_doc_gen.rs
[KiB 2.703]    │  ├──• 🦀 fn_doc_id.rs
[  B 570.0]    │  ├──• 🦀 fn_doc_models.rs
[KiB 4.593]    │  ├──• 🦀 fn_doc_write.rs
[KiB 1.964]    │  ├──• 🦀 fn_files_blacklist.rs
[KiB 8.222]    │  ├──• 🦀 fn_filespath.rs
[KiB 4.604]    │  ├──• 🦀 fn_filestree.rs
[  B 724.0]    │  ├──• 🦀 fn_path_utils.rs
[KiB 1.546]    │  ├──• 🦀 fn_pathtype.rs
[KiB 4.278]    │  ├──• 🦀 fn_plotfiles.rs
[KiB 3.602]    │  ├──• 🦀 fn_weight.rs
[  B 288.0]    │  └──• 🦀 mod.rs
               └──┬ 📂 tui
[KiB 1.393]       ├──• 🦀 dist.rs
[KiB 4.244]       ├──• 🦀 doc.rs
[KiB 1.487]       ├──• 🦀 mod.rs
[KiB 1.023]       ├──• 🦀 stamp.rs
[KiB 2.616]       ├──• 🦀 tree.rs
[KiB 6.317]       └──• 🦀 utils.rs

```

---
> 🚀 Wygenerowano przy użyciu [cargo-plot](https://crates.io/crates/cargo-plot) | Źródło: [GitHub](https://github.com/j-Cis/cargo-plot)

