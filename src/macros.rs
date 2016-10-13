#![macro_use]

macro_rules! fail {
    ($expr:expr) => (
        return ::std::result::Result::Err(::std::convert::From::from($expr));
    )
}
