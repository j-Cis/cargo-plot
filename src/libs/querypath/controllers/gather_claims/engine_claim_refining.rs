use crate::libs::querypath::controllers::{
	traits::ApplyClaim,
	types::{ClaimRefining, GatherClaims},
};

impl Default for ClaimRefining {
	fn default() -> Self {
		Self {
			skip_doted_dirs_empty: false,
			skip_nodot_dirs_empty: false,
			skip_doted_dirs_laden: false,
			skip_nodot_dirs_laden: false,
			skip_doted_noext_files_empty: false,
			skip_doted_withext_files_empty: false,
			skip_nodoted_noext_files_empty: false,
			skip_nodoted_withext_files_empty: false,
			skip_doted_noext_files_binary: false,
			skip_doted_withext_files_binary: false,
			skip_nodoted_noext_files_binary: false,
			skip_nodoted_withext_files_binary: false,
			skip_doted_noext_files_text: false,
			skip_doted_withext_files_text: false,
			skip_nodoted_noext_files_text: false,
			skip_nodoted_withext_files_text: false,
		}
	}
}

impl ClaimRefining {
	#[must_use]
	pub fn new() -> Self { Self::default() }

	#[must_use]
	pub fn skip_doted_dirs_empty(mut self, val: bool) -> Self {
		self.skip_doted_dirs_empty = val;
		self
	}

	#[must_use]
	pub fn skip_nodot_dirs_empty(mut self, val: bool) -> Self {
		self.skip_nodot_dirs_empty = val;
		self
	}

	#[must_use]
	pub fn skip_doted_dirs_laden(mut self, val: bool) -> Self {
		self.skip_doted_dirs_laden = val;
		self
	}

	#[must_use]
	pub fn skip_nodot_dirs_laden(mut self, val: bool) -> Self {
		self.skip_nodot_dirs_laden = val;
		self
	}

	#[must_use]
	pub fn skip_doted_noext_files_empty(mut self, val: bool) -> Self {
		self.skip_doted_noext_files_empty = val;
		self
	}

	#[must_use]
	pub fn skip_doted_withext_files_empty(mut self, val: bool) -> Self {
		self.skip_doted_withext_files_empty = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_noext_files_empty(mut self, val: bool) -> Self {
		self.skip_nodoted_noext_files_empty = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_withext_files_empty(mut self, val: bool) -> Self {
		self.skip_nodoted_withext_files_empty = val;
		self
	}

	#[must_use]
	pub fn skip_doted_noext_files_binary(mut self, val: bool) -> Self {
		self.skip_doted_noext_files_binary = val;
		self
	}

	#[must_use]
	pub fn skip_doted_withext_files_binary(mut self, val: bool) -> Self {
		self.skip_doted_withext_files_binary = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_noext_files_binary(mut self, val: bool) -> Self {
		self.skip_nodoted_noext_files_binary = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_withext_files_binary(mut self, val: bool) -> Self {
		self.skip_nodoted_withext_files_binary = val;
		self
	}

	#[must_use]
	pub fn skip_doted_noext_files_text(mut self, val: bool) -> Self {
		self.skip_doted_noext_files_text = val;
		self
	}

	#[must_use]
	pub fn skip_doted_withext_files_text(mut self, val: bool) -> Self {
		self.skip_doted_withext_files_text = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_noext_files_text(mut self, val: bool) -> Self {
		self.skip_nodoted_noext_files_text = val;
		self
	}

	#[must_use]
	pub fn skip_nodoted_withext_files_text(mut self, val: bool) -> Self {
		self.skip_nodoted_withext_files_text = val;
		self
	}
}

impl ApplyClaim for ClaimRefining {
	fn apply(self, claims: &mut GatherClaims) { claims.refining = self; }
}
