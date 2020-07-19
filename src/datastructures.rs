#[derive(PartialEq, Eq, Clone)]
pub struct Variable(pub String);
#[derive(PartialEq, Eq, Clone)]
pub enum Expr {
    Nil,
    Var(Variable),
}
pub enum Op {
    AtomEq(Expr, Expr),
    AtomNeq(Expr, Expr),
}
pub enum Pure {
    And(Vec<Op>),
    True,
}
pub enum Spatial {
    PointsTo(Expr, Expr),
    SepConj(Box<Spatial>, Box<Spatial>),
    LS(Expr, Expr),
    Emp,
}
pub struct Formula(pub Pure, pub Spatial);
pub struct Entailment {
    pub antecedent: Formula,
    pub consequent: Formula,
}
pub trait Rule {
    fn predicate(&self, goal: Entailment) -> bool;
    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>>;
}

impl Formula {
    pub fn get_pure(&mut self) -> &mut Pure {
        &mut self.0
    }

    pub fn get_spatial(&mut self) -> &mut Spatial {
        &mut self.1
    }
}

impl Entailment {
    pub fn destroy(self) -> (Formula, Formula) {
        (self.antecedent, self.consequent)
    }
}
