use crate::datastructures::{Entailment, Op::AtomNeq, Pure::And, Rule};

/// Π ∧ E!=E | Σ  |-  Π' | Σ'
pub struct Contradiction;
impl Rule for Contradiction {
    fn predicate(&self, _goal: &Entailment) -> bool {
        true
    }

    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        if let And(pure_sub) = goal.antecedent.get_pure() {
            if pure_sub.iter().any(|x| match x {
                AtomNeq(l, r) => l == r,
                _ => false,
            }) {
                return Some(vec![]);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::Contradiction;
    use crate::datastructures::{
        Entailment, Expr, Formula,
        Op::AtomNeq,
        Pure::{And, True},
        Rule,
        Spatial::Emp,
    };

    #[test]
    pub fn test_contradiction() -> Result<(), String> {
        let goal = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(Expr::new_var("y"), Expr::new_var("y"))]),
                Emp,
            ),
            consequent: Formula(True, Emp),
        };

        let premisses = Contradiction.premisses(goal);
        if let Some(prem) = premisses {
            assert_eq!(0, prem.len());
        } else {
            return Err("Expected first test to succeed!".to_string());
        }

        let goal2 = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(Expr::new_var("y"), Expr::new_var("x"))]),
                Emp,
            ),
            consequent: Formula(True, Emp),
        };

        let premisses = Contradiction.premisses(goal2);
        if premisses.is_some() {
            return Err("Expected second test to fail!".to_string());
        }

        Ok(())
    }
}
