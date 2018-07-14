use value::Value;
use std::ops;
use std::fmt;

impl<Idx: Index> ops::Index<Idx> for Value {
    type Output = Value;

    #[inline]
    fn index(&self, index: Idx) -> &Value {
        static NIL : Value = Value::Nil;
        index.index_into(self).unwrap_or(&NIL)
    }
}

impl<Idx: Index> ops::IndexMut<Idx> for Value {
    #[inline]
    fn index_mut(&mut self, index: Idx) -> &mut Value {
        index.index_or_insert(self)
    }
}

pub trait Index {
    fn index_into<'a>(&self, v: &'a Value) -> Option<&'a Value>;
    fn index_into_mut<'a>(&self, v: &'a mut Value) -> Option<&'a mut Value>;
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}

struct Type<'a>(&'a Value);

impl<'a> fmt::Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Value::Nil => formatter.write_str("nil"),
            Value::Bool(_) => formatter.write_str("boolean"),
            Value::Int(_) => formatter.write_str("int"),
            Value::Double(_) => formatter.write_str("double"),
            Value::String(_) => formatter.write_str("string"),
            Value::Data(_) => formatter.write_str("data"),
            Value::Date(_) => formatter.write_str("date"),
            Value::Array(_) => formatter.write_str("array"),
            Value::Dictionary(_) => formatter.write_str("dictionary"),
        }
    }
}

impl Index for usize {
    fn index_into<'a>(&self, v: &'a Value) -> Option<&'a Value> {
        match *v {
            Value::Array(ref v) => { v.get(*self) },
            Value::Dictionary(ref v) => { v.get(&Value::Int(*self as i64)) }
            _ => None
        }
    }
    fn index_into_mut<'a>(&self, v: &'a mut Value) -> Option<&'a mut Value> {
        match *v {
            Value::Array(ref mut v) => { v.get_mut(*self) },
            Value::Dictionary(ref mut v) => { v.get_mut(&Value::Int(*self as i64)) }
            _ => None
        }
    }
    fn index_or_insert<'a>(&self, v: &'a mut Value) -> &'a mut Value {
        match *v {
            Value::Array(ref mut a) => {
                let len = a.len();
                match a.get_mut(*self) {
                    Some(x) => x,
                    _ => panic!(
                        "cannot access index {} of SION array of length {}",
                        self, len
                    )
                }
            },
            Value::Dictionary(ref mut d) => {
                match d.get_mut(&Value::Int(*self as i64)) {
                    Some(x) => x,
                    _ => panic!(
                        "cannot access key {:?} in SION dictionary",
                        self,
                    ),
                }
            }
            _ => panic!("cannot access index {} of SION {}", self, Type(v))
        }
    }
}

impl Index for str {
    fn index_into<'a>(&self, v: &'a Value) -> Option<&'a Value> {
        match *v {
            Value::Dictionary(ref v) => { v.get(&Value::String(self.to_string())) }
            _ => None
        }
    }
    fn index_into_mut<'a>(&self, v: &'a mut Value) -> Option<&'a mut Value> {
        match *v {
            Value::Dictionary(ref mut v) => { v.get_mut(&Value::String(self.to_string())) }
            _ => None
        }
    }
    fn index_or_insert<'a>(&self, v: &'a mut Value) -> &'a mut Value {
        match *v {
            Value::Dictionary(ref mut d) => {
                match d.get_mut(&Value::String(self.to_string())) {
                    Some(v) => v,
                    _ => panic!(
                        "cannot access key {:?} in SION dictionary",
                        self,
                    )
                }
            }
            _ => panic!("cannot access key {:?} in SION {}", self, Type(v))
        }
    }
}

impl Index for String {
    fn index_into<'a>(&self, v: &'a Value) -> Option<&'a Value> {
        self[..].index_into(v)
    }
    fn index_into_mut<'a>(&self, v: &'a mut Value) -> Option<&'a mut Value> {
        self[..].index_into_mut(v)
    }
    fn index_or_insert<'a>(&self, v: &'a mut Value) -> &'a mut Value {
        self[..].index_or_insert(v)
    }
}

