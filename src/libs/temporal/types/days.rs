//! Path: `src/libs/temporal/types/days.rs`
//! Last significant modified: 2026-W43-1 13:57:43 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::num::NonZeroU16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearWith365Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearWith366Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith31Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith30Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith29Day(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthWith28Day(pub NonZeroU16);

macro_rules! impl_try_from_day {
	($type:ty, $max:expr) => {
		impl TryFrom<u16> for $type {
			type Error = &'static str;

			fn try_from(day: u16) -> Result<Self, Self::Error> {
				if (1..=$max).contains(&day) {
					Ok(Self(NonZeroU16::new(day).unwrap()))
				} else {
					Err(concat!("Day must be in range 1..=", $max))
				}
			}
		}
	};
}

impl_try_from_day!(YearWith365Day, 365);
impl_try_from_day!(YearWith366Day, 366);
impl_try_from_day!(MonthWith31Day, 31);
impl_try_from_day!(MonthWith30Day, 30);
impl_try_from_day!(MonthWith29Day, 29);
impl_try_from_day!(MonthWith28Day, 28);
