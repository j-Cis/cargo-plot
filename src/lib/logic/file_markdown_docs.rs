use std::{
	fs,
	path::{Path, PathBuf},
};

use console::strip_ansi_codes;

#[allow(clippy::upper_case_acronyms)]
enum ScOrCs {
	SOTC,
	COTS,
}

use crate::lib::logic::{LangMapper, PathNode, TableOutput, TagTime};

pub struct DocMarkdown {
	cwd: PathNode,
	wrk: PathNode,
	table: TableOutput,
	timetag: TagTime,
	content: String,
}

impl DocMarkdown {
	pub fn new(content: impl Into<String>, table: TableOutput, cwd: PathNode, wrk: PathNode, timetag: TagTime) -> Self {
		let raw = content.into();

		let clean_content = strip_ansi_codes(&raw).into_owned().replace('\t', "    ");

		Self { cwd, wrk, table, timetag, content: clean_content }
	}

	// ============================================================================
	// BUILD & RENDER
	// ============================================================================

	fn structure_of_the_content_render(&self, title: Option<&str>) -> String {
		let title_str = title.map_or(String::new(), |t| format!("{} ", t));
		format!(
			"# {}(STRUCTURE OF THE CONTENT v:{})\n\n```plaintext\n{}\n```\n",
			title_str, self.timetag.0, self.content
		)
	}

	fn content_of_the_structure_render(&self, code_blocks: &str, title: Option<&str>) -> String {
		let title_str = title.map_or(String::new(), |t| format!("{} ", t));
		format!(
			"# {}(CONTENT OF THE STRUCTURE v:{})\n\n```plaintext\n{}\n```\n\n{}",
			title_str, self.timetag.0, self.content, code_blocks
		)
	}

	pub fn content_of_the_structure_build(&self, target_dir: &Path) -> String {
		let mut content = String::new();

		let (rows_to_show, index_offset) = if let Some(size) = self.table.trim_size {
			let start = self.table.trim_page.saturating_sub(1) * size;
			(self.table.data.rows.iter().skip(start).take(size).collect::<Vec<_>>(), start)
		} else {
			(self.table.data.rows.iter().collect::<Vec<_>>(), 0)
		};

		for (i, row) in rows_to_show.iter().enumerate() {
			let actual_idx = index_offset + i + 1;
			let p_str = &row.path;

			if p_str.ends_with('/') {
				continue;
			}

			let clean_rel = p_str.strip_prefix("./").unwrap_or(p_str);
			let absolute_path = target_dir.join(clean_rel);

			let ext = absolute_path.extension().unwrap_or_default().to_string_lossy().to_string();

			let lang = LangMapper::get_md_lang(&ext);

			if LangMapper::is_blacklisted(&ext) {
				content.push_str(&format!(
					"## {:03}: `{}`\n\n> *(Plik binarny/graficzny - pominięto zawartość)*\n\n",
					actual_idx, p_str
				));
				continue;
			}

			match fs::read_to_string(&absolute_path) {
				Ok(file_content) => {
					let safe_content = file_content.replace('\t', "    ");
					content.push_str(&format!(
						"## {:03}: `{}`\n\n```{}\n{}\n```\n\n",
						actual_idx, p_str, lang, safe_content
					));
				}
				Err(_) => {
					content.push_str(&format!(
						"## {:03}: `{}`\n\n> *(Błąd odczytu / plik nie jest UTF-8)*\n\n",
						actual_idx, p_str
					));
				}
			}
		}

		content
	}

	// ============================================================================
	// SAVE AS
	// ============================================================================

	pub fn structure_of_the_content_save_as(&self, relpath: &str, title: Option<&str>) -> std::io::Result<()> {
		let file_path = self.path_build(relpath, ScOrCs::SOTC)?;
		let content = self.structure_of_the_content_render(title);

		Self::fs_write(&file_path, content)?;
		Ok(())
	}

	pub fn content_of_the_structure_save_as(&self, relpath: &str, title: Option<&str>) -> std::io::Result<()> {
		let file_path = self.path_build(relpath, ScOrCs::COTS)?;
		let code_blocks = self.content_of_the_structure_build(&self.wrk.buf);
		let content = self.content_of_the_structure_render(&code_blocks, title);

		Self::fs_write(&file_path, content)?;
		Ok(())
	}

	// ============================================================================
	// UTILS "DRY"
	// ============================================================================

	fn fs_write(file_path: &PathBuf, content: String) -> std::io::Result<()> {
		fs::write(file_path, content)?;
		println!("📦 Zapisano archiwum kodu do: {}", file_path.display());
		Ok(())
	}

	fn path_build(&self, relpath: &str, suffix: ScOrCs) -> std::io::Result<PathBuf> {
		let base_dir_path: PathBuf = self.cwd.buf.join(relpath);

		let stem = base_dir_path.file_stem().unwrap_or_else(|| std::ffi::OsStr::new("output")).to_string_lossy();

		let suffix_str = match suffix {
			ScOrCs::COTS => "COTS",
			ScOrCs::SOTC => "SOTC",
		};

		let file_name = format!("{}_{}_{}.md", stem, self.timetag.0, suffix_str);

		let file_path = base_dir_path.with_file_name(file_name);

		if let Some(parent) = file_path.parent() {
			std::fs::create_dir_all(parent)?;
		}

		Ok(file_path)
	}
}
