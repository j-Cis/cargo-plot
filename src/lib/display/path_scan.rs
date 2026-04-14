use super::Icon;
use crate::lib::logic::{PathScan, PathScanStat}; // Zwróć uwagę, że usunąłem DrawTree, bo nie jest tu już potrzebne

impl std::fmt::Display for PathScanStat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} {} | {} {} | {} {} | {} {}",
			Icon::ENTRY,
			self.relation.select_dir.buf.display(),
			Icon::FILE,
			self.count_files,
			Icon::FOLDER,
			self.count_folder,
			Icon::EMPTY,
			self.count_empty
		)
	}
}

impl std::fmt::Display for PathScan {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Domyślny format dla całego skanera to po prostu czytelne statystyki!
		write!(f, "📊 Stats -> {}", self.stat)
	}
}
