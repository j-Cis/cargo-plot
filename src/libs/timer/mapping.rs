use super::structs::T;

impl T {
    /// Formatuje strukturę `T` na podstawie podanego wzorca.
    pub fn format_string(&self, format: &str) -> String {
        let mut result = format.to_string();

        // [WYYY] - Rok tygodniowy (np. 1998)
        result = result.replace("[WYYY]", &format!("{:04}", self.year.week_year));

        // [WW] - Numer tygodnia (np. 23)
        // Zakładamy, że self.week to enum (np. W23), rzutujemy na u8/i64 lub pobieramy wartość.
        let week_num = self.week as u8; 
        result = result.replace("[WW]", &format!("{:02}", week_num));

        // [WDD] - Dzień tygodnia jako skrót (np. Thu)
        let weekday_str = match self.day.in_week {
            super::enums::WeekDay::Mon => "Mon",
            super::enums::WeekDay::Tue => "Tue",
            super::enums::WeekDay::Wed => "Wed",
            super::enums::WeekDay::Thu => "Thu",
            super::enums::WeekDay::Fri => "Fri",
            super::enums::WeekDay::Sat => "Sat",
            super::enums::WeekDay::Sun => "Sun",
        };
        result = result.replace("[WDD]", weekday_str);

        // Tutaj możesz dodać kolejne mapowania, np. [YYYY], [MM], [DD] itp.

        result
    }
}

// Pomocnicza metoda dla prostszego API w Timer
impl super::structs::Timer {
    pub fn format(&self, week_start: super::enums::WeekFirstDay, format_str: &str) -> String {
        // Generujemy pełne dane kalendarzowe T przekazując start tygodnia
        let t = super::structs::T::new(*self, week_start);
        t.format_string(format_str)
    }
}