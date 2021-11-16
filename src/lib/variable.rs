use crate::{term::TermTrait, SharedPtr};
use std::rc::Rc;

pub trait VariableTrait: std::fmt::Display {
    fn add_term(&mut self, term: SharedPtr<dyn TermTrait>);
    fn set_value(&mut self, value: f64);
    fn value(&self) -> f64;
    fn fuzzify(&self, value: f64);

    fn term(&self, name: &String) -> Option<&SharedPtr<dyn TermTrait>>;
}

pub struct Variable {
    value: f64,
    name: String,
    terms: std::collections::HashMap<String, SharedPtr<dyn TermTrait>>,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self {
            value: 0.0,
            name,
            terms: Default::default(),
        }
    }
}

impl VariableTrait for Variable {
    fn add_term(&mut self, term: SharedPtr<dyn TermTrait>) {
        let name = term.borrow().name().clone();
        self.terms.insert(name, term);
    }

    fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn fuzzify(&self, _value: f64) {}

    fn term(&self, name: &String) -> Option<&SharedPtr<dyn TermTrait>> {
        self.terms.get(name)
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
write!(f, "{}", self.name)
    }
}
