//! Path: `src/libs/temporal/logic/calendar_math.rs`
//! Last significant modified: 2026-W43-1 13:57:43 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Zwraca (Rok, Miesiąc, Dzień, Rok-w-Erze 400-letniej, czy_przestepny)
pub fn compute_ymd(days: i64) -> (i32, u32, u32, u32, bool) {
	let z = days + 719468;
	let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
	let doe = (z - era * 146097) as u32;
	let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
	let y = yoe as i32 + era as i32 * 400;

	let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
	let mp = (5 * doy + 2) / 153;
	let d = doy - (153 * mp + 2) / 5 + 1;
	let m = if mp < 10 { mp + 3 } else { mp - 9 };
	let year = if m <= 2 { y + 1 } else { y };

	let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

	(year, m, d, yoe, is_leap)
}
