//! Path: `src/libs/temporal/types/dates.rs`
//! Last significant modified: 2026-W43-1 15:46:20 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use super::enums::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateYearWeekDay {
	pub week_year: i16,
	pub week: Week,
	// week_day - Możliwość zamiennego użycia WeekDayWhrSunIs0 i WeekDayWhrSunIs7 (trait lub generyk, lub ?)
	pub week_day: WeekDayWhrSunIs7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateYearDay {
	pub year: i16,
	pub year_day: YearDay,
	// week_day - Możliwość zamiennego użycia WeekDayWhrSunIs0 i WeekDayWhrSunIs7 (trait lub generyk, lub ?)
	pub week_day: WeekDayWhrSunIs7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateMonthDay {
	pub year: i16,
	pub month: Month,
	pub month_day: MonthDay,
	// week_day - Możliwość zamiennego użycia WeekDayWhrSunIs0 i WeekDayWhrSunIs7 (trait lub generyk, lub ?)
	pub week_day: WeekDayWhrSunIs7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateExtra {
	pub century: i16,
	pub cycle_400: u16,
	pub year_is_leap: bool,
	// season_astro - Pory roku astronomiczne
	// season_meteo - pory roku metreologiczne:
	//                      jesień: wrzesień, październik, listopad
	//                      zima: grudzień, styczeń, luty;
	//                      wiosna: marzec, kwiecień, maj;
	//                      lato: czerwiec,lipiec, sierpień;
	pub season_astro: Season,
	pub season_meteo: Season,
	pub year_quater: Quater,

	// week_day_infirst_year_day, week_day_infinal_year_day - dodać możliwość zamiennie użycia WeekDayWhrSunIs0 i
	// WeekDayWhrSunIs7 week_num_infirst_year_day, week_num_infinal_year_day - zależne od tego czy użyjemy
	// WeekDayWhrSunIs7 czy WeekDayWhrSunIs0
	pub week_day_infirst_year_day: WeekDayWhrSunIs7,
	pub week_day_infinal_year_day: WeekDayWhrSunIs7,
	pub week_num_infirst_year_day: Week,
	pub week_num_infinal_year_day: Week,
}
