//! Path: `src/libs/querypath/utils/check_is_binary.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::{fs, io::Read};

/// Badanie pierwszego kilobajta pliku pod kątem bajtu zerowego (`0x00`).
pub fn check_is_binary(path_str: &str) -> bool {
	let Ok(mut file) = fs::File::open(path_str) else {
		return false;
	};
	let mut buffer = [0u8; 1024];
	let Ok(n) = file.read(&mut buffer) else {
		return false;
	};
	buffer.iter().take(n).any(|&byte| byte == 0)
}
