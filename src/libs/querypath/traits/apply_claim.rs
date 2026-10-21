//! Path: `src/libs/querypath/traits/apply_claim.rs`
//! Last significant modified: 2026-W43-4 00:04 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Trait aplikujący konfiguracje do głównego zbiornika roszczeń.
pub trait ApplyClaim {
	fn apply(self, claims: &mut GatherClaims);
}
