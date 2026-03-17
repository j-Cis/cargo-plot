// [EN]: Functions for creating consistent date and time stamps.
// [PL]: Funkcje do tworzenia spójnych sygnatur daty i czasu.

use chrono::{Datelike, Local, Timelike, Weekday};
pub use chrono::{NaiveDate, NaiveTime};

/// [EN]: Utility struct for generating consistent time tags.
/// [PL]: Struktura narzędziowa do generowania spójnych sygnatur czasowych.
pub struct TimeTag;

impl TimeTag {
    /// [EN]: Generates a time_tag for the current local time.
    /// [PL]: Generuje time_tag dla obecnego, lokalnego czasu.
    #[must_use]
    pub fn now() -> String {
        let now = Local::now();
        Self::format(now.date_naive(), now.time())
    }

    /// [EN]: Generates a time_tag for a specific provided date and time.
    /// [PL]: Generuje time_tag dla konkretnej, podanej daty i czasu.
    #[must_use]
    pub fn custom(date: NaiveDate, time: NaiveTime) -> String {
        Self::format(date, time)
    }

    // [EN]: Private function that performs manual string construction (DRY principle).
    // [PL]: PRYWATNA funkcja, która wykonuje ręczne budowanie ciągu znaków (zasada DRY).
    fn format(date: NaiveDate, time: NaiveTime) -> String {
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

        // [EN]: Format: YYYYQn Dnnn Wnn _ Day DD Mon _ HH MM SS mmm
        // [PL]: Format: RRRRQn Dnnn Wnn _ Dzień DD Miesiąc _ GG MM SS mmm
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
}
