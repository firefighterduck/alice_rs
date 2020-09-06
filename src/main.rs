mod datastructures;
mod misc;
mod parser;
mod rules;
use combine::{stream::position::Stream, Parser};
use datastructures::{Entailment, Rule};
use parser::parse_entailment;
use rules::*;
use std::env;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: alice \"[Entailment with possible whitespaces]\"");
        return Err(());
    }

    let entailment_raw = &args[1];
    let entailemnt_parsed_result = parse_entailment().parse(Stream::new(&**entailment_raw));

    if let Ok((entailment, _)) = entailemnt_parsed_result {
        ps(entailment)
    } else {
        println!("{:?}", entailemnt_parsed_result);
        Err(())
    }
}

const RULES: [&dyn Rule; 13] = [
    &Tautology,
    &Contradiction,
    &Substitution,
    &EqReflexiveL,
    &NilNotLVal,
    &StarPartial,
    &UnrollCollapse,
    &EqReflexiveR,
    &EmptyLs,
    &Hypothesis,
    &Frame,
    &NonEmptyLS,
    &Cleanup,
];

fn ps(goal: Entailment) -> Result<(), ()> {
    for &rule in RULES.iter() {
        if rule.predicate(&goal) {
            if let Some(new_goals) = rule.premisses(goal.clone()) {
                for new_goal in new_goals {
                    ps(new_goal)?;
                }
                return Ok(());
            }
        }
    }
    Err(())
}

#[cfg(test)]
pub mod test {
    use super::ps;
    use crate::datastructures::{
        AtomSpatial::{PointsTo, LS},
        Entailment, Expr,
        Expr::Nil,
        Formula,
        Op::{AtomEq, AtomNeq},
        Pure::{And, True},
        Spatial::SepConj,
    };

    #[test]
    pub fn test_ps() {
        let valid = Entailment {
            antecedent: Formula(
                And(vec![AtomNeq(Expr::new_var("x"), Expr::new_var("y"))]),
                SepConj(vec![
                    PointsTo(Expr::new_var("x"), Expr::new_var("y")),
                    PointsTo(Expr::new_var("y"), Nil),
                ]),
            ),
            consequent: Formula(True, SepConj(vec![LS(Expr::new_var("x"), Nil)])),
        };
        assert_eq!(Ok(()), ps(valid));

        let invalid = Entailment {
            antecedent: Formula(
                True,
                SepConj(vec![
                    PointsTo(Expr::new_var("x"), Nil),
                    PointsTo(Expr::new_var("y"), Nil),
                ]),
            ),
            consequent: Formula(
                And(vec![AtomEq(Expr::new_var("x"), Expr::new_var("y"))]),
                SepConj(vec![PointsTo(Expr::new_var("y"), Nil)]),
            ),
        };
        assert_eq!(Err(()), ps(invalid));
    }
}
