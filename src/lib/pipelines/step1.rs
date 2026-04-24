use std::{fs, path::Path};

use anyhow::{Context, Result};
use chrono::{DateTime, Local};

use crate::lib::{
	logic::{
		self,
		ScanMatchLabel,
		ScanNodeDirScanned,
		ScanNodeIs,
		ScanNodeScanned,
		ScanPartition,
		ScanPartitionScanned,
	},
	schema::{PipelineJobRow, PipelineJobTab, ReadyJob, SharedJobPart},
};

// ============================================================================
// TRAIT POMOCNICZY (Unifikacja typów węzłów)
// ============================================================================

/// Pozwala na bezproblemową konwersję specjalistycznych węzłów na węzeł uniwersalny.
pub trait AsScannedNode {
	fn as_scanned_node(&self) -> logic::ScanNodeScanned;
}

impl AsScannedNode for logic::ScanNodeFileScanned {
	fn as_scanned_node(&self) -> logic::ScanNodeScanned {
		logic::ScanNodeScanned {
			name: self.name.clone(),
			path: self.path.clone(),
			node: ScanNodeIs::File,
			tier: self.tier,
			id_self: self.id_self,
			id_path: self.id_path.clone(),
			dir_has_subdirs: None,
			dir_has_files_binary: None,
			dir_has_files_text: None,
			dir_has_symlinks: None,
			file_is_binary: Some(self.is_binary),
			file_is_empty: Some(self.is_empty),
		}
	}
}

impl AsScannedNode for ScanNodeDirScanned {
	fn as_scanned_node(&self) -> ScanNodeScanned {
		ScanNodeScanned {
			name: self.name.clone(),
			path: self.path.clone(),
			node: ScanNodeIs::Dir,
			tier: self.tier,
			id_self: self.id_self,
			id_path: self.id_path.clone(),
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
// GATHER & INSPECT (Faza Hydracji z Dysku)
// ============================================================================

fn piece_gather<L>(job: &ReadyJob, p: &ScanPartition<L>) -> Vec<PipelineJobRow>
where
	L: ScanMatchLabel,
	L::Node: AsScannedNode, {
	p.nodes
		.iter()
		.filter_map(|n| {
			let unified_node = n.as_scanned_node();
			data_inspect(job, unified_node).ok()
		})
		.collect()
}

fn data_inspect(job: &ReadyJob, scanned_node: ScanNodeScanned) -> Result<PipelineJobRow> {
	let clean_rel = scanned_node.path.str.strip_prefix("./").unwrap_or(&scanned_node.path.str);

	// Budujemy pełną ścieżkę fizyczną z workspace_dir zdefiniowanego w ReadyJob
	let absolute_path = job.explorer().workspace_dir().join(clean_rel);

	// 2. Pobranie metadanych systemu plików
	let metadata = fs::metadata(&absolute_path).context("Nie można odczytać metadanych")?;

	// Konwersja czasu systemowego na czas lokalny, bezpieczna dla potoku
	let dt_modified: DateTime<Local> = metadata.modified()?.into();
	let name_with_ext = absolute_path.file_name().unwrap_or_default().to_string_lossy().to_string();
	let size_real = if metadata.is_dir() { get_dir_size(&absolute_path) } else { metadata.len() };

	Ok(PipelineJobRow { dt_modified, name_with_ext, size_real, node: scanned_node })
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

// ============================================================================
// MAIN PIPELINE
// ============================================================================

pub fn data_gather(job: &ReadyJob, b: &ScanPartitionScanned) -> PipelineJobTab {
	let mut final_rows = Vec::new();
	let mut t_max = 0;
	let mut n_max = 0;
	let mut p_max = 0;

	let parts = job.explorer().parts();

	// Leniwe pobieranie danych - z dysku tylko dla wybranych partycji (MD, MF, XD, XF)
	if parts.contains(&SharedJobPart::MD) {
		final_rows.extend(piece_gather(job, &b.m_d));
		t_max = t_max.max(b.m_d.tier_max);
		n_max = n_max.max(b.m_d.name_len_max);
		p_max = p_max.max(b.m_d.path_len_max);
	}
	if parts.contains(&SharedJobPart::MF) {
		final_rows.extend(piece_gather(job, &b.m_f));
		t_max = t_max.max(b.m_f.tier_max);
		n_max = n_max.max(b.m_f.name_len_max);
		p_max = p_max.max(b.m_f.path_len_max);
	}
	if parts.contains(&SharedJobPart::XD) {
		final_rows.extend(piece_gather(job, &b.x_d));
		t_max = t_max.max(b.x_d.tier_max);
		n_max = n_max.max(b.x_d.name_len_max);
		p_max = p_max.max(b.x_d.path_len_max);
	}
	if parts.contains(&SharedJobPart::XF) {
		final_rows.extend(piece_gather(job, &b.x_f));
		t_max = t_max.max(b.x_f.tier_max);
		n_max = n_max.max(b.x_f.name_len_max);
		p_max = p_max.max(b.x_f.path_len_max);
	}

	// Sortujemy złączony wynik, aby przywrócić spójną strukturę drzewa po ścieżkach
	final_rows.sort_unstable_by(|row_a, row_b| row_a.node.path.str.cmp(&row_b.node.path.str));

	PipelineJobTab { rows: final_rows, tier_max: t_max, name_len_max: n_max, path_len_max: p_max }
}
//============================================================================
/// [Step 1] Skaner (Punkt wejściowy pierwszego etapu)
pub fn engine_step1_scanner(job: &ReadyJob) -> PipelineJobTab {
	// Odpalamy skaner systemu plików używając Explorera
	let partition_scanned = ScanPartitionScanned::scan(job.explorer());

	// Pobieramy metadane z dysku dla pomyślnie dopasowanych węzłów
	data_gather(job, &partition_scanned)
}
