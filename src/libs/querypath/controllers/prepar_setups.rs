//! Path: `src/libs/querypath/controllers/prepar_setups.rs`
//! Last significant modified: 2026-W43-3 19:12:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::gather_claims::{ClaimCreating, ClaimOrdering, DebugVerboses};
use crate::libs::querypath::features::{
	HandlePreparedPatterns, HandleVerifiedDirs, HandleWalkScan, step04_execute_query::QueryExecute,
};

/// Twardy, zwalidowany stan, z którego korzystać będą potoki tworzenia i
/// drukowania.
pub struct PreparedSetup {
	pub entry: HandleVerifiedDirs,
	pub patts: HandlePreparedPatterns,
	pub walk: HandleWalkScan,
	pub execute: QueryExecute,
	pub ordering: ClaimOrdering,
	pub creating: ClaimCreating,
	pub verboses: DebugVerboses,
}
