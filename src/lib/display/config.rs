use super::{Color, Icon};
use crate::lib::logic::{ConfigJob, ConfigManifest};

impl std::fmt::Display for ConfigJob {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let name = self.name.as_deref().unwrap_or("Brak nazwy");
		let desc = self.description.as_deref().unwrap_or("Brak opisu");

		let quiet_status = if self.quiet_work { "🔇 (Quiet)" } else { "🔊 (Verbose)" };

		write!(f, "【ID】🔖 [{}] | {} | {}", Color::num(&self.id), Color::folder(name), quiet_status)?;
		write!(f, "\n      📜 {}", Color::border(desc))?;
		Ok(())
	}
}

impl std::fmt::Display for ConfigManifest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.job.is_empty() {
			return write!(f, "{} Brak zdefiniowanych zadań (jobs) w pliku konfiguracyjnym.", Icon::EMPTY);
		}

		let mut lines = Vec::new();
		lines.push("░".repeat(80));

		for job in &self.job {
			lines.push(job.to_string());
		}

		write!(f, "{}", lines.join("\n"))
	}
}
