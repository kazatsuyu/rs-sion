use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::f64::NAN;

use utils::f64_cmp;

#[derive(Debug, Copy, Clone)]
pub struct Date(f64);

impl Date {
    pub fn to_f64(&self) -> f64 { self.0 }
}

impl PartialEq for Date {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => true,
            _ => self.0 == other.0,
        }
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        f64_cmp(self.0, other.0)
    }
}

impl Hash for Date {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0.is_nan() {
            state.write_u64(NAN.to_bits())
        } else {
            state.write_u64(self.0.to_bits())
        }
    }
}
