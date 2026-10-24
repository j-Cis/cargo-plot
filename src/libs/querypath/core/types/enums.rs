//! Path: `src/libs/querypath/types/enums.rs`
//! Repository: https://github.com/j-Cis/cargo-plot.git
//! ORCID: https://orcid.org/0009-0009-2907-3792

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumNodeGroup {
	Directory,
	TextFile,
	BinaryFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumGroupStrategy {
	Mixed,
	Custom(Vec<EnumNodeGroup>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumSameNamePriority {
	DirectoryFirst,
	FileFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumNoExtPriority {
	Above,
	Below,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumCharClass {
	Special,
	Digit,
	Letter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumCasePrecedence {
	Insensitive,
	UpperFirst,
	LowerFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumColumn {
	Weight,
	Temporal,
	Path,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumUnitSystem {
	Binary,  // IEC: B, KiB, MiB, GiB, TiB (dzielnik 1024)
	Decimal, // SI:  B, KB, MB, GB, TB   (dzielnik 1000)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumDirWeightDisplay {
	None,
	MatchedOnly,
	RealOnly,
	Both, // np. "12.5 MiB / 140.2 MiB"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumWeightPrecision {
	Tenths,     // Dokładność do części dziesiętnych (np. 12.5 MiB)
	Hundredths, // Dokładność do części setnych (np. 12.54 MiB)
}
