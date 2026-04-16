// use super::{DrawTree, Icon};
// use crate::lib::logic::{MatchLabel, Partition, PartitioningResult};
//
// impl<L: MatchLabel> std::fmt::Display for Partition<L> {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		writeln!(f, "{}", self.label)?;
// 		for path in &self.paths {
// 			writeln!(f, "{} {}", DrawTree::ITEM, path)?;
// 		}
// 		Ok(())
// 	}
// }
//
// impl std::fmt::Display for PartitioningResult {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		// Pierwsza linia z enterem
// 		writeln!(f, "{} {} Matched (m): {}", DrawTree::list(0, 3), Icon::BOOL_TRUE, self.m.paths.len())?;
// 		// Ostatnia linia BEZ entera i bez żadnych wiszących spacji (ident)
// 		write!(f, "{} {} Mismatched (x): {}", DrawTree::list(2, 3), Icon::BOOL_FALSE, self.x.paths.len())
// 	}
// }
