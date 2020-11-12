use crate::datastructures::{
    Entailment,
    Pure::{And, True},
    Rule,
    Spatial::{Emp, SepConj},
};

pub struct Cleanup;

impl Rule for Cleanup {
    fn predicate(&self, _goal: &Entailment) -> bool {
        true
    }

    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        let (mut antecedent, mut consequent) = goal.destroy();
        let change_apv = if let And(apv) = antecedent.get_pure() {
            apv.is_empty()
        } else {
            false
        };
        let change_asv = if let SepConj(asv) = antecedent.get_spatial() {
            asv.is_empty()
        } else {
            false
        };
        let change_cpv = if let And(cpv) = consequent.get_pure() {
            cpv.is_empty()
        } else {
            false
        };
        let change_csv = if let SepConj(csv) = consequent.get_spatial() {
            csv.is_empty()
        } else {
            false
        };

        if change_apv {
            antecedent.0 = True;
        }
        if change_asv {
            antecedent.1 = Emp;
        }
        if change_cpv {
            consequent.0 = True;
        }
        if change_csv {
            consequent.1 = Emp;
        }

        if change_apv || change_asv || change_cpv || change_csv {
            Some(vec![Entailment {
                antecedent,
                consequent,
            }])
        } else {
            None
        }
    }
}
