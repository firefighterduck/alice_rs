use crate::datastructures::{Entailment, Rule};
pub struct NonEmptyLS;

impl Rule for NonEmptyLS {
    fn predicate(&self, goal: &Entailment) -> bool {
        todo!()
    }
    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        todo!()
    }
}
