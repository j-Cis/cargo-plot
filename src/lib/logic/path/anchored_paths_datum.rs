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

		

		Ok(Self { execution_dir: PathNode::new(execution_dir_buf), workspace_dir: PathNode::new(workspace_dir_buf)})
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
}