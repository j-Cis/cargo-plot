use super::DrawTree;
use crate::lib::logic::{PathCanonicalCtx, PathNode};

impl std::fmt::Display for PathNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Domyślnie pokazujemy znormalizowany ciąg
		write!(f, "{}", self.str)
	}
}

impl std::fmt::Display for PathCanonicalCtx {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut lines = Vec::new();

		// 1. Sekcja CWD
		lines.push("🅰️ 📍 execut_dir (CWD)".to_string());
		lines.push(format!("{} buf: {}", DrawTree::ENTRY_BRANCH, self.execut_dir.buf.display()));
		lines.push(format!("{} str: {}", DrawTree::ENTRY_TERMINAL, self.execut_dir.str));

		// 2. Sekcja TARGET
		lines.push("🅱️ 🎯 select_dir (TARGET)".to_string());
		lines.push(format!("{} buf: {}", DrawTree::ENTRY_BRANCH, self.select_dir.buf.display()));
		lines.push(format!("{} str: {}", DrawTree::ENTRY_TERMINAL, self.select_dir.str));

		// 3. Sekcja RELATIVE
		lines.push("🆎🔗 relat_path (RELATIVE, BETWEEN CWD & TARGET)".to_string());
		lines.push(format!("{} str: {}", DrawTree::ENTRY_TERMINAL, self.relat_path));

		// Sklejamy wszystko znakiem nowej linii i wypisujemy zwykłym write!
		write!(f, "{}", lines.join("\n"))
	}
}
