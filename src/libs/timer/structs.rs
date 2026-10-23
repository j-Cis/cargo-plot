use super::enums::{MonthDay, Months, Quater, Season, Week, WeekDay, YearDay};
use std::num::NonZeroU16;

#[derive(Debug, Clone, Copy)]
pub struct Timer(pub i64, pub u32, pub u32, pub i32);

#[derive(Debug)]
pub struct T {
	pub day: Day,
	pub year: Year,
	pub week: Week,
	pub month: Month,
	pub time: Time,
}

#[derive(Debug)]
pub struct Day {
	pub in_year: YearDay,
	pub in_month: MonthDay,
	pub in_week: WeekDay,
}

#[derive(Debug)]
pub struct Year {
	pub week_year: i16,
	pub year: i16,
	pub season: Season,
	pub quater: Quater,
	pub is_leap: bool,
	pub cycle: Cycle,
	pub infirst_day: WeekDay,
	pub infinal_day: WeekDay,
	pub infirst_weeknum: Week, // Skondensowane z infirst_sun7_weeknum i infirst_sun0_weeknum
	pub infinal_weeknum: Week, // Skondensowane z infinal_sun7_weeknum i infinal_sun0_weeknum
}

#[derive(Debug)]
pub struct Month {
	pub month: Months,
	pub has_days: MonthDay,
}

#[derive(Debug)]
pub struct Cycle {
	pub century: i16,
	pub cycl400: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearWith365Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearWith366Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith31Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith30Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith29Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith28Day(pub NonZeroU16);

//-------------------------------------------------------- CZAS

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range1To12(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range0To23(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range0To59(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range0To999(pub u16);

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
