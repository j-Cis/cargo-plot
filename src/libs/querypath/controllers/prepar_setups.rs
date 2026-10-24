//! Path: `src/libs/querypath/controllers/prepar_setups.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::gather_claims::types::{ClaimCreating, ClaimOrdering, DebugVerboses};
use crate::libs::querypath::core::features::{
	HandleCorpus,
	HandlePreparedPatterns,
	HandleVerifiedDirs,
	HandleWalkScan,
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
