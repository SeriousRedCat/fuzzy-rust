use crate::hedge::Hedge;
use crate::term::TermTrait;
// use crate::variable::VariableTrait;
use crate::WeakPtr;
use crate::variable::VariableTrait;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;

pub trait ExpressionTrait: std::fmt::Display{
    fn activation_degree(&self) -> f64;
}

pub struct Proposition {
    pub hedges: std::vec::Vec<Hedge>,
    pub term: WeakPtr<dyn TermTrait>,
    // pub variable: &'a dyn VariableTrait,
}

impl Proposition {}

impl ExpressionTrait for Proposition {
    fn activation_degree(&self) -> f64 {
        let t = self.term.upgrade().unwrap();
        let term: &RefCell<Box<dyn TermTrait>> = t.borrow();
        let v = term.borrow().variable().unwrap();
        let var: &RefCell<Box<dyn VariableTrait>> = v.borrow();
        
        let x = term.borrow().membership(var.borrow().value());
        x
        // self.term.upgrade().unwrap().borrow().membership(0f64)
    }
}

impl std::fmt::Display for Proposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = self.term.upgrade().unwrap();
        let term: &RefCell<Box<dyn TermTrait>> = t.borrow();
        let v = term.borrow().variable().unwrap();
        let var: &RefCell<Box<dyn VariableTrait>> = v.borrow();
        let x = write!(f, "{} is {}", var.borrow(), term.borrow());
        x
    }
}
