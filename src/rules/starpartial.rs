use crate::datastructures::{
    AtomSpatial::PointsTo, Entailment, Expr::Nil, Formula, Op::AtomNeq, Pure::And, Rule,
    Spatial::SepConj,
};

pub struct StarPartial;
impl Rule for StarPartial {
    fn predicate(&self, goal: &Entailment) -> bool {
        let mut add_new = false;
        let antecedent = &goal.antecedent;
        if let SepConj(atom_spatials) = antecedent.get_spatial() {
            if atom_spatials.len() < 2 {
                return false;
            }

            if let And(pure_ops) = antecedent.get_pure() {
                let first_vec = atom_spatials.as_slice();
                'outer: for first_ptf in first_vec {
                    'inner: for second_ptf in atom_spatials {
                        if let PointsTo(l1, _) = first_ptf {
                            if let PointsTo(l2, _) = second_ptf {
                                if *l1 == *l2 {
                                    continue 'inner;
                                }

                                if let Some(_) = pure_ops.iter().find(move |op| match op {
                                    &AtomNeq(le, re) => {
                                        (*le == *l1 && *re == *l2) || (*re == *l1 && *le == *l2)
                                    }
                                    _ => false,
                                }) {
                                    continue 'inner;
                                } else {
                                    add_new = true;
                                    break 'outer;
                                }
                            }
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
        let mut neq_to_add = AtomNeq(Nil, Nil);

        if let SepConj(atom_spatials) = &ant_spatial {
            if let And(pure_ops) = &ant_pure {
                'outer: for first_ptf in atom_spatials.as_slice() {
                    'inner: for second_ptf in atom_spatials.as_slice() {
                        if let PointsTo(l1, _) = first_ptf {
                            if let PointsTo(l2, _) = second_ptf {
                                if *l1 == *l2 {
                                    continue 'inner;
                                }

                                if let Some(_) = pure_ops.iter().find(move |op| match op {
                                    &AtomNeq(le, re) => {
                                        (*le == *l1 && *re == *l2) || (*re == *l1 && *le == *l2)
                                    }
                                    _ => false,
                                }) {
                                    continue 'inner;
                                } else {
                                    neq_to_add = AtomNeq(l1.clone(), l2.clone());
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            } else {
                let mut first = None;
                let mut second = None;
                for atom_spatial in atom_spatials {
                    if let PointsTo(l, _) = atom_spatial {
                        if let &Some(_) = &first {
                            second = Some(l.clone());
                            break;
                        }
                        first = Some(l.clone());
                    }
                }
                neq_to_add = AtomNeq(first.unwrap(), second.unwrap());
            }
        }

        if let And(pure_ops) = &mut ant_pure {
            pure_ops.push(neq_to_add);
        } else {
            ant_pure = And(vec![neq_to_add]);
        }

        Some(vec![Entailment {
            antecedent: Formula(ant_pure, ant_spatial),
            consequent,
        }])
    }
}

#[cfg(test)]
mod test {
    use super::StarPartial;
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
    pub fn test_star_partial() -> Result<(), ()> {
        let goal_not_applicable = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomNeq(Expr::new_var("z"), Nil),
                    AtomNeq(Expr::new_var("y"), Expr::new_var("x")),
                ]),
                SepConj(vec![
                    PointsTo(Expr::new_var("y"), Expr::new_var("x")),
                    PointsTo(Expr::new_var("x"), Expr::new_var("z")),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        assert_eq!(false, StarPartial.predicate(&goal_not_applicable));

        let goal1 = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(Expr::new_var("z"), Nil)]),
                SepConj(vec![
                    PointsTo(Expr::new_var("y"), Expr::new_var("x")),
                    PointsTo(Expr::new_var("x"), Expr::new_var("z")),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        assert!(StarPartial.predicate(&goal1));

        let goal_expected1 = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomNeq(Expr::new_var("z"), Nil),
                    AtomNeq(Expr::new_var("y"), Expr::new_var("x")),
                ]),
                SepConj(vec![
                    PointsTo(Expr::new_var("y"), Expr::new_var("x")),
                    PointsTo(Expr::new_var("x"), Expr::new_var("z")),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        let premisses = StarPartial.premisses(goal1);
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

        assert!(StarPartial.predicate(&goal2));

        let goal_expected2 = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(Expr::new_var("y"), Expr::new_var("x"))]),
                SepConj(vec![
                    PointsTo(Expr::new_var("y"), Expr::new_var("x")),
                    PointsTo(Expr::new_var("x"), Expr::new_var("z")),
                ]),
            ),
            consequent: Formula(True, Emp),
        };

        let premisses = StarPartial.premisses(goal2);
        if let Some(prem) = premisses {
            assert_eq!(1, prem.len());
            assert_eq!(goal_expected2, prem[0]);
            Ok(())
        } else {
            Err(())
        }
    }
}
