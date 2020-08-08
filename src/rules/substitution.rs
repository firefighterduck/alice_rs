use crate::{
    datastructures::{
        AtomSpatial,
        AtomSpatial::{PointsTo, LS},
        Entailment, Expr,
        Expr::{Nil, Var},
        Formula, Op,
        Op::{AtomEq, AtomNeq},
        Pure,
        Pure::{And, True},
        Rule, Spatial,
        Spatial::{Emp, SepConj},
    },
    misc::find_and_remove,
};

pub struct Substitution;
impl Substitution {
    fn subst_impl(subst: &(String, Expr), x: &Expr) -> Expr {
        match x {
            Var(v) => {
                if v.0 == subst.0 {
                    subst.1.clone()
                } else {
                    Var(v.clone())
                }
            }
            Nil => Nil,
        }
    }

    fn subst_pure(subst: &(String, Expr), p: &mut Pure) -> Pure {
        match p {
            And(pure_sub) => {
                let pure_vec: Vec<Op> = pure_sub
                    .iter_mut()
                    .map(|x| match x {
                        AtomEq(l, r) => {
                            AtomEq(Self::subst_impl(&subst, l), Self::subst_impl(&subst, r))
                        }
                        AtomNeq(l, r) => {
                            AtomNeq(Self::subst_impl(&subst, l), Self::subst_impl(&subst, r))
                        }
                    })
                    .collect();
                And(pure_vec)
            }
            True => True,
        }
    }

    fn subst_atom_spatial(subst: &(String, Expr), sp: &mut AtomSpatial) -> AtomSpatial {
        match sp {
            PointsTo(v, e) => PointsTo(Self::subst_impl(subst, v), Self::subst_impl(subst, e)),
            LS(v, e) => LS(Self::subst_impl(subst, v), Self::subst_impl(subst, e)),
        }
    }

    fn subst_spatial(subst: &(String, Expr), sp: &mut Spatial) -> Spatial {
        match sp {
            SepConj(atom_spatials) => SepConj(
                atom_spatials
                    .iter_mut()
                    .map(move |asp| Self::subst_atom_spatial(subst, asp))
                    .collect::<Vec<AtomSpatial>>(),
            ),

            Emp => Emp,
        }
    }

    fn substitute(subst: (String, Expr), goal: Entailment) -> Entailment {
        let (mut antecedent, mut consequent) = goal.destroy();

        let new_pure_ant = Self::subst_pure(&subst, antecedent.get_pure_mut());

        let new_spatial_ant = Self::subst_spatial(&subst, antecedent.get_spatial_mut());

        let new_pure_cons = Self::subst_pure(&subst, consequent.get_pure_mut());

        let new_spatial_cons = Self::subst_spatial(&subst, consequent.get_spatial_mut());

        Entailment {
            antecedent: Formula(new_pure_ant, new_spatial_ant),
            consequent: Formula(new_pure_cons, new_spatial_cons),
        }
    }
}
impl Rule for Substitution {
    fn predicate(&self, _goal: &Entailment) -> bool {
        true
    }

    fn premisses(&self, mut goal: Entailment) -> Option<Vec<Entailment>> {
        if let And(pure_sub) = goal.antecedent.get_pure_mut() {
            if let Some(elem) = find_and_remove(pure_sub, move |x| x.is_eq()) {
                let mut subst = ("".to_string(), Nil);

                if let AtomEq(l, r) = elem {
                    if let Var(v) = l {
                        subst = (v.0.clone(), r.clone());
                    } else if let Var(v) = r {
                        subst = (v.0.clone(), l.clone());
                    }

                    return Some(vec![Self::substitute(subst, goal)]);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::Substitution;
    use crate::datastructures::{
        AtomSpatial::{PointsTo, LS},
        Entailment,
        Expr::{Nil, Var},
        Formula,
        Op::{AtomEq, AtomNeq},
        Pure::And,
        Rule,
        Spatial::SepConj,
        Variable,
    };

    #[test]
    pub fn test_substitute() {
        let goal = Entailment {
            antecedent: Formula(
                And(vec![
                    AtomEq(Var(Variable("x".to_string())), Nil),
                    AtomNeq(
                        Var(Variable("y".to_string())),
                        Var(Variable("x".to_string())),
                    ),
                ]),
                SepConj(vec![PointsTo(
                    Var(Variable("y".to_string())),
                    Var(Variable("x".to_string())),
                )]),
            ),
            consequent: Formula(
                And(vec![AtomNeq(
                    Var(Variable("z".to_string())),
                    Var(Variable("x".to_string())),
                )]),
                SepConj(vec![LS(Var(Variable("x".to_string())), Nil)]),
            ),
        };

        let goal_expected = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(Var(Variable("y".to_string())), Nil)]),
                SepConj(vec![PointsTo(Var(Variable("y".to_string())), Nil)]),
            ),
            consequent: Formula(
                And(vec![AtomNeq(Var(Variable("z".to_string())), Nil)]),
                SepConj(vec![LS(Nil, Nil)]),
            ),
        };

        let premisses = Substitution.premisses(goal);
        if let Some(prem) = premisses {
            assert_eq!(1, prem.len());
            assert_eq!(goal_expected, prem[0]);
        } else {
            assert!(false);
        }
    }
}
