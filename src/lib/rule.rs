use crate::antecedent::{Antecedent, AntecedentTrait};
use crate::consequent::Consequent;
use crate::norm::NormTrait;

pub trait RuleTrait {
    fn activate_with(&self, conjunction: &dyn NormTrait, disjunction: &dyn NormTrait);
    fn trigger(&self, implication: &dyn NormTrait);
    fn activation_degree(&self) -> f64;

    fn to_string(&self) -> String;
}

#[derive(Default)]
pub struct Rule<Ant: AntecedentTrait, Cons> {
    antecedent: Ant,
    consequent: Cons,
    weigth: f64,
}

impl<Ant: AntecedentTrait, Cons> Rule<Ant, Cons> {
    pub fn new(antecedent: Ant, consequent: Cons) -> Self {
        Rule {
            antecedent,
            consequent,
            weigth: 1f64,
        }
    }
}

impl<Ant: AntecedentTrait, Cons> RuleTrait for Rule<Ant, Cons> {
    fn activate_with(&self, _conjunction: &dyn NormTrait, _disjunction: &dyn NormTrait) {}
    fn trigger(&self, _implication: &dyn NormTrait) {}
    fn activation_degree(&self) -> f64 {
        0.0
    }

    fn to_string(&self) -> String {
        format!("if {} then", self.antecedent)
    }
}
