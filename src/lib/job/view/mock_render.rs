use crate::lib::job;

// =========================================================================
// FUNKCJA POMOCNICZA (MOCK RENDERER - zalążek Step 5)
// =========================================================================
pub fn mock_render(rows: &[job::FormattedRow], cols_cfg: &job::ValidTableColumnsParams) -> String {
	let mut multiline = String::new();

	for row in rows {
		let mut line = String::new();
		let cells_len = row.cells.len();

		for (col_idx, cell) in row.cells.iter().enumerate() {
			line.push_str(cell);

			if col_idx + 1 < cells_len {
				let current = cols_cfg.columns.get(col_idx);
				let next = cols_cfg.columns.get(col_idx + 1);

				let is_date_time_pair = matches!(
					(current, next),
					(Some(job::ValidTableColumnsFlags::Date), Some(job::ValidTableColumnsFlags::Time))
						| (Some(job::ValidTableColumnsFlags::Time), Some(job::ValidTableColumnsFlags::Date))
				);

				if is_date_time_pair {
					line.push(' ');
				} else {
					line.push_str(" | ");
				}
			}
		}

		multiline.push_str(&line);
		multiline.push('\n');
	}

	multiline
}
