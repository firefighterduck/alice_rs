use crate::{
    datastructures::{Entailment, Op::AtomEq, Pure::And, Rule},
    misc::find_and_remove,
};

pub struct EqReflexiveL;
impl Rule for EqReflexiveL {
    fn predicate(&self, _goal: crate::datastructures::Entailment) -> bool {
        true
    }

    fn premisses(
        &self,
        goal: crate::datastructures::Entailment,
    ) -> Option<Vec<crate::datastructures::Entailment>> {
        let (mut antecedent, consequent) = goal.destroy();

        if let And(pure_vec) = antecedent.get_pure_mut() {
            if let Some(_) = find_and_remove(pure_vec, move |x| match x {
                AtomEq(l, r) => *l == *r,
                _ => false,
            }) {
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
    use super::EqReflexiveL;
    use crate::datastructures::{
        Entailment,
        Expr::{Nil, Var},
        Formula,
        Op::{AtomEq, AtomNeq},
        Pure::{And, True},
        Rule,
        Spatial::Emp,
        Variable,
    };

    #[test]
    fn test_EqReflexiveL() {
        let goal1 = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomEq(Nil, Nil),
                    AtomNeq(Nil, Var(Variable("x".to_string()))),
                ]),
                Emp,
            ),
            consequent: Formula(True, Emp),
        };
        let goal1_expected = Entailment {
            antecedent: Formula(And(vec![AtomNeq(Nil, Var(Variable("x".to_string())))]), Emp),
            consequent: Formula(True, Emp),
        };

        let premisses1 = EqReflexiveL.premisses(goal1);
        if let Some(prem) = premisses1 {
            assert_eq!(1, prem.len());
            assert_eq!(goal1_expected, prem[0]);
        } else {
            assert!(false);
        }

        let goal2 = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomEq(Nil, Nil),
                    AtomEq(Nil, Var(Variable("x".to_string()))),
                ]),
                Emp,
            ),
            consequent: Formula(True, Emp),
        };
        let goal2_expected = Entailment {
            antecedent: Formula(And(vec![AtomEq(Nil, Var(Variable("x".to_string())))]), Emp),
            consequent: Formula(True, Emp),
        };

        let premisses2 = EqReflexiveL.premisses(goal2);
        if let Some(prem) = premisses2 {
            assert_eq!(1, prem.len());
            assert_eq!(goal2_expected, prem[0]);
        } else {
            assert!(false);
        }

        let goal3 = Entailment {
            antecedent: Formula(
                And(vec![AtomEq(
                    Var(Variable("x".to_string())),
                    Var(Variable("x".to_string())),
                )]),
                Emp,
            ),
            consequent: Formula(True, Emp),
        };
        let goal3_expected = Entailment {
            antecedent: Formula(And(vec![]), Emp),
            consequent: Formula(True, Emp),
        };

        let premisses3 = EqReflexiveL.premisses(goal3);
        if let Some(prem) = premisses3 {
            assert_eq!(1, prem.len());
            assert_eq!(goal3_expected, prem[0]);
        } else {
            assert!(false);
        }
    }
}
