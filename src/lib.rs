extern crate core;
extern crate serde;
extern crate indexmap;

pub mod date;
pub mod de;
pub mod error;
pub mod map;
pub mod ser;
pub mod value;

mod macros;

pub use date::Date;

pub use de::{
    // Structs
    Deserializer, IoRead, SliceRead, StrRead, StreamDeserializer,
    // Traits
    Read, Functions,
    // Functions
    from_reader, from_slice, from_str,
};

pub use error::{
    // Structs
    Error,
    // Enums
    Category,
    // Type Definitions
    Result,
};

pub use map::{
    // Structs
    IntoIter, Iter, IterMut, Keys, Map, OccupiedEntry, VacantEntry, Values, ValuesMut,
    // Enums
    Entry,
};

pub use ser::{
    // Structs
    CompactFormatter, PrettyFormatter, Serializer,
    // Enums
    CharEscape,
    // Traits
    Formatter,
    // Functions
    to_string, to_string_pretty, to_vec, to_vec_pretty, to_writer, to_writer_pretty,
};

pub use value::{
    // Enums
    Value,
    // Traits
    Index,
    // Functions
    from_value, to_value,
};
