

use super::functions::{
	civil_from_days, days_from_civil, days_in_month, is_leap, iso_weekday, month_day, months_of, nz,
	quater_of, season, week_of,
};

use super::structs::{
	Cycle, Day, Month, MonthWith28Day, MonthWith29Day, MonthWith30Day, MonthWith31Day, 
	Range0To23, Range0To59, Range0To999, Range1To12,  T, Time, Timer,  Year,
	YearWith365Day, YearWith366Day,
};

use super::enums::{
	 Week, WeekDay, WeekFirstDay, 
	YearDay,
};
use std::num::NonZeroU16;



impl Timer {
	pub fn now() -> Self {
		let n = jiff::Zoned::now();
		let total_secs_utc = (n.timestamp().as_nanosecond() / 1_000_000_000) as i64;
		let subsec_nanos = (n.timestamp().as_nanosecond() % 1_000_000_000) as u32;

		// Pobranie offsetu i zapisanie jako oddzielna zmienna
		let offset_secs = n.offset().seconds() as i32;
		let base_secs = total_secs_utc + i64::from(offset_secs);
		// let base_secs = total_secs_utc + (n.offset().seconds() as i64);

		// Używamy dzielenia euklidesowego dla bezpieczeństwa dat ujemnych (przed 1970)
		let days = base_secs.div_euclid(86_400);
		let rem_secs = base_secs.rem_euclid(86_400) as u32;

		Self(days, rem_secs, subsec_nanos, offset_secs)
	}
	pub fn fmt(self, week_start: WeekFirstDay) -> T {
		T::new(self, week_start)
	}
}

// ------------------------------------------------------------ KONSTRUKCJA T  Timer
impl T {
	/// Buduje komplet danych kalendarzowych i czasu z `Timer`
	/// (dni od 1970-01-01 w czasie lokalnym, sekundy doby, nanosekundy).
	pub fn new(timer: Timer, week_start: WeekFirstDay) -> Self {
		let Timer(days, rem_secs, subsec_nanos, offset_secs) = timer;
		// Chwila w UTC (sekundy uniksowe), potrzebna do porównania z równonocami i przesileniami.

		// Zmienna pomocnicza do funkcji week_of()
		let sun0 = week_start == WeekFirstDay::Sun;

		let now_unix =
			(days * 86_400 + i64::from(rem_secs) - i64::from(offset_secs)) as f64 + f64::from(subsec_nanos) / 1e9;

		// ---- data
		let (y, m, d) = civil_from_days(days);
		let month_len = days_in_month(y, m);
		let leap = is_leap(y);
		let jan1 = days_from_civil(y, 1, 1);
		let dec31 = days_from_civil(y, 12, 31);
		let day_of_year = (days - jan1 + 1) as u16;
		let iso_wd = iso_weekday(days);

		let day = Day {
			in_year: if leap {
				YearDay::Y366D(YearWith366Day(nz(day_of_year)))
			} else {
				YearDay::Y365D(YearWith365Day(nz(day_of_year)))
			},
			in_month: month_day(month_len, d as u16),
			in_week: WeekDay::from_iso(iso_wd),
		};

		// Tutaj używamy sun0, aby obliczyć tylko jeden zestaw danych:
		let year = Year {
			week_year: week_of(days, sun0).1 as i16,
			year: y as i16,
			season: season(y, now_unix),
			quater: quater_of(m),
			is_leap: leap,
			cycle: Cycle { century: ((y - 1).div_euclid(100) + 1) as i16, cycl400: y.rem_euclid(400) as u16 },
			infirst_day: WeekDay::from_iso(iso_weekday(jan1)),
			infinal_day: WeekDay::from_iso(iso_weekday(dec31)),
			infirst_weeknum: Week::from_number(week_of(jan1, sun0).0),
			infinal_weeknum: Week::from_number(week_of(dec31, sun0).0),
		};

		let month = Month { month: months_of(m), has_days: month_day(month_len, month_len as u16) };

		let week = Week::from_number(week_of(days, sun0).0);

		// ---- czas
		let hour24 = (rem_secs / 3_600) as u16;
		let minute = (rem_secs % 3_600 / 60) as u16;
		let second = (rem_secs % 60) as u16;

		// Ułamek sekundy dzielony kolejno przez 60 (tercja, kwadra, kwinta, seksta, septyma).
		let mut rest = u64::from(subsec_nanos);
		let mut next_sexagesimal = || {
			rest *= 60;
			let digit = (rest / 1_000_000_000) as u16;
			rest %= 1_000_000_000;
			digit
		};
		let tierce = next_sexagesimal();
		let quadra = next_sexagesimal();
		let quinta = next_sexagesimal();
		let sextus = next_sexagesimal();
		let septim = next_sexagesimal();

		let time = Time {
			hour24: Range0To23(hour24),
			hour12: Range1To12(nz((hour24 + 11) % 12 + 1)),
			hour_is_pm: hour24 >= 12,
			minute_in_hou: Range0To59(minute),
			second_in_min: Range0To59(second),
			tierce_in_sec: Range0To59(tierce),
			quadra_in_tie: Range0To59(quadra),
			quinta_in_qua: Range0To59(quinta),
			sextus_in_qui: Range0To59(sextus),
			septim_in_sex: Range0To59(septim),
			ms_in_second: Range0To999((subsec_nanos / 1_000_000) as u16),
			us_in_millis: Range0To999((subsec_nanos / 1_000 % 1_000) as u16),
			ns_in_micros: Range0To999((subsec_nanos % 1_000) as u16),
		};

		Self { day, year, week, month, time }
	}
}

impl WeekDay {
	/// `iso`: poniedziałek = 1 ... niedziela = 7.
	fn from_iso(iso: i64) -> Self {
		match iso {
			1 => Self::Mon,
			2 => Self::Tue,
			3 => Self::Wed,
			4 => Self::Thu,
			5 => Self::Fri,
			6 => Self::Sat,
			_ => Self::Sun,
		}
	}
}

impl Week {
	const ALL: [Week; 53] = [
		Week::W01,
		Week::W02,
		Week::W03,
		Week::W04,
		Week::W05,
		Week::W06,
		Week::W07,
		Week::W08,
		Week::W09,
		Week::W10,
		Week::W11,
		Week::W12,
		Week::W13,
		Week::W14,
		Week::W15,
		Week::W16,
		Week::W17,
		Week::W18,
		Week::W19,
		Week::W20,
		Week::W21,
		Week::W22,
		Week::W23,
		Week::W24,
		Week::W25,
		Week::W26,
		Week::W27,
		Week::W28,
		Week::W29,
		Week::W30,
		Week::W31,
		Week::W32,
		Week::W33,
		Week::W34,
		Week::W35,
		Week::W36,
		Week::W37,
		Week::W38,
		Week::W39,
		Week::W40,
		Week::W41,
		Week::W42,
		Week::W43,
		Week::W44,
		Week::W45,
		Week::W46,
		Week::W47,
		Week::W48,
		Week::W49,
		Week::W50,
		Week::W51,
		Week::W52,
		Week::W53,
	];

	/// `n` w zakresie 1..=53; poza zakresem zwraca `W01`.
	fn from_number(n: i64) -> Self {
		usize::try_from(n - 1).ok().and_then(|i| Self::ALL.get(i)).copied().unwrap_or(Self::W01)
	}
}

// ------------------------------------------------------------ DNI W MIESIĄCU I ROKU - MAKRO
macro_rules! impl_try_from_day {
	($type:ty, $max:expr) => {
		impl TryFrom<u16> for $type {
			type Error = &'static str;

			fn try_from(day: u16) -> Result<Self, Self::Error> {
				if (1..=$max).contains(&day) {
					Ok(Self(NonZeroU16::new(day).unwrap()))
				} else {
					Err(concat!("Day must be in range 1..=", $max))
				}
			}
		}
	};
}

impl_try_from_day!(YearWith365Day, 365);
impl_try_from_day!(YearWith366Day, 366);
impl_try_from_day!(MonthWith31Day, 31);
impl_try_from_day!(MonthWith30Day, 30);
impl_try_from_day!(MonthWith29Day, 29);
impl_try_from_day!(MonthWith28Day, 28);
