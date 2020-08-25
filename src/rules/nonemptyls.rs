use crate::{
    datastructures::{
        AtomSpatial::{PointsTo, LS},
        Entailment,
        Op::AtomNeq,
        Pure::And,
        Rule,
        Spatial::SepConj,
    },
    misc::find_and_remove,
};
pub struct NonEmptyLS;

impl Rule for NonEmptyLS {
    fn predicate(&self, goal: &Entailment) -> bool {
        goal.is_normal_form()
    }

    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        let (mut antecedent, mut consequent) = goal.destroy();
        let mut pair_opt = None;
        if let SepConj(cons_spatials) = consequent.get_spatial() {
            if let And(ant_pures) = antecedent.get_pure() {
                if let SepConj(ant_spatials) = antecedent.get_spatial() {
                    'outer: for cons_spatial in cons_spatials {
                        if let LS(e1, e2) = cons_spatial {
                            'middle: for ant_pure in ant_pures {
                                if let AtomNeq(e3, e4) = ant_pure {
                                    if !((e1 == e4 && e2 == e3) || (e1 == e3 && e2 == e4)) {
                                        continue 'middle;
                                    }

                                    'inner: for ant_spatial in ant_spatials {
                                        if let PointsTo(e5, e6) = ant_spatial {
                                            if e1 != e5 {
                                                continue 'inner;
                                            }

                                            pair_opt = Some((e1.clone(), e6.clone(), e3.clone()));
                                            break 'outer;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some((e1, e2, e3)) = pair_opt {
            if let SepConj(ant_spatials) = antecedent.get_spatial_mut() {
                find_and_remove(ant_spatials, |spatial| {
                    if let PointsTo(l, r) = spatial {
                        *l == e1 && *r == e2
                    } else {
                        false
                    }
                });
            }
            if let SepConj(cons_spatials) = consequent.get_spatial_mut() {
                find_and_remove(cons_spatials, |spatial| {
                    if let LS(l, r) = spatial {
                        *l == e1 && *r == e3
                    } else {
                        false
                    }
                });
                cons_spatials.push(LS(e2, e3));
            }
            return Some(vec![Entailment {
                antecedent,
                consequent,
            }]);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::NonEmptyLS;
    use crate::datastructures::{
        AtomSpatial::{PointsTo, LS},
        Entailment, Expr,
        Expr::Nil,
        Formula,
        Op::AtomNeq,
        Pure::And,
        Rule,
        Spatial::SepConj,
    };

    #[test]
    fn test_nil_not_lval() -> Result<(), String> {
        let invalid_goal1 = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(Expr::new_var("x"), Nil)]),
                SepConj(vec![
                    PointsTo(Expr::new_var("z"), Expr::new_var("x")),
                    LS(Expr::new_var("x"), Nil),
                ]),
            ),
            consequent: Formula(
                And(vec![AtomNeq(Expr::new_var("x"), Nil)]),
                SepConj(vec![
                    LS(Expr::new_var("z"), Nil),
                    LS(Expr::new_var("y"), Nil),
                ]),
            ),
        };
        let premisses1 = NonEmptyLS.premisses(invalid_goal1);
        if let Some(_) = premisses1 {
            return Err("The first test should have failed!".to_string());
        }

        let invalid_goal2 = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomNeq(Expr::new_var("x"), Nil),
                    AtomNeq(Expr::new_var("y"), Expr::new_var("z")),
                ]),
                SepConj(vec![LS(Expr::new_var("x"), Nil)]),
            ),
            consequent: Formula(
                And(vec![AtomNeq(Expr::new_var("x"), Nil)]),
                SepConj(vec![
                    LS(Expr::new_var("z"), Nil),
                    LS(Expr::new_var("y"), Nil),
                ]),
            ),
        };
        let premisses2 = NonEmptyLS.premisses(invalid_goal2);
        if let Some(_) = premisses2 {
            return Err("The second test should have failed!".to_string());
        }

        let valid_goal = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomNeq(Expr::new_var("x"), Nil),
                    AtomNeq(Expr::new_var("y"), Expr::new_var("z")),
                ]),
                SepConj(vec![
                    PointsTo(Expr::new_var("z"), Expr::new_var("x")),
                    LS(Expr::new_var("x"), Nil),
                ]),
            ),
            consequent: Formula(
                And(vec![AtomNeq(Expr::new_var("x"), Nil)]),
                SepConj(vec![
                    LS(Expr::new_var("z"), Expr::new_var("y")),
                    LS(Expr::new_var("y"), Nil),
                ]),
            ),
        };
        let expected = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomNeq(Expr::new_var("x"), Nil),
                    AtomNeq(Expr::new_var("y"), Expr::new_var("z")),
                ]),
                SepConj(vec![LS(Expr::new_var("x"), Nil)]),
            ),
            consequent: Formula(
                And(vec![AtomNeq(Expr::new_var("x"), Nil)]),
                SepConj(vec![
                    LS(Expr::new_var("y"), Nil),
                    LS(Expr::new_var("x"), Expr::new_var("y")),
                ]),
            ),
        };

        let premisses3 = NonEmptyLS.premisses(valid_goal);
        if let Some(premisses) = premisses3 {
            assert_eq!(1, premisses.len());
            assert_eq!(expected, premisses[0]);
            Ok(())
        } else {
            Err("The third goal should have succeeded!".to_string())
        }
    }
}
