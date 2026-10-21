//! Path: `src/libs/temporal/logic/time_math.rs`
//! Last significant modified: 2026-W43-1 13:57:43 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::super::types::{
	primitives::{Range0To23, Range0To59, Range0To999, Range1To12},
	time::Time,
};
use std::num::NonZeroU16;

pub fn compute_time(rem_secs: u16, subsec_nanos: u32) -> Time {
	let hour = rem_secs / 3600;
	let minute = (rem_secs % 3600) / 60;
	let second = rem_secs % 60;

	let hour12 = if hour % 12 == 0 { 12 } else { hour % 12 };

	// Obliczenia na u64 by uniknąć przepełnienia przy mnożeniu
	let sub = subsec_nanos as u64;

	Time {
		hour24: Range0To23(hour),
		hour12: Range1To12(NonZeroU16::new(hour12).unwrap()),
		hour_is_pm: hour >= 12,
		minute_in_hou: Range0To59(minute),
		second_in_min: Range0To59(second),
		tierce_in_sec: Range0To59(((sub * 60) / 1_000_000_000) as u16),
		quadra_in_tie: Range0To59((((sub * 3600) / 1_000_000_000) % 60) as u16),
		quinta_in_qua: Range0To59((((sub * 216_000) / 1_000_000_000) % 60) as u16),
		sextus_in_qui: Range0To59((((sub * 12_960_000) / 1_000_000_000) % 60) as u16),
		septim_in_sex: Range0To59((((sub * 777_600_000) / 1_000_000_000) % 60) as u16),
		ms_in_second: Range0To999((subsec_nanos / 1_000_000) as u16),
		us_in_millis: Range0To999(((subsec_nanos / 1_000) % 1_000) as u16),
		ns_in_micros: Range0To999((subsec_nanos % 1_000) as u16),
	}
}
