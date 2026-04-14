use std::{fs, io::Read};

use chrono::{DateTime, Local};

use super::{
	path_canonical_ctx::PathCanonicalCtx,
	paths_result::{FilterList, MatchLabel},
	table_spec::{TabColumn, TabSortBy, TabSortOrder, TableSpec},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileKind {
	Dir,
	Text,
	Binary,
	Other,
}

/// Idiomatyczny wiersz danych
#[derive(Debug, Clone)]
pub struct TableRow {
	pub path: String,
	pub size: u64,
	pub modified: DateTime<Local>,
	pub kind: FileKind,
}

/// Idiomatyczny kontener zebranych danych
#[derive(Debug, Clone)]
pub struct TableData {
	pub rows: Vec<TableRow>,
	pub is_tree: bool,
}

/// Ostateczny wynik materializacji
pub struct TableOutput {
	pub data: TableData,
	pub limit: Option<usize>,
	pub columns: Vec<TabColumn>,
	pub page: Option<usize>,
	pub page_size: Option<usize>,
	pub extended_icons: bool,
}

fn is_binary(path: &std::path::Path) -> std::io::Result<bool> {
	let mut file = fs::File::open(path)?;
	let mut buffer = [0u8; 1024];
	let n = file.read(&mut buffer)?;
	Ok(buffer[..n].contains(&0))
}

fn get_dir_size(path: &std::path::Path) -> u64 {
	walkdir::WalkDir::new(path)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.file_type().is_file())
		.filter_map(|e| e.metadata().ok())
		.map(|m| m.len())
		.sum()
}

impl TableData {
	pub fn gather<L: MatchLabel>(list: &FilterList<L>) -> Self {
		let rows = list.paths.iter().filter_map(|p| Self::inspect(p, &list.entry).ok()).collect();
		Self { rows, is_tree: false }
	}

	fn inspect(rel_path: &str, relation: &PathCanonicalCtx) -> anyhow::Result<TableRow> {
		let clean_rel = rel_path.strip_prefix("./").unwrap_or(rel_path);
		let absolute_path = relation.select_dir.buf.join(clean_rel);

		let metadata = fs::metadata(&absolute_path)?;
		let modified = DateTime::from(metadata.modified()?);

		let (kind, size) = if metadata.is_dir() {
			(FileKind::Dir, get_dir_size(&absolute_path))
		} else {
			let k = if is_binary(&absolute_path)? { FileKind::Binary } else { FileKind::Text };
			(k, metadata.len())
		};

		Ok(TableRow { path: rel_path.to_string(), size, modified, kind })
	}

	pub fn sort(mut self, by: TabSortBy, order: TabSortOrder, is_tree: bool) -> Self {
		self.is_tree = is_tree;

		fn get_merge_key(path: &str) -> &str {
			let trimmed = path.trim_end_matches('/');
			if let Some(idx) = trimmed.rfind('.')
				&& idx > 0 && trimmed.as_bytes()[idx - 1] != b'/'
			{
				return &trimmed[..idx];
			}
			trimmed
		}

		let compare = |a: &TableRow, b: &TableRow| {
			let a_is_dir = a.kind == FileKind::Dir;
			let b_is_dir = b.kind == FileKind::Dir;
			let a_merge = get_merge_key(&a.path);
			let b_merge = get_merge_key(&b.path);

			let mut cmp = match by {
				TabSortBy::Name => a.path.cmp(&b.path),
				TabSortBy::Size => a.size.cmp(&b.size),
				TabSortBy::Date => a.modified.cmp(&b.modified),
				TabSortBy::Kind => (a.kind.clone() as u8).cmp(&(b.kind.clone() as u8)),
				TabSortBy::FileFirst => (a_is_dir, &a.path).cmp(&(b_is_dir, &b.path)),
				TabSortBy::DirFirst => (!a_is_dir, &a.path).cmp(&(!b_is_dir, &b.path)),
				TabSortBy::FileFirstMerge => (a_merge, a_is_dir, &a.path).cmp(&(b_merge, b_is_dir, &b.path)),
				TabSortBy::DirFirstMerge => (a_merge, !a_is_dir, &a.path).cmp(&(b_merge, !b_is_dir, &b.path)),
			};
			if matches!(order, TabSortOrder::Desc) {
				cmp = cmp.reverse();
			}
			cmp
		};

		if is_tree {
			use std::{
				collections::BTreeMap,
				path::{Path, PathBuf},
			};

			let clean_paths: Vec<PathBuf> =
				self.rows.iter().map(|r| PathBuf::from(r.path.trim_end_matches('/'))).collect();
			let mut tree_map: BTreeMap<PathBuf, Vec<usize>> = BTreeMap::new();

			for (i, p) in clean_paths.iter().enumerate() {
				let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
				tree_map.entry(parent).or_default().push(i);
			}

			for indices in tree_map.values_mut() {
				indices.sort_by(|&a, &b| compare(&self.rows[a], &self.rows[b]));
			}

			let mut root_indices = Vec::new();
			for (i, p) in clean_paths.iter().enumerate() {
				let parent = p.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);
				let is_root = parent == Path::new(".") || parent == Path::new("") || !clean_paths.contains(&parent);
				if is_root {
					root_indices.push(i);
				}
			}
			root_indices.sort_by(|&a, &b| compare(&self.rows[a], &self.rows[b]));

			let mut flat_indices = Vec::with_capacity(self.rows.len());
			fn flatten(
				indices: &[usize],
				tree_map: &BTreeMap<PathBuf, Vec<usize>>,
				clean_paths: &[PathBuf],
				out: &mut Vec<usize>,
			) {
				for &idx in indices {
					out.push(idx);
					if let Some(children) = tree_map.get(&clean_paths[idx]) {
						flatten(children, tree_map, clean_paths, out);
					}
				}
			}

			flatten(&root_indices, &tree_map, &clean_paths, &mut flat_indices);

			let old_rows = std::mem::take(&mut self.rows);
			let mut temp_rows: Vec<Option<TableRow>> = old_rows.into_iter().map(Some).collect();
			let mut new_rows = Vec::with_capacity(temp_rows.len());

			for idx in flat_indices {
				new_rows.push(temp_rows[idx].take().unwrap());
			}
			self.rows = new_rows;
		} else {
			self.rows.sort_by(compare);
		}

		self
	}

	pub fn into_output(self, spec: &TableSpec) -> TableOutput {
		TableOutput {
			data: self,
			limit: spec.limit,
			columns: spec.columns.clone(),
			page: spec.page,
			page_size: spec.page_size,
			extended_icons: spec.extended_icons,
		}
	}
}
