use super::{Color, DrawTree, Icon};
use crate::lib::logic::{ConfigJob, ConfigManifest};

impl std::fmt::Display for ConfigJob {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let name = self.name.as_deref().unwrap_or("Brak nazwy");
		let desc = self.description.as_deref().unwrap_or("Brak opisu");

		let quiet_status = if self.quiet_work { "🤫 Cichy (Quiet)" } else { "🔊 Głośny (Verbose)" };

		writeln!(f, "{} ID: [{}] | {}", Icon::ENTRY, Color::num(&self.id), Color::folder(name))?;
		writeln!(f, "{} {}", DrawTree::ENTRY_BRANCH, Color::border(desc))?;
		writeln!(
			f,
			"{} 🎯 Cel: {} | ⚙️ Tryb: {} | {}",
			DrawTree::ITEM_LAST,
			Color::file(&self.scan.work_path),
			Color::size(&self.spec.mode),
			Color::date(quiet_status)
		)
	}
}

impl std::fmt::Display for ConfigManifest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.job.is_empty() {
			return write!(f, "{} Brak zdefiniowanych zadań (jobs) w pliku konfiguracyjnym.", Icon::EMPTY);
		}

		let mut lines = Vec::new();
		lines.push(format!("{} Zdefiniowane zadania wsadowe (Łącznie: {}):", Icon::H2, self.job.len()));
		lines.push("░".repeat(80));

		for job in &self.job {
			lines.push(job.to_string());
		}

		lines.push("░".repeat(80));

		write!(f, "{}", lines.join("\n"))
	}
}
