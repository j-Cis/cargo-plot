use crate::lib::{
	logic::ScanNodeIs,
	schema::{
		PipelineJobRow,
		PipelineJobTab,
		ReadyJob,
		SharedJobAttributeToSelect,
		SharedJobOptForAttrItem,
		SharedJobOptForAttrSize,
		SharedJobRunMode,
	},
};

// ============================================================================
// STRUKTURA WYJŚCIOWA (Transport do kolejnych etapów)
// ============================================================================

#[derive(Debug, Clone)]
pub struct FormattedRow {
	pub raw: PipelineJobRow,
	pub cells: Vec<String>,
}

// ============================================================================
// FORMATTER (Step 2) - GŁÓWNA PĘTLA
// ============================================================================

pub fn engine_step2_data_formater(
	job: &ReadyJob,
	tab: &PipelineJobTab, // ⚡ Zmiana typu
) -> Vec<FormattedRow> {
	let attr = job.attributes();
	let select = attr.select();
	let opts = attr.for_item();

	// Sprawdzamy, czy użytkownik zażyczył sobie inspekcji (debugowania)
	let is_inspection = job
		.run_modes()
		.iter()
		.any(|m| matches!(m, SharedJobRunMode::SaveWithInspection | SharedJobRunMode::PrintWithInspection));

	// ? let item_cfg = &j.item;
	// ? let col_cfg = &j.cols;
	// ? let date_cfg = &j.date;
	// ? let time_cfg = &j.time;
	// ? let size_cfg = &j.size;

	let total_rows = tab.rows.len();
	let num_width = total_rows.to_string().len();
	let mut formatted_table = Vec::with_capacity(total_rows);

	for (idx, row) in tab.rows.iter().enumerate() {
		let mut cells = Vec::with_capacity(select.len() + 1);

		for col in select {
			let cell_str = match col {
				SharedJobAttributeToSelect::Date => row.dt_modified.format(attr.for_date()).to_string(),
				SharedJobAttributeToSelect::Time => row.dt_modified.format(attr.for_time()).to_string(),
				SharedJobAttributeToSelect::Size => format_size(&attr.for_size(), row.size_real),
				SharedJobAttributeToSelect::Item => format_item(idx, num_width, tab, opts, row),
				// Tu w przyszłości wejdzie zaawansowana logika ikon i drzewa
				// ?? let u_params = (i, num_width, tab.tier_max, tab.name_len_max);
				// ?? item_cfg.format_item(u_params, row, &tab.rows)
				// row.name_with_ext.clone()
				//format_item_column(idx, num_width, item_opts, row, &tab.rows)
				//}
				SharedJobAttributeToSelect::Path => {
					let mut p_str = row.node.path.str.clone();
					// Zgodnie z Twoją logiką: align jeśli nie ma AlignHide
					if !opts.contains(&SharedJobOptForAttrItem::AlignHide) {
						let padding = tab.path_len_max.saturating_sub(p_str.chars().count());
						p_str.push_str(&" ".repeat(padding));
					}
					p_str
				}
			};
			cells.push(cell_str);
		}

		if is_inspection {
			let type_node = match row.node.node {
				ScanNodeIs::Dir => "📂",
				ScanNodeIs::File => "📃",
			};

			// Formatujemy debugowy string z ID węzła i ścieżką ID rodziców
			let debug_info = format!("🆔{}[{:03}] 🔙📂{:?}", type_node, row.node.id_self, row.node.id_path);
			cells.push(debug_info);
		}

		formatted_table.push(FormattedRow { raw: row.clone(), cells });
	}

	formatted_table
}

// ============================================================================
// FORMATOWANIE: ITEM (Ikony + Drzewo + Nazwa + Wyrównanie)
// ============================================================================

fn format_item(
	index: usize,
	num_width: usize,
	tab: &PipelineJobTab,
	opts: &[SharedJobOptForAttrItem],
	row: &PipelineJobRow,
) -> String {
	let mut parts = Vec::new();
	let num_str = format!("{:>width$}.", index + 1, width = num_width);

	// 1. Jeśli NumListBef (num_is_first)
	if opts.contains(&SharedJobOptForAttrItem::NumListBef) {
		parts.push(num_str.clone());
	}

	// 2. Struktura Listy (Zaktualizowane draw_list)
	let is_tree = opts.contains(&SharedJobOptForAttrItem::ListTree);
	let is_flat = opts.contains(&SharedJobOptForAttrItem::ListFlat);
	if is_tree || is_flat {
		parts.push(draw_list(is_tree, index, &tab.rows));
	}

	// 3. Ikony
	let hide_icons = opts.contains(&SharedJobOptForAttrItem::IconsHide);
	if !hide_icons {
		let is_more = opts.contains(&SharedJobOptForAttrItem::IconsMore);
		parts.push(draw_icon(is_more, &row.node.node, &row.name_with_ext).to_string());
	}

	// 4. Jeśli NumListAft (num_is_after)
	if opts.contains(&SharedJobOptForAttrItem::NumListAft) {
		parts.push(num_str);
	}

	// 5. Nazwa pliku
	if !opts.contains(&SharedJobOptForAttrItem::NameHide) {
		parts.push(row.name_with_ext.clone());
	}

	let mut base = parts.join(" ");

	// 6. Align End (Trailing space)
	if !opts.contains(&SharedJobOptForAttrItem::AlignHide) {
		let p_tier = (tab.tier_max.saturating_sub(row.node.tier)) * 3;
		let p_name = tab.name_len_max.saturating_sub(row.node.name.chars().count());
		base.push_str(&" ".repeat(p_tier + p_name));
	}

	base
}

// ============================================================================
// FORMATOWANIE: DRZEWO (Twoja zoptymalizowana logika mostków)
// ============================================================================

struct TreeLast;
impl TreeLast {
	const DIR_NO_CHILDREN: &'static str = "└───";
	const DIR_WITH_CHILDREN: &'static str = "└──┬";
	const FILE: &'static str = "└──•";
	const INDENT: &'static str = "   ";
	const BRIDGE_START: &'static str = "└──•";
	// const BRIDGE_NEXT: &'static str = "──•";
	// const BRIDGE_LAST: &'static str = "───";
}

struct TreeMid;
impl TreeMid {
	const DIR_NO_CHILDREN: &'static str = "├───";
	const DIR_WITH_CHILDREN: &'static str = "├──┬";
	const FILE: &'static str = "├──•";
	const INDENT: &'static str = "│  ";
	const BRIDGE_START: &'static str = "├──•";
	const BRIDGE_NEXT: &'static str = "──•";
	// const BRIDGE_LAST: &'static str = "───";
}

struct DrawTree;
impl DrawTree {
	const ITEM_BETWEEN: &'static str = "  ├──•";
	const ITEM_FIRST: &'static str = "  ┌──•";
	const ITEM_LAST: &'static str = "  └──•";
	const ITEM_ONEFOLD: &'static str = "   ──•";

	fn list(index: usize, total: usize) -> &'static str {
		if total <= 1 {
			Self::ITEM_ONEFOLD
		} else if index == 0 {
			Self::ITEM_FIRST
		} else if index == total - 1 {
			Self::ITEM_LAST
		} else {
			Self::ITEM_BETWEEN
		}
	}
}

fn draw_list(is_tree: bool, index: usize, rows: &[PipelineJobRow]) -> String {
	if !is_tree {
		return DrawTree::list(index, rows.len()).to_string();
	}

	let node = &rows[index].node;
	let t = node.tier;

	// 1. Logika Lookahead O(1) - dzieci w tabeli
	let is_dir = node.node == ScanNodeIs::Dir;
	let has_children = if is_dir {
		if index + 1 < rows.len() { rows[index + 1].node.tier > t } else { false }
	} else {
		false
	};

	// 2. Logika Lookahead O(n) - obliczanie is_last
	let mut is_last = vec![true; t + 1];
	let mut active_levels = t;

	for j in (index + 1)..rows.len() {
		let next_tier = rows[j].node.tier;
		if next_tier <= active_levels {
			is_last[next_tier] = false;
			active_levels = next_tier - 1;
		}
		if active_levels == 0 {
			break;
		}
	}

	// --- DETEKCJA NAJBLIŻSZEGO WIDOCZNEGO RODZICA (Dla mostka) ---
	let mut vis_parent_tier = 0;
	for i_prev in (0..index).rev() {
		if node.path.str.starts_with(&rows[i_prev].node.path.str) && rows[i_prev].node.node == ScanNodeIs::Dir {
			vis_parent_tier = rows[i_prev].node.tier;
			break;
		}
	}

	let mut result = String::new();
	let has_bridge = t > vis_parent_tier + 1;

	// 3. Generujemy wcięcia LUB segmenty mostka (poziomy 1 do T-1)
	for l in 1..t {
		if has_bridge && l > vis_parent_tier {
			if l == vis_parent_tier + 1 {
				result.push_str(if is_last[l] { TreeLast::BRIDGE_START } else { TreeMid::BRIDGE_START });
			} else {
				result.push_str(TreeMid::BRIDGE_NEXT);
			}
		} else if is_last[l] {
			result.push_str(TreeLast::INDENT);
		} else {
			result.push_str(TreeMid::INDENT);
		}
	}

	// 4. Dobieramy odpowiedni symbol gałęzi końcowej
	let am_i_last = is_last[t];
	let branch = if is_dir {
		match (am_i_last, has_children) {
			(true, true) => TreeLast::DIR_WITH_CHILDREN,
			(false, true) => TreeMid::DIR_WITH_CHILDREN,
			(true, false) => TreeLast::DIR_NO_CHILDREN,
			(false, false) => TreeMid::DIR_NO_CHILDREN,
		}
	} else if am_i_last {
		TreeLast::FILE
	} else {
		TreeMid::FILE
	};

	// 5. Łączymy gałąź z ewentualnym mostkiem
	if has_bridge {
		let tail: String = branch.chars().skip(1).collect();
		result.push_str("");
		result.push_str(&tail);
	} else {
		result.push_str(branch);
	}

	result
}

// ============================================================================
// FORMATOWANIE: IKONY ORAZ ROZMIAR
// ============================================================================

fn draw_icon(is_more: bool, node_type: &ScanNodeIs, name: &str) -> &'static str {
	if !is_more {
		if *node_type == ScanNodeIs::Dir { "📂" } else { "📝" }
	} else if *node_type == ScanNodeIs::Dir {
		"📁"
	} else if name.ends_with(".rs") {
		"🦀"
	} else if name.ends_with(".toml") {
		"⚙️"
	} else if name.ends_with(".md") {
		"📖"
	} else {
		"📄"
	}
}

fn format_size(mode: &SharedJobOptForAttrSize, bytes: u64) -> String {
	let base = match mode {
		SharedJobOptForAttrSize::Decimal => 1000.0,
		SharedJobOptForAttrSize::Binary => 1024.0,
	};

	let suffix = match mode {
		SharedJobOptForAttrSize::Decimal => ["B ", "kB", "MB", "GB"],
		SharedJobOptForAttrSize::Binary => ["B ", "KiB", "MiB", "GiB"],
	};

	let bytes_f = bytes as f64;

	if bytes_f < base {
		format!("{:>6} {}", bytes, suffix[0])
	} else if bytes_f < base.powi(2) {
		format!("{:>6.2} {}", bytes_f / base, suffix[1])
	} else if bytes_f < base.powi(3) {
		format!("{:>6.2} {}", bytes_f / base.powi(2), suffix[2])
	} else {
		format!("{:>6.2} {}", bytes_f / base.powi(3), suffix[3])
	}
}

// ============================================================================
// MOCK RENDERER (Renderowanie do podglądu konsolowego)
// ============================================================================

pub fn render_table(rows: &[FormattedRow], select: &[SharedJobAttributeToSelect]) -> String {
	let mut multiline = String::new();

	for row in rows {
		let mut line = String::new();
		let cells_len = row.cells.len();

		for (col_idx, cell) in row.cells.iter().enumerate() {
			line.push_str(cell);

			if col_idx + 1 < cells_len {
				let current = select.get(col_idx);
				let next = select.get(col_idx + 1);

				let is_date_time_pair = matches!(
					(current, next),
					(Some(SharedJobAttributeToSelect::Date), Some(SharedJobAttributeToSelect::Time))
						| (Some(SharedJobAttributeToSelect::Time), Some(SharedJobAttributeToSelect::Date))
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
