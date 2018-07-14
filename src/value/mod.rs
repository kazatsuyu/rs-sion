use std::vec;
use serde::{ser, de};

use error::Error;

pub use map::Map;
pub use date::Date;

pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Double(f64),
    String(String),
    Data(vec::Vec<u8>),
    Date(Date),
    Array(vec::Vec<Value>),
    Dictionary(Map<Value, Value>),
}

pub trait Index {

}

pub fn from_value<T: de::DeserializeOwned>(value: Value) -> Result<T, Error> {
    unimplemented!()
}

pub fn to_value<T: ser::Serialize>(value: T) -> Result<Value, Error> {
    unimplemented!()
}
