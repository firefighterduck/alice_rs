use crate::datastructures::{
    AtomSpatial::{PointsTo, LS},
    Entailment, Rule,
    Spatial::SepConj,
};
pub struct Frame;

impl Rule for Frame {
    fn predicate(&self, goal: &Entailment) -> bool {
        goal.is_normal_form()
    }

    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        let (mut antecedent, mut consequent) = goal.destroy();

        let mut indices = None;
        if let SepConj(spatial_ant_vec) = antecedent.get_spatial() {
            if let SepConj(spatial_cons_vec) = consequent.get_spatial() {
                'outer: for (i, o_spat) in spatial_ant_vec.iter().enumerate() {
                    '_inner: for (j, i_spat) in spatial_cons_vec.iter().enumerate() {
                        match o_spat {
                            LS(o_l, o_r) => {
                                if let LS(i_l, i_r) = i_spat {
                                    if (*o_l == *i_l && *o_r == *i_r)
                                        || (*o_l == *i_r && *o_r == *i_l)
                                    {
                                        indices = Some((i, j));
                                        break 'outer;
                                    }
                                }
                            }
                            PointsTo(o_l, o_r) => {
                                if let PointsTo(i_l, i_r) = i_spat {
                                    if (*o_l == *i_l && *o_r == *i_r)
                                        || (*o_l == *i_r && *o_r == *i_l)
                                    {
                                        indices = Some((i, j));
                                        break 'outer;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };

        if let Some((i, j)) = indices {
            if let SepConj(spatial_ant_vec) = antecedent.get_spatial_mut() {
                if let SepConj(spatial_cons_vec) = consequent.get_spatial_mut() {
                    spatial_ant_vec.remove(i);
                    spatial_cons_vec.remove(j);
                    return Some(vec![Entailment {
                        antecedent,
                        consequent,
                    }]);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::Frame;
    use crate::datastructures::{
        AtomSpatial::{PointsTo, LS},
        Entailment, Expr,
        Expr::Nil,
        Formula,
        Pure::True,
        Rule,
        Spatial::SepConj,
    };

    #[test]
    fn test_hypothesis() -> Result<(), String> {
        let goal1 = Entailment {
            antecedent: Formula(True, SepConj(vec![LS(Expr::new_var("x"), Nil)])),
            consequent: Formula(True, SepConj(vec![PointsTo(Expr::new_var("x"), Nil)])),
        };

        let premisses1 = Frame.premisses(goal1);
        if let Some(_) = premisses1 {
            return Err("Expected first test to fail!".to_string());
        }

        let goal2 = Entailment {
            antecedent: Formula(
                True,
                SepConj(vec![
                    LS(Expr::new_var("x"), Nil),
                    PointsTo(Expr::new_var("z"), Nil),
                ]),
            ),
            consequent: Formula(True, SepConj(vec![LS(Expr::new_var("x"), Nil)])),
        };
        let goal2_expected = Entailment {
            antecedent: Formula(True, SepConj(vec![PointsTo(Expr::new_var("z"), Nil)])),
            consequent: Formula(True, SepConj(vec![])),
        };

        let premisses2 = Frame.premisses(goal2);
        if let Some(prem) = premisses2 {
            assert_eq!(1, prem.len());
            assert_eq!(goal2_expected, prem[0]);
            Ok(())
        } else {
            Err("Expected third test to succeed!".to_string())
        }
    }
}
