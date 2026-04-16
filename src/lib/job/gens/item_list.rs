use crate::lib::job::ValidModeItemList;
// pub fn draw_list(mode: &ValidModeItemList, node: &ScannedNode) -> String {
// 	match mode {
// 		ValidModeItemList::Flat => "•".to_string(),
// 		ValidModeItemList::Tree => {
// 			// Tier 1 (root) nie ma wcięcia, Tier 2 ma jedno, itd.
// 			let indent = "│  ".repeat(node.tier.saturating_sub(1));
// 			format!("{}├──", indent)
// 		}
// 		ValidModeItemList::None => unreachable!("None jest obsługiwane lokalnie przed wywołaniem"),
// 	}
// }
use crate::lib::{job::RawRow, logic::NodeIs};

pub struct TreeLast;
impl TreeLast {
	pub const DIR_NO_CHILDREN: &'static str = "└───";
	pub const DIR_WITH_CHILDREN: &'static str = "└──┬";
	pub const FILE: &'static str = "└──•";
	pub const INDENT: &'static str = "   ";
	pub const BRIDGE_START: &'static str =   "└──•";
	pub const BRIDGE_NEXT: &'static str =    "──•";
	pub const BRIDGE_LAST: &'static str =    "───";
}

pub struct TreeMid;
impl TreeMid {
	pub const DIR_NO_CHILDREN: &'static str = "├───";
	pub const DIR_WITH_CHILDREN: &'static str = "├──┬";	
	pub const FILE: &'static str = "├──•";
	pub const INDENT: &'static str = "│  ";
	pub const BRIDGE_START: &'static str =   "├──•";
	pub const BRIDGE_NEXT: &'static str =    "──•";
	pub const BRIDGE_LAST: &'static str =    "───";
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

pub fn draw_list(mode: &ValidModeItemList, index: usize, tab: &[RawRow], _tier_max: usize) -> String {
	match mode {
		ValidModeItemList::Flat => DrawTree::list(index, tab.len()).to_string(),
		ValidModeItemList::Tree => {
			let node = &tab[index].node;
			let t = node.tier;

			// 1. Sprawdzamy czy ma wyrenderowane dzieci (Lookahead O(1))
			// Jeśli następny element ma większy tier, to znaczy, że wpadł "do środka" tego folderu.
			let is_dir = node.node == NodeIs::Dir;
			let has_children = if is_dir {
				if index + 1 < tab.len() { tab[index + 1].node.tier > t } else { false }
			} else {
				false
			};

			// 2. Lookahead O(n): obliczamy `is_last` dla węzła i wszystkich jego przodków
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

			// 3. Generujemy wcięcia (indents) dla przodków (od poziomu 1 do T-1)
			let mut result = String::new();
			for l in 1..t {
				if is_last[l] {
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

			result.push_str(branch);
			result
		}
		ValidModeItemList::None => unreachable!("None jest obsługiwane lokalnie przed wywołaniem"),
	}
}


// 🧠 ten plik prawidłowo drukuje Tree, za wyjątkiem MOSTKÓW