use crate::libs::querypath::{
	controllers::{
		traits::ApplyClaim,
		types::{ClaimOrdering, GatherClaims},
	},
	core::types::enums::{
		EnumCasePrecedence,
		EnumCharClass,
		EnumGroupStrategy,
		EnumNoExtPriority,
		EnumSameNamePriority,
	},
};

// Ręczna implementacja Default rozwiązuje błąd E0277 dla enumów
impl Default for ClaimOrdering {
	fn default() -> Self {
		Self {
			order_group_strategy: EnumGroupStrategy::Mixed,
			order_same_name_priority: EnumSameNamePriority::DirectoryFirst,
			order_ignore_leading_dot: false,
			order_split_extension: true,
			order_no_ext_priority: EnumNoExtPriority::Above,
			order_char_class_order: [EnumCharClass::Special, EnumCharClass::Digit, EnumCharClass::Letter],
			order_case_precedence: EnumCasePrecedence::Insensitive,
			order_reverse: false,
			numerate_dirs: true,
			numerate_binaries: true,
			numerate_start_from: 1,
		}
	}
}

impl ClaimOrdering {
	#[must_use]
	pub fn new() -> Self { Self::default() }

	#[must_use]
	pub fn order_reverse(mut self, reverse: bool) -> Self {
		self.order_reverse = reverse;
		self
	}

	#[must_use]
	pub fn numerate_start_from(mut self, start: usize) -> Self {
		self.numerate_start_from = start;
		self
	}
}

impl ApplyClaim for ClaimOrdering {
	fn apply(self, claims: &mut GatherClaims) { claims.ordering = self; }
}
