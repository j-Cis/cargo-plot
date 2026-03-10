// Plik: src/cli/doc.rs
use crate::cli::args::{DocArgs, IdStyle, InsertTreeMethod};
use crate::cli::utils::{build_weight_config, collect_tasks};
use lib::fn_doc_gen::generate_docs;
use lib::fn_doc_models::DocTask;
use lib::fn_filespath::filespath;

pub fn handle_doc(args: DocArgs) {
    let tasks = collect_tasks(&args.shared);
    let w_cfg = build_weight_config(&args.shared);

    // Klonujemy wywołanie z konsoli, aby umieścić je w pliku
    let cmd_str = if args.print_command {
        Some(std::env::args().collect::<Vec<_>>().join(" "))
    } else {
        None
    };

    let watermark_str = match args.watermark {
        crate::cli::args::WatermarkPosition::First => "first",
        crate::cli::args::WatermarkPosition::Last => "last",
        crate::cli::args::WatermarkPosition::None => "none",
    };

    let doc_task = DocTask {
        output_filename: &args.out,
        insert_tree: match args.insert_tree {
            InsertTreeMethod::DirsFirst => "dirs-first",
            InsertTreeMethod::None => "with-out",
            _ => "files-first",
        },
        id_style: match args.id_style {
            IdStyle::Num => "id-num",
            IdStyle::None => "id-non",
            _ => "id-tag",
        },
        tasks,
        weight_config: w_cfg,
        watermark: watermark_str,
        command_str: cmd_str,
    };

    if args.dry_run {
        println!(
            "[!] SYMULACJA: Wykryto {} plików do przetworzenia.",
            filespath(&doc_task.tasks).len()
        );
        return;
    }

    if let Err(e) = generate_docs(vec![doc_task], &args.out_dir) {
        eprintln!("[-] Błąd generowania raportu w '{}': {}", args.out_dir, e);
    }
}