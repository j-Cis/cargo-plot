// Plik: src/cli/tree.rs
use crate::cli::args::{SortMethod, TreeArgs};
use crate::cli::utils::{build_weight_config, collect_tasks};
use lib::fn_filespath::filespath;
use lib::fn_filestree::filestree;
use lib::fn_plotfiles::plotfiles_cli;

pub fn handle_tree(args: TreeArgs) {
    let tasks = collect_tasks(&args.shared);
    let paths = filespath(&tasks);

    let sort_str = match args.sort {
        SortMethod::DirsFirst => "dirs-first",
        SortMethod::FilesFirst => "files-first",
        _ => "alpha",
    };

    // POBIERAMY KONFIGURACJĘ WAG NA PODSTAWIE FLAG CLI
    let w_cfg = build_weight_config(&args.shared);

    let nodes = filestree(paths, sort_str, &w_cfg);

    // ==========================================
    // NOWA LOGIKA WYDRUKU / ZAPISU DO PLIKU
    // ==========================================

    // 1. Zawsze drukuj do konsoli, chyba że użytkownik podał plik i NIE poprosił o konsolę
    let print_to_console = args.out_file.is_none() || args.print_console;

    if print_to_console {
        println!("{}", plotfiles_cli(&nodes, "", None));
    }

    // 2. Zapisz do pliku, jeśli podano argument --out-file
    // 2. Zapisz do pliku, jeśli podano argument --out-file
    if let Some(out_file) = args.out_file {
        let stamp = lib::fn_datestamp::datestamp_now();

        // Magia ucinania rozszerzenia (np. z "plik.md" robimy "plik__STAMP.md")
        let final_out_file = if args.suffix_stamp {
            let path = std::path::Path::new(&out_file);
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let ext = path.extension().unwrap_or_default().to_string_lossy();
            let parent = path.parent().unwrap_or_else(|| std::path::Path::new(""));

            let new_name = if ext.is_empty() {
                format!("{}__{}", stem, stamp)
            } else {
                format!("{}__{}.{}", stem, stamp, ext)
            };

            let pb = parent.join(new_name);
            if parent.as_os_str().is_empty() {
                pb.to_string_lossy().into_owned()
            } else {
                pb.to_string_lossy().replace('\\', "/")
            }
        } else {
            out_file.clone()
        };

        let mut content = String::new();

        // ==========================================
        // LOGIKA TYTUŁU DLA TREE
        // ==========================================
        let mut title_line = format!("# {}", args.title_file);
        if !args.suffix_stamp {
            title_line.push_str(&format!(" {}", stamp));
        }
        if args.title_file_with_path {
            title_line.push_str(&format!(" ({})", final_out_file));
        }
        content.push_str(&title_line);
        content.push_str("\n\n");
        // ==========================================

        let watermark_text = "> 🚀 Wygenerowano przy użyciu [cargo-plot](https://crates.io/crates/cargo-plot) | Źródło: [GitHub](https://github.com/j-Cis/cargo-plot)\n\n";

        if args.watermark == crate::cli::args::WatermarkPosition::First {
            content.push_str(watermark_text);
        }

        if args.print_command {
            let cmd = std::env::args().collect::<Vec<_>>().join(" ");
            content.push_str(&format!("**Wywołana komenda:**\n```bash\n{}\n```\n\n", cmd));
        }

        let txt = lib::fn_plotfiles::plotfiles_txt(&nodes, "", None);
        content.push_str(&format!("```text\n{}\n```\n", txt));

        if args.watermark == crate::cli::args::WatermarkPosition::Last {
            content.push_str("\n---\n");
            content.push_str(watermark_text);
        }

        std::fs::write(&final_out_file, content).unwrap();
        println!(" [+] Sukces! Drzewo zapisano do pliku: {}", final_out_file);
    }
}
