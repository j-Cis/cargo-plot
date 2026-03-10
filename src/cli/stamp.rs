// Plik: src/cli/stamp.rs
use crate::cli::args::StampArgs;
use lib::fn_datestamp::{NaiveDate, NaiveTime, datestamp, datestamp_now};

pub fn handle_stamp(args: StampArgs) {
    if let (Some(d_str), Some(t_str)) = (args.date, args.time) {
        let d = NaiveDate::parse_from_str(&d_str, "%Y-%m-%d").expect("Błędny format daty");
        let t = NaiveTime::parse_from_str(&format!("{}.{}", t_str, args.millis), "%H:%M:%S%.3f")
            .expect("Błędny format czasu");
        println!("{}", datestamp(d, t));
    } else {
        println!("{}", datestamp_now());
    }
}