extern crate indexmap;

use self::indexmap::map::IndexMap;
use std::collections::hash_map::RandomState;

pub type Map<K, V, S = RandomState> = self::IndexMap<K, V, S>;
