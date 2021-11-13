#[macro_export]
macro_rules! assert_float_eq {
    ($a: expr, $b: expr) => {
        println!("expected: {}, result: {}", $a, $b);
        assert!(($a - $b).abs() < f64::EPSILON);
    };
}
