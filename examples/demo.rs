// ./examples/demo.rs

use plot::lib::logic::{TabColumn, DocEngine, MX, TabSortBy, TabSortOrder};

fn main() {
	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
	let a1 = ".";
	let a2 = vec!["./{.rustfmt,Cargo}.toml&/", "./{src,examples,tests}/**{/*.rs,/}&/"];
	// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

	let view_row = (TabSortBy::Name, TabSortOrder::Desc, true);
	let view_col = &[
		TabColumn::Date,
		TabColumn::Time,
		TabColumn::Size,
		TabColumn::TreeList,
		TabColumn::Icon,
		TabColumn::Number,
		TabColumn::Path,
	];

	// Zobacz jakie to teraz czyste! Cała logika zamknięta w jednym potężnym
	// łańcuchu.
	DocEngine::new(a1, a2, true, view_row, view_col)
        .view(MX::M, false, false)
		.save_structure_of_the_content("./docs/raport", Some("abcd"))
        .view(MX::M, true, false)
		.save_content_of_the_structure("./docs/raport",Some("efgh"));

	// 2. Od razu płynnie zrzut tabeli ODRZUCONEJ (X), z limitem do 10, bez
	//    statystyk, z promo!
	//.view(MX::X, false, true);
}
