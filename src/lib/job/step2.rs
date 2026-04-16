use crate::lib::job::{self, RawRow};

// ============================================================================
// STRUKTURA WYJŚCIOWA (Transport do kolejnych etapów)
// ============================================================================

#[derive(Debug, Clone)]
pub struct FormattedRow {
	pub raw: RawRow,        // Zachowujemy oryginał do sortowania (Step 3) i limitowania (Step 4)
	pub cells: Vec<String>, // Sformatowane kolumny, każda jako niezależny String (dla Step 5)
}

// ============================================================================
// FORMATTER (Step 2)
// ============================================================================

pub fn engine_step2_data_formater(
	c: (&job::ValidColumnItemConfig, &job::ValidTableColumnsConfig),
	s: (&job::ValidColumnDateConfig, &job::ValidColumnTimeConfig, &job::ValidColumnSizeConfig),
	tab: &job::ScannedTable, // ⚡ Zmiana typu
) -> Vec<FormattedRow> {
	let item_cfg = c.0;
	let col_cfg = c.1;
	let date_cfg = s.0;
	let time_cfg = s.1;
	let size_cfg = s.2;

	let total_rows = tab.rows.len();
	let num_width = total_rows.to_string().len();

	let mut formatted_table = Vec::with_capacity(total_rows);

	for (i, row) in tab.rows.iter().enumerate() {
		// ⚡ Iterujemy po tab.rows
		let mut cells = Vec::with_capacity(col_cfg.columns.len());

		for col in &col_cfg.columns {
			let cell_str = match col {
				job::ValidTableColumns::Date => date_cfg.format_date(row.dt_modified),
				job::ValidTableColumns::Time => time_cfg.format_time(row.dt_modified),
				job::ValidTableColumns::Size => size_cfg.format_size(row.size_real),
				job::ValidTableColumns::Item => {
					let u_params = (i, num_width, tab.tier_max, tab.name_len_max);
					item_cfg.format_item(u_params, row, &tab.rows)
				}
				job::ValidTableColumns::Path => row.node.path.str.clone(),
			};
			cells.push(cell_str);
		}

		formatted_table.push(FormattedRow { raw: row.clone(), cells });
	}

	formatted_table
}

// ============================================================================
// HELPERY FORMATUJĄCE
// ============================================================================
