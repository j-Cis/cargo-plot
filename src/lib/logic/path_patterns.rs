use regex::Regex;

use crate::lib::logic::PathContext;

// ============================================================================
// ENV ABSTRAKCJA (ODCIĘCIE OD FS / STRUKTUR KOLEKCJI)
// ============================================================================

pub trait PattEnvIndex {
	fn has_dir(&self, dir: &str) -> bool;
	fn has_file_with_prefix(&self, prefix: &str) -> bool;
	fn any_file_in_dir(&self, dir: &str, check: &mut dyn FnMut(&str) -> bool) -> bool;
}

// ============================================================================
// PATTERNS (PUBLIC API)
// ============================================================================

/// [DOMENA]: Silnie typowana lista surowych wzorców
#[derive(Debug, Clone)]
pub struct PattRaw(pub Vec<String>);

/// [DOMENA]: Silnie typowana lista rozwiniętych wzorców
#[derive(Debug, Clone)]
pub struct PattExp(pub Vec<String>);

#[derive(Debug, Clone)]
pub struct PatternsQueries {
	pub patterns: PattRaw,
	pub expanded: PattExp,
	compiled: Vec<PatternCompiled>,
}

// ============================================================================
// COMPILED RULE
// ============================================================================

#[derive(Debug, Clone)]
struct PatternCompiled {
	regex: Regex,
	targets_file: bool,
	requires_sibling: bool,
	requires_orphan: bool,
	is_deep: bool,
	include_parents: bool,
	base_name: String,
	pub is_negated: bool,
	#[allow(dead_code)]
	pub original: String,
}

// ============================================================================
// PATTERNS IMPLEMENTATION
// ============================================================================

impl PatternsQueries {
	pub fn new<I, S>(patterns: I, ignore_case_sensitive: bool) -> Self
	where
		I: IntoIterator<Item = S>,
		S: AsRef<str>, {
		let mut patterns_vec = Vec::new();
		let mut expanded_vec = Vec::new();

		for p in patterns {
			let s = p.as_ref();
			patterns_vec.push(s.to_string());
			expanded_vec.extend(Self::expand_braces(s));
		}

		let compiled = expanded_vec.iter().map(|p| PatternCompiled::new(p, ignore_case_sensitive).unwrap()).collect();

		Self {
			// Pakujemy wektory w nasze nowe typy domenowe!
			patterns: PattRaw(patterns_vec),
			expanded: PattExp(expanded_vec),
			compiled,
		}
	}

	/// public API
	pub fn is_match<E: PattEnvIndex>(&self, path: &str, env: &E) -> bool {
		let mut has_positive = false;
		let mut matched = false;

		for c in &self.compiled {
			if c.is_negated {
				if c.is_match(path, env) {
					return false;
				}
			} else {
				has_positive = true;
				if !matched && c.is_match(path, env) {
					matched = true;
				}
			}
		}

		if has_positive { matched } else { true }
	}

	pub fn rules_count(&self) -> usize { self.compiled.len() }

	// =========================================================================
	// BRACE EXPANSION (DRY, REKURENCYJNE {{a,b},c} - znajdź najgłębsze)
	// =========================================================================

	fn expand_braces(input: &str) -> Vec<String> {
		// Szukamy pierwszej zamykającej klamry
		if let Some(end) = input.find('}') {
			// Szukamy otwierającej klamry najbliżej tej zamykającej (najgłębszy poziom)
			if let Some(start) = input[..end].rfind('{') {
				let prefix = &input[..start];
				let suffix = &input[end + 1..];
				let options = &input[start + 1..end];

				let mut result = Vec::new();
				for opt in options.split(',') {
					let merged = format!("{}{}{}", prefix, opt, suffix);
					// Rekurencja dla kolejnych klamer
					result.extend(Self::expand_braces(&merged));
				}
				return result;
			}
		}
		vec![input.to_string()]
	}
}

// ============================================================================
// COMPILATION & MATCHING LOGIC
// ============================================================================

impl PatternCompiled {
	pub fn new(pattern: &str, ignore_case_sensitive: bool) -> Result<Self, regex::Error> {
		let is_negated = pattern.starts_with('!');
		let p = if is_negated { &pattern[1..] } else { pattern };

		let is_deep = p.ends_with('+');
		let include_parents = p.ends_with("&/") || p.ends_with("&\\");
		let requires_sibling = p.contains('@');
		let requires_orphan = p.contains('$');

		let mut clean = p.replace(['@', '$', '+'], "");

		if include_parents {
			clean = clean.strip_suffix("&/").or_else(|| clean.strip_suffix("&\\")).unwrap_or(&clean).to_string();
		}

		// 🧠 ŹLE DZIAŁA FLAGA `+` np w "./dist/+"
		// ⚡ KRYTYCZNA ZMIANA: Obliczamy targets_file ZANIM utniemy ukośnik!
		// let targets_file = !clean.ends_with('/') && !clean.ends_with("**");
		// // ⚡ Usuwamy wiszący ukośnik, aby uniknąć podwójnego slasha `//` w regexie
		// if is_deep && clean.ends_with('/') {
		//     clean.pop();
		// }

		let base_name = clean
			.trim_end_matches('/')
			.split('/')
			.next_back()
			.unwrap_or("")
			.split('.')
			.next()
			.unwrap_or("")
			.to_string();

		let mut re = String::new();

		if ignore_case_sensitive {
			re.push_str("(?i)");
		}

		let mut p_str = clean.as_str();
		let mut anchored = false;

		let targets_file = !p_str.ends_with('/') && !p_str.ends_with("**");

		if p_str.starts_with("./") {
			anchored = true;
			p_str = &p_str[2..];
		} else if p_str.starts_with("**/") {
			anchored = true;
		}

		if anchored {
			re.push('^');
		} else {
			re.push_str("(?:^|/)");
		}

		let chars: Vec<char> = p_str.chars().collect();
		let mut i = 0;

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
						re.push_str(".+");
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
				// Klamry są już obsłużone przez expand_braces!
				_ => re.push_str(&regex::escape(&chars[i].to_string())),
			}
			i += 1;
		}

		if is_deep {
			re.push_str("(?:/.*)?$");
		} else {
			re.push('$');
		}

		Ok(Self {
			regex: Regex::new(&re)?,
			targets_file,
			requires_sibling,
			requires_orphan,
			is_deep,
			include_parents,
			base_name,
			is_negated,
			original: pattern.to_string(),
		})
	}

	pub fn is_match<E: PattEnvIndex>(&self, path: &str, env: &E) -> bool {
		let is_dir = path.ends_with('/');
		let clean = path.strip_prefix("./").unwrap_or(path);

		// 1. Ochrona rodziców (Flaga &/)
		if self.targets_file && is_dir {
			if self.include_parents {
				return env.any_file_in_dir(path, &mut |p| self.regex.is_match(p.strip_prefix("./").unwrap_or(p)));
			}
			return false;
		}

		if !self.regex.is_match(clean) {
			if is_dir && self.include_parents {
				return env.any_file_in_dir(path, &mut |p| self.regex.is_match(p.strip_prefix("./").unwrap_or(p)));
			}
			return false;
		}

		// 2. Relacje dla plików (@ i $)
		if (self.requires_sibling || self.requires_orphan) && !is_dir {
			if self.is_deep && self.requires_sibling {
				if !self.check_authorized_root(path, env) {
					return false;
				}
				return true;
			}

			let ctx = PathContext::from(path);
			let expected_folder = if ctx.parent.is_empty() {
				format!("{}/", self.base_name)
			} else {
				format!("{}/{}/", ctx.parent, self.base_name)
			};

			let exists = env.has_dir(&expected_folder);

			if self.requires_sibling && !exists {
				return false;
			}
			if self.requires_orphan && exists {
				return false;
			}
		}

		// 3. Relacje dla folderów (@ i $)
		if (self.requires_sibling || self.requires_orphan) && is_dir {
			if self.is_deep && self.requires_sibling {
				if !self.check_authorized_root(path, env) {
					return false;
				}
			} else {
				let dir_no_slash = path.trim_end_matches('/');
				let search_prefix = format!("{}.", dir_no_slash);
				let has_file_sibling = env.has_file_with_prefix(&search_prefix);

				if self.requires_sibling && !has_file_sibling {
					return false;
				}
				if self.requires_orphan && has_file_sibling {
					return false;
				}
			}
		}

		true
	}

	fn check_authorized_root<E: PattEnvIndex>(&self, path: &str, env: &E) -> bool {
		let clean = path.strip_prefix("./").unwrap_or(path);
		let components: Vec<&str> = clean.split('/').collect();

		for i in 0..components.len() {
			let comp_core = components[i].split('.').next().unwrap_or("");

			if comp_core == self.base_name {
				let base_dir = if i == 0 {
					self.base_name.clone()
				} else {
					format!("{}/{}", components[0..i].join("/"), self.base_name)
				};

				let full_base_dir = if path.starts_with("./") { format!("./{}", base_dir) } else { base_dir };
				let dir_path = format!("{}/", full_base_dir);

				let has_dir = env.has_dir(&dir_path);
				let search_prefix = format!("{}.", full_base_dir);
				let has_file = env.has_file_with_prefix(&search_prefix);

				if has_dir && has_file {
					return true;
				}
			}
		}
		false
	}
}
