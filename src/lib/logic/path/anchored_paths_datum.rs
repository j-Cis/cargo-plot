use std::{
	env,
	ffi::OsStr,
	fs,
	io,
	path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use console::strip_ansi_codes;

use crate::lib::logic::{LangMapper, TableSotcTreeOutput, TagTime, tag_time};

/// Reprezentacja pojedynczego węzła ścieżki (buf + string)
#[derive(Debug, Clone)]
pub struct PathNode {
	pub buf: PathBuf,
	pub str: String,
}

fn normalize_path<P: AsRef<Path>>(p: P) -> String {
	p.as_ref().to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/")
}

impl PathNode {
	pub fn new(buf: PathBuf) -> Self {
		let str = normalize_path(&buf);
		Self { buf, str }
	}
}

/// Relacja między katalogiem wykonania a katalogiem docelowym
#[derive(Debug, Clone)]
pub struct AnchoredPathsDatum {
	pub execution_dir: PathNode,
	pub workspace_dir: PathNode,
}

impl AnchoredPathsDatum {
	pub fn new<P: AsRef<Path>>(input: P) -> Result<Self> {
		let input = input.as_ref();

		let execution_dir_buf = env::current_dir().context("Nie można odczytać katalogu roboczego (CWD)")?;

		let workspace_dir_buf = fs::canonicalize(input)
			.with_context(|| format!("Nie można ustalić ścieżki '{}'", input.to_string_lossy()))?;

		Ok(Self { execution_dir: PathNode::new(execution_dir_buf), workspace_dir: PathNode::new(workspace_dir_buf) })
	}

	/// Metoda obliczająca względną ścieżkę między execution_dir a workspace_dir.
	pub fn to_relative_path(&self) -> String {
		match self.workspace_dir.buf.strip_prefix(&self.execution_dir.buf) {
			Ok(rel) => {
				let s = rel.to_string_lossy().replace('\\', "/");
				if s.is_empty() { "./".to_string() } else { format!("./{}/", s) }
			}
			Err(_) => normalize_path(&self.workspace_dir.buf),
		}
	}
	/// Prywatna metoda pomocnicza: Wyznacza fizyczną lokalizację dla pliku raportu i gwarantuje istnienie folderów nadrzędnych.
	fn build_output_file_path(&self, relpath: &str, time_tag: String, suffix: &str) -> io::Result<PathBuf> {
		// Wszystkie raporty lądują relatywnie do execution_dir
		let base_dir_path = self.execution_dir.buf.join(relpath);

		let stem = base_dir_path.file_stem().unwrap_or_else(|| OsStr::new("output")).to_string_lossy();

		// Składamy pełną nazwę pliku w jednym kroku
		let file_name = format!("{}_{}_{}.md", stem, time_tag, suffix);
		let file_path = base_dir_path.with_file_name(file_name);

		if let Some(parent) = file_path.parent() {
			fs::create_dir_all(parent)?;
		}

		Ok(file_path)
	}

	/// Buduje ścieżkę dla dokumentu SOTC (Structure of the content) - i zapisuje
	pub fn save_sotc_tree(&self, relpath: &str, title: Option<&str>, table_sotc_tree_cli: String) -> io::Result<()> {
		let tt: TagTime = tag_time();
		let file_path = self.build_output_file_path(relpath, tt.0.clone(), "SOTC")?;
		let title_str = title.map_or(String::new(), |t| format!("{} ", t));
		let content = format!(
			"# {}(STRUCTURE OF THE CONTENT v:{})\n{}",
			title_str,
			tt.0,
			Self::md_plaintext(table_sotc_tree_cli)
		);
		//
		Self::fs_write(&file_path, content)?;
		Ok(())
	}

	/// Buduje ścieżkę dla dokumentu COTS (Content of the structure) - i zapisuje
	pub fn save_cots_plot(
		&self,
		relpath: &str,
		title: Option<&str>,
		table_sotc_tree_cli: String,
		table_sotc_tree_raw: TableSotcTreeOutput,
	) -> io::Result<()> {
		let tt: TagTime = tag_time();
		let file_path = self.build_output_file_path(relpath, tt.0.clone(), "COTS")?;
		let title_str = title.map_or(String::new(), |t| format!("{} ", t));
		let content = format!(
			"# {}(CONTENT OF THE STRUCTURE v:{})\n{}\n{}",
			title_str,
			tt.0,
			Self::md_plaintext(table_sotc_tree_cli),
			self.md_build(table_sotc_tree_raw)
		);
		Self::fs_write(&file_path, content)?;
		Ok(())
	}

	fn fs_write(file_path: &PathBuf, content: String) -> std::io::Result<()> {
		fs::write(file_path, content)?;
		println!("📦 Zapisano archiwum kodu do: {}", file_path.display());
		Ok(())
	}

	fn md_plaintext(raw: String) -> String { format!("\n```plaintext\n{}\n```\n", Self::md_clean_txt(raw)) }
	fn md_clean_txt(content: impl Into<String>) -> String {
		let raw = content.into();
		strip_ansi_codes(&raw).into_owned().replace('\t', "    ")
	}

	fn md_build(&self, tabtree: TableSotcTreeOutput) -> String {
		let mut content = String::new();

		let (rows_to_show, index_offset) = if let Some(size) = tabtree.trim_size {
			let start = tabtree.trim_page.saturating_sub(1) * size;
			(tabtree.data.rows.iter().skip(start).take(size).collect::<Vec<_>>(), start)
		} else {
			(tabtree.data.rows.iter().collect::<Vec<_>>(), 0)
		};

		for (i, row) in rows_to_show.iter().enumerate() {
			let actual_idx = index_offset + i + 1;
			let p_str = &row.path;

			if p_str.ends_with('/') {
				continue;
			}

			let clean_rel = p_str.strip_prefix("./").unwrap_or(p_str);
			let absolute_path = self.workspace_dir.buf.join(clean_rel);

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
}
