use std::marker::PhantomData;

use crate::term::TermTrait;
use crate::variable::{self, VariableTrait};

pub struct Engine<Id, Input, Output, T> {
    inputs: std::collections::HashMap<Id, Input>,
    outputs: std::collections::HashMap<Id, Output>,

    _marker: PhantomData<T>,
}

impl<Id, Input, Output, T> Engine<Id, Input, Output, T>
where
    Id: Eq + std::hash::Hash,
    Input: VariableTrait<T>,
    Output: VariableTrait<T>,
    T: TermTrait,
{
    pub fn new() -> Self {
        Engine {
            inputs: Default::default(),
            outputs: Default::default(),
            _marker: Default::default(),
        }
    }

    pub fn set_input_value(id: &Id, value: f64) {}

    pub fn register_input(&mut self, id: Id, variable: Input) {
        self.inputs.insert(id, variable);
    }

    pub fn register_output(&mut self, id: Id, variable: Output) {
        self.outputs.insert(id, variable);
    }

    pub fn process(&mut self) {
        for input in self.inputs.values() {
            input.fuzzify(input.value());
        }
    }
}
