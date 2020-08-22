use crate::datastructures::{Entailment, Rule};
pub struct NonEmptyLS;

impl Rule for NonEmptyLS {
    fn predicate(&self, goal: &Entailment) -> bool {
        goal.is_normal_form()
    }
    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        todo!()
    }
}
