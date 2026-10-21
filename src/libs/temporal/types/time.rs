//! Path: `src/libs/temporal/types/time.rs`
//! Last significant modified: 2026-W43-1 15:46:37 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::primitives::{Range0To23, Range0To59, Range0To999, Range1To12};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
	pub hour24: Range0To23,
	pub hour12: Range1To12,
	pub hour_is_pm: bool,
	pub minute_in_hou: Range0To59,
	pub second_in_min: Range0To59,
	pub tierce_in_sec: Range0To59,
	pub quadra_in_tie: Range0To59,
	pub quinta_in_qua: Range0To59,
	pub sextus_in_qui: Range0To59,
	pub septim_in_sex: Range0To59,

	// 1 tercja ma  (50/3)    milisekund  (około  16.67  ms)
	// 1 kwadra ma  (2500/9)  mikrosekund (około 277.8   µs)
	// 1 kwinta ma  (125/27)  mikrosekund (około   4.63  µs)
	// 1 seksta ma  (6250/81) nanosekundy (około  77.16  ns)
	// 1 septyma ma (625/486) nanosekund  (około   1.286 ns)
	pub ms_in_second: Range0To999,
	pub us_in_millis: Range0To999,
	pub ns_in_micros: Range0To999,
}
