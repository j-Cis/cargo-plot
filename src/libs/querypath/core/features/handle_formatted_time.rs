//! Path: `src/libs/querypath/features/handle_formatted_time.rs`

use crate::libs::querypath::core::features::VerboseDisplay;
use crate::libs::temporal::Temporal;
use anyhow::{Result, bail};
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandleFormattedTime {
	pub pattern_tree: &'static str,
	pub pattern_text: &'static str,
	pub pattern_name: &'static str,
}

impl Default for HandleFormattedTime {
	fn default() -> Self {
		Self {
			pattern_tree: "WYYY-WW-D:hh:mm:ss",
			pattern_text: "WYYY-WW-D hh:mm:ss",
			pattern_name: "WYYYWW-Dhhmm-ssttqq",
		}
	}
}

impl HandleFormattedTime {
	pub fn build(
		pattern_tree: &'static str,
		pattern_text: &'static str,
		pattern_name: &'static str,
		debug: bool,
	) -> Result<Self> {
		if pattern_tree.trim().is_empty() {
			bail!("Wzorzec czasu dla drzewa (pattern_tree) nie może być pusty.");
		}
		if pattern_text.trim().is_empty() {
			bail!("Wzorzec czasu dla tekstu (pattern_text) nie może być pusty.");
		}
		if pattern_name.trim().is_empty() {
			bail!("Wzorzec czasu dla nazwy pliku (pattern_name) nie może być pusty.");
		}

		let instance = Self { pattern_tree, pattern_text, pattern_name };

		if debug {
			println!("{}", instance.display_debug());
		}

		Ok(instance)
	}

	#[inline]
	pub fn format_tree(&self, timestamp_secs: u64) -> String {
		Temporal::format(timestamp_secs, self.pattern_tree)
	}

	#[inline]
	pub fn format_text(&self, timestamp_secs: u64) -> String {
		Temporal::format(timestamp_secs, self.pattern_text)
	}

	#[inline]
	pub fn format_name(&self, timestamp_secs: u64) -> String {
		Temporal::format(timestamp_secs, self.pattern_name)
	}

	#[inline]
	pub fn format_system_time_tree(&self, st: SystemTime) -> String {
		Temporal::format_system_time(st, self.pattern_tree)
	}

	#[inline]
	pub fn format_system_time_name(&self, st: SystemTime) -> String {
		Temporal::format_system_time(st, self.pattern_name)
	}
}

impl VerboseDisplay for HandleFormattedTime {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "00X📕 [ structs: HandleFormattedTime ] 🔰")?;
		writeln!(f, "  🌲 pattern_tree 🕒: {:?}", self.pattern_tree)?;
		writeln!(f, "  📄 pattern_text 🕒: {:?}", self.pattern_text)?;
		writeln!(f, "  🏷️  pattern_name 🕒: {:?}", self.pattern_name)?;
		writeln!(f, "00X📕 [ structs: HandleFormattedTime ] 🏁")?;
		Ok(())
	}
}
