// [EN]: Simple stats object to avoid manual counting in the Engine.
// [PL]: Prosty obiekt statystyk, aby uniknąć ręcznego liczenia w Engine.
#[derive(Debug, Default, Clone)]
pub struct MatchStats {
    pub matched: usize,
    pub rejected: usize,
    pub total: usize,
    pub included: Vec<String>,
    pub excluded: Vec<String>,
}
