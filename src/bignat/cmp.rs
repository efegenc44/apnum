use crate::{BigNat, BigDigit, APNum};

impl std::cmp::Ord for BigNat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        if let ord @ (Greater | Less) = self.digits.len().cmp(&other.digits.len()) {
            return ord;
        }

        for (left_digit, right_digit) in
            std::iter::zip(self.digits.iter().rev(), other.digits.iter().rev())
        {
            if let ord @ (Greater | Less) = left_digit.cmp(right_digit) {
                return ord;
            }
        }

        Equal
    }
}

impl std::cmp::PartialOrd for BigNat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! cmp_digit_size {
    ($( $cmp_func:ident -> $ty:ident );*) => ($(
        pub fn $cmp_func(&self, other: $ty) -> std::cmp::Ordering {
            use std::cmp::Ordering::*;
    
            if let ord @ (Greater | Less) = self.digit_count().cmp(&1) {
                return ord;
            }
    
            self.digits[0].cmp(&(other as BigDigit))
        }
    )*);
}

impl BigNat {
    cmp_digit_size!(cmp_u8 -> u8; cmp_u16 -> u16; cmp_u32 -> u32);
}
