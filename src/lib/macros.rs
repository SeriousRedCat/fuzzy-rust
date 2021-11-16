#[macro_export]
macro_rules! assert_float_eq {
    ($a: expr, $b: expr) => {
        println!("expected: {}, result: {}", $a, $b);
        assert!(($a - $b).abs() < f64::EPSILON);
    };
}

#[macro_export]
macro_rules! make_shared {
    ($a: expr) => {
        std::rc::Rc::new(std::cell::RefCell::new(Box::new($a)))
    };
}
