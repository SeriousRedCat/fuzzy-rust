use crate::hedge::HedgeTrait;
use crate::term::TermTrait;
// use crate::variable::VariableTrait;

pub trait ExpressionTrait {
    fn activation_degree(&self) -> f64;
}

pub struct Proposition<'a, Hedge> {
    pub hedges: std::vec::Vec<Hedge>,
    pub term: &'a dyn TermTrait,
    // pub variable: &'a dyn VariableTrait<dyn TermTrait>,
}

impl<'a, Hedge> Proposition<'a, Hedge> {}

impl<'a, Hedge> ExpressionTrait for Proposition<'a, Hedge>
where
    Hedge: HedgeTrait,
{
    fn activation_degree(&self) -> f64 {
        self.term.membership(0f64)
    }
}
