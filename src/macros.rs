#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use value::Value::*;
#[allow(unused_imports)]
use std::f64::INFINITY;
 #[allow(unused_imports)]
use std::f64::NAN;

#[macro_export]
macro_rules! sion {
    ($($sion:tt)+) => { sion_internal!($($sion)+) };
}

#[macro_export]
macro_rules! sion_internal {
    (nil) => { $crate::Value::Nil };
    (true) => { $crate::Value::Bool(true) };
    (false) => { $crate::Value::Bool(false) };
    (inf) => { $crate::Value::Double($crate::macros::INFINITY) };
    (-inf) => { $crate::Value::Double(-$crate::macros::INFINITY) };
    (nan) => { $crate::Value::Double($crate::macros::NAN) };
    ([:]) => { $crate::Value::Dictionary($crate::map::Map::new()) };
    ([ $key:tt : $value:tt ]) => {
        sion_internal!(@dict $key : $value)
    };
    ([ $key:tt : $value:tt , $($rest:tt),+ ]) => {
        sion_internal!(@dict $key : $value, $($rest),+ )
    };
    ([ $($tt:tt),* ]) => {
        sion_internal!(@array $($tt),* )
    };
    ($other:expr) => {
        $crate::Value::Nil;
    };
    (@dict $($key:tt : $value:tt),+ ) => {{
        let mut dict = $crate::map::Map::new();
        $(dict.insert(sion!($key), sion!($value));)+
        $crate::Value::Dictionary(dict)
    }};
    (@array $($tt:tt),* ) => {
        $crate::Value::Array(vec![$(sion!($tt)),*])
    };
}

#[cfg(test)]
mod tests {
    use value::Value::*;
    use map::Map;
    use std::f64;
    #[test]
    fn it_works() {
        assert_eq!(sion!(nil), Nil);
        assert_eq!(sion!(true), Bool(true));
        assert_eq!(sion!(false), Bool(false));
        assert_eq!(sion!(inf), Double(f64::INFINITY));
        assert_eq!(sion!(-inf), Double(-f64::INFINITY));
        assert_eq!(sion!(nan), Double(-f64::NAN));
        assert_eq!(sion!([]), Array(vec![]));
        assert_eq!(sion!([:]), Dictionary(Map::new()));
        assert_eq!(sion!([[0]:[0]]), Dictionary(Map::new()));
        assert_eq!(sion!([[0]]), Array(vec!(Array(vec![Int(0)]))));
    }
}
