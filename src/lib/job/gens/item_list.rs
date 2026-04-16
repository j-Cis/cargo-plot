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

// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// 🧠 POWYŻSZY KOD PRAWIDŁOWO DRUKUJE TREE BEZ MOSTKÓW
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// 🧠 poniższy KOD DRUKUJE TYLKO MOSTKI - WSZYSTKO OPRÓCZ MOSTKÓW ROBI BEZNADZIEJNIE
/*
 * use crate::lib::{job::RawRow, logic::NodeIs, job::ValidModeItemList};
 * 
 * pub struct TreeLast;
 * impl TreeLast {
 *     pub const DIR_NO_CHILDREN: &'static str = "└───";
 *     pub const DIR_WITH_CHILDREN: &'static str = "└──┬";
 *     pub const FILE: &'static str = "└──•";
 *     pub const INDENT: &'static str = "   ";
 * }
 * 
 * pub struct TreeMid;
 * impl TreeMid {
 *     pub const DIR_NO_CHILDREN: &'static str = "├───";
 *     pub const DIR_WITH_CHILDREN: &'static str = "├──┬";
 *     pub const FILE: &'static str = "├──•";
 *     pub const INDENT: &'static str = "│  ";
 * }
 * 
 * pub struct DrawTree;
 * impl DrawTree {
 *     pub const ITEM_BETWEEN: &'static str = "  ├──•";
 *     pub const ITEM_FIRST: &'static str = "  ┌──•";
 *     pub const ITEM_LAST: &'static str = "  └──•";
 *     pub const ITEM_ONEFOLD: &'static str = "   ──•";
 *     
 *     // Symbole dla "Mostka" (Jumping Bridge)
 *     pub const BRIDGE: &'static str = "•──";
 * 
 *     pub fn list(index: usize, total: usize) -> &'static str {
 *         if total <= 1 {
 *             Self::ITEM_ONEFOLD
 *         } else if index == 0 {
 *             Self::ITEM_FIRST
 *         } else if index == total - 1 {
 *             Self::ITEM_LAST
 *         } else {
 *             Self::ITEM_BETWEEN
 *         }
 *     }
 * }
 * 
 * pub fn draw_list(mode: &ValidModeItemList, index: usize, tab: &[RawRow], _tier_max: usize) -> String {
 *     match mode {
 *         ValidModeItemList::Flat => DrawTree::list(index, tab.len()).to_string(),
 *         ValidModeItemList::Tree => {
 *             let node = &tab[index].node;
 *             let t = node.tier;
 *             let is_dir = node.node == NodeIs::Dir;
 * 
 *             // 1. Sprawdzamy czy ma fizyczne dzieci w aktualnej tabeli (Lookahead)
 *             let has_children = if is_dir && index + 1 < tab.len() {
 *                 tab[index + 1].node.path.str.starts_with(&node.path.str)
 *             } else {
 *                 false
 *             };
 * 
 *             // 2. Obliczamy is_last dla wszystkich poziomów (Lookahead)
 *             let mut is_last = vec![true; t + 1];
 *             let mut active_levels = t;
 *             for j in (index + 1)..tab.len() {
 *                 let next_tier = tab[j].node.tier;
 *                 if next_tier <= active_levels {
 *                     for l in next_tier..=active_levels {
 *                         is_last[l] = false;
 *                     }
 *                     active_levels = next_tier - 1;
 *                 }
 *                 if active_levels == 0 {
 *                     break;
 *                 }
 *             }
 * 
 *             // 3. ⚡ KOREKTA: Znajdujemy najbliższego fizycznego przodka w tabeli
 *             // Pozwala to wykryć "skok" (np. z poziomu 1 od razu na 3)
 *             let mut phys_parent_tier = 0;
 *             for i in (0..index).rev() {
 *                 if node.path.str.starts_with(&tab[i].node.path.str) && tab[i].node.node == NodeIs::Dir {
 *                     phys_parent_tier = tab[i].node.tier;
 *                     break;
 *                 }
 *             }
 * 
 *             let mut result = String::new();
 * 
 *             // 4. Rysujemy wcięcia dla poziomów, które mają fizycznych rodziców
 *             for l in 1..phys_parent_tier {
 *                 result.push_str(if is_last[l] { TreeLast::INDENT } else { TreeMid::INDENT });
 *             }
 * 
 *             // 5. ⚡ LOGIKA MOSTKA (Jumping logic)
 *             if t > phys_parent_tier {
 *                 if phys_parent_tier + 1 < t {
 *                     // Mamy skok! Rysujemy początek gałęzi i mostek przez brakujące poziomy
 *                     let jump_start = if is_last[phys_parent_tier + 1] { "└──" } else { "├──" };
 *                     result.push_str(jump_start);
 * 
 *                     // Środkowe mostki
 *                     for _ in (phys_parent_tier + 2)..t {
 *                         result.push_str(DrawTree::BRIDGE);
 *                     }
 * 
 *                     // Zakończenie skoku symbolem docelowym
 *                     let jump_end = if is_dir {
 *                         if has_children { "•──┬" } else { "•───" }
 *                     } else {
 *                         "•──•"
 *                     };
 *                     result.push_str(jump_end);
 *                 } else {
 *                     // Standardowe rysowanie gałęzi (bez skoku)
 *                     if phys_parent_tier > 0 {
 *                         result.push_str(if is_last[phys_parent_tier] { TreeLast::INDENT } else { TreeMid::INDENT });
 *                     }
 *                     
 *                     let branch = if is_dir {
 *                         match (is_last[t], has_children) {
 *                             (true, true) => TreeLast::DIR_WITH_CHILDREN,
 *                             (false, true) => TreeMid::DIR_WITH_CHILDREN,
 *                             (true, false) => TreeLast::DIR_NO_CHILDREN,
 *                             (false, false) => TreeMid::DIR_NO_CHILDREN,
 *                         }
 *                     } else if is_last[t] {
 *                         TreeLast::FILE
 *                     } else {
 *                         TreeMid::FILE
 *                     };
 *                     result.push_str(branch);
 *                 }
 *             }
 * 
 *             result
 *         }
 *         ValidModeItemList::None => unreachable!("None jest obsługiwane przed wywołaniem"),
 *     }
 * }
 * */