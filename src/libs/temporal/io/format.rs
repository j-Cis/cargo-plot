//! Path: `src/libs/temporal/io/format.rs`
//! Last significant modified: 2026-W43-3 01:13:56 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::super::types::{
	dates::{DateExtra, DateMonthDay},
	days::*,
	enums::*,
	time::Time,
};

pub struct TemporalFormatter;

impl TemporalFormatter {
	/// Formatuje bezpieczne struktury domenowe do ciągów znaków
	pub fn format(dmd: &DateMonthDay, time: &Time, extra: &DateExtra, offset_seconds: i32, pattern: &str) -> String {
		let mut out = String::with_capacity(pattern.len() + 32);
		let mut remaining = pattern;

		let day_num = match dmd.month_day {
			MonthDay::M31D(MonthWith31Day(d)) => d.get(),
			MonthDay::M30D(MonthWith30Day(d)) => d.get(),
			MonthDay::M29D(MonthWith29Day(d)) => d.get(),
			MonthDay::M28D(MonthWith28Day(d)) => d.get(),
		};

		let month_num = dmd.month as u32;
		let weekday_num = dmd.week_day as u32;

		while !remaining.is_empty() {
			if let Some(rest) = remaining.strip_prefix("ZONE") {
				let sign = if offset_seconds >= 0 { '+' } else { '-' };
				let abs_offset = offset_seconds.abs();
				out.push_str(&format!("{}{:02}{:02}UTC", sign, abs_offset / 3600, (abs_offset % 3600) / 60));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("WYYY") {
				out.push_str(&format!("{:04}", dmd.year));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("YYYY") {
				out.push_str(&format!("{:04}", dmd.year));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("OOO") {
				out.push_str(&format!("{:03}", extra.cycle_400));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("hhhh") {
				let h12 = time.hour12.0.get();
				let ampm = if time.hour_is_pm { "pm" } else { "am" };
				out.push_str(&format!("{}{:02}", ampm, h12));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("AAA") {
				out.push_str(match extra.season_astro {
					Season::Aut => "AUT",
					Season::Win => "WIN",
					Season::Spr => "SPR",
					Season::Sum => "SUM",
				});
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("SSS") {
				out.push_str(match extra.season_meteo {
					Season::Aut => "AUT",
					Season::Win => "WIN",
					Season::Spr => "SPR",
					Season::Sum => "SUM",
				});
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("MMM") {
				out.push_str(match dmd.month {
					Month::Jan => "JAN",
					Month::Feb => "FEB",
					Month::Mar => "MAR",
					Month::Apr => "APR",
					Month::May => "MAY",
					Month::Jun => "JUN",
					Month::Jul => "JUL",
					Month::Aug => "AUG",
					Month::Sep => "SEP",
					Month::Oct => "OCT",
					Month::Nov => "NOV",
					Month::Dec => "DEC",
				});
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("DDD") {
				out.push_str(match dmd.week_day {
					WeekDayWhrSunIs7::Mon => "MON",
					WeekDayWhrSunIs7::Tue => "TUE",
					WeekDayWhrSunIs7::Wed => "WED",
					WeekDayWhrSunIs7::Thu => "THU",
					WeekDayWhrSunIs7::Fri => "FRI",
					WeekDayWhrSunIs7::Sat => "SAT",
					WeekDayWhrSunIs7::Sun => "SUN",
				});
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("zzz") {
				out.push_str(&format!("{:03}", time.ms_in_second.0));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("WY") {
				out.push_str(&format!("{:02}", (dmd.year % 100).abs()));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("YY") {
				out.push_str(&format!("{:02}", (dmd.year % 100).abs()));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("MM") {
				out.push_str(&format!("{:02}", month_num));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("MD") {
				out.push_str(&format!("{:02}", day_num));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("WW") {
				out.push_str(&format!("{:02}", extra.week_num_infinal_year_day as u32));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("hh") {
				out.push_str(&format!("{:02}", time.hour24.0));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("mm") {
				out.push_str(&format!("{:02}", time.minute_in_hou.0));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("ss") {
				out.push_str(&format!("{:02}", time.second_in_min.0));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("tt") {
				out.push_str(&format!("{:02}", time.tierce_in_sec.0));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("qq") {
				out.push_str(&format!("{:02}", time.quadra_in_tie.0));
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("A") {
				out.push_str(&(extra.season_astro as u32).to_string());
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("S") {
				out.push_str(&(extra.season_meteo as u32).to_string());
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("Q") {
				out.push_str(&(extra.year_quater as u32).to_string());
				remaining = rest;
			} else if let Some(rest) = remaining.strip_prefix("D") {
				out.push_str(&weekday_num.to_string());
				remaining = rest;
			} else {
				let mut chars = remaining.chars();
				if let Some(c) = chars.next() {
					out.push(c);
					remaining = chars.as_str();
				}
			}
		}

		out
	}
}
