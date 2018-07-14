use std::cmp::Ordering;

pub fn f64_cmp(x: f64, y: f64) -> Ordering {
    use self::Ordering::*;
    match (x.is_nan(), y.is_nan()) {
        (true, true) => Equal,
        (true, _) => Less,
        (_, true) => Greater,
        _ => x.partial_cmp(&y).unwrap(),
    }
}

pub fn f64_eq(x: f64, y: f64) -> bool {
    match (x.is_nan(), y.is_nan()) {
        (true, true) => true,
        (true, _) => false,
        (_, true) => false,
        _ => x.eq(&y),
    }
}
