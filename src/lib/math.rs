pub fn bell(x: f64, center: f64, width: f64, slope: f64) -> f64 {
    1.0 / (1.0 + ((x - center) / width).abs().powf(2.0 * slope))
}

pub fn binary(x: f64, start: std::ops::Bound<f64>, positive: bool) -> f64 {
    match start {
        std::ops::Bound::Included(value) => {
            if (x >= value) == positive {
                1.0
            } else {
                0.0
            }
        }
        std::ops::Bound::Excluded(value) => {
            if (x > value) == positive {
                1.0
            } else {
                0.0
            }
        }
        _ => {
            if positive {
                1.0
            } else {
                0.0
            }
        }
    }
}

mod tests {
    use std::collections::HashMap;

    use crate::assert_float_eq;

    use super::*;

    #[test]
    fn test_bell() {
        
        let mut expected: std::vec::Vec<(f64, f64, f64, f64, f64)> = Default::default();
        expected.push((-3.0, 0.0, 5.0, 1.0, 0.7352941176470589));

        for test in expected {
            assert_float_eq!(bell(test.0, test.1, test.2, test.3), test.4);
        }
    }

    #[test]
    fn test_binary() {
        assert_float_eq!(binary(2.0, std::ops::Bound::Included(5.0), true), 0.0);
        assert_float_eq!(binary(4.0, std::ops::Bound::Included(5.0), true), 0.0);
        assert_float_eq!(binary(5.0, std::ops::Bound::Included(5.0), true), 1.0);
        assert_float_eq!(binary(9.0, std::ops::Bound::Included(5.0), true), 1.0);

        assert_float_eq!(binary(2.0, std::ops::Bound::Excluded(5.0), true), 0.0);
        assert_float_eq!(binary(4.0, std::ops::Bound::Excluded(5.0), true), 0.0);
        assert_float_eq!(binary(5.0, std::ops::Bound::Excluded(5.0), true), 0.0);
        assert_float_eq!(binary(9.0, std::ops::Bound::Excluded(5.0), true), 1.0);

        assert_float_eq!(binary(2.0, std::ops::Bound::Included(5.0), false), 1.0);
        assert_float_eq!(binary(4.0, std::ops::Bound::Included(5.0), false), 1.0);
        assert_float_eq!(binary(5.0, std::ops::Bound::Included(5.0), false), 0.0);
        assert_float_eq!(binary(9.0, std::ops::Bound::Included(5.0), false), 0.0);

        assert_float_eq!(binary(2.0, std::ops::Bound::Excluded(5.0), false), 1.0);
        assert_float_eq!(binary(4.0, std::ops::Bound::Excluded(5.0), false), 1.0);
        assert_float_eq!(binary(5.0, std::ops::Bound::Excluded(5.0), false), 1.0);
        assert_float_eq!(binary(9.0, std::ops::Bound::Excluded(5.0), false), 0.0);
    }
}
