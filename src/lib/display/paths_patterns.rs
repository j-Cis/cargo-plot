use super::{BoolExt, DrawTree, Icon};
use crate::lib::logic::{PathsPatterns, PattExp, PattRaw};

impl std::fmt::Display for PattRaw {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Zwróć uwagę na .0, którym dobieramy się do wewnętrznego Vec<String>
		let raw_len = self.0.len();
		if raw_len == 0 {
			return write!(f, "{} {} Brak reguł wejściowych", DrawTree::ITEM_LAST, Icon::EMPTY);
		}

		let mut lines = Vec::new();
		for (i, pat) in self.0.iter().enumerate() {
			let prefix = DrawTree::list(i, raw_len);
			lines.push(format!("{} {}", prefix, pat));
		}
		write!(f, "{}", lines.join("\n"))
	}
}

impl std::fmt::Display for PattExp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let matchers_len = self.0.len();
		if matchers_len == 0 {
			return write!(f, "{} {} Brak aktywnych reguł", DrawTree::ITEM_LAST, Icon::EMPTY);
		}

		let mut lines = Vec::new();
		for (i, pattern_text) in self.0.iter().enumerate() {
			let prefix = DrawTree::list(i, matchers_len);
			let is_negated = pattern_text.starts_with('!');
			let rule_icon = (!is_negated).as_symbol();
			lines.push(format!("{} [{}] {}", prefix, rule_icon, pattern_text));
		}
		write!(f, "{}", lines.join("\n"))
	}
}

impl std::fmt::Display for PathsPatterns {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut lines = Vec::new();

		// Magia! Delegate Display do wewnętrznych typów:
		lines.push(format!("{}", self.patterns));
		lines.push(format!("   {}", Icon::EXPAND));
		lines.push(format!("{}", self.expanded));

		write!(f, "{}", lines.join("\n"))
	}
}
