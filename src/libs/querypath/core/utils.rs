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

mod find_lowest_common_ancestor;
pub use find_lowest_common_ancestor::find_lowest_common_ancestor;