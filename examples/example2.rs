use anyhow::Result;

use cargo_plot::timer::{Timer,WeekFirstDay};
fn main() -> Result<()> {
	let a = Timer::now();
	println!("now {a:?}");
	println!("now {:?}", a.fmt(WeekFirstDay::Mon));
	println!("Sformatowany string: {}", a.format(WeekFirstDay::Mon, "[WYYY]-W[WW]-[WDD]"));
	// #[repr(i64)]
	// enum SundayDayIn {
	// 	WeekIs0 = 0,
	// 	WeekIs7 = 7,
	// }
	// fn weekday(days: i64, sunday_day_in: SundayDayIn) -> i64 {
	// 	(days + 3).rem_euclid(7) + (sunday_day_in as i64 / 7)
	// }

	// let now = SystemTime::now();
	// let secs = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

	// let days = (secs / 86_400) as i64;
	// let time = secs % 86_400;

	// let now2 = jiff::Zoned::now();

	// println!("czas i ofset: {:?}", Timer::now());
	// println!("Offset w sekundach: {}", now2.offset().seconds());
	// println!("Strefa czasowa: {:?}", now2.time_zone());
	// //--------------
	// //  Dzień tygodnia ISO: poniedziałek = 1, niedziela = 7.
	// let day_in_week = (days + 3).rem_euclid(7) + 1;

	// println!("day_in_week {day_in_week:?}");
	// println!("now {now:?}");
	// println!("secs {secs:?}");
	// println!("days {days:?}");
	// println!("time {time:?}");
	Ok(())
}
