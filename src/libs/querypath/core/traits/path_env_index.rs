//! Path: `src/libs/querypath/traits/path_env_index.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Trait umożliwiający bez-dyskowe (zero-I/O) sprawdzanie obecności
/// plików i katalogów w pamięci podczas weryfikacji relacji `@` oraz `$`.
pub trait PattEnvIndex {
	fn has_dir(&self, dir: &str) -> bool;
	fn has_file_with_prefix(&self, prefix: &str) -> bool;
	fn any_file_in_dir(&self, dir: &str, check: &mut dyn FnMut(&str) -> bool) -> bool;
}
