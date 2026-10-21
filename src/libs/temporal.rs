//! Path: `src/libs/temporal.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

mod io;
mod logic;
mod types;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub use io::{format::TemporalFormatter, parse::TemporalParser};

pub struct Temporal;

impl Temporal {
	pub fn format(timestamp_secs: u64, pattern: &str) -> String {
		Self::format_nanos(timestamp_secs as u128 * 1_000_000_000, pattern)
	}

	pub fn format_millis(timestamp_millis: u64, pattern: &str) -> String {
		Self::format_nanos(timestamp_millis as u128 * 1_000_000, pattern)
	}

	pub fn format_system_time(st: SystemTime, pattern: &str) -> String {
		let duration = st.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
		Self::format_nanos(duration.as_nanos(), pattern)
	}

	pub fn format_nanos(timestamp_nanos: u128, pattern: &str) -> String {
		Self::format_nanos_with_offset(timestamp_nanos, 0, pattern)
	}

	pub fn format_nanos_with_offset(timestamp_nanos: u128, offset_seconds: i32, pattern: &str) -> String {
		use logic::{
			calendar_math::compute_ymd, iso_math::compute_iso_week, offset::apply_offset, time_math::compute_time,
		};
		use types::{
			dates::{DateExtra, DateMonthDay},
			days::*,
			enums::*,
		};

		let (days, rem_secs, subsec_nanos) = apply_offset(timestamp_nanos, offset_seconds);
		let (year, month_num, day_num, yoe, is_leap) = compute_ymd(days);
		let (_doy_calendar, weekday_num, iso_week, _iso_year) =
			compute_iso_week(days, year, month_num, day_num, is_leap);
		let time = compute_time(rem_secs, subsec_nanos);

		let month = match month_num {
			1 => Month::Jan,
			2 => Month::Feb,
			3 => Month::Mar,
			4 => Month::Apr,
			5 => Month::May,
			6 => Month::Jun,
			7 => Month::Jul,
			8 => Month::Aug,
			9 => Month::Sep,
			10 => Month::Oct,
			11 => Month::Nov,
			_ => Month::Dec,
		};

		let month_day = match month_num {
			1 | 3 | 5 | 7 | 8 | 10 | 12 => MonthDay::M31D(MonthWith31Day::try_from(day_num as u16).unwrap()),
			4 | 6 | 9 | 11 => MonthDay::M30D(MonthWith30Day::try_from(day_num as u16).unwrap()),
			2 if is_leap => MonthDay::M29D(MonthWith29Day::try_from(day_num as u16).unwrap()),
			_ => MonthDay::M28D(MonthWith28Day::try_from(day_num as u16).unwrap()),
		};

		let week_day = match weekday_num {
			1 => WeekDayWhrSunIs7::Mon,
			2 => WeekDayWhrSunIs7::Tue,
			3 => WeekDayWhrSunIs7::Wed,
			4 => WeekDayWhrSunIs7::Thu,
			5 => WeekDayWhrSunIs7::Fri,
			6 => WeekDayWhrSunIs7::Sat,
			_ => WeekDayWhrSunIs7::Sun,
		};

		let dmd = DateMonthDay { year: year as i16, month, month_day, week_day };

		let season_meteo = match month_num {
			9..=11 => Season::Aut,
			12 | 1 | 2 => Season::Win,
			3..=5 => Season::Spr,
			_ => Season::Sum,
		};

		let season_astro = match (month_num, day_num) {
			(3, 21..=31) | (4, _) | (5, _) | (6, 1..=21) => Season::Spr,
			(6, 22..=31) | (7, _) | (8, _) | (9, 1..=22) => Season::Sum,
			(9, 23..=31) | (10, _) | (11, _) | (12, 1..=21) => Season::Aut,
			_ => Season::Win,
		};

		let year_quater = match ((month_num - 1) / 3) + 1 {
			1 => Quater::JanFebMar,
			2 => Quater::AprMayJun,
			3 => Quater::JulAugSep,
			_ => Quater::OctNovDec,
		};

		let week_enum = match iso_week.clamp(1, 53) {
			1 => Week::W01,
			2 => Week::W02,
			3 => Week::W03,
			4 => Week::W04,
			5 => Week::W05,
			6 => Week::W06,
			7 => Week::W07,
			8 => Week::W08,
			9 => Week::W09,
			10 => Week::W10,
			11 => Week::W11,
			12 => Week::W12,
			13 => Week::W13,
			14 => Week::W14,
			15 => Week::W15,
			16 => Week::W16,
			17 => Week::W17,
			18 => Week::W18,
			19 => Week::W19,
			20 => Week::W20,
			21 => Week::W21,
			22 => Week::W22,
			23 => Week::W23,
			24 => Week::W24,
			25 => Week::W25,
			26 => Week::W26,
			27 => Week::W27,
			28 => Week::W28,
			29 => Week::W29,
			30 => Week::W30,
			31 => Week::W31,
			32 => Week::W32,
			33 => Week::W33,
			34 => Week::W34,
			35 => Week::W35,
			36 => Week::W36,
			37 => Week::W37,
			38 => Week::W38,
			39 => Week::W39,
			40 => Week::W40,
			41 => Week::W41,
			42 => Week::W42,
			43 => Week::W43,
			44 => Week::W44,
			45 => Week::W45,
			46 => Week::W46,
			47 => Week::W47,
			48 => Week::W48,
			49 => Week::W49,
			50 => Week::W50,
			51 => Week::W51,
			52 => Week::W52,
			_ => Week::W53,
		};

		let extra = DateExtra {
			century: (year / 100) as i16,
			cycle_400: yoe as u16,
			year_is_leap: is_leap,
			season_astro,
			season_meteo,
			year_quater,
			week_day_infirst_year_day: WeekDayWhrSunIs7::Mon,
			week_day_infinal_year_day: WeekDayWhrSunIs7::Sun,
			week_num_infirst_year_day: Week::W01,
			week_num_infinal_year_day: week_enum,
		};

		TemporalFormatter::format(&dmd, &time, &extra, offset_seconds, pattern)
	}
}
