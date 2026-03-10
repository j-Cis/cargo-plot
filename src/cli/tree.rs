// Plik: src/cli/tree.rs
use crate::cli::args::{SortMethod, TreeArgs};
use crate::cli::utils::{build_weight_config, collect_tasks};
use lib::fn_filespath::filespath;
use lib::fn_filestree::filestree;
use lib::fn_plotfiles::plotfiles_cli;
// use lib::fn_weight::{UnitSystem, WeightConfig}; // Zaimportowane wagi z Kroku 1

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
    println!("{}", plotfiles_cli(&nodes, "", None));
}