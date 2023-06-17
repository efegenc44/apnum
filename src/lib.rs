// Aritrary Precision Numbers (APNum)
mod bigint;
mod bignat;

/// Arbitrary Precision Natural Number
#[derive(PartialEq, Eq, Clone)]
pub struct BigNat {
    /// Base10 digits in reverse order
    pub(crate) digits: Vec<u8>,
}

/// Arbitrary Precision Integer Number
#[derive(PartialEq, Eq, Clone)]
pub struct BigInt {
    pub(crate) sign: Sign,
    pub(crate) digits: BigNat,
}

#[derive(PartialEq, Eq, Clone)]
pub(crate) enum Sign {
    Positive,
    Negative,
    Zero,
}

pub trait APNum {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

#[derive(Debug)]
pub enum APNumParseError {
    Empty,
    Invalid,
}

#[macro_export]
macro_rules! impl_from_for_integer {
    ($($t:ty)* ; $nt:ident) => ($(
        impl From<$t> for $nt {
            fn from(value: $t) -> Self {
                value.to_string().parse().unwrap()
            }
        }
    )*)
}
