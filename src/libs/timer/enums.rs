use super::structs::{MonthWith28Day, MonthWith29Day, MonthWith30Day, MonthWith31Day, YearWith365Day, YearWith366Day};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekFirstDay {
	Mon,
	Sun,
}

// ------------------------------------------------------------ TYDZIEŃ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Week {
	W01 = 1,
	W02 = 2,
	W03 = 3,
	W04 = 4,
	W05 = 5,
	W06 = 6,
	W07 = 7,
	W08 = 8,
	W09 = 9,
	W10 = 10,
	W11 = 11,
	W12 = 12,
	W13 = 13,
	W14 = 14,
	W15 = 15,
	W16 = 16,
	W17 = 17,
	W18 = 18,
	W19 = 19,
	W20 = 20,
	W21 = 21,
	W22 = 22,
	W23 = 23,
	W24 = 24,
	W25 = 25,
	W26 = 26,
	W27 = 27,
	W28 = 28,
	W29 = 29,
	W30 = 30,
	W31 = 31,
	W32 = 32,
	W33 = 33,
	W34 = 34,
	W35 = 35,
	W36 = 36,
	W37 = 37,
	W38 = 38,
	W39 = 39,
	W40 = 40,
	W41 = 41,
	W42 = 42,
	W43 = 43,
	W44 = 44,
	W45 = 45,
	W46 = 46,
	W47 = 47,
	W48 = 48,
	W49 = 49,
	W50 = 50,
	W51 = 51,
	W52 = 52,
	W53 = 53,
}

// ------------------------------------------------------------ MIESIĄC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Months {
	Jan = 1,
	Feb = 2,
	Mar = 3,
	Apr = 4,
	May = 5,
	Jun = 6,
	Jul = 7,
	Aug = 8,
	Sep = 9,
	Oct = 10,
	Nov = 11,
	Dec = 12,
}

// ------------------------------------------------------------ DZIEŃ W TYGODNIU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekDay {
	Mon,
	Tue,
	Wed,
	Thu,
	Fri,
	Sat,
	Sun,
}

// ------------------------------------------------------------ DZIEŃ W ROKU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearDay {
	Y365D(YearWith365Day),
	Y366D(YearWith366Day),
}

// ------------------------------------------------------------ DZIEŃ W MIESIĄCU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthDay {
	M31D(MonthWith31Day),
	M30D(MonthWith30Day),
	M29D(MonthWith29Day),
	M28D(MonthWith28Day),
}

// ------------------------------------------------------------ PORA ROKU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season {
	Aut = 1,
	Win = 2,
	Spr = 3,
	Sum = 4,
}
// ------------------------------------------------------------ ĆWIARTKA ROKU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quater {
	JanFebMar = 1,
	AprMayJun = 2,
	JulAugSep = 3,
	OctNovDec = 4,
}
