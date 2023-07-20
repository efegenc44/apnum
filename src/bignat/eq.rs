use crate::{BigNat, BigDigit, APNum};

macro_rules! eq_digit_size {
    ($( $eq_func:ident -> $ty:ident );*) => ($(
        pub fn $eq_func(&self, other: $ty) -> bool {
            self.digit_count() == 1 &&
            self.digits[0] == other as BigDigit
        }
    )*);
}

impl BigNat {
    eq_digit_size!(eq_u8 -> u8; eq_u16 -> u16; eq_u32 -> u32);
}
