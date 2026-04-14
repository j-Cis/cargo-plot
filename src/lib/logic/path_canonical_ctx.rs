use std::{
	env,
	fs,
	path::{Path, PathBuf},
};

use anyhow::{Context, Result};

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
pub struct PathCanonicalCtx {
	pub execut_dir: PathNode,
	pub select_dir: PathNode,
	pub relat_path: String,
}

impl PathCanonicalCtx {
	pub fn new<P: AsRef<Path>>(input: P) -> Result<Self> {
		let input = input.as_ref();

		let execut_dir_buf = env::current_dir().context("Nie można odczytać katalogu roboczego (CWD)")?;

		let select_dir_buf = fs::canonicalize(input)
			.with_context(|| format!("Nie można ustalić ścieżki '{}'", input.to_string_lossy()))?;

		let relat_path = match select_dir_buf.strip_prefix(&execut_dir_buf) {
			Ok(rel) => {
				let s = rel.to_string_lossy().replace('\\', "/");
				if s.is_empty() { "./".to_string() } else { format!("./{}/", s) }
			}
			Err(_) => normalize_path(input),
		};

		Ok(Self { execut_dir: PathNode::new(execut_dir_buf), select_dir: PathNode::new(select_dir_buf), relat_path })
	}
}
