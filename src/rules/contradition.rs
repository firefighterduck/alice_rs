use crate::datastructures::{Entailment, Op::AtomNeq, Pure::And, Rule};

pub struct Contradiction;
impl Rule for Contradiction {
    fn predicate(&self, _goal: Entailment) -> bool {
        true
    }

    fn premisses(&self, mut goal: Entailment) -> Option<Vec<Entailment>> {
        if let And(pure_sub) = goal.consequent.get_pure() {
            if let Some(_) = pure_sub.iter().find(|&x| match x {
                AtomNeq(l, r) => l == r,
                _ => return false,
            }) {
                return Some(vec![]);
            }
        }
        None
    }
}
