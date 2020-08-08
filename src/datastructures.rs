#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Variable(pub String);
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
    Nil,
    Var(Variable),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Op {
    AtomEq(Expr, Expr),
    AtomNeq(Expr, Expr),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Pure {
    And(Vec<Op>),
    True,
}

#[derive(PartialEq, Eq, Debug)]
pub enum AtomSpatial {
    PointsTo(Expr, Expr),
    LS(Expr, Expr),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Spatial {
    SepConj(Vec<AtomSpatial>),
    Emp,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Formula(pub Pure, pub Spatial);
#[derive(PartialEq, Eq, Debug)]
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

    pub fn is_neq(&self) -> bool {
        match self {
            &Op::AtomEq(_, _) => false,
            &Op::AtomNeq(_, _) => true,
        }
    }
}

impl Formula {
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
