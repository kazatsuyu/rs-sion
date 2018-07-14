use std::collections::hash_map;
use indexmap;

mod iter;
mod entry;

pub use self::iter::{
    // Structs
    IntoIter, Iter, IterMut,
};
pub use self::entry::{
    // Structs
    Keys, OccupiedEntry, VacantEntry, Values, ValuesMut,
    // Enums
    Entry,
};

pub struct Map<K, V, S = hash_map::RandomState> {
    map: indexmap::IndexMap<K, V, S>,
}
