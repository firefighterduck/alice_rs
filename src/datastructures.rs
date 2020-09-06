#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Variable(pub String);

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
    Nil,
    Var(Variable),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Op {
    AtomEq(Expr, Expr),
    AtomNeq(Expr, Expr),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Pure {
    And(Vec<Op>),
    True,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AtomSpatial {
    PointsTo(Expr, Expr),
    LS(Expr, Expr),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Spatial {
    SepConj(Vec<AtomSpatial>),
    Emp,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Formula(pub Pure, pub Spatial);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Entailment {
    pub antecedent: Formula,
    pub consequent: Formula,
}

pub trait Rule {
    fn predicate(&self, goal: &Entailment) -> bool;
    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>>;
}

impl Op {
    pub fn is_eq(&self) -> bool {
        match self {
            &Op::AtomEq(_, _) => true,
            &Op::AtomNeq(_, _) => false,
        }
    }
}

impl Expr {
    pub fn get_var_opt(&self) -> Option<Variable> {
        match self {
            Self::Var(v) => Some(v.clone()),
            Self::Nil => None,
        }
    }

    pub fn new_var(name: &str) -> Self {
        Expr::Var(Variable(name.to_string()))
    }
}

impl Formula {
    pub fn get_pure_vars(&self) -> Option<Vec<Variable>> {
        if let Pure::And(pure_vec) = &self.0 {
            let mut var_vec: Vec<Variable> = Vec::with_capacity(pure_vec.len());
            for op in pure_vec {
                let (l, r) = match op {
                    Op::AtomEq(l, r) => (l, r),
                    Op::AtomNeq(l, r) => (l, r),
                };
                if let Some(v) = l.get_var_opt() {
                    var_vec.push(v);
                }
                if let Some(v) = r.get_var_opt() {
                    var_vec.push(v);
                }
            }
            return Some(var_vec);
        }
        None
    }

    pub fn get_spatial_vars(&self) -> Option<Vec<Variable>> {
        if let Spatial::SepConj(spatial_vec) = &self.1 {
            let mut var_vec: Vec<Variable> = Vec::with_capacity(spatial_vec.len());
            for atom in spatial_vec {
                let (l, r) = match atom {
                    AtomSpatial::LS(l, r) => (l, r),
                    AtomSpatial::PointsTo(l, r) => (l, r),
                };
                if let Some(v) = l.get_var_opt() {
                    var_vec.push(v);
                }
                if let Some(v) = r.get_var_opt() {
                    var_vec.push(v);
                }
            }
            return Some(var_vec);
        }
        None
    }

    pub fn get_pure(&self) -> &Pure {
        &self.0
    }

    pub fn get_pure_mut(&mut self) -> &mut Pure {
        &mut self.0
    }

    pub fn get_spatial_mut(&mut self) -> &mut Spatial {
        &mut self.1
    }

    pub fn get_spatial(&self) -> &Spatial {
        &self.1
    }

    pub fn destroy(self) -> (Pure, Spatial) {
        (self.0, self.1)
    }
}

impl Entailment {
    pub fn destroy(self) -> (Formula, Formula) {
        (self.antecedent, self.consequent)
    }

    pub fn is_normal_form(&self) -> bool {
        if let Spatial::SepConj(vec) = self.antecedent.get_spatial() {
            if vec.iter().any(|x: &AtomSpatial| x.is_ls()) {
                return false; //There are no LS allowed for normal form
            }
        }

        let mut vars = Vec::new();
        if let Some(ref mut vec) = self.antecedent.get_pure_vars() {
            vars.append(vec);
        }
        if let Some(ref mut vec) = self.antecedent.get_spatial_vars() {
            vars.append(vec);
        }

        if let Pure::And(pures) = self.antecedent.get_pure() {
            'outer: for o_var in vars.as_slice() {
                if !pures.iter().any(|x| {
                    if let Op::AtomNeq(l, r) = x {
                        (*l == Expr::Var(o_var.clone()) && *r == Expr::Nil)
                            || (*r == Expr::Var(o_var.clone()) && *l == Expr::Nil)
                    } else {
                        return false; //There are no AtomEqs allowed for normal form
                    }
                }) {
                    return false; //There is no inequality for the variable o_var with Nil which is necessary for normal form
                }
                for i_var in vars.as_slice() {
                    if i_var == o_var {
                        continue 'outer;
                    }
                    if !pures.iter().any(|x| {
                        if let Op::AtomNeq(l, r) = x {
                            (*l == Expr::Var(o_var.clone()) && *r == Expr::Var(i_var.clone()))
                                || (*r == Expr::Var(o_var.clone())
                                    && *l == Expr::Var(i_var.clone()))
                        } else {
                            return false; //There are no AtomEqs allowed for normal form
                        }
                    }) {
                        return false; //There is no inequality for the variable o_var with i_var which is necessary for normal form
                    }
                }
            }
        }

        true
    }
}

impl AtomSpatial {
    pub fn is_points_to(&self) -> bool {
        match self {
            AtomSpatial::PointsTo(_, _) => true,
            _ => false,
        }
    }

    pub fn is_ls(&self) -> bool {
        match self {
            AtomSpatial::LS(_, _) => true,
            _ => false,
        }
    }
}

impl Spatial {
    pub fn add(mut self, new: AtomSpatial) -> Self {
        match &mut self {
            Spatial::SepConj(vec) => {
                vec.push(new);
                self
            }
            Spatial::Emp => Spatial::SepConj(vec![new]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        AtomSpatial::{PointsTo, LS},
        Entailment, Expr,
        Expr::Nil,
        Formula,
        Pure::{And, True},
        Spatial::{Emp, SepConj},
    };

    #[test]
    fn test_is_nomal_form() {
        let not_normal1 = Entailment {
            antecedent: Formula(True, SepConj(vec![LS(Expr::new_var("x"), Nil)])),
            consequent: Formula(True, Emp),
        };
        assert_eq!(false, not_normal1.is_normal_form());

        let normal1 = Entailment {
            antecedent: Formula(
                And(vec![super::Op::AtomNeq(Expr::new_var("x"), Nil)]),
                SepConj(vec![PointsTo(Expr::new_var("x"), Nil)]),
            ),
            consequent: Formula(True, Emp),
        };
        assert!(normal1.is_normal_form());
    }
}
