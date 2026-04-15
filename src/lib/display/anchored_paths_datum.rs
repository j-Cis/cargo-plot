use super::DrawTree;
use crate::lib::logic::{AnchoredPathsDatum, PathNode};

impl std::fmt::Display for PathNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Domyślnie pokazujemy znormalizowany ciąg
		write!(f, "{}", self.str)
	}
}

impl std::fmt::Display for AnchoredPathsDatum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut lines = Vec::new();

		// 1. Sekcja CWD
		lines.push("🅰️ 📍 execut_dir (CWD)".to_string());
		lines.push(format!("{} buf: {}", DrawTree::ENTRY_BRANCH, self.execution_dir.buf.display()));
        lines.push(format!("{} str: {}", DrawTree::ENTRY_TERMINAL, self.execution_dir.str));

		// 2. Sekcja TARGET
		lines.push("🅱️ 🎯 select_dir (TARGET)".to_string());
		lines.push(format!("{} buf: {}", DrawTree::ENTRY_BRANCH, self.workspace_dir.buf.display()));
        lines.push(format!("{} str: {}", DrawTree::ENTRY_TERMINAL, self.workspace_dir.str));

		// 3. Sekcja RELATIVE
		lines.push("🆎🔗 relat_path (RELATIVE, BETWEEN CWD & TARGET)".to_string());
		lines.push(format!("{} str: {}", DrawTree::ENTRY_TERMINAL, self.to_relative_path()));

		// Sklejamy wszystko znakiem nowej linii i wypisujemy zwykłym write!
		write!(f, "{}", lines.join("\n"))
	}
}
