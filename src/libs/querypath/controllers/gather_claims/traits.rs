//! Path: `src/libs/querypath/controllers/gather_claims/traits.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792
use crate::libs::querypath::controllers::types::{ClaimDefaults, GatherClaims};
// ============================================================================
// 1. TRAIT KONFIGURACYJNY I KROTKI (Pozwala pominąć Enum Gather)
// ============================================================================

/// Trait aplikujący konfiguracje do głównego zbiornika roszczeń.
pub trait ApplyClaim {
	fn apply(self, claims: &mut GatherClaims);
}

impl ApplyClaim for ClaimDefaults {
	fn apply(self, _claims: &mut GatherClaims) {}
}

// Obsługa krotek dla jednoczesnego wstrzykiwania wielu różnych konfiguracji.
impl<A: ApplyClaim, B: ApplyClaim> ApplyClaim for (A, B) {
	fn apply(self, claims: &mut GatherClaims) {
		self.0.apply(claims);
		self.1.apply(claims);
	}
}

impl<A: ApplyClaim, B: ApplyClaim, C: ApplyClaim> ApplyClaim for (A, B, C) {
	fn apply(self, claims: &mut GatherClaims) {
		self.0.apply(claims);
		self.1.apply(claims);
		self.2.apply(claims);
	}
}

impl<A: ApplyClaim, B: ApplyClaim, C: ApplyClaim, D: ApplyClaim> ApplyClaim for (A, B, C, D) {
	fn apply(self, claims: &mut GatherClaims) {
		self.0.apply(claims);
		self.1.apply(claims);
		self.2.apply(claims);
		self.3.apply(claims);
	}
}
