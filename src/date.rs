use std::{str, fmt, time, result};
use core::{cmp, convert};
use serde::{ser, de};

use error::{Error, Result};

#[derive(Clone, Copy)]
pub struct Date(f64);

impl Date {
    pub fn as_f64(&self) -> f64 { self.0 }
    pub fn from_f64(f: f64) -> Self { Date(f) }
}

impl str::FromStr for Date {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        unimplemented!()
    }
}

impl cmp::PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

impl cmp::Eq for Date {}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl ser::Serialize for Date {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> result::Result<S::Ok, S::Error> {
        unimplemented!()
    }
}

impl<'de> de::Deserialize<'de> for Date {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> result::Result<Date, D::Error> {
        unimplemented!()
    } 
}

impl<'de> de::Deserializer<'de> for Date {
    type Error = Error;
    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_char<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_unit_struct<V: de::Visitor<'de>>(self, name: &'static str, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_newtype_struct<V: de::Visitor<'de>>(self, name: &'static str, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_tuple<V: de::Visitor<'de>>(self, len: usize, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V
    ) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_map<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V
    ) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_enum<V: de::Visitor<'de>>(self, 
        name: &'static str, 
        variants: &'static [&'static str], 
        visitor: V
    ) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_identifier<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, visitor: V) -> result::Result<V::Value, Error> {
        unimplemented!()
    }
}

impl convert::From<time::Instant> for Date {
    fn from(u: time::Instant) -> Self {
        unimplemented!()
    }
}
