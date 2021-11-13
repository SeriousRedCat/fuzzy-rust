use crate::variable;

pub struct Engine<Id, Input> {
    inputs: std::collections::HashMap<Id, Input>,
}

impl<Id, Input> Engine<Id, Input>
where
    Id: Eq + std::hash::Hash {
    pub fn new() -> Self {
        Engine {
            inputs: Default::default(),
        }
    }

    pub fn set_input_value(id: &Id, value: f64) {

    }

    pub fn register_input(&mut self, id: Id, variable: Input) {
        self.inputs.insert(id, variable);
    }

    pub fn process(&mut self) {}
}
