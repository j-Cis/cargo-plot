// use super::Icon;
// use crate::lib::logic::{PartitionScanned, StatsScannedTreeFs};
//
// impl std::fmt::Display for StatsScannedTreeFs {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(
// 			f,
// 			"{} F: {} (Txt: {}, Bin: {}, Ø: {}) | {} D: {} (Ø: {})",
// 			Icon::FILE,
// 			self.count_files,
// 			self.count_files_text,
// 			self.count_files_binary,
// 			self.count_files_empty,
// 			Icon::FOLDER,
// 			self.count_dirs,
// 			self.count_dirs_empty
// 		)
// 	}
// }
//
// impl std::fmt::Display for PartitionScanned {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "{} {} | 📊 Stats -> {}", Icon::ENTRY, self.stat.where_scanned().str, self.stat)
// 	}
// }
