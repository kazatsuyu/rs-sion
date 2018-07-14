#[macro_export]
macro_rules! sion {
    ($($sion:tt)+) => { self::sion_internal!($($sion)+) };
}

#[macro_export]
macro_rules! sion_internal {
    (nil) => { unimplemented!() }
}

