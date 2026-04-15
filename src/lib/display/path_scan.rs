use super::Icon;
use crate::lib::logic::{ScanPathStat, ScannedToApply}; // Zwróć uwagę, że usunąłem DrawTree, bo nie jest tu już potrzebne

impl std::fmt::Display for ScanPathStat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} {} | {} {} | {} {} | {} {}",
			Icon::ENTRY,
			self.relation.workspace_dir.buf.display(),
			Icon::FILE,
			self.count_files,
			Icon::FOLDER,
			self.count_folder,
			Icon::EMPTY,
			self.count_empty
		)
	}
}

impl std::fmt::Display for ScannedToApply {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Domyślny format dla całego skanera to po prostu czytelne statystyki!
		write!(f, "📊 Stats -> {}", self.stat)
	}
}
