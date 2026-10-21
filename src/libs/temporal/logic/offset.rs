//! Path: `src/libs/temporal/logic/offset.rs`
//! Last significant modified: 2026-W43-1 13:57:43 CEST (UTC+02:00)
//! Author: Jan Roman Cisowski (j-Cis, code@cisowscy.com)
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

/// Oblicza bazowy czas na podstawie globalnych nanosekund i lokalnego offsetu.
pub fn apply_offset(nanos_total: u128, offset_seconds: i32) -> (i64, u16, u32) {
	let total_secs_utc = (nanos_total / 1_000_000_000) as i64;
	let subsec_nanos = (nanos_total % 1_000_000_000) as u32;

	let base_secs = total_secs_utc + (offset_seconds as i64);

	// Używamy dzielenia euklidesowego dla bezpieczeństwa dat ujemnych (przed
	// 1970)
	let days = base_secs.div_euclid(86400);
	let rem_secs = base_secs.rem_euclid(86400) as u16;

	(days, rem_secs, subsec_nanos)
}
