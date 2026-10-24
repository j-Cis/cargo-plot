use crate::libs::querypath::controllers::{
	traits::ApplyClaim,
	types::{ClaimPatterns, GatherClaims},
};

impl ClaimPatterns {
	#[must_use]
	pub fn new() -> Self { Self::default() }

	#[must_use]
	pub fn entry_dirs<I, S>(mut self, dirs: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>, {
		self.entry_dirs = dirs.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn patterns_all_matched<I, S>(mut self, patterns: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>, {
		self.patterns_all_matched = patterns.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn patterns_no_numerate<I, S>(mut self, patterns: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>, {
		self.patterns_no_numerate = patterns.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn patterns_no_copypast<I, S>(mut self, patterns: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>, {
		self.patterns_no_copypast = patterns.into_iter().map(Into::into).collect();
		self
	}

	#[must_use]
	pub fn case_sensitive(mut self, sensitive: bool) -> Self {
		self.case_sensitive = sensitive;
		self
	}

	#[must_use]
	pub fn entry_dirs_ancestors(mut self, ancestors: bool) -> Self {
		self.entry_dirs_ancestors = ancestors;
		self
	}
}

impl ApplyClaim for ClaimPatterns {
	fn apply(self, claims: &mut GatherClaims) { claims.patterns = self; }
}
