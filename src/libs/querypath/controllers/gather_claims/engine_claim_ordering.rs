//! Path: `src/libs/querypath/controllers/gather_claims/engine_claim_ordering.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::cmp::Ordering;

use crate::libs::querypath::{
	controllers::{
		traits::ApplyClaim,
		types::{ClaimOrdering, GatherClaims},
	},
	core::{
		structs::PathNode,
		types::enums::{
			EnumCasePrecedence,
			EnumCharClass,
			EnumGroupStrategy,
			EnumNoExtPriority,
			EnumNodeGroup,
			EnumSameNamePriority,
		},
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
	pub fn order_group_strategy(mut self, strategy: EnumGroupStrategy) -> Self {
		self.order_group_strategy = strategy;
		self
	}

	#[must_use]
	pub fn order_same_name_priority(mut self, priority: EnumSameNamePriority) -> Self {
		self.order_same_name_priority = priority;
		self
	}

	#[must_use]
	pub fn order_ignore_leading_dot(mut self, ignore: bool) -> Self {
		self.order_ignore_leading_dot = ignore;
		self
	}

	#[must_use]
	pub fn order_split_extension(mut self, split: bool) -> Self {
		self.order_split_extension = split;
		self
	}

	#[must_use]
	pub fn order_no_ext_priority(mut self, priority: EnumNoExtPriority) -> Self {
		self.order_no_ext_priority = priority;
		self
	}

	#[must_use]
	pub fn order_char_class_order(mut self, order: [EnumCharClass; 3]) -> Self {
		self.order_char_class_order = order;
		self
	}

	#[must_use]
	pub fn order_case_precedence(mut self, precedence: EnumCasePrecedence) -> Self {
		self.order_case_precedence = precedence;
		self
	}

	#[must_use]
	pub fn order_reverse(mut self, reverse: bool) -> Self {
		self.order_reverse = reverse;
		self
	}

	#[must_use]
	pub fn numerate_dirs(mut self, numerate: bool) -> Self {
		self.numerate_dirs = numerate;
		self
	}

	#[must_use]
	pub fn numerate_binaries(mut self, numerate: bool) -> Self {
		self.numerate_binaries = numerate;
		self
	}

	#[must_use]
	pub fn numerate_start_from(mut self, start: usize) -> Self {
		self.numerate_start_from = start;
		self
	}

	// ------------------------------------------------------------------------
	// GŁÓWNY KOMPARATOR HIERARCHICZNY
	// ------------------------------------------------------------------------

	pub fn compare_nodes(&self, a: &PathNode, b: &PathNode) -> Ordering {
		let res = self.compare_paths_hierarchical(a, b);
		if self.order_reverse { res.reverse() } else { res }
	}

	fn compare_paths_hierarchical(&self, a: &PathNode, b: &PathNode) -> Ordering {
		let parts_a: Vec<&str> =
			a.str.trim_start_matches("./").trim_end_matches('/').split('/').filter(|s| !s.is_empty()).collect();

		let parts_b: Vec<&str> =
			b.str.trim_start_matches("./").trim_end_matches('/').split('/').filter(|s| !s.is_empty()).collect();

		let min_len = parts_a.len().min(parts_b.len());

		for i in 0..min_len {
			if parts_a[i] != parts_b[i] {
				let is_dir_a = if i < parts_a.len() - 1 { true } else { a.is_dir };
				let is_bin_a = if i < parts_a.len() - 1 { false } else { a.is_binary };

				let is_dir_b = if i < parts_b.len() - 1 { true } else { b.is_dir };
				let is_bin_b = if i < parts_b.len() - 1 { false } else { b.is_binary };

				return self.compare_segment(parts_a[i], is_dir_a, is_bin_a, parts_b[i], is_dir_b, is_bin_b);
			}
		}

		// Jeśli jedna ścieżka jest prefiksem drugiej (np. "./src/" i "./src/main.rs")
		if parts_a.len() != parts_b.len() {
			return parts_a.len().cmp(&parts_b.len());
		}

		// Identyczna ścieżka: katalog ma pierwszeństwo przed plikiem
		if a.is_dir != b.is_dir {
			return if a.is_dir { Ordering::Less } else { Ordering::Greater };
		}

		Ordering::Equal
	}

	fn compare_segment(
		&self,
		seg_a: &str,
		is_dir_a: bool,
		is_bin_a: bool,
		seg_b: &str,
		is_dir_b: bool,
		is_bin_b: bool,
	) -> Ordering {
		// 1. Grupy niestandardowe (EnumGroupStrategy::Custom)
		if let EnumGroupStrategy::Custom(ref group_order) = self.order_group_strategy {
			let group_a = self.node_group(is_dir_a, is_bin_a);
			let group_b = self.node_group(is_dir_b, is_bin_b);

			if group_a != group_b {
				let pos_a = group_order.iter().position(|g| *g == group_a).unwrap_or(usize::MAX);
				let pos_b = group_order.iter().position(|g| *g == group_b).unwrap_or(usize::MAX);
				return pos_a.cmp(&pos_b);
			}
		}

		// 2. Ignorowanie wiodącej kropki
		let clean_a = if self.order_ignore_leading_dot { seg_a.strip_prefix('.').unwrap_or(seg_a) } else { seg_a };

		let clean_b = if self.order_ignore_leading_dot { seg_b.strip_prefix('.').unwrap_or(seg_b) } else { seg_b };

		// 3. Rozdzielenie na nazwę bazową i rozszerzenie
		if self.order_split_extension {
			let (base_a, ext_a) = self.split_name_ext(clean_a, is_dir_a);
			let (base_b, ext_b) = self.split_name_ext(clean_b, is_dir_b);

			let base_cmp = self.compare_strings(&base_a, &base_b);
			if base_cmp != Ordering::Equal {
				return base_cmp;
			}

			// Rozstrzygnięcie konfliktu, gdy obie nazwy bazowe są równe, ale jedna to plik a druga to folder
			if is_dir_a != is_dir_b {
				return match self.order_same_name_priority {
					EnumSameNamePriority::DirectoryFirst => {
						if is_dir_a {
							Ordering::Less
						} else {
							Ordering::Greater
						}
					}
					EnumSameNamePriority::FileFirst => {
						if is_dir_a {
							Ordering::Greater
						} else {
							Ordering::Less
						}
					}
				};
			}

			// Porównanie rozszerzeń z uwzględnieniem no_ext_priority
			match (ext_a, ext_b) {
				(None, None) => Ordering::Equal,
				(None, Some(_)) => match self.order_no_ext_priority {
					EnumNoExtPriority::Above => Ordering::Less,
					EnumNoExtPriority::Below => Ordering::Greater,
				},
				(Some(_), None) => match self.order_no_ext_priority {
					EnumNoExtPriority::Above => Ordering::Greater,
					EnumNoExtPriority::Below => Ordering::Less,
				},
				(Some(e_a), Some(e_b)) => self.compare_strings(e_a, e_b),
			}
		} else {
			self.compare_strings(clean_a, clean_b)
		}
	}

	fn compare_strings(&self, s1: &str, s2: &str) -> Ordering {
		let chars1: Vec<char> = s1.chars().collect();
		let chars2: Vec<char> = s2.chars().collect();
		let min_len = chars1.len().min(chars2.len());

		for i in 0..min_len {
			let c1 = chars1[i];
			let c2 = chars2[i];

			if c1 == c2 {
				continue;
			}

			let class1 = self.classify_char(c1);
			let class2 = self.classify_char(c2);

			if class1 != class2 {
				let pos1 = self.order_char_class_order.iter().position(|c| *c == class1).unwrap_or(3);
				let pos2 = self.order_char_class_order.iter().position(|c| *c == class2).unwrap_or(3);
				return pos1.cmp(&pos2);
			}

			if class1 == EnumCharClass::Letter {
				let lower1 = c1.to_lowercase().next().unwrap_or(c1);
				let lower2 = c2.to_lowercase().next().unwrap_or(c2);

				if lower1 != lower2 {
					return lower1.cmp(&lower2);
				}

				match self.order_case_precedence {
					EnumCasePrecedence::Insensitive => continue,
					EnumCasePrecedence::UpperFirst => {
						if c1.is_uppercase() && c2.is_lowercase() {
							return Ordering::Less;
						} else if c1.is_lowercase() && c2.is_uppercase() {
							return Ordering::Greater;
						}
					}
					EnumCasePrecedence::LowerFirst => {
						if c1.is_lowercase() && c2.is_uppercase() {
							return Ordering::Less;
						} else if c1.is_uppercase() && c2.is_lowercase() {
							return Ordering::Greater;
						}
					}
				}
			} else {
				let char_cmp = c1.cmp(&c2);
				if char_cmp != Ordering::Equal {
					return char_cmp;
				}
			}
		}

		chars1.len().cmp(&chars2.len())
	}

	fn classify_char(&self, c: char) -> EnumCharClass {
		if c.is_ascii_digit() {
			EnumCharClass::Digit
		} else if c.is_alphabetic() {
			EnumCharClass::Letter
		} else {
			EnumCharClass::Special
		}
	}

	fn split_name_ext<'a>(&self, name: &'a str, is_dir: bool) -> (&'a str, Option<&'a str>) {
		if is_dir {
			return (name, None);
		}

		if let Some(pos) = name.rfind('.').filter(|&pos| pos > 0) {
			let (base, ext_with_dot) = name.split_at(pos);
			let ext = ext_with_dot.strip_prefix('.').unwrap_or(ext_with_dot);
			return (base, Some(ext));
		}

		(name, None)
	}

	fn node_group(&self, is_dir: bool, is_binary: bool) -> EnumNodeGroup {
		if is_dir {
			EnumNodeGroup::Directory
		} else if is_binary {
			EnumNodeGroup::BinaryFile
		} else {
			EnumNodeGroup::TextFile
		}
	}
}

impl ApplyClaim for ClaimOrdering {
	fn apply(self, claims: &mut GatherClaims) { claims.ordering = self; }
}
