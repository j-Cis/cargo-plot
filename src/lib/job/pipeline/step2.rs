use crate::lib::{
	job::{self, ValidResultMainRow},
	logic,
};
// ============================================================================
// STRUKTURA WYJŚCIOWA (Transport do kolejnych etapów)
// ============================================================================

#[derive(Debug, Clone)]
pub struct FormattedRow {
	pub raw: ValidResultMainRow, // Zachowujemy oryginał do sortowania (Step 3) i limitowania (Step 4)
	pub cells: Vec<String>,      // Sformatowane kolumny, każda jako niezależny String (dla Step 5)
}

// ============================================================================
// FORMATTER (Step 2)
// ============================================================================

pub fn engine_step2_data_formater(
	j: &job::ValidPreparedJobParams,
	tab: &job::ValidResultMainTab, // ⚡ Zmiana typu
) -> Vec<FormattedRow> {
	let item_cfg = &j.item;
	let col_cfg = &j.cols;
	let date_cfg = &j.date;
	let time_cfg = &j.time;
	let size_cfg = &j.size;

	let total_rows = tab.rows.len();
	let num_width = total_rows.to_string().len();

	let mut formatted_table = Vec::with_capacity(total_rows);

	for (i, row) in tab.rows.iter().enumerate() {
		// ⚡ Iterujemy po tab.rows
		let mut cells = Vec::with_capacity(col_cfg.columns.len());

		for col in &col_cfg.columns {
			let cell_str = match col {
				job::ValidTableColumnsFlags::Date => date_cfg.format_date(row.dt_modified),
				job::ValidTableColumnsFlags::Time => time_cfg.format_time(row.dt_modified),
				job::ValidTableColumnsFlags::Size => size_cfg.format_size(row.size_real),
				job::ValidTableColumnsFlags::Item => {
					let u_params = (i, num_width, tab.tier_max, tab.name_len_max);
					item_cfg.format_item(u_params, row, &tab.rows)
				}
				job::ValidTableColumnsFlags::Path => {
					let mut p_str = row.node.path.str.clone();

					// ⚡ PROPAGACJA: Jeśli Item ma włączone wyrównanie (ws-show), wyrównujemy też ścieżkę
					if item_cfg.align_end {
						let padding = tab.path_len_max.saturating_sub(p_str.chars().count());
						p_str.push_str(&" ".repeat(padding));
					}
					p_str
				}
			};
			cells.push(cell_str);
		}

		if job::IS_DEBUG {
			let type_node = match row.node.node {
				logic::NodeIs::Dir => "📂",
				logic::NodeIs::File => "📃",
			};
			// id_self (formatujemy jako [ 🆔 XXX ]) &&  id_path (formatujemy jako wektor ID [0, 1, ...])
			cells.push(format!("🆔{}[{:?}] 🔙📂{:?}", type_node, row.node.id_self, row.node.id_path));
		}

		formatted_table.push(FormattedRow { raw: row.clone(), cells });
	}

	formatted_table
}

// ============================================================================
// HELPERY FORMATUJĄCE
// ============================================================================
