use crate::datastructures::{
    AtomSpatial::PointsTo, Entailment, Expr::Nil, Expr::Var, Formula, Op::AtomNeq, Pure::And, Rule,
    Spatial::SepConj,
};

pub struct NilNotLVal;
impl Rule for NilNotLVal {
    fn predicate(&self, goal: crate::datastructures::Entailment) -> bool {
        let mut add_new = false;
        let (antecedent, _) = goal.destroy();
        if let SepConj(atom_spatials) = antecedent.get_spatial() {
            let points_to_facts = atom_spatials.iter().filter(move |x| x.is_points_to());

            if let And(pure_ops) = antecedent.get_pure() {
                for points_to_fact in points_to_facts {
                    if let PointsTo(l, _) = points_to_fact {
                        if let Some(_) = pure_ops.iter().find(move |op| match op {
                            &AtomNeq(le, re) => {
                                (*le == *l && *re == Nil) || (*re == *l && *le == Nil)
                            }
                            _ => false,
                        }) {
                            continue;
                        } else {
                            add_new = true;
                            break;
                        }
                    }
                }
            } else {
                add_new = true;
            }
        }
        add_new
    }

    fn premisses(
        &self,
        goal: crate::datastructures::Entailment,
    ) -> Option<Vec<crate::datastructures::Entailment>> {
        let (antecedent, consequent) = goal.destroy();
        let (mut ant_pure, ant_spatial) = antecedent.destroy();

        let points_to_to_add = if let SepConj(points_to_facts) = &ant_spatial {
            if let Some(PointsTo(nonnil, _)) = points_to_facts.iter().find(|ptf| {
                if let PointsTo(l, _) = ptf {
                    if let And(pure_ops) = &ant_pure {
                        if let Some(_) = pure_ops.iter().find(|&op| match op {
                            AtomNeq(le, re) => {
                                (*le == *l && *re == Nil) || (*re == *l && *le == Nil)
                            }
                            _ => false,
                        }) {
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                } else {
                    false
                }
            }) {
                nonnil.clone()
            } else {
                Nil
            }
        } else {
            Nil
        };

        if let &Var(_) = &points_to_to_add {
            if let And(pure_ops) = &mut ant_pure {
                pure_ops.push(AtomNeq(points_to_to_add, Nil));
            }
        }

        Some(vec![Entailment {
            antecedent: Formula(ant_pure, ant_spatial),
            consequent,
        }])
    }
}

mod test {
    use super::NilNotLVal;
    use crate::datastructures::{
        AtomSpatial::PointsTo,
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
    pub fn test_nil_not_lval() -> Result<(), ()> {
        let goal = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(Var(Variable("y".to_string())), Nil)]),
                SepConj(vec![
                    PointsTo(
                        Var(Variable("y".to_string())),
                        Var(Variable("x".to_string())),
                    ),
                    PointsTo(
                        Var(Variable("x".to_string())),
                        Var(Variable("z".to_string())),
                    ),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        let goal_expected = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomNeq(Var(Variable("y".to_string())), Nil),
                    AtomNeq(Var(Variable("x".to_string())), Nil),
                ]),
                SepConj(vec![
                    PointsTo(
                        Var(Variable("y".to_string())),
                        Var(Variable("x".to_string())),
                    ),
                    PointsTo(
                        Var(Variable("x".to_string())),
                        Var(Variable("z".to_string())),
                    ),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        let premisses = NilNotLVal.premisses(goal);
        if let Some(prem) = premisses {
            assert_eq!(1, prem.len());
            assert_eq!(goal_expected, prem[0]);
            return Ok(());
        } else {
            return Err(());
        }
    }
}
