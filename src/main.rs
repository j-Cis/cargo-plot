use clap::Parser;
// Importy z naszej biblioteki
use plot::lib::schema::CargoCliRoot;
use plot::lib::{mapping::route_and_execute, pipelines::{
	engine_step1_scanner,
	engine_step2_data_formater,
	engine_step6_data_save,
	engine_step7_data_view
}};

fn main() {
	// 0. trik deweloperski
	let mut args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] != "plot" {
        args.insert(1, "plot".to_string());
    }
	// AUTOMATYCZNA POMOC: Jeśli nie ma flag po 'plot', wymuszamy '-h'
    // args.len() == 1 -> odpalono po prostu `cargo run`
    // args.len() == 2 && args[1] == "plot" -> odpalono `cargo run -- plot` lub `cargo plot`
    if args.len() == 1 || (args.len() == 2 && args[1] == "plot") {
        if args.len() == 1 {
            args.push("plot".to_string());
        }
        args.push("-h".to_string());
    }
	// 1. Parsowanie surowych argumentów wejściowych od użytkownika
	let CargoCliRoot::Plot(cli) = CargoCliRoot::parse_from(args);

	// 2. Przekazanie flag do silnika decyzyjnego wraz z callbackiem (naszym potokiem)
	route_and_execute(cli, |ready_job| {


		// =========================================================================
		// STEP 1: Skanowanie i pobranie metadanych z dysku
		// =========================================================================
		let step1 = engine_step1_scanner(ready_job);


		// =========================================================================
		// STEP 2: Formatowanie komórek
		// =========================================================================
		let step2 = engine_step2_data_formater(ready_job, &step1);

		// =========================================================================
		// STEP 6: ZAPISYWANIE
		// =========================================================================
		let _ = engine_step6_data_save(ready_job, &step2).unwrap();

		// =========================================================================
		// STEP 7: WYŚWIETLANIE W TERMINALU (OPCJONALNE, zależnie od flag)
		// =========================================================================

		let _ = engine_step7_data_view(ready_job, &step2).unwrap();

		let modes = ready_job.run_modes();
        if !modes.contains(&plot::lib::schema::SharedJobRunMode::PrintNothing) 
            && !modes.contains(&plot::lib::schema::SharedJobRunMode::PrintOnlyWarning) 
        {
            println!("\n=== KONIEC ===\n");
        }
		
		true
	});
}
