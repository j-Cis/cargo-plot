use std::num::NonZeroU16;

use super::{
	consts::{SOLAR_EVENTS_Y0, SOLAR_EVENTS_Y2000, SOLAR_PERIODIC},
	enums::{MonthDay, Months, Quater, Season},
	structs::{MonthWith28Day, MonthWith29Day, MonthWith30Day, MonthWith31Day},
};
// ------------------------------------------------------------ FUNKCJE POMOCNICZE
/// `NonZeroU16` z wartości, która z konstrukcji algorytmów jest zawsze >= 1.
pub fn nz(v: u16) -> NonZeroU16 { NonZeroU16::new(v).unwrap_or(NonZeroU16::MIN) }

/// Dni od 1970-01-01 -> (rok, miesiąc 1..=12, dzień 1..=31).
pub fn civil_from_days(days: i64) -> (i64, i64, i64) {
	let z = days + 719_468;
	let era = z.div_euclid(146_097);
	let doe = z.rem_euclid(146_097);
	let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
	let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
	let mp = (5 * doy + 2) / 153;
	let d = doy - (153 * mp + 2) / 5 + 1;
	let m = if mp < 10 { mp + 3 } else { mp - 9 };
	let y = yoe + era * 400 + i64::from(m <= 2);
	(y, m, d)
}

/// (rok, miesiąc, dzień) -> dni od 1970-01-01.
pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
	let y = y - i64::from(m <= 2);
	let era = y.div_euclid(400);
	let yoe = y.rem_euclid(400);
	let mp = if m > 2 { m - 3 } else { m + 9 };
	let doy = (153 * mp + 2) / 5 + d - 1;
	let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
	era * 146_097 + doe - 719_468
}

pub fn is_leap(y: i64) -> bool { (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 }

pub fn days_in_month(y: i64, m: i64) -> i64 {
	match m {
		1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
		4 | 6 | 9 | 11 => 30,
		_ if is_leap(y) => 29,
		_ => 28,
	}
}

/// Dzień tygodnia ISO: poniedziałek = 1 ... niedziela = 7.
pub fn iso_weekday(days: i64) -> i64 { (days + 3).rem_euclid(7) + 1 }

/// (numer tygodnia, rok tygodnia). Tydzień 1 to pierwszy tydzień z co najmniej
/// 4 dniami w nowym roku. `sun0 = false`: tydzień zaczyna się w poniedziałek
/// (ISO 8601), `sun0 = true`: w niedzielę.
pub fn week_of(days: i64, sun0: bool) -> (i64, i64) {
	let iso = iso_weekday(days);
	let pos = if sun0 { iso % 7 } else { iso - 1 }; // 0..=6 od początku tygodnia
	let pivot = days + (3 - pos); // czwarty dzień tygodnia decyduje o roku tygodnia
	let (py, _, _) = civil_from_days(pivot);
	let pivot_doy = pivot - days_from_civil(py, 1, 1) + 1;
	((pivot_doy - 1) / 7 + 1, py)
}

pub fn month_day(len: i64, d: u16) -> MonthDay {
	match len {
		31 => MonthDay::M31D(MonthWith31Day(nz(d))),
		30 => MonthDay::M30D(MonthWith30Day(nz(d))),
		29 => MonthDay::M29D(MonthWith29Day(nz(d))),
		_ => MonthDay::M28D(MonthWith28Day(nz(d))),
	}
}

pub fn months_of(m: i64) -> Months {
	match m {
		1 => Months::Jan,
		2 => Months::Feb,
		3 => Months::Mar,
		4 => Months::Apr,
		5 => Months::May,
		6 => Months::Jun,
		7 => Months::Jul,
		8 => Months::Aug,
		9 => Months::Sep,
		10 => Months::Oct,
		11 => Months::Nov,
		_ => Months::Dec,
	}
}

pub fn quater_of(m: i64) -> Quater {
	match (m - 1) / 3 {
		0 => Quater::JanFebMar,
		1 => Quater::AprMayJun,
		2 => Quater::JulAugSep,
		_ => Quater::OctNovDec,
	}
}

/// Pora roku astronomiczna, półkula północna: wiosna od równonocy marcowej, lato od przesilenia
/// czerwcowego, jesień od równonocy wrześniowej, zima od przesilenia grudniowego.
/// Chwile zdarzeń liczone są dla roku `y` (Meeus, "Astronomical Algorithms", rozdz. 27)
/// i porównywane z chwilą `now_unix` (UTC), więc granica przechodzi w dokładnej minucie.
pub fn season(y: i64, now_unix: f64) -> Season {
	let [mar, jun, sep, dec] = solar_events_unix(y);
	if now_unix < mar {
		Season::Win
	} else if now_unix < jun {
		Season::Spr
	} else if now_unix < sep {
		Season::Sum
	} else if now_unix < dec {
		Season::Aut
	} else {
		Season::Win
	}
}

/// Chwile [równonoc marcowa, przesilenie czerwcowe, równonoc wrześniowa,
/// przesilenie grudniowe] w roku `y`, jako sekundy uniksowe UTC.
/// Algorytm jest opisany dla lat -1000..3000.
pub fn solar_events_unix(y: i64) -> [f64; 4] {
	let (table, yy) = if y < 1_000 {
		(SOLAR_EVENTS_Y0, y as f64 / 1_000.0)
	} else {
		(SOLAR_EVENTS_Y2000, (y - 2_000) as f64 / 1_000.0)
	};
	let delta_t = delta_t_seconds(y);

	table.map(|c| {
		let jde0 = c[0] + yy * (c[1] + yy * (c[2] + yy * (c[3] + yy * c[4])));
		let t = (jde0 - 2_451_545.0) / 36_525.0;
		let w = (35_999.373 * t - 2.47).to_radians();
		let dl = 1.0 + 0.0334 * w.cos() + 0.0007 * (2.0 * w).cos();
		let s: f64 = SOLAR_PERIODIC.iter().map(|&(a, b, k)| a * (b + k * t).to_radians().cos()).sum();
		let jde = jde0 + 0.00001 * s / dl; // czas dynamiczny (TT)
		(jde - 2_440_587.5) * 86_400.0 - delta_t // -> UTC, sekundy uniksowe
	})
}

/// Przybliżenie różnicy TT - UTC w sekundach (Espenak/Meeus).
/// Dla lat 2005..2050 wielomian, poza tym parabola długoterminowa (mniej dokładna).
pub fn delta_t_seconds(y: i64) -> f64 {
	let yf = y as f64 + 0.5;
	if (2_005..2_050).contains(&y) {
		let t = yf - 2_000.0;
		62.92 + 0.322_17 * t + 0.005_589 * t * t
	} else {
		let u = (yf - 1_820.0) / 100.0;
		-20.0 + 32.0 * u * u
	}
}
