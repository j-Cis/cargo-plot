use crate::lib::job::{self};

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// PRZETWARZANIE PARAMETRY NA FALAGI
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

pub trait TraitConfigToFlags {
	/// Przekształca zwalidowaną konfigurację z powrotem na listę flag CLI
	fn to_flags(&self) -> Vec<String>;
}
impl TraitConfigToFlags for job::schema::ValidExecutionParams {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![];
		if self.quiet {
			f.push("quiet".to_string());
		}
		if self.mute {
			f.push("mute".to_string());
		}
		if self.only_dry {
			f.push("dry".to_string());
		}
		if self.debug {
			f.push("verbose".to_string());
		}
		if self.save_as {
			f.push("save".to_string());
		}
		if self.cli_color {
			f.push("color".to_string());
		}
		f
	}
}
impl TraitConfigToFlags for job::schema::ValidSaveAsParams {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![];
		f.push(format!("outdir={}", self.out_raw));
		f.push(format!("title={}", self.title));

		if !self.name.is_empty() {
			f.push(format!("name={}", self.name));
		}
		if self.name_is_prefix {
			f.push("name-prefix".to_string());
		} else {
			f.push("name-suffix".to_string());
		}
		if self.not_separately {
            f.push("not-separately".to_string());
        }
		f
	}
}
impl TraitConfigToFlags for job::schema::ValidTablePartParams {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![];
		if self.md {
			f.push("MD".to_string());
		}
		if self.mf {
			f.push("MF".to_string());
		}
		if self.xd {
			f.push("XD".to_string());
		}
		if self.xf {
			f.push("XF".to_string());
		}
		f
	}
}
impl TraitConfigToFlags for job::schema::ValidColumnItemParams {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![];
		match self.list {
			job::schema::ModeListForValidColumnItem::Tree => f.push("list-tree".to_string()),
			job::schema::ModeListForValidColumnItem::Flat => f.push("list-flat".to_string()),
			job::schema::ModeListForValidColumnItem::None => f.push("list-none".to_string()),
		}
		match self.icons {
			job::schema::ModeIconsForValidColumnItem::Lite => f.push("icons-lite".to_string()),
			job::schema::ModeIconsForValidColumnItem::More => f.push("icons-more".to_string()),
			job::schema::ModeIconsForValidColumnItem::None => f.push("icons-none".to_string()),
		}
		if self.name {
			f.push("name-show".to_string());
		} else {
			f.push("name-none".to_string());
		}
		if self.align_end {
			f.push("ws-show".to_string());
		} else {
			f.push("ws-none".to_string());
		}
		if self.num_is_first {
			f.push("num-prefix".to_string());
		} else {
			f.push("num-suffix".to_string());
		}
		f
	}
}
impl TraitConfigToFlags for job::schema::ValidTableColumnsParams {
	fn to_flags(&self) -> Vec<String> {
		self.columns
			.iter()
			.map(|c| {
				match c {
					job::schema::ValidTableColumnsFlags::Date => "date",
					job::schema::ValidTableColumnsFlags::Time => "time",
					job::schema::ValidTableColumnsFlags::Size => "size",
					job::schema::ValidTableColumnsFlags::Item => "item",
					job::schema::ValidTableColumnsFlags::Path => "path",
				}
				.to_string()
			})
			.collect()
	}
}
impl TraitConfigToFlags for job::schema::ValidColumnSizeParams {
	fn to_flags(&self) -> Vec<String> {
		match self.mode {
			job::schema::ValidColumnSizeFlags::Decimal => vec!["dec".to_string()],
			job::schema::ValidColumnSizeFlags::Binary => vec!["bin".to_string()],
		}
	}
}
impl TraitConfigToFlags for job::schema::ValidColumnDateParams {
	fn to_flags(&self) -> Vec<String> {
		// Ponieważ aktualnie wspieramy tylko tryb "default", zawsze to zwracamy.
		// Jeśli w przyszłości dodasz inne formaty, trzeba będzie tu dać `match`.
		vec!["default".to_string()]
	}
}
impl TraitConfigToFlags for job::schema::ValidColumnTimeParams {
	fn to_flags(&self) -> Vec<String> { vec!["default".to_string()] }
}
impl TraitConfigToFlags for job::schema::ValidPatternParams {
	fn to_flags(&self) -> Vec<String> { self.patterns.clone() }
}
impl TraitConfigToFlags for job::schema::ValidWorkspaceParams {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![self.workspace_raw.clone()];
		if self.ignore_case {
			f.push("ignore-case".to_string());
		}
		f
	}
}
impl TraitConfigToFlags for job::schema::ValidSortByParams {
	fn to_flags(&self) -> Vec<String> {
		let mut f = vec![];
		match &self.strategy {
			job::schema::StrategyForValidSortBy::Date { reverse } => {
				f.push("date".to_string());
				if *reverse {
					f.push("rev".to_string());
				}
			}
			job::schema::StrategyForValidSortBy::Size { reverse } => {
				f.push("size".to_string());
				if *reverse {
					f.push("rev".to_string());
				}
			}
			job::schema::StrategyForValidSortBy::Name { mode, reverse, file_group } => {
				f.push("name".to_string());
				f.push(mode.clone());
				if *reverse {
					f.push("rev".to_string());
				}
				match file_group {
					job::schema::ModeFileGroupForValidSortBy::Name => f.push("group-name".to_string()),
					job::schema::ModeFileGroupForValidSortBy::Exte => f.push("group-exte".to_string()),
					job::schema::ModeFileGroupForValidSortBy::None => f.push("group-none".to_string()),
				}
			}
			job::schema::StrategyForValidSortBy::Path { mode, reverse, dir_split, file_group } => {
				f.push("path".to_string());
				f.push(mode.clone());
				if *reverse {
					f.push("rev".to_string());
				}
				if *dir_split {
					f.push("dir-split-true".to_string());
				} else {
					f.push("dir-split-false".to_string());
				}
				match file_group {
					job::schema::ModeFileGroupForValidSortBy::Name => f.push("group-name".to_string()),
					job::schema::ModeFileGroupForValidSortBy::Exte => f.push("group-exte".to_string()),
					job::schema::ModeFileGroupForValidSortBy::None => f.push("group-none".to_string()),
				}
			}
		}
		f
	}
}
