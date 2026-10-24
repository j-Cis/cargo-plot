//! Path: `src/libs/querypath/controllers/gather_claims.rs`
//! Last significant modified: 2026-W43-3 22:13:11 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

pub mod traits;
pub mod types;
pub mod engine;
mod consts;
pub use consts::{HARDCODED_TIME_TREE,HARDCODED_TIME_HEAD,HARDCODED_TIME_SAVE};
pub use types::{DebugVerboses,ClaimCreating,ClaimDefaults,ClaimOrdering,ClaimPatterns,GatherClaims, ClaimRefining};