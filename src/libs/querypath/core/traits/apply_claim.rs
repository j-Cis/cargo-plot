//! Path: `src/libs/querypath/traits/apply_claim.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Trait aplikujący konfiguracje do głównego zbiornika roszczeń.
pub trait ApplyClaim {
	fn apply(self, claims: &mut GatherClaims);
}
