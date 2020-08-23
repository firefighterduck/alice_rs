use crate::{
    datastructures::{
        AtomSpatial::{PointsTo, LS},
        Entailment,
        Op::AtomNeq,
        Pure::And,
        Rule,
        Spatial::SepConj,
    },
    misc::find_and_remove,
};
pub struct NonEmptyLS;

impl Rule for NonEmptyLS {
    fn predicate(&self, goal: &Entailment) -> bool {
        goal.is_normal_form()
    }

    fn premisses(&self, goal: Entailment) -> Option<Vec<Entailment>> {
        let (mut antecedent, mut consequent) = goal.destroy();
        let mut pair_opt = None;
        if let SepConj(cons_spatials) = consequent.get_spatial() {
            if let And(ant_pures) = antecedent.get_pure() {
                if let SepConj(ant_spatials) = antecedent.get_spatial() {
                    'outer: for cons_spatial in cons_spatials {
                        if let LS(e1, e2) = cons_spatial {
                            'middle: for ant_pure in ant_pures {
                                if let AtomNeq(e3, e4) = ant_pure {
                                    if !((e1 == e4 && e2 == e3) || (e1 == e3 && e2 == e4)) {
                                        continue 'middle;
                                    }

                                    'inner: for ant_spatial in ant_spatials {
                                        if let PointsTo(e5, e6) = ant_spatial {
                                            if e1 != e5 {
                                                continue 'inner;
                                            }

                                            pair_opt = Some((e1.clone(), e6.clone(), e3.clone()));
                                            break 'outer;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some((e1, e2, e3)) = pair_opt {
            if let SepConj(ant_spatials) = antecedent.get_spatial_mut() {
                find_and_remove(ant_spatials, |spatial| {
                    if let PointsTo(l, r) = spatial {
                        *l == e1 && *r == e2
                    } else {
                        false
                    }
                });
            }
            if let SepConj(cons_spatials) = consequent.get_spatial_mut() {
                find_and_remove(cons_spatials, |spatial| {
                    if let LS(l, r) = spatial {
                        *l == e1 && *r == e3
                    } else {
                        false
                    }
                });
                cons_spatials.push(LS(e2, e3));
            }
            return Some(vec![Entailment {
                antecedent,
                consequent,
            }]);
        }

        None
    }
}
