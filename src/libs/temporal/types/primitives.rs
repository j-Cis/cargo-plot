//! Path: `src/libs/temporal/types/primitives.rs`
//! Last significant modified: 2026-W43-1 13:57:43 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

use std::num::NonZeroU16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range1To12(pub NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range0To23(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range0To59(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range0To999(pub u16);
