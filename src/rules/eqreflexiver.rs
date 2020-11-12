use crate::datastructures::{Entailment, Op::AtomEq, Pure::And, Rule};
use crate::misc::find_and_remove;

/// Π | Σ  |-  Π' | Σ' ==> Π | Σ  |-  Π' ∧ E=E | Σ'
pub struct EqReflexiveR;

impl Rule for EqReflexiveR {
    fn predicate(&self, _goal: &Entailment) -> bool {
        true
    }

    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        let (antecedent, mut consequent) = goal.destroy();

        if let And(pure_vec) = consequent.get_pure_mut() {
            if find_and_remove(pure_vec, move |x| match x {
                AtomEq(l, r) => l == r,
                _ => false,
            })
            .is_some()
            {
                return Some(vec![Entailment {
                    antecedent,
                    consequent,
                }]);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::EqReflexiveR;
    use crate::datastructures::{
        Entailment, Expr,
        Expr::Nil,
        Formula,
        Op::{AtomEq, AtomNeq},
        Pure::{And, True},
        Rule,
        Spatial::Emp,
    };

    #[test]
    fn test_eq_reflexive_l() -> Result<(), String> {
        let goal1 = Entailment {
            antecedent: Formula(True, Emp),
            consequent: Formula(
                And(vec![AtomEq(Nil, Nil), AtomNeq(Nil, Expr::new_var("x"))]),
                Emp,
            ),
        };
        let goal1_expected = Entailment {
            antecedent: Formula(True, Emp),
            consequent: Formula(And(vec![AtomNeq(Nil, Expr::new_var("x"))]), Emp),
        };

        let premisses1 = EqReflexiveR.premisses(goal1);
        if let Some(prem) = premisses1 {
            assert_eq!(1, prem.len());
            assert_eq!(goal1_expected, prem[0]);
        } else {
            return Err("Expected first test to succeed!".to_string());
        }

        let goal2 = Entailment {
            antecedent: Formula(True, Emp),
            consequent: Formula(
                And(vec![AtomEq(Nil, Nil), AtomEq(Nil, Expr::new_var("x"))]),
                Emp,
            ),
        };
        let goal2_expected = Entailment {
            antecedent: Formula(True, Emp),
            consequent: Formula(And(vec![AtomEq(Nil, Expr::new_var("x"))]), Emp),
        };

        let premisses2 = EqReflexiveR.premisses(goal2);
        if let Some(prem) = premisses2 {
            assert_eq!(1, prem.len());
            assert_eq!(goal2_expected, prem[0]);
        } else {
            return Err("Expected second test to succeed!".to_string());
        }

        let goal3 = Entailment {
            antecedent: Formula(True, Emp),
            consequent: Formula(
                And(vec![AtomEq(Expr::new_var("x"), Expr::new_var("x"))]),
                Emp,
            ),
        };
        let goal3_expected = Entailment {
            antecedent: Formula(True, Emp),
            consequent: Formula(And(vec![]), Emp),
        };

        let premisses3 = EqReflexiveR.premisses(goal3);
        if let Some(prem) = premisses3 {
            assert_eq!(1, prem.len());
            assert_eq!(goal3_expected, prem[0]);
            Ok(())
        } else {
            Err("Expected third test to succeed!".to_string())
        }
    }
}
