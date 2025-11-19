macro_rules! f64 {
    (eq $one: expr, $two: expr) => {
        ($one - $two).abs() <= f64::EPSILON
    };
    (ne $one: expr, $two: expr) => {
        ($one - $two).abs() > f64::EPSILON
    };
}

pub use f64;
