use std::{
	num::NonZeroU16,
	// ops::{Add, Sub},
	time::{/*Duration,*/ SystemTime, UNIX_EPOCH},
};

use super::{
	consts::WEEK_FIRST_DAY_IS_MON_HARDCODED,
	enums::{MonthDay, Months, Season, Week, WeekDay, WeekFirstDay, YearDay},
	functions::{
		civil_from_days,
		days_from_civil,
		days_in_month,
		is_leap,
		iso_weekday,
		month_day,
		months_of,
		nz,
		quater_of,
		season,
		week_of,
	},
	structs::{
		Cycle,
		Day,
		Month,
		MonthWith28Day,
		MonthWith29Day,
		MonthWith30Day,
		MonthWith31Day,
		Range0To23,
		Range0To59,
		Range0To999,
		Range1To12,
		T,
		Time,
		Timer,
		Year,
		YearWith365Day,
		YearWith366Day,
	},
};

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
	pub fn format_without_format(self /*week_start: WeekFirstDay*/) -> T {
		T::new(self /*week_start*/)
	}
	pub fn format(&self, /*week_start: WeekFirstDay,*/ pattern: &str) -> String {
		// Przekazujemy self (kopia Timera) i start tygodnia do budowy danych kalendarzowych
		let t = T::new(*self /*week_start*/);
		// Zgodnie ze strukturą Timer(pub i64, pub u32, pub u32, pub i32), offset jest pod indeksem 3
		t.format_string(pattern, self.3 /*week_start*/)
	}
	/// Tworzy `Timer` z czasu ostatniej modyfikacji pliku.
	pub fn from_path_modified(path: impl AsRef<std::path::Path>) -> Self {
		let st = std::fs::metadata(path).and_then(|m| m.modified()).unwrap_or(UNIX_EPOCH);
		Self::from_system_time(st)
	}
	/// Tworzy strukturę `Timer` z systemowego czasu modyfikacji pliku (`std::time::SystemTime`).
	pub fn from_system_time(st: SystemTime) -> Self {
		let duration = st.duration_since(UNIX_EPOCH).unwrap_or_default();
		Self::from_secs_and_nanos(duration.as_secs(), duration.subsec_nanos())
	}

	/// Tworzy strukturę `Timer` ze znacznika czasu w sekundach UNIX (np. pobranego z metadanych).
	pub fn from_secs(timestamp_secs: u64) -> Self { Self::from_secs_and_nanos(timestamp_secs, 0) }

	/// Tworzy strukturę `Timer` ze znacznika czasu w milisekundach UNIX.
	pub fn from_millis(timestamp_millis: u64) -> Self {
		let secs = timestamp_millis / 1_000;
		let nanos = (timestamp_millis % 1_000) as u32 * 1_000_000;
		Self::from_secs_and_nanos(secs, nanos)
	}

	/// Prywatna funkcja pomocnicza przeliczająca całkowity czas UTC i aplikująca lokalny offset.
	fn from_secs_and_nanos(total_secs_utc: u64, subsec_nanos: u32) -> Self {
		let offset_secs = jiff::Zoned::now().offset().seconds() as i32;
		let base_secs = (total_secs_utc as i64) + i64::from(offset_secs);

		let days = base_secs.div_euclid(86_400);
		let rem_secs = base_secs.rem_euclid(86_400) as u32;

		Self(days, rem_secs, subsec_nanos, offset_secs)
	}
	/// Tworzy `Timer` ze znacznika czasu w mikrosekundach UNIX.
	pub fn from_micros(timestamp_micros: u64) -> Self {
		let secs = timestamp_micros / 1_000_000;
		let nanos = (timestamp_micros % 1_000_000) as u32 * 1_000;
		Self::from_secs_and_nanos(secs, nanos)
	}

	/// Tworzy `Timer` ze znacznika czasu w nanosekundach UNIX.
	pub fn from_nanos(timestamp_nanos: u128) -> Self {
		let secs = (timestamp_nanos / 1_000_000_000) as u64;
		let nanos = (timestamp_nanos % 1_000_000_000) as u32;
		Self::from_secs_and_nanos(secs, nanos)
	}
	pub fn to_unix_secs(&self) -> i64 { self.0 * 86_400 + i64::from(self.1) - i64::from(self.3) }

	pub fn to_unix_nanos(&self) -> i128 { (self.to_unix_secs() as i128) * 1_000_000_000 + i128::from(self.2) }
}

// Implementacja Traitu TryFrom dla SystemTime -> Timer
impl TryFrom<SystemTime> for Timer {
	type Error = std::time::SystemTimeError;

	fn try_from(st: SystemTime) -> Result<Self, Self::Error> {
		let duration = st.duration_since(UNIX_EPOCH)?;
		Ok(Self::from_secs_and_nanos(duration.as_secs(), duration.subsec_nanos()))
	}
}

// ------------------------------------------------------------ KONSTRUKCJA T  Timer
impl T {
	/// Buduje komplet danych kalendarzowych i czasu z `Timer`
	/// (dni od 1970-01-01 w czasie lokalnym, sekundy doby, nanosekundy).
	pub fn new(timer: Timer /* week_start: WeekFirstDay*/) -> Self {
		let Timer(days, rem_secs, subsec_nanos, offset_secs) = timer;
		// Chwila w UTC (sekundy uniksowe), potrzebna do porównania z równonocami i przesileniami.

		// Zmienna pomocnicza do funkcji week_of()
		/* let sun0 = week_start == WeekFirstDay::Sun;*/
		let sun0 = WEEK_FIRST_DAY_IS_MON_HARDCODED == WeekFirstDay::Sun;

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

	/// Formatuje strukturę `T` na podstawie podanego wzorca (jednoprzebiegowy parser).
	pub fn format_string(&self, pattern: &str, offset_seconds: i32 /*week_start: WeekFirstDay*/) -> String {
		let mut out = String::with_capacity(pattern.len() + 16);
		let mut remaining = pattern;

		let day_num = match self.day.in_month {
			MonthDay::M31D(d) => d.0.get(),
			MonthDay::M30D(d) => d.0.get(),
			MonthDay::M29D(d) => d.0.get(),
			MonthDay::M28D(d) => d.0.get(),
		};

		let month_num = self.month.month as u32;

		// Baza to ISO (Poniedziałek = 1 ... Niedziela = 7)
		let iso_weekday_num = match self.day.in_week {
			WeekDay::Mon => 1,
			WeekDay::Tue => 2,
			WeekDay::Wed => 3,
			WeekDay::Thu => 4,
			WeekDay::Fri => 5,
			WeekDay::Sat => 6,
			WeekDay::Sun => 7,
		};

		// Dopasowanie do ogólnoświatowych standardów:
		// Jeśli startujemy w poniedziałek (ISO): 1-7
		// Jeśli startujemy w niedzielę (Cron/POSIX): Niedziela to 0, Poniedziałek to 1...
		let weekday_num = match /*week_start*/ WEEK_FIRST_DAY_IS_MON_HARDCODED {
			WeekFirstDay::Mon => iso_weekday_num,
			WeekFirstDay::Sun => {
				if iso_weekday_num == 7 {
					0
				} else {
					iso_weekday_num
				}
			}
		};
		let day_in_year_num = match self.day.in_year {
			YearDay::Y365D(d) => d.0.get(),
			YearDay::Y366D(d) => d.0.get(),
		};

		while !remaining.is_empty() {
			// 1. Podwojone nawiasy traktujemy jako ucieczkę do znaku dosłownego
			if let Some(rest) = remaining.strip_prefix("[[") {
				out.push('[');
				remaining = rest;
				continue;
			}
			if let Some(rest) = remaining.strip_prefix("]]") {
				out.push(']');
				remaining = rest;
				continue;
			}

			// 2. Blok tokenu [TOKEN]
			if let Some(rest) = remaining.strip_prefix('[') {
				if let Some(end_idx) = rest.find(']') {
					let token = &rest[..end_idx];
					remaining = &rest[end_idx + 1..];

					match token {
						// Strefa
						"ZONE" => {
							let sign = if offset_seconds >= 0 { '+' } else { '-' };
							let abs_offset = offset_seconds.abs();
							out.push_str(&format!(
								"{}{:02}{:02}UTC",
								sign,
								abs_offset / 3600,
								(abs_offset % 3600) / 60
							));
						}

						// Rok
						"GGGG" => out.push_str(&format!("{:04}", self.year.week_year)),
						"YYYY" => out.push_str(&format!("{:04}", self.year.year)),
						"GG" => out.push_str(&format!("{:02}", (self.year.week_year % 100).abs())),
						"YY" => out.push_str(&format!("{:02}", (self.year.year % 100).abs())),
						// "OOO" => out.push_str(&format!("{:03}", self.year.cycle.cycl400)),

						// Miesiąc
						"MM" => out.push_str(&format!("{:02}", month_num)),
						"MMM" => out.push_str(match self.month.month {
							Months::Jan => "JAN",
							Months::Feb => "FEB",
							Months::Mar => "MAR",
							Months::Apr => "APR",
							Months::May => "MAY",
							Months::Jun => "JUN",
							Months::Jul => "JUL",
							Months::Aug => "AUG",
							Months::Sep => "SEP",
							Months::Oct => "OCT",
							Months::Nov => "NOV",
							Months::Dec => "DEC",
						}),

						// Dzień
						"D" => out.push_str(&weekday_num.to_string()),
						"DM" => out.push_str(&format!("{:02}", day_num)),
						"DDY" => out.push_str(&format!("{:03}", day_in_year_num)),
						"DDD" => out.push_str(match self.day.in_week {
							WeekDay::Mon => "MON",
							WeekDay::Tue => "TUE",
							WeekDay::Wed => "WED",
							WeekDay::Thu => "THU",
							WeekDay::Fri => "FRI",
							WeekDay::Sat => "SAT",
							WeekDay::Sun => "SUN",
						}),

						// Tydzień
						"WW" => out.push_str(&format!("{:02}", self.week as u32)),

						// Pora roko oraz kwartał
						"SSS" => out.push_str(match self.year.season {
							Season::Aut => "AUT",
							Season::Win => "WIN",
							Season::Spr => "SPR",
							Season::Sum => "SUM",
						}),
						"S" => out.push_str(&(self.year.season as u32).to_string()),
						"Q" => out.push_str(&(self.year.quater as u32).to_string()),

						// Czas
						"hh" => out.push_str(&format!("{:02}", self.time.hour24.0)),
						"hhhh" => {
							let h12 = self.time.hour12.0.get();
							let ampm = if self.time.hour_is_pm { "pm" } else { "am" };
							out.push_str(&format!("{}{:02}", ampm, h12));
						}
						// 1/60
						"mm" => out.push_str(&format!("{:02}", self.time.minute_in_hou.0)),
						"ss" => out.push_str(&format!("{:02}", self.time.second_in_min.0)),
						"tt" => out.push_str(&format!("{:02}", self.time.tierce_in_sec.0)),
						"qa" => out.push_str(&format!("{:02}", self.time.quadra_in_tie.0)),
						"qi" => out.push_str(&format!("{:02}", self.time.quinta_in_qua.0)),
						"sx" => out.push_str(&format!("{:02}", self.time.sextus_in_qui.0)),
						"sp" => out.push_str(&format!("{:02}", self.time.septim_in_sex.0)),
						// 1/1000
						"ssmil" => out.push_str(&format!("{:03}ms", self.time.ms_in_second.0)),
						"ssmic" => out.push_str(&format!("{:03}µs", self.time.us_in_millis.0)),
						"ssnan" => out.push_str(&format!("{:03}ns", self.time.ns_in_micros.0)),

						// Nieznany token wewnątrz [...] – przepisujemy go dosłownie wraz z nawiasami
						unknown => {
							out.push('[');
							out.push_str(unknown);
							out.push(']');
						}
					}
					continue;
				}
			}

			// Przepisz zwykły znak
			if let Some(c) = remaining.chars().next() {
				out.push(c);
				remaining = &remaining[c.len_utf8()..];
			}
		}

		out
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
