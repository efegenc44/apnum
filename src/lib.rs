// Aritrary Precision Numbers (APNum)
mod bignat;

/// Arbitrary Precision Natural Number
#[derive(PartialEq, Eq, Clone)]
pub struct BigNat {
    /// Base10 digits in reverse order
    pub(crate) digits: Vec<u8>,
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
