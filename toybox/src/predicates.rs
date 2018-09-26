// Contains predicates that are useful for interventions.
// These can all be implemented over json, but exporting some fast
// ones from Rust will be helpful.

pub struct AmidarPredicates  {
    pub sim: Simulation
}
pub struct BreakoutPredicates {
    pub sim: Simulation
}

impl AmidarPredicates {
}

impl BreakoutPredicates {
    fn all_but_one() -> bool {
        
    }
}