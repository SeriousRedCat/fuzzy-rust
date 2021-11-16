pub mod antecedent;
pub mod consequent;
pub mod engine;
pub mod expression;
pub mod hedge;
pub mod macros;
pub mod math;
pub mod norm;
pub mod rule;
pub mod rule_block;
pub mod term;
pub mod variable;

type PtrBase<T> = std::cell::RefCell<Box<T>>;
pub type SharedPtr<T> = std::rc::Rc<PtrBase<T>>;
pub type WeakPtr<T> = std::rc::Weak<PtrBase<T>>;
