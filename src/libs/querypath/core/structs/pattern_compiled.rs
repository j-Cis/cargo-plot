//! Path: `src/libs/querypath/structs/pattern_compiled.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use crate::querypath::core::traits::PattEnvIndex;
use anyhow::Result;
use regex::Regex;

// ============================================================================
// TYPY DOMENOWE
// ============================================================================

/// Silnie typowana lista surowych wzorców wejściowych.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PattRaw(pub Vec<String>);

/// Silnie typowana lista wzorców rozwiniętych po klamrach `{a|b}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PattExp(pub Vec<String>);

// ============================================================================
// SKOMPILOWANA REGUŁA (PATTERN COMPILED)
// ============================================================================

#[derive(Debug, Clone)]
pub struct PatternCompiled {
	pub regex: Regex,
	pub targets_dir_only: bool,
	pub requires_sibling: bool,
	pub requires_orphan: bool,
	pub base_name: String,
	pub is_negated: bool,
}

// ============================================================================
// KOMPILACJA I DOPASOWANIE REGUŁY
// ============================================================================

impl PatternCompiled {
	pub fn compile(pattern: &str, ignore_case: bool) -> Result<Self, regex::Error> {
		let is_negated: bool = pattern.starts_with('!');
		let mut p: &str = if is_negated { pattern.get(1..).unwrap_or("") } else { pattern };

		let mut requires_sibling: bool = false;
		let mut requires_orphan: bool = false;

		if p.starts_with('@') {
			requires_sibling = true;
			p = p.get(1..).unwrap_or("");
		} else if p.starts_with('$') {
			requires_orphan = true;
			p = p.get(1..).unwrap_or("");
		}

		let is_direct_regex: bool = p.starts_with("re:");
		if is_direct_regex {
			let raw_re: &str = p.get(3..).unwrap_or("");
			let re_str: String = if ignore_case { format!("(?i){raw_re}") } else { raw_re.to_string() };
			let regex = Regex::new(&re_str)?;
			return Ok(Self {
				regex,
				targets_dir_only: pattern.ends_with('/'),
				requires_sibling,
				requires_orphan,
				base_name: String::new(),
				is_negated,
			});
		}

		// Tylko jawnny slash '/' na końcu oznacza wzorzec wyłącznie dla
		// katalogów
		let targets_dir_only: bool = p.ends_with('/');

		let base_name: String =
			p.trim_end_matches('/').split('/').next_back().unwrap_or("").split('.').next().unwrap_or("").to_string();

		let mut re = String::new();
		if ignore_case {
			re.push_str("(?i)");
		}

		let mut anchored: bool = false;

		if p.starts_with("./") {
			anchored = true;
			p = p.get(2..).unwrap_or("");
		} else if p.starts_with("**/") {
			// Wzorzec **/ oznacza wyszukiwanie od dowolnego poziomu
			anchored = false;
			p = p.get(3..).unwrap_or("");
		}

		if anchored {
			re.push('^');
		} else {
			re.push_str("(?:^|/)");
		}

		let chars: Vec<char> = p.chars().collect();
		let mut i: usize = 0;

		while i < chars.len() {
			match chars[i] {
				'\\' => {
					i += 1;
					if i < chars.len() {
						re.push_str(&regex::escape(&chars[i].to_string()));
					}
				}
				'.' => re.push_str("\\."),
				'/' => re.push('/'),
				'*' => {
					if i + 1 < chars.len() && chars[i + 1] == '*' {
						// ** dopasowuje dowolny ciąg znaków (włącznie z /)
						re.push_str(".*");
						i += 1;
					} else {
						re.push_str("[^/]*");
					}
				}
				'?' => re.push_str("[^/]"),
				'[' => {
					re.push('[');
					if i + 1 < chars.len() && chars[i + 1] == '!' {
						re.push('^');
						i += 1;
					}
				}
				']' | '-' | '^' => re.push(chars[i]),
				c => re.push_str(&regex::escape(&c.to_string())),
			}
			i += 1;
		}

		re.push('$');

		Ok(
			Self {
				regex: Regex::new(&re)?,
				targets_dir_only,
				requires_sibling,
				requires_orphan,
				base_name,
				is_negated,
			},
		)
	}

	#[allow(clippy::bool_comparison)]
	pub fn is_match<E: PattEnvIndex>(&self, path: &str, env: &E) -> bool {
		let is_dir: bool = path.ends_with('/');
		let clean: &str = path.strip_prefix("./").unwrap_or(path);

		if self.targets_dir_only && is_dir == false {
			return false;
		}

		if self.regex.is_match(clean) == false {
			return false;
		}

		// Relacje rodzeństwa/sieroty dla plików
		if (self.requires_sibling || self.requires_orphan) && is_dir == false {
			let parent: &str = match clean.rsplit_once('/') {
				Some((p, _)) => p,
				std::option::Option::None => "",
			};

			let mut expected_folder = String::new();
			if !parent.is_empty() {
				expected_folder.push_str(parent);
				expected_folder.push('/');
			}
			expected_folder.push_str(&self.base_name);
			expected_folder.push('/');

			let exists: bool = env.has_dir(&expected_folder);

			if self.requires_sibling && exists == false {
				return false;
			}
			if self.requires_orphan && exists {
				return false;
			}
		}

		// Relacje rodzeństwa/sieroty dla katalogów
		if (self.requires_sibling || self.requires_orphan) && is_dir {
			let dir_no_slash: &str = path.trim_end_matches('/');
			let mut search_prefix = String::from(dir_no_slash);
			search_prefix.push('.');

			let has_file_sibling: bool = env.has_file_with_prefix(&search_prefix);

			if self.requires_sibling && has_file_sibling == false {
				return false;
			}
			if self.requires_orphan && has_file_sibling {
				return false;
			}
		}

		true
	}
}
