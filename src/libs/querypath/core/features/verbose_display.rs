//! Path: `src/libs/querypath/features/verbose_display.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::fmt;

pub trait VerboseDisplay {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

	/// Adapter pozwalający na wyświetlenie w `println!("{}",
	/// x.display_debug())`
	fn display_debug(&self) -> DebugViewer<'_, Self> { DebugViewer(self) }
}

pub struct DebugViewer<'a, T: ?Sized>(pub &'a T);

impl<'a, T: VerboseDisplay + ?Sized> fmt::Display for DebugViewer<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { VerboseDisplay::fmt(self.0, f) }
}
