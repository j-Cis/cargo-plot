// Plik: src/cli/tree.rs
use crate::cli::args::{SortMethod, TreeArgs};
use crate::cli::utils::collect_tasks;
use lib::fn_filespath::filespath;
use lib::fn_filestree::filestree;
use lib::fn_plotfiles::plotfiles_cli;
use lib::fn_weight::{UnitSystem, WeightConfig}; // Zaimportowane wagi z Kroku 1

pub fn handle_tree(args: TreeArgs) {
    let tasks = collect_tasks(&args.shared);
    let paths = filespath(&tasks);

    let sort_str = match args.sort {
        SortMethod::DirsFirst => "dirs-first",
        SortMethod::FilesFirst => "files-first",
        _ => "alpha",
    };

    // Tymczasowo, dla wdrożenia z Kroku 1
    let mut w_cfg = WeightConfig::default();
    w_cfg.system = UnitSystem::None;

    let nodes = filestree(paths, sort_str, &w_cfg); // Przekazujemy w_cfg
    println!("{}", plotfiles_cli(&nodes, "", None));
}