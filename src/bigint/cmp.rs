use crate::{BigInt, Sign};

impl std::cmp::Ord for BigInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        use Sign::*;

        match (&self.sign, &other.sign) {
            (Positive, Positive) => self.natural.cmp(&other.natural),
            (Negative, Negative) => self.natural.cmp(&other.natural).reverse(),
            (Positive, Negative) => Greater,
            (Negative, Positive) => Less,
            (Positive, Zero) => Greater,
            (Negative, Zero) => Less,
            (Zero, Negative) => Greater,
            (Zero, Positive) => Less,
            (Zero, Zero) => Equal,
        }
    }
}

impl std::cmp::PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! cmp_digit_size {
    ( $( $cmp_func_u:ident -> $uty:ident );* | $( $cmp_func_s:ident, $cmp_func_ss:ident -> $sty:ident );* ) => ($(
        pub fn $cmp_func_u(&self, other: $uty) -> std::cmp::Ordering {
            match self.sign {
                Sign::Negative => self.natural.$cmp_func_u(other).reverse(),
                _ => self.natural.$cmp_func_u(other)
            }
        }

        pub fn $cmp_func_s(&self, other: $sty) -> std::cmp::Ordering {
            use Sign::*;
            use std::cmp::Ordering::*;

            match (&self.sign, other.signum()) {
                (Positive, 1) => self.natural.$cmp_func_ss(other.unsigned_abs()),
                (Negative, -1) => self.natural.$cmp_func_ss(other.unsigned_abs()).reverse(),
                (Positive, -1) => Greater,
                (Negative, 1) => Less,
                (Positive, 0) => Greater,
                (Negative, 0) => Less,
                (Zero, -1) => Greater,
                (Zero, 1) => Less,
                (Zero, 0) => Equal,
                _ => unreachable!()         
            }
        }
    )*);
}

impl BigInt {
    cmp_digit_size!(cmp_u8 -> u8; cmp_u16 -> u16; cmp_u32 -> u32 |
                    cmp_i8, cmp_u8 -> i8; cmp_i16, cmp_u16 -> i16; cmp_i32, cmp_u32 -> i32);
}
