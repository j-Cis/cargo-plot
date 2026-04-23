use crate::lib::job::{self, TraitConfigToFlags};

impl job::schema::ValidPreparedJobParams {
	/// Generuje plik TOML będący odwrotnością parsowania (eksportuje flagi)
	pub fn to_toml(&self, _id: &str, _name: Option<&str>, _description: Option<&str>) -> String {
		use std::fmt::Write; // Wymagane, aby writeln! działało na Stringu

		let i = "job";
		let mut out = String::from("# Cargo-Plot Configuration File\n\n");

		// TWOJE ZMIENNE TESTOWE (Przywrócone)
		let only_test_id = "default_job";
		let only_test_name = "snapshot-project";
		let only_test_description = "【ENG】 Standard Rust project snapshot / 【POL】 Standardowy zrzut projektu Rust";

		// Nagłówek zadania - {} dla braku cudzysłowów, {:?} dla wartości
		let _ = writeln!(out, "[[{}]]", i);
		let _ = writeln!(out, "id = {:?}", only_test_id);
		let _ = writeln!(out, "name = {:?}", only_test_name);
		let _ = writeln!(out, "description = {:?}", only_test_description);
		let _ = writeln!(out, "# [quiet, mute, dry, verbose, save, color]");
		let _ = writeln!(out, "run_mode = {:?}\n", self.exec.to_flags());

		// Sekcja SCAN
		let _ = writeln!(out, "[{}.explorer]", i);
		let _ = writeln!(out, "workspace = {:?}", self.work.workspace_raw.as_str());
		let _ = writeln!(out, "ignore_case = {}", self.work.ignore_case);
		let _ = writeln!(out, "patterns = {:?}", self.patt.patterns);
		let _ = writeln!(out, "# [MD,MF,XD,XF]");
		let _ = writeln!(out, "parts = {:?}\n", self.part.to_flags());

		// Sekcja TABLE
		let _ = writeln!(out, "[{}.attributes]", i);
		let _ = writeln!(out, "select = {:?}\n", self.cols.to_flags());

		// Sekcja COLUMNś
		let _ = writeln!(out, "[{}.attributes.config]", i);
		let _ = writeln!(out, "item = {:?}", self.item.to_flags());
		let _ = writeln!(out, "date = {:?}", "default");
		let _ = writeln!(out, "time = {:?}", "default");
		let _ = writeln!(
			out,
			"size = {:?}\n",
			if self.size.mode == job::schema::ValidColumnSizeFlags::Decimal { "decimal" } else { "binary" }
		);

		let _ = writeln!(out, "[{}.tuples]", i);
		let _ = writeln!(out, "sort = {:?}\n", self.sort.to_flags());

		// Sekcja SAVE (Odkomentowana i sformatowana)
		let _ = writeln!(out, "[{}.tuples.save]", i);
		let _ = writeln!(out, "out_dir = {:?}", self.save.out_dir);
		let _ = writeln!(out, "title = {:?}", self.save.title);
		let _ = writeln!(out, "name = {:?}", self.save.name);
		let _ = writeln!(out, "name_is_prefix = {}", self.save.name_is_prefix);
		let _ = writeln!(out, "not_separately = {}\n", self.save.not_separately); 

		out
	}
}
