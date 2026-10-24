



// ============================================================================
// ALGORYTM NAJNIŻSZEGO WSPÓLNEGO PRZODKA (LCA)
// ============================================================================

pub fn find_lowest_common_ancestor(paths: &[String]) -> Option<String> {
	if paths.is_empty() {
		return None;
	}
	if paths.len() == 1 {
		let trimmed = paths[0].trim_end_matches('/');
		return trimmed.rsplit_once('/').map(|(parent, _)| {
			if parent.is_empty() {
				"./".to_string()
			} else {
				format!("{parent}/")
			}
		});
	}

	let components: Vec<Vec<&str>> = paths
		.iter()
		.map(|p| p.trim_end_matches('/').split('/').collect())
		.collect();

	let min_len = components.iter().map(|c| c.len()).min().unwrap_or(0);
	let mut common: Vec<&str> = Vec::new();

	for i in 0..min_len {
		let first = components[0][i];
		if components.iter().all(|c| c[i] == first) {
			common.push(first);
		} else {
			break;
		}
	}

	if common.is_empty() {
		None
	} else {
		let mut res = common.join("/");
		if !res.ends_with('/') {
			res.push('/');
		}
		Some(res)
	}
}