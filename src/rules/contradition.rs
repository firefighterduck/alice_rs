use crate::datastructures::{Entailment, Op::AtomNeq, Pure::And, Rule};

pub struct Contradiction;
impl Rule for Contradiction {
    fn predicate(&self, _goal: &Entailment) -> bool {
        true
    }

    fn premisses(&self, mut goal: Entailment) -> Option<Vec<Entailment>> {
        if let And(pure_sub) = goal.antecedent.get_pure() {
            if let Some(_) = pure_sub.iter().find(|&x| match x {
                AtomNeq(l, r) => *l == *r,
                _ => return false,
            }) {
                return Some(vec![]);
            }
        }
        None
    }
}

mod test {
    use super::Contradiction;
    use crate::datastructures::{
        Entailment,
        Expr::Var,
        Formula,
        Op::AtomNeq,
        Pure::{And, True},
        Rule,
        Spatial::Emp,
        Variable,
    };

    #[test]
    pub fn test_contradiction() -> Result<(), String> {
        let goal = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(
                    Var(Variable("y".to_string())),
                    Var(Variable("y".to_string())),
                )]),
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
                And(vec![AtomNeq(
                    Var(Variable("y".to_string())),
                    Var(Variable("x".to_string())),
                )]),
                Emp,
            ),
            consequent: Formula(True, Emp),
        };

        let premisses = Contradiction.premisses(goal2);
        if let Some(_) = premisses {
            return Err("Expected second test to fail!".to_string());
        }

        return Ok(());
    }
}
