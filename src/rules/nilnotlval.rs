use crate::datastructures::{
    AtomSpatial::PointsTo,
    Entailment,
    Expr::{Nil, Var},
    Formula,
    Op::AtomNeq,
    Pure::And,
    Rule,
    Spatial::SepConj,
};

pub struct NilNotLVal;
impl Rule for NilNotLVal {
    fn predicate(&self, goal: &Entailment) -> bool {
        let mut add_new = false;
        let antecedent = &goal.antecedent;
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
            } else {
                ant_pure = And(vec![AtomNeq(points_to_to_add, Nil)]);
            }
        }

        Some(vec![Entailment {
            antecedent: Formula(ant_pure, ant_spatial),
            consequent,
        }])
    }
}

#[cfg(test)]
mod test {
    use super::NilNotLVal;
    use crate::datastructures::{
        AtomSpatial::PointsTo,
        Entailment, Expr,
        Expr::Nil,
        Formula,
        Op::AtomNeq,
        Pure::{And, True},
        Rule,
        Spatial::{Emp, SepConj},
    };

    #[test]
    pub fn test_nil_not_lval() -> Result<(), ()> {
        let goal_not_applicable = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomNeq(Expr::new_var("y"), Nil),
                    AtomNeq(Expr::new_var("x"), Nil),
                ]),
                SepConj(vec![
                    PointsTo(Expr::new_var("y"), Expr::new_var("x")),
                    PointsTo(Expr::new_var("x"), Expr::new_var("z")),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        assert_eq!(false, NilNotLVal.predicate(&goal_not_applicable));

        let goal1 = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(Expr::new_var("y"), Nil)]),
                SepConj(vec![
                    PointsTo(Expr::new_var("y"), Expr::new_var("x")),
                    PointsTo(Expr::new_var("x"), Expr::new_var("z")),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        assert!(NilNotLVal.predicate(&goal1));

        let goal_expected1 = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomNeq(Expr::new_var("y"), Nil),
                    AtomNeq(Expr::new_var("x"), Nil),
                ]),
                SepConj(vec![
                    PointsTo(Expr::new_var("y"), Expr::new_var("x")),
                    PointsTo(Expr::new_var("x"), Expr::new_var("z")),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        let premisses = NilNotLVal.premisses(goal1);
        if let Some(prem) = premisses {
            assert_eq!(1, prem.len());
            assert_eq!(goal_expected1, prem[0]);
        } else {
            return Err(());
        }

        let goal2 = Entailment {
            antecedent: Formula(
                True,
                SepConj(vec![
                    PointsTo(Expr::new_var("y"), Expr::new_var("x")),
                    PointsTo(Expr::new_var("x"), Expr::new_var("z")),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        assert!(NilNotLVal.predicate(&goal2));

        let goal_expected2 = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(Expr::new_var("y"), Nil)]),
                SepConj(vec![
                    PointsTo(Expr::new_var("y"), Expr::new_var("x")),
                    PointsTo(Expr::new_var("x"), Expr::new_var("z")),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        let premisses = NilNotLVal.premisses(goal2);
        if let Some(prem) = premisses {
            assert_eq!(1, prem.len());
            assert_eq!(goal_expected2, prem[0]);
            return Ok(());
        } else {
            return Err(());
        }
    }
}
