/// [POL]: Główny moduł logiki dopasowywania ścieżek.
/// [ENG]: Core module for path matching logic.
pub mod matcher;
pub mod matchers;
pub mod sort;
pub mod stats;

pub use self::matcher::PathMatcher;
pub use self::matchers::PathMatchers;
pub use self::sort::SortStrategy;
pub use self::stats::MatchStats;
