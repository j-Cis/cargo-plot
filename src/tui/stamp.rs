use cliclack::{confirm, input, intro, outro};
use lib::fn_datestamp::{NaiveDate, NaiveTime, datestamp, datestamp_now};

pub fn run_stamp_flow() {
    intro(" 🕒 Generator Sygnatur Czasowych ").unwrap();

    let custom = confirm("Czy chcesz podać własną datę i czas?")
        .initial_value(false)
        .interact()
        .unwrap();

    if custom {
        let d_str: String = input("Data (RRRR-MM-DD):")
            .placeholder("2026-03-10")
            .interact()
            .unwrap();

        let t_str: String = input("Czas (GG:MM:SS):")
            .placeholder("14:30:00")
            .interact()
            .unwrap();

        let d = NaiveDate::parse_from_str(&d_str, "%Y-%m-%d").expect("Błędny format daty");
        let t = NaiveTime::parse_from_str(&t_str, "%H:%M:%S").expect("Błędny format czasu");

        let s = datestamp(d, t);
        outro(format!("Wygenerowana sygnatura: {}", s)).unwrap();
    } else {
        let s = datestamp_now();
        outro(format!("Aktualna sygnatura: {}", s)).unwrap();
    }
}
