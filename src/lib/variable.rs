pub trait VariableTrait<T> {
    fn add_term(&mut self, term: T);
    fn set_value(&mut self, value: f64);
}

pub struct Variable<T> {
    value: f64,
    name: String,
    terms: std::vec::Vec<T>,
}

impl<T> Variable<T> {
    pub fn new(name: String) -> Self {
        Self {
            value: 0.0,
            name,
            terms: vec![],
        }
    }
}

impl<T> VariableTrait<T> for Variable<T> {
    fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    fn add_term(&mut self, term: T) {
        self.terms.push(term);
    }
}
