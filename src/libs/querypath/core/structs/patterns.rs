//! Path: `src/libs/querypath/structs/patterns.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use anyhow::{Context, Result};

use crate::querypath::core::{
	structs::{PattExp, PattRaw, PatternCompiled},
	traits::PattEnvIndex,
};

/// Główna struktura zarządzająca dopasowywaniem wzorców do ścieżek.
#[derive(Debug, Clone)]
pub struct PathsPatterns {
	pub raw: PattRaw,
	pub expanded: PattExp,
	pub compiled: Vec<PatternCompiled>,
	pub case_sensitive: bool,
}

// ============================================================================
// IMPLEMENTACJA PATHS PATTERNS
// ============================================================================

impl PathsPatterns {
	/// Tworzy i kompiluje zbiór wzorców.
	/// Rozwija klamry `{a|b}` oraz przelicza reguły na wyrażenia regularne.
	pub fn build<I, S>(patterns: I, case_sensitive: bool) -> Result<Self>
	where
		I: IntoIterator<Item = S>,
		S: AsRef<str>, {
		let mut raw_vec = Vec::new();
		let mut expanded_vec = Vec::new();

		for p in patterns {
			let s: &str = p.as_ref();
			if s.trim().is_empty() {
				continue;
			}
			raw_vec.push(s.to_string());
			expanded_vec.extend(Self::expand_braces(s));
		}

		let mut compiled = Vec::with_capacity(expanded_vec.len());
		let ignore_case = !case_sensitive;
		for p in &expanded_vec {
			let rule =
				PatternCompiled::compile(p, ignore_case).with_context(|| format!("Błąd kompilacji wzorca: '{p}'"))?;
			compiled.push(rule);
		}

		Ok(Self { raw: PattRaw(raw_vec), expanded: PattExp(expanded_vec), compiled, case_sensitive })
	}

	/// Sprawdza, czy podana ścieżka spełnia zasady narzucone przez zbiór
	/// wzorców. Wzorce zanegowane (`!`) działają jako bezwarunkowe weto.
	#[must_use]
	pub fn is_match<E: PattEnvIndex>(&self, path: &str, env: &E) -> bool {
		if self.compiled.is_empty() {
			return true;
		}

		let mut has_positive: bool = false;
		let mut matched_positive: bool = false;

		for c in &self.compiled {
			if c.is_negated {
				if c.is_match(path, env) {
					return false;
				}
			} else {
				has_positive = true;
				if !matched_positive && c.is_match(path, env) {
					matched_positive = true;
				}
			}
		}

		if has_positive { matched_positive } else { true }
	}

	#[must_use]
	pub fn rules_count(&self) -> usize { self.compiled.len() }

	/// Rekurencyjnie rozwija alternatywy zaimplementowane w klamrach `{a|b}`.
	pub fn expand_braces(input: &str) -> Vec<String> {
		let Some(end) = input.find('}') else {
			return vec![input.to_string()];
		};

		let Some(sub) = input.get(..end) else {
			return vec![input.to_string()];
		};

		let Some(start) = sub.rfind('{') else {
			return vec![input.to_string()];
		};

		let (Some(prefix), Some(suffix), Some(options)) =
			(input.get(..start), input.get(end + 1..), input.get(start + 1..end))
		else {
			return vec![input.to_string()];
		};

		let mut result = Vec::new();
		for opt in options.split('|') {
			let merged = format!("{prefix}{opt}{suffix}");
			result.extend(Self::expand_braces(&merged));
		}
		result
	}
}
