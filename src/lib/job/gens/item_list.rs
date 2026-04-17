use crate::lib::job::ModeListForValidColumnItem;
// pub fn draw_list(mode: &ModeListForValidColumnItem, node: &ScannedNode) -> String {
// 	match mode {
// 		ModeListForValidColumnItem::Flat => "•".to_string(),
// 		ModeListForValidColumnItem::Tree => {
// 			// Tier 1 (root) nie ma wcięcia, Tier 2 ma jedno, itd.
// 			let indent = "│  ".repeat(node.tier.saturating_sub(1));
// 			format!("{}├──", indent)
// 		}
// 		ModeListForValidColumnItem::None => unreachable!("None jest obsługiwane lokalnie przed wywołaniem"),
// 	}
// }
use crate::lib::{job::ValidResultMainRow, logic::NodeIs};

pub struct TreeLast;
impl TreeLast {
	pub const DIR_NO_CHILDREN: &'static str = "└───";
	pub const DIR_WITH_CHILDREN: &'static str = "└──┬";
	pub const FILE: &'static str = "└──•";
	pub const INDENT: &'static str = "   ";
	pub const BRIDGE_START: &'static str = "└──•";
	pub const BRIDGE_NEXT: &'static str = "──•";
	pub const BRIDGE_LAST: &'static str = "───";
}

pub struct TreeMid;
impl TreeMid {
	pub const DIR_NO_CHILDREN: &'static str = "├───";
	pub const DIR_WITH_CHILDREN: &'static str = "├──┬";
	pub const FILE: &'static str = "├──•";
	pub const INDENT: &'static str = "│  ";
	pub const BRIDGE_START: &'static str = "├──•";
	pub const BRIDGE_NEXT: &'static str = "──•";
	pub const BRIDGE_LAST: &'static str = "───";
}

pub struct DrawTree;
impl DrawTree {
	pub const ITEM_BETWEEN: &'static str = "  ├──•";
	pub const ITEM_FIRST: &'static str = "  ┌──•";
	pub const ITEM_LAST: &'static str = "  └──•";
	pub const ITEM_ONEFOLD: &'static str = "   ──•";

	pub fn list(index: usize, total: usize) -> &'static str {
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

pub fn draw_list(
	mode: &ModeListForValidColumnItem,
	index: usize,
	tab: &[ValidResultMainRow],
	_tier_max: usize,
) -> String {
	match mode {
		ModeListForValidColumnItem::Flat => DrawTree::list(index, tab.len()).to_string(),
		ModeListForValidColumnItem::Tree => {
			let node = &tab[index].node;
			let t = node.tier;

			// 1. Logika Lookahead O(1) - dzieci w tabeli
			let is_dir = node.node == NodeIs::Dir;
			let has_children = if is_dir {
				if index + 1 < tab.len() { tab[index + 1].node.tier > t } else { false }
			} else {
				false
			};

			// 2. Logika Lookahead O(n) - obliczanie is_last
			let mut is_last = vec![true; t + 1];
			let mut active_levels = t;

			for j in (index + 1)..tab.len() {
				let next_tier = tab[j].node.tier;
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
				if node.path.str.starts_with(&tab[i_prev].node.path.str) && tab[i_prev].node.node == NodeIs::Dir {
					vis_parent_tier = tab[i_prev].node.tier;
					break;
				}
			}

			let mut result = String::new();
			let has_bridge = t > vis_parent_tier + 1;

			// 3. Generujemy wcięcia LUB segmenty mostka (poziomy 1 do T-1)
			for l in 1..t {
				if has_bridge && l > vis_parent_tier {
					// Rysujemy segmenty mostka (Jumping Bridge)
					if l == vis_parent_tier + 1 {
						result.push_str(if is_last[l] { TreeLast::BRIDGE_START } else { TreeMid::BRIDGE_START });
					} else {
						result.push_str(TreeMid::BRIDGE_NEXT);
					}
				} else {
					// Twoja ORYGINALNA logika wcięć (Indents)
					if is_last[l] {
						result.push_str(TreeLast::INDENT);
					} else {
						result.push_str(TreeMid::INDENT);
					}
				}
			}

			// 4. Dobieramy odpowiedni symbol gałęzi końcowej (Twoja ORYGINALNA logika)
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
				result.push_str(""); // błąd "─") // Łącznik łączący kropkę mostka z gałęzią
				result.push_str(&tail);
			} else {
				result.push_str(branch);
			}

			result
		}
		ModeListForValidColumnItem::None => unreachable!("None jest obsługiwane lokalnie przed wywołaniem"),
	}
}
