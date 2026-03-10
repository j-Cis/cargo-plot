// ./lib/fn_datestamp.rs
use chrono::{Datelike, Local, Timelike, Weekday};
pub use chrono::{NaiveDate, NaiveTime};

/// Generuje datestamp dla obecnego, lokalnego czasu.
/// Wywołanie: `datestamp_now()`
pub fn datestamp_now() -> String {
    let now = Local::now();
    format_datestamp(now.date_naive(), now.time())
}

/// Generuje datestamp dla konkretnej, podanej daty i czasu.
/// Wywołanie: `datestamp(date, time)`
pub fn datestamp(date: NaiveDate, time: NaiveTime) -> String {
    format_datestamp(date, time)
}

/// PRYWATNA funkcja, która odwala całą brudną robotę (zasada DRY).
/// Nie ma modyfikatora `pub`, więc jest niewidoczna poza tym plikiem.
fn format_datestamp(date: NaiveDate, time: NaiveTime) -> String {
    let year = date.year();
    let quarter = ((date.month() - 1) / 3) + 1;

    let weekday = match date.weekday() {
        Weekday::Mon => "Mon",
        Weekday::Tue => "Tue",
        Weekday::Wed => "Wed",
        Weekday::Thu => "Thu",
        Weekday::Fri => "Fri",
        Weekday::Sat => "Sat",
        Weekday::Sun => "Sun",
    };

    let month = match date.month() {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => unreachable!(),
    };

    let millis = time.nanosecond() / 1_000_000;

    format!(
        "{}Q{}D{:03}W{:02}_{}{:02}{}_{:02}{:02}{:02}{:03}",
        year,
        quarter,
        date.ordinal(),
        date.iso_week().week(),
        weekday,
        date.day(),
        month,
        time.hour(),
        time.minute(),
        time.second(),
        millis
    )
}
