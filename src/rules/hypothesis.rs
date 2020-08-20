use crate::datastructures::{
    Entailment,
    Op::{AtomEq, AtomNeq},
    Pure::And,
    Rule,
};
pub struct Hypothesis;

impl Rule for Hypothesis {
    fn predicate(&self, _goal: &Entailment) -> bool {
        true
    }

    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        let (mut antecedent, mut consequent) = goal.destroy();

        let mut indices = None;
        if let And(pure_ant_vec) = antecedent.get_pure() {
            if let And(pure_cons_vec) = consequent.get_pure() {
                'outer: for (i, o_op) in pure_ant_vec.iter().enumerate() {
                    '_inner: for (j, i_op) in pure_cons_vec.iter().enumerate() {
                        match o_op {
                            AtomEq(o_l, o_r) => {
                                if let AtomEq(i_l, i_r) = i_op {
                                    if (*o_l == *i_l && *o_r == *i_r)
                                        || (*o_l == *i_r && *o_r == *i_l)
                                    {
                                        indices = Some((i, j));
                                        break 'outer;
                                    }
                                }
                            }
                            AtomNeq(o_l, o_r) => {
                                if let AtomNeq(i_l, i_r) = i_op {
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
            if let And(pure_ant_vec) = antecedent.get_pure_mut() {
                if let And(pure_cons_vec) = consequent.get_pure_mut() {
                    pure_ant_vec.remove(i);
                    pure_cons_vec.remove(j);
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
    use super::Hypothesis;
    use crate::datastructures::{
        Entailment,
        Expr::{Nil, Var},
        Formula,
        Op::{AtomEq, AtomNeq},
        Pure::And,
        Rule,
        Spatial::Emp,
        Variable,
    };

    #[test]
    fn test_hypothesis() -> Result<(), String> {
        let goal1 = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomEq(Var(Variable("y".to_string())), Nil),
                    AtomNeq(Nil, Var(Variable("z".to_string()))),
                ]),
                Emp,
            ),
            consequent: Formula(
                And(vec![
                    AtomEq(Nil, Nil),
                    AtomNeq(Nil, Var(Variable("x".to_string()))),
                ]),
                Emp,
            ),
        };

        let premisses1 = Hypothesis.premisses(goal1);
        if let Some(_) = premisses1 {
            return Err("Expected first test to fail!".to_string());
        }

        let goal2 = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomEq(Var(Variable("x".to_string())), Nil),
                    AtomNeq(Nil, Var(Variable("z".to_string()))),
                ]),
                Emp,
            ),
            consequent: Formula(
                And(vec![
                    AtomEq(Nil, Nil),
                    AtomEq(Nil, Var(Variable("x".to_string()))),
                ]),
                Emp,
            ),
        };
        let goal2_expected = Entailment {
            antecedent: Formula(And(vec![AtomNeq(Nil, Var(Variable("z".to_string())))]), Emp),
            consequent: Formula(And(vec![AtomEq(Nil, Nil)]), Emp),
        };

        let premisses2 = Hypothesis.premisses(goal2);
        if let Some(prem) = premisses2 {
            assert_eq!(1, prem.len());
            assert_eq!(goal2_expected, prem[0]);
            Ok(())
        } else {
            Err("Expected third test to succeed!".to_string())
        }
    }
}
