// Plik: src/cli/mod.rs
pub mod args;
mod dist;
mod doc;
mod stamp;
mod tree;
mod utils;

use args::Commands;

pub fn run_command(cmd: Commands) {
    match cmd {
        Commands::Tree(args) => tree::handle_tree(args),
        Commands::Doc(args) => doc::handle_doc(args),
        Commands::Stamp(args) => stamp::handle_stamp(args),
        Commands::DistCopy(args) => dist::handle_dist_copy(args),
    }
}