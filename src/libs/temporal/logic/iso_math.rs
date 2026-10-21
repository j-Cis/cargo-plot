//! Path: `src/libs/temporal/logic/iso_math.rs`
//! Last significant modified: 2026-W43-1 13:57:43 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Zwraca (Dzień w roku, Dzień tygodnia 1-7, Tydzień ISO, Rok ISO)
pub fn compute_iso_week(days: i64, year: i32, m: u32, d: u32, is_leap: bool) -> (u32, u16, u32, i32) {
	let weekday_num = (days.div_euclid(7) + 3).rem_euclid(7) as u16 + 1;

	let doy_calendar = if m > 2 {
		let month_days: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
		month_days[(m - 1) as usize] + d + if is_leap { 1 } else { 0 }
	} else {
		let month_days: [u32; 2] = [0, 31];
		month_days[(m - 1) as usize] + d
	};

	let iso_w = (doy_calendar as i32 - weekday_num as i32 + 10) / 7;
	let (iso_week, iso_year) = if iso_w < 1 {
		(52, year - 1)
	} else if iso_w > 52 {
		(1, year + 1)
	} else {
		(iso_w as u32, year)
	};

	(doy_calendar, weekday_num, iso_week, iso_year)
}
