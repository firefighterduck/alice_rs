use crate::{
    datastructures::{
        Entailment, Expr,
        Expr::{Nil, Var},
        Formula, Op,
        Op::{AtomEq, AtomNeq},
        Pure,
        Pure::{And, True},
        Rule, Spatial,
        Spatial::{Emp, PointsTo, SepConj, LS},
    },
    misc::find_first,
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

    fn subst_spatial(subst: &(String, Expr), sp: &mut Spatial) -> Spatial {
        match sp {
            PointsTo(v, e) => PointsTo(Self::subst_impl(subst, v), Self::subst_impl(subst, e)),
            SepConj(l, r) => SepConj(
                Box::new(Self::subst_spatial(subst, l)),
                Box::new(Self::subst_spatial(subst, r)),
            ),
            LS(v, e) => LS(Self::subst_impl(subst, v), Self::subst_impl(subst, e)),
            Emp => Emp,
        }
    }

    fn substitute(subst: (String, Expr), goal: Entailment) -> Entailment {
        let (mut antecedent, mut consequent) = goal.destroy();

        let new_pure_ant = Self::subst_pure(&subst, antecedent.get_pure());

        let new_spatial_ant = Self::subst_spatial(&subst, antecedent.get_spatial());

        let new_pure_cons = Self::subst_pure(&subst, consequent.get_pure());

        let new_spatial_cons = Self::subst_spatial(&subst, consequent.get_spatial());

        Entailment {
            antecedent: Formula(new_pure_ant, new_spatial_ant),
            consequent: Formula(new_pure_cons, new_spatial_cons),
        }
    }
}
impl Rule for Substitution {
    fn predicate(&self, _goal: Entailment) -> bool {
        true
    }

    fn premisses(&self, mut goal: Entailment) -> Option<Vec<Entailment>> {
        if let And(pure_sub) = goal.antecedent.get_pure() {
            if let Some(index) = find_first(&pure_sub, move |x| match x {
                &AtomEq(_, _) => {
                    return true;
                }
                _ => return false,
            }) {
                let mut subst = ("".to_string(), Nil);

                if let AtomEq(l, r) = pure_sub.remove(index) {
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
