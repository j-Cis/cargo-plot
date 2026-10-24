//! Path: `src/libs/querypath/utils/normalize_path.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::path::Path;
// ============================================================================
// NORMALIZACJA ŚCIEŻEK
// ============================================================================

/// Normalizuje ścieżkę do ujednoliconego formatu tekstowego (POSIX-like).
/// - Zmienia `\` na `/`
/// - Usuwa Windowsowe prefiksy UNC (`\\?\`)
/// - Redukuje zduplikowane sekwencje `/./` do `/`
#[must_use]
pub fn normalize_path<P: AsRef<Path>>(p: P) -> String {
	p.as_ref().to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/").replace("/./", "/")
}
