use chrono::{Datelike, Local, Timelike};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagTime(pub String);

/// Generuje tag czasowy w postaci R<RRRR>W<WW>D<D>H<HH>M<MM>S<SS>T<TT>Q<QQ>
/// Gdzie T to Tercja (1/60 sekundy), a Q to Qwadra/Kwarta (1/60 tercji).
pub fn tag_time() -> TagTime {
	let now = Local::now();

	let r = now.year(); // R: Rok
	let w = now.iso_week().week(); // W: Tydzień ISO
	let d = now.weekday().number_from_monday(); // D: Dzień tygodnia (1-7)
	let h = now.hour(); // H: Godzina
	let m = now.minute(); // M: Minuta
	let s = now.second(); // S: Sekunda

	// Pobieramy nanosekundy jako bazę do obliczeń ułamków sekundy (od 0 do
	// 999_999_999)
	let nanos = now.nanosecond() as u64;

	// T: Tercja (1/60 sekundy). Zasięg 0-59.
	let t = (nanos * 60 / 1_000_000_000) as u32;

	// Q: Qwadra (1/60 tercji). Zasięg 0-59.
	// Najpierw wyliczamy, w której z 3600 kwart ułamka sekundy jesteśmy,
	// a potem bierzemy modulo 60, by uzyskać kwartę w obrębie bieżącej tercji.
	let q = ((nanos * 3600 / 1_000_000_000) % 60) as u32;

	TagTime(format!("R{r:04}W{w:02}D{d}H{h:02}M{m:02}S{s:02}T{t:02}Q{q:02}"))
}

//let date_str = Color::date(&row.modified.format("%Y W%V %a").to_string());
//let time_str = Color::time(&row.modified.format("%H:%M:%S.%3f").to_string());
