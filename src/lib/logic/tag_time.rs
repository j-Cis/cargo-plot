use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Weekday};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagTime(pub String);

// =====================================================================
// ZASADY / REGUŁY (Czysta matematyka czasu)
// =====================================================================

/// Konwertuje nanosekundy (0 - 999_999_999) na Tercje i Qwadry.
/// Zwraca krotkę: (tercja, qwadra)
fn nanos_to_tq(nanos: u32) -> (u32, u32) {
	let nanos_u64 = nanos as u64;
	let t = (nanos_u64 * 60 / 1_000_000_000) as u32;
	let q = ((nanos_u64 * 3600 / 1_000_000_000) % 60) as u32;
	(t, q)
}

/// Konwertuje Tercje i Qwadry z powrotem na nanosekundy.
/// Używane podczas rekonstrukcji czasu z tagu.
fn tq_to_nanos(t: u32, q: u32) -> u32 {
	let total_q = (t as u64 * 60) + q as u64;
	// Odwrócenie wzoru z zachowaniem maksymalnej precyzji:
	((total_q * 1_000_000_000) / 3600) as u32
}

// =====================================================================
// CZYSTE FUNKCJE MAPUJĄCE (Pure Functions)
// =====================================================================

/// Generuje TagTime na podstawie dowolnego obiektu czasu.
/// Akceptuje wszystko, co implementuje Datelike i Timelike (np. DateTime<Local>, NaiveDateTime).
pub fn tag_from_time<T: Datelike + Timelike>(time: &T) -> TagTime {
	// Używamy roku ISO, aby utworzona z tygodniem ISO para (Rok, Tydzień) była jednoznaczna.
	let r = time.iso_week().year();
	let w = time.iso_week().week();
	let d = time.weekday().number_from_monday();

	let h = time.hour();
	let m = time.minute();
	let s = time.second();

	let (t, q) = nanos_to_tq(time.nanosecond());

	TagTime(format!("R{r:04}W{w:02}D{d}H{h:02}M{m:02}S{s:02}T{t:02}Q{q:02}"))
}

/// Wygodny wrapper: Pobiera aktualny czas systemowy i od razu zwraca TagTime.
pub fn tag_from_time_now() -> TagTime {
	let now = Local::now();
	tag_from_time(&now)
}

/// Parsuje TagTime z powrotem do naiwnego obiektu czasu (NaiveDateTime).
/// Ponieważ sam tag nie przechowuje strefy czasowej, zwracamy czas "naiwny".
pub fn tag_to_time(tag: &TagTime) -> Option<NaiveDateTime> {
	let s = &tag.0;

	// RRRRWWDHHMMSSTTQQ + litery = dokładnie 25 znaków
	// np. "R2023W42D1H14M30S15T20Q15"
	if s.len() != 25 {
		return None;
	}

	// Bezpieczne, pozycyjne wyciąganie danych z gwarantowanej struktury ASCII
	let r: i32 = s[1..5].parse().ok()?;
	let w: u32 = s[6..8].parse().ok()?;
	let d: u32 = s[9..10].parse().ok()?;
	let h: u32 = s[11..13].parse().ok()?;
	let m: u32 = s[14..16].parse().ok()?;
	let sec: u32 = s[17..19].parse().ok()?;
	let t: u32 = s[20..22].parse().ok()?;
	let q: u32 = s[23..25].parse().ok()?;

	// Odtworzenie daty (Rok ISO, Tydzień ISO, Dzień tygodnia)
	let weekday = match d {
		1 => Weekday::Mon,
		2 => Weekday::Tue,
		3 => Weekday::Wed,
		4 => Weekday::Thu,
		5 => Weekday::Fri,
		6 => Weekday::Sat,
		7 => Weekday::Sun,
		_ => return None,
	};
	let date = NaiveDate::from_isoywd_opt(r, w, weekday)?;

	// Odtworzenie czasu
	let nanos = tq_to_nanos(t, q);
	let time = NaiveTime::from_hms_nano_opt(h, m, sec, nanos)?;

	Some(NaiveDateTime::new(date, time))
}
