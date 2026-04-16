use crate::lib::{job::ValidModeItemIcons, logic::NodeIs};

pub fn draw_icon(mode: &ValidModeItemIcons, node_type: &NodeIs, name: &str) -> &'static str {
	match mode {
		ValidModeItemIcons::Lite => {
			if *node_type == NodeIs::Dir {
				"📂"
			} else {
				"📝"
			}
		}
		ValidModeItemIcons::More => {
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
		ValidModeItemIcons::None => unreachable!("None jest obsługiwane lokalnie przed wywołaniem"),
	}
}
