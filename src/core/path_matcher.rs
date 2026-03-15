/// [POL]: Główny moduł logiki dopasowywania ścieżek.
/// [ENG]: Core module for path matching logic.
pub mod matcher;
pub mod matcher_utils;
pub mod matchers;

pub use self::matcher::PathMatcher;
pub use self::matcher_utils::{expand_braces,SortStrategy,sort_paths};
pub use self::matchers::PathMatchers;
