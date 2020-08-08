use crate::datastructures::{Entailment, Pure::True, Rule, Spatial::Emp};

pub struct Tautology;
impl Rule for Tautology {
    fn predicate(&self, _goal: &Entailment) -> bool {
        true
    }

    fn premisses(&self, mut goal: Entailment) -> Option<Vec<Entailment>> {
        if let Emp = goal.antecedent.get_spatial() {
            if let &Emp = goal.consequent.get_spatial() {
                if let &True = goal.consequent.get_pure() {
                    return Some(vec![]);
                }
            }
        }
        None
    }
}

mod test {
    use super::Tautology;
    use crate::datastructures::{
        Entailment,
        Expr::{Nil, Var},
        Formula,
        Op::{AtomEq, AtomNeq},
        Pure::{And, True},
        Rule,
        Spatial::{Emp, SepConj},
        Variable,
    };

    #[test]
    pub fn test_tautology() -> Result<(), String> {
        let goal1 = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomNeq(
                        Var(Variable("y".to_string())),
                        Var(Variable("y".to_string())),
                    ),
                    AtomEq(Nil, Nil),
                ]),
                Emp,
            ),
            consequent: Formula(True, Emp),
        };

        let premisses = Tautology.premisses(goal1);
        if let Some(prem) = premisses {
            assert_eq!(0, prem.len());
        } else {
            return Err("Expected first test to succed!".to_string());
        }

        let goal2 = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomNeq(
                        Var(Variable("y".to_string())),
                        Var(Variable("y".to_string())),
                    ),
                    AtomEq(Nil, Nil),
                ]),
                Emp,
            ),
            consequent: Formula(True, SepConj(vec![])),
        };

        let premisses = Tautology.premisses(goal2);
        if let Some(_) = premisses {
            return Err("Expected second test to fail!".to_string());
        }

        return Ok(());
    }
}
