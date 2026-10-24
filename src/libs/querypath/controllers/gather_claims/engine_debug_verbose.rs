use crate::libs::querypath::controllers::{
	traits::ApplyClaim,
	types::{DebugVerboses, GatherClaims},
};

impl DebugVerboses {
	#[must_use]
	pub fn new() -> Self { Self::default() }

	#[must_use]
	pub fn step1b_entrydir(mut self, enable: bool) -> Self {
		self.step1b_entrydir = enable;
		self
	}

	#[must_use]
	pub fn step1a_patterns(mut self, enable: bool) -> Self {
		self.step1a_patterns = enable;
		self
	}

	#[must_use]
	pub fn step1c_walk(mut self, enable: bool) -> Self {
		self.step1c_walk = enable;
		self
	}

	#[must_use]
	pub fn step1d_execute(mut self, enable: bool) -> Self {
		self.step1d_execute = enable;
		self
	}
}

impl ApplyClaim for DebugVerboses {
	fn apply(self, claims: &mut GatherClaims) { claims.verboses = self; }
}
