use crate::{
    datastructures::{AtomSpatial::LS, Entailment, Rule, Spatial::SepConj},
    misc::find_and_remove,
};

/// Π | Σ  |-  Π' | Σ' ==> Π | Σ  |-  Π' | ls(E,E) * Σ'
pub struct EmptyLs;

impl Rule for EmptyLs {
    fn predicate(&self, _goal: &Entailment) -> bool {
        true
    }
    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        let (antecedent, mut consequent) = goal.destroy();

        if let SepConj(spatial_vec) = consequent.get_spatial_mut() {
            if let Some(_) = find_and_remove(spatial_vec, move |x| match x {
                LS(l, r) => *l == *r,
                _ => false,
            }) {
                return Some(vec![Entailment {
                    antecedent,
                    consequent,
                }]);
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::EmptyLs;
    use crate::datastructures::{
        AtomSpatial::LS,
        Entailment, Expr, Formula,
        Pure::True,
        Rule,
        Spatial::{Emp, SepConj},
    };

    #[test]
    fn test_emptyls() -> Result<(), String> {
        let goal1 = Entailment {
            antecedent: Formula(True, Emp),
            consequent: Formula(
                True,
                SepConj(vec![LS(Expr::new_var("x"), Expr::new_var("y"))]),
            ),
        };

        let premisses1 = EmptyLs.premisses(goal1);
        if let Some(_) = premisses1 {
            return Err("Expected first test to fail!".to_string());
        }

        let goal2 = Entailment {
            antecedent: Formula(True, Emp),
            consequent: Formula(
                True,
                SepConj(vec![LS(Expr::new_var("x"), Expr::new_var("x"))]),
            ),
        };
        let goal2_expected = Entailment {
            antecedent: Formula(True, Emp),
            consequent: Formula(True, SepConj(vec![])),
        };

        let premisses2 = EmptyLs.premisses(goal2);
        if let Some(prem) = premisses2 {
            assert_eq!(1, prem.len());
            assert_eq!(goal2_expected, prem[0]);
            Ok(())
        } else {
            Err("Expected second test to succeed!".to_string())
        }
    }
}
