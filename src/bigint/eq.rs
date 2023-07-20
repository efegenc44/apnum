use crate::{BigInt, BigDigit, APNum, Sign};

macro_rules! eq_digit_size {
    ($( $eq_func_u:ident -> $uty:ident );* | $( $eq_func_s:ident -> $sty:ident );*) => ($(
        pub fn $eq_func_u(&self, other: $uty) -> bool {
            self.is_positive() &&
            self.digit_count() == 1 &&
            self.natural.digits[0] == other as BigDigit
        }

        pub fn $eq_func_s(&self, other: $sty) -> bool {
            self.digit_count() == 1 &&
            (match (&self.sign, other.signum()) {
                (Sign::Negative, -1) |
                (Sign::Positive, 1) |
                (Sign::Zero, 0) => true,
                _ => false
            }) &&
            self.natural.digits[0] == other as BigDigit
        }
    )*);
}

impl BigInt {
    eq_digit_size!(eq_u8 -> u8; eq_u16 -> u16; eq_u32 -> u32 |
                   eq_i8 -> i8; eq_i16 -> i16; eq_i32 -> i32);
}
