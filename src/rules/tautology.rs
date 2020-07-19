use crate::datastructures::{Entailment, Pure::True, Rule, Spatial::Emp};

pub struct Tautology;
impl Rule for Tautology {
    fn predicate(&self, _goal: Entailment) -> bool {
        true
    }

    fn premisses(&self, mut goal: Entailment) -> Option<Vec<Entailment>> {
        if let Emp = goal.antecedent.get_spatial() {
            if let &mut Emp = goal.consequent.get_spatial() {
                if let &mut True = goal.consequent.get_pure() {
                    return Some(vec![]);
                }
            }
        }
        None
    }
}
