use anyhow::Result;
use cargo_plot::timer::Timer;
fn main() -> Result<()> {
	let a = Timer::now();
	// println!("now {a:?}");
	// println!("now {:?}", a.fmt(WeekFirstDay::Mon));
	println!("Sformatowany string: {}", a.format("[YW]-W[WW]-[D][DDD] [hh]:[mm]:[ss]:[ssmil]"));

	Ok(())
}
