use std::collections::HashMap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Double(f64),
    String(std::string::String),
    Data(Vec<u8>),
    Date(f64),
    Array(Vec<Value>),
    Dictionary(HashMap<Value, Value>),
}
use Value::*;

impl Value {
}

fn f64_eq(x: &f64, y: &f64) -> bool {
    use Ordering::*;
    match (x.is_nan(), y.is_nan()) {
        (true, true) => true,
        (true, _) => false,
        (_, true) => false,
        _ => x.eq(y),
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Nil, &Nil) => true,
            (&Bool(ref x), &Bool(ref y)) => x.eq(y),
            (&Int(ref x), &Int(ref y)) => x.eq(y),
            (&Double(ref x), &Double(ref y)) => f64_eq(x, y),
            (&String(ref x), &String(ref y)) => x.eq(y),
            (&Data(ref x), &Data(ref y)) => x.eq(y),
            (&Date(ref x), &Date(ref y)) => f64_eq(x, y),
            (&Array(ref x), &Array(ref y)) => x.eq(y),
            (&Dictionary(ref x), &Dictionary(ref y)) => x.eq(y),
            _ => false,
        }
    }
}

impl Eq for Value { }

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            &Nil => state.write_u8(0),
            &Bool(false) => state.write_u8(1),
            &Bool(true) => state.write_u8(2),
            &Int(v) => {
                state.write_u8(3);
                state.write_i64(v);
            },
            &Double(v) => {
                state.write_u8(4);
                state.write_u64(v.to_bits());
            },
            &String(ref v) => {
                state.write_u8(5);
                state.write(v.as_bytes());
            },
            &Data(ref v) => {
                state.write_u8(6);
                state.write(v.as_slice());
            },
            &Date(v) => {
                state.write_u8(7);
                state.write_u64(v.to_bits());
            },
            &Array(ref v) => {
                state.write_u8(8);
                v.iter().for_each(|v|{ v.hash(state) });
            },
            &Dictionary(ref v) => {
                state.write_u8(9);
                let mut c = v.iter().collect::<Vec<(&Value, &Value)>>();
                c.sort();
                c.iter().for_each(|&(k, v)|{
                    k.hash(state);
                    v.hash(state);
                });
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn f64_cmp(x: &f64, y: &f64) -> Ordering {
    use Ordering::*;
    match (x.is_nan(), y.is_nan()) {
        (true, true) => Equal,
        (true, _) => Less,
        (_, true) => Greater,
        _ => x.partial_cmp(y).unwrap(),
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        match (self, other) {
            (&Nil, &Nil) => Equal,
            (&Bool(ref x), &Bool(ref y)) => x.cmp(y),
            (&Int(ref x), &Int(ref y)) => x.cmp(y),
            (&Double(ref x), &Double(ref y)) => f64_cmp(x, y),
            (&String(ref x), &String(ref y)) => x.cmp(y),
            (&Data(ref x), &Data(ref y)) => x.cmp(y),
            (&Date(ref x), &Date(ref y)) => f64_cmp(x, y),
            (&Array(ref x), &Array(ref y)) => x.cmp(y),
            (&Dictionary(ref x), &Dictionary(ref y)) => {
                let mut x1 = x.iter().collect::<Vec<(&Value, &Value)>>();
                x1.sort();
                let mut y1 = y.iter().collect::<Vec<(&Value, &Value)>>();
                y1.sort();
                x1.cmp(&y1)
            },
            (&Nil, _) => Less,
            (_, &Nil) => Greater,
            (&Bool(_), _) => Less,
            (_, &Bool(_)) => Greater,
            (&Int(_), _) => Less,
            (_, &Int(_)) => Greater,
            (&Double(_), _) => Less,
            (_, &Double(_)) => Greater,
            (&String(_), _) => Less,
            (_, &String(_)) => Greater,
            (&Data(_), _) => Less,
            (_, &Data(_)) => Greater,
            (&Date(_), _) => Less,
            (_, &Date(_)) => Greater,
            (&Array(_), _) => Less,
            (_, &Array(_)) => Greater,
        }
    }
}

#[allow(unused_imports)]
use std::f64::INFINITY;
 #[allow(unused_imports)]
use std::f64::NAN;

#[macro_export]
macro_rules! sion {
    (nil) => { $crate::Nil };
    (true) => { $crate::Bool(true) };
    (false) => { $crate::Bool(false) };
    (inf) => { $crate::Double($crate::INFINITY) };
    (-inf) => { $crate::Double(-$crate::INFINITY) };
    (nan) => { $crate::Double($crate::NAN) };
    ([:]) => {
        $crate::Dictionary($crate::HashMap::new())
    };
    ([ $($tt:tt),* ]) => {{
        $crate::Array(vec![$($tt)*])
    }};
    ([ $($tt:tt):+, ]) => {$crate::Dictionary({
        let mut dict = $crate::HashMap::<$crate::Value, $crate::Value>::new();
        dict.insert(sion!($($tt)+), sion!($($tt)+));
        dict
    })};
}

#[cfg(test)]
mod tests {
    use Value::*;
    use std::collections::HashMap;
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
        assert_eq!(sion!([:]), Dictionary(HashMap::new()));
        // assert_eq!(sion!([[0]:[0],]), Dictionary(HashMap::new()));
    }
}
