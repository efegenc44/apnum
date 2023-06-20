// Aritrary Precision Numbers (APNum)
mod bigint;
mod bignat;

pub type BigDigit = u32;
pub type BiggerDigit = u64;

pub const BASE: u64 = u32::MAX as u64 + 1;

/// Arbitrary Precision Natural Number
#[derive(PartialEq, Eq, Clone)]
pub struct BigNat {
    /// Digits in reverse order
    pub(crate) digits: Vec<BigDigit>,
}

/// Arbitrary Precision Integer Number
#[derive(PartialEq, Eq, Clone)]
pub struct BigInt {
    pub(crate) sign: Sign,
    pub(crate) natural: BigNat,
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
    fn zero_normalized(self) -> Self;
    fn digit_count(&self) -> usize;
}

#[derive(Debug)]
pub enum APNumParseError {
    Empty,
    Invalid,
}
