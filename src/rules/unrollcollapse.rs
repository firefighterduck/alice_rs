use crate::{
    datastructures::{
        AtomSpatial::{PointsTo, LS},
        Entailment,
        Expr::Var,
        Formula,
        Op::{AtomEq, AtomNeq},
        Pure::And,
        Rule,
        Spatial::SepConj,
        Variable,
    },
    misc::find_and_remove,
};

pub struct UnrollCollapse;

impl Rule for UnrollCollapse {
    fn predicate(&self, _goal: &Entailment) -> bool {
        true
    }

    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        let (antecedent, consequent) = goal.destroy();
        let (ant_pure, mut ant_spatial) = antecedent.destroy();
        if let SepConj(ref mut spatial_vec) = ant_spatial {
            if let Some(ls) = find_and_remove(spatial_vec, |sp| sp.is_ls()) {
                if let LS(l, r) = ls {
                    if let Var(Variable(name_l)) = &l {
                        let mut new_pure1 = ant_pure.clone();
                        if let And(ref mut pure_vec) = new_pure1 {
                            pure_vec.push(AtomEq(l.clone(), r.clone()));
                        } else {
                            new_pure1 = And(vec![AtomEq(l.clone(), r.clone())]);
                        }
                        let new_antecedent1 = Formula(new_pure1, ant_spatial.clone());
                        let new_goal1 = Entailment {
                            antecedent: new_antecedent1,
                            consequent: consequent.clone(),
                        };

                        let new_x = Var(Variable(name_l.clone() + "x"));
                        let mut new_pure2 = ant_pure;
                        if let And(ref mut pure_vec) = new_pure2 {
                            pure_vec.push(AtomNeq(l.clone(), r.clone()));
                            pure_vec.push(AtomNeq(new_x.clone(), r.clone()));
                        } else {
                            new_pure2 = And(vec![
                                AtomNeq(l.clone(), r.clone()),
                                AtomNeq(new_x.clone(), r.clone()),
                            ]);
                        }
                        ant_spatial = ant_spatial.add(PointsTo(l, new_x.clone()));
                        ant_spatial = ant_spatial.add(PointsTo(new_x, r));
                        let new_goal2 = Entailment {
                            antecedent: Formula(new_pure2, ant_spatial),
                            consequent: consequent,
                        };

                        return Some(vec![new_goal1, new_goal2]);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::UnrollCollapse;
    use crate::datastructures::{
        AtomSpatial::{PointsTo, LS},
        Entailment,
        Expr::Nil,
        Expr::Var,
        Formula,
        Op::{AtomEq, AtomNeq},
        Pure::{And, True},
        Rule,
        Spatial::SepConj,
        Variable,
    };
    #[test]
    pub fn test_unrollcollapse() {
        let goal = Entailment {
            antecedent: Formula(True, SepConj(vec![LS(Var(Variable("z".to_string())), Nil)])),
            consequent: Formula(
                And(vec![AtomEq(Nil, Var(Variable("x".to_string())))]),
                SepConj(vec![LS(Var(Variable("x".to_string())), Nil)]),
            ),
        };

        let expected = vec![
            Entailment {
                antecedent: Formula(
                    And(vec![AtomEq(Var(Variable("z".to_string())), Nil)]),
                    SepConj(vec![]),
                ),
                consequent: Formula(
                    And(vec![AtomEq(Nil, Var(Variable("x".to_string())))]),
                    SepConj(vec![LS(Var(Variable("x".to_string())), Nil)]),
                ),
            },
            Entailment {
                antecedent: Formula(
                    And(vec![
                        AtomNeq(Var(Variable("z".to_string())), Nil),
                        AtomNeq(Var(Variable("zx".to_string())), Nil),
                    ]),
                    SepConj(vec![
                        PointsTo(
                            Var(Variable("z".to_string())),
                            Var(Variable("zx".to_string())),
                        ),
                        PointsTo(Var(Variable("zx".to_string())), Nil),
                    ]),
                ),
                consequent: Formula(
                    And(vec![AtomEq(Nil, Var(Variable("x".to_string())))]),
                    SepConj(vec![LS(Var(Variable("x".to_string())), Nil)]),
                ),
            },
        ];

        if let Some(premisses) = UnrollCollapse.premisses(goal) {
            for (expected, actual) in expected.iter().zip(premisses) {
                assert_eq!(*expected, actual);
            }
        }
    }
}
