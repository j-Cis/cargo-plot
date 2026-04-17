use crate::lib::{job::ModeIconsForValidColumnItem, logic::NodeIs};

pub fn draw_icon(mode: &ModeIconsForValidColumnItem, node_type: &NodeIs, name: &str) -> &'static str {
	match mode {
		ModeIconsForValidColumnItem::Lite => {
			if *node_type == NodeIs::Dir {
				"📂"
			} else {
				"📝"
			}
		}
		ModeIconsForValidColumnItem::More => {
			if *node_type == NodeIs::Dir {
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
		ModeIconsForValidColumnItem::None => unreachable!("None jest obsługiwane lokalnie przed wywołaniem"),
	}
}
