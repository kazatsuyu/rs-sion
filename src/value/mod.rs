use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops;
// use std::io;
// use std::mem;
// use std::str;

// use serde::de::DeserializeOwned;
// use serde::ser::Serialize;

// use error::Error;
pub use map::Map;
pub use date::Date;
use utils::{f64_eq, f64_cmp};

pub use self::index::Index;


// use self::ser::Serializer;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Double(f64),
    String(String),
    Data(Vec<u8>),
    Date(Date),
    Array(Vec<Value>),
    Dictionary(Map<Value, Value>),
}

impl Value {
    /*
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }
    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        match *self {
            Value::Object(ref map) => Some(map),
            _ => None,
        }
    }
    pub fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>> {
        match *self {
            Value::Object(ref mut map) => Some(map),
            _ => None,
        }
    }
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref array) => Some(&*array),
            _ => None,
        }
    }
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut list) => Some(list),
            _ => None,
        }
    }
    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(s),
            _ => None,
        }
    }
    pub fn is_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false,
        }
    }
    pub fn is_i64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_i64(),
            _ => false,
        }
    }
    pub fn is_u64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_u64(),
            _ => false,
        }
    }
    pub fn is_f64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_f64(),
            _ => false,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::Number(ref n) => n.as_i64(),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::Number(ref n) => n.as_u64(),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::Number(ref n) => n.as_f64(),
            _ => None,
        }
    }
    pub fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }
    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }
    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }
    pub fn pointer<'a>(&'a self, pointer: &str) -> Option<&'a Value> {
        if pointer == "" {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        let tokens = pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"));
        let mut target = self;

        for token in tokens {
            let target_opt = match *target {
                Value::Object(ref map) => map.get(&token),
                Value::Array(ref list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
    }
    pub fn pointer_mut<'a>(&'a mut self, pointer: &str) -> Option<&'a mut Value> {
        if pointer == "" {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        let tokens = pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"));
        let mut target = self;

        for token in tokens {
            // borrow checker gets confused about `target` being mutably borrowed too many times because of the loop
            // this once-per-loop binding makes the scope clearer and circumvents the error
            let target_once = target;
            let target_opt = match *target_once {
                Value::Object(ref mut map) => map.get_mut(&token),
                Value::Array(ref mut list) => {
                    parse_index(&token).and_then(move |x| list.get_mut(x))
                }
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
    }
    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }
    */
}

impl Default for Value {
    fn default() -> Value { Value::Nil }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Value::Nil, &Value::Nil) => true,
            (&Value::Bool(x), &Value::Bool(y)) => x.eq(&y),
            (&Value::Int(x), &Value::Int(y)) => x.eq(&y),
            (&Value::Double(x), &Value::Double(y)) => f64_eq(x, y),
            (&Value::String(ref x), &Value::String(ref y)) => x.eq(y),
            (&Value::Data(ref x), &Value::Data(ref y)) => x.eq(y),
            (&Value::Date(x), &Value::Date(y)) => x.eq(&y),
            (&Value::Array(ref x), &Value::Array(ref y)) => x.eq(y),
            (&Value::Dictionary(ref x), &Value::Dictionary(ref y)) => x.eq(y),
            _ => false,
        }
    }
}

impl Eq for Value { }

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Value::Nil => state.write_u8(0),
            Value::Bool(false) => state.write_u8(1),
            Value::Bool(true) => state.write_u8(2),
            Value::Int(v) => {
                state.write_u8(3);
                state.write_i64(v);
            },
            Value::Double(v) => {
                state.write_u8(4);
                state.write_u64(v.to_bits());
            },
            Value::String(ref v) => {
                state.write_u8(5);
                state.write(v.as_bytes());
            },
            Value::Data(ref v) => {
                state.write_u8(6);
                state.write(v.as_slice());
            },
            Value::Date(v) => {
                state.write_u8(7);
                state.write_u64(v.to_f64().to_bits());
            },
            Value::Array(ref v) => {
                state.write_u8(8);
                v.iter().for_each(|v|{ v.hash(state) });
            },
            Value::Dictionary(ref v) => {
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

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use self::Ordering::*;
        match (self, other) {
            (&Value::Nil, &Value::Nil) => Equal,
            (&Value::Bool(x), &Value::Bool(y)) => x.cmp(&y),
            (&Value::Int(x), &Value::Int(y)) => x.cmp(&y),
            (&Value::Double(x), &Value::Double(y)) => f64_cmp(x, y),
            (&Value::String(ref x), &Value::String(ref y)) => x.cmp(y),
            (&Value::Data(ref x), &Value::Data(ref y)) => x.cmp(y),
            (&Value::Date(ref x), &Value::Date(ref y)) => x.cmp(&y),
            (&Value::Array(ref x), &Value::Array(ref y)) => x.cmp(y),
            (&Value::Dictionary(ref x), &Value::Dictionary(ref y)) => {
                let mut x1 = x.iter().collect::<Vec<(&Value, &Value)>>();
                x1.sort();
                let mut y1 = y.iter().collect::<Vec<(&Value, &Value)>>();
                y1.sort();
                x1.cmp(&y1)
            },
            (&Value::Nil, _) => Less,
            (_, &Value::Nil) => Greater,
            (&Value::Bool(_), _) => Less,
            (_, &Value::Bool(_)) => Greater,
            (&Value::Int(_), _) => Less,
            (_, &Value::Int(_)) => Greater,
            (&Value::Double(_), _) => Less,
            (_, &Value::Double(_)) => Greater,
            (&Value::String(_), _) => Less,
            (_, &Value::String(_)) => Greater,
            (&Value::Data(_), _) => Less,
            (_, &Value::Data(_)) => Greater,
            (&Value::Date(_), _) => Less,
            (_, &Value::Date(_)) => Greater,
            (&Value::Array(_), _) => Less,
            (_, &Value::Array(_)) => Greater,
        }
    }
}

mod de;
mod from;
mod index;
// mod partial_eq;
mod ser;

#[cfg(test)]
mod tests {
    #[test]
    fn name() {
        unimplemented!();
    }
}