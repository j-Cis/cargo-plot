// Plik: src/cli/dist.rs
use crate::cli::args::DistCopyArgs;
use lib::fn_copy_dist::{DistConfig, copy_dist};

pub fn handle_dist_copy(args: DistCopyArgs) {
    let bin_refs: Vec<&str> = args.bin.iter().map(|s| s.as_str()).collect();
    let config = DistConfig {
        target_dir: &args.target_dir,
        dist_dir: &args.dist_dir,
        binaries: bin_refs,
        clear_dist: args.clear,
        overwrite: !args.no_overwrite,
        dry_run: args.dry_run,
    };

    match copy_dist(&config) {
        Ok(files) => {
            for (s, d) in files {
                println!(" [+] {} -> {}", s.display(), d.display());
            }
        }
        Err(e) => eprintln!("[-] Błąd dystrybucji: {}", e),
    }
}
