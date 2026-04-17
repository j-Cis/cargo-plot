use std::{fs, path::Path};

use anyhow::{Context, Result};
use chrono::DateTime;

use crate::lib::{job, logic};

// ============================================================================
// TRAIT POMOCNICZY (Unifikacja typów węzłów)
// ============================================================================

/// Pozwala na bezproblemową konwersję specjalistycznych węzłów na węzeł uniwersalny.
pub trait AsScannedNode {
	fn as_scanned_node(&self) -> logic::ScannedNode;
}

impl AsScannedNode for logic::ScannedFileNode {
	fn as_scanned_node(&self) -> logic::ScannedNode {
		logic::ScannedNode {
			name: self.name.clone(),
			path: self.path.clone(),
			node: logic::NodeIs::File,
			tier: self.tier,
			id_self: self.id_self,         // ⚡ PRZEPISANIE ID
			id_path: self.id_path.clone(), // ⚡ PRZEPISANIE ŚCIEŻKI
			dir_has_subdirs: None,
			dir_has_files_binary: None,
			dir_has_files_text: None,
			dir_has_symlinks: None,
			file_is_binary: Some(self.is_binary),
			file_is_empty: Some(self.is_empty),
		}
	}
}

impl AsScannedNode for logic::ScannedDirNode {
	fn as_scanned_node(&self) -> logic::ScannedNode {
		logic::ScannedNode {
			name: self.name.clone(),
			path: self.path.clone(),
			node: logic::NodeIs::Dir,
			tier: self.tier,
			id_self: self.id_self,         // ⚡ PRZEPISANIE ID
			id_path: self.id_path.clone(), // ⚡ PRZEPISANIE ŚCIEŻKI
			dir_has_subdirs: Some(self.has_subdirs),
			dir_has_files_binary: Some(self.has_files_binary),
			dir_has_files_text: Some(self.has_files_text),
			dir_has_symlinks: Some(self.has_symlinks),
			file_is_binary: None,
			file_is_empty: None,
		}
	}
}

// ============================================================================
// GATHER & INSPECT (Faza Hydracji)
// ============================================================================

/// Przechodzi po całej partycji, konwertuje generyczne węzły i uderza do dysku po metadane.
fn piece_gather<L>(a: &logic::AnchoredPathsDatum, p: &logic::Partition<L>) -> Vec<job::ValidResultMainRow>
where
	L: logic::MatchLabel,
	L::Node: AsScannedNode, //  Wymagamy, aby węzeł w partycji potrafił stać się ScannedNode
{
	p.nodes
		.iter()
		.filter_map(|n| {
			let unified_node = n.as_scanned_node();
			data_inspect(a, unified_node).ok() // Ignorujemy pliki, które np. usunięto w międzyczasie
		})
		.collect()
}

/// Zderza zunifikowany węzeł z fizycznym dyskiem i buduje ostateczny wiersz.
fn data_inspect(a: &logic::AnchoredPathsDatum, scanned_node: logic::ScannedNode) -> Result<job::ValidResultMainRow> {
	// 1. Ustalenie fizycznej ścieżki absolutnej
	// ⁉️ poco ponownie ustalamy
	let clean_rel = scanned_node.path.str.strip_prefix("./").unwrap_or(&scanned_node.path.str);
	let absolute_path = a.workspace_dir.buf.join(clean_rel);

	// 2. Pobranie metadanych systemu plików
	let metadata = fs::metadata(&absolute_path).context("Nie można odczytać metadanych")?;

	// 3. Ekstrakcja danych
	let dt_modified = DateTime::from(metadata.modified()?);

	let name_with_ext = absolute_path.file_name().unwrap_or_default().to_string_lossy().to_string();

	let size_real = if metadata.is_dir() { get_dir_size(&absolute_path) } else { metadata.len() };

	Ok(job::ValidResultMainRow { dt_modified, name_with_ext, size_real, node: scanned_node })
}

/// Oblicza faktyczny (fizyczny) rozmiar katalogu sumując wszystkie pliki wewnątrz.
fn get_dir_size(path: &Path) -> u64 {
	walkdir::WalkDir::new(path)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.file_type().is_file())
		.filter_map(|e| e.metadata().ok())
		.map(|m| m.len())
		.sum()
}

//============================================================================
//job::ValidTablePartConfig

pub fn data_gather(
	a: &logic::AnchoredPathsDatum,
	b: &logic::PartitionScanned,
	c: job::ValidTablePartConfig,
) -> job::ValidResultMainTab {
	let mut final_rows = Vec::new();
	let mut t_max = 0;
	let mut n_max = 0;
	let mut p_max = 0;

	// Leniwe pobieranie danych - uderzamy do dysku tylko po wybrane pule
	if c.md {
		final_rows.extend(piece_gather(a, &b.m_d));
		t_max = t_max.max(b.m_d.tier_max);
		n_max = n_max.max(b.m_d.name_len_max);
		p_max = p_max.max(b.m_d.path_len_max);
	}
	if c.mf {
		final_rows.extend(piece_gather(a, &b.m_f));
		t_max = t_max.max(b.m_f.tier_max);
		n_max = n_max.max(b.m_f.name_len_max);
		p_max = p_max.max(b.m_f.path_len_max);
	}
	if c.xd {
		final_rows.extend(piece_gather(a, &b.x_d));
		t_max = t_max.max(b.x_d.tier_max);
		n_max = n_max.max(b.x_d.name_len_max);
		p_max = p_max.max(b.x_d.path_len_max);
	}
	if c.xf {
		final_rows.extend(piece_gather(a, &b.x_f));
		t_max = t_max.max(b.x_f.tier_max);
		n_max = n_max.max(b.x_f.name_len_max);
		p_max = p_max.max(b.x_f.path_len_max);
	}

	// Sortujemy złączony wynik, aby przywrócić spójną strukturę drzewa po ścieżkach
	final_rows.sort_unstable_by(|row_a, row_b| row_a.node.path.str.cmp(&row_b.node.path.str));

	job::ValidResultMainTab { rows: final_rows, tier_max: t_max, name_len_max: n_max, path_len_max: p_max }
}
//============================================================================
/// [Step 1] Skaner - teraz przyjmuje konfiguratory zamiast surowych struktur Job
pub fn engine_step1_scanner(
	c: job::ValidTablePartConfig,
	workspace: &job::ValidWorkspaceConfig, // Zamiast ScanRawJob
	patterns: &job::ValidPatternConfig,    // Zamiast ScanRawJob
) -> job::ValidResultMainTab {
	// 1. Korzystamy z Twoich nowych metod .get() - one robią całą magię inicjalizacji i walidacji
	let anchored_paths_datum = workspace.get();
	let patterns_queries = patterns.get();

	// 2. Bezpośrednie wywołanie skanera z logiki
	let partition_scanned = logic::PartitionScanned::scan(&anchored_paths_datum, &patterns_queries);

	// 3. Zbieranie danych (hydracja z dysku) - funkcja data_gather pozostaje bez zmian
	data_gather(&anchored_paths_datum, &partition_scanned, c)
}
