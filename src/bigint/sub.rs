use crate::{BigInt, Sign};

impl std::ops::Sub for &BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &-rhs
    }
}

impl std::ops::Sub for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

macro_rules! impl_digit_size_subtraction {
    ( $( $uty:ident );* | $( $sty:ident );*) => ($(
        impl std::ops::Sub<$uty> for &BigInt {
            type Output = BigInt;
        
            fn sub(self, rhs: $uty) -> Self::Output {
                use Sign::*;

                match self.sign {
                    Positive => &self.natural - rhs,
                    Negative => -(&self.natural - rhs),
                    Zero     => -BigInt::from(rhs)
                }
            }
        }

        impl std::ops::Sub<BigInt> for $uty {
            type Output = BigInt;
            fn sub(self, rhs: BigInt) -> Self::Output {
                -(&rhs - self)
            }
        }

        impl std::ops::Sub<&BigInt> for $uty {
            type Output = BigInt;
            fn sub(self, rhs: &BigInt) -> Self::Output {
                -(rhs - self)
            }
        }

        impl std::ops::Sub<$uty> for BigInt {
            type Output = BigInt;
            fn sub(self, rhs: $uty) -> Self::Output {
                (&self).sub(rhs)
            }
        }

        impl std::ops::Sub<$sty> for &BigInt {
            type Output = BigInt;
        
            fn sub(self, rhs: $sty) -> Self::Output {
                match rhs.signum() {
                    1  => self - rhs.unsigned_abs(),
                    0  => self.clone(),
                    -1 => self + rhs.unsigned_abs(),
                    _ => unreachable!()
                }
            }
        }
        
        impl std::ops::Sub<BigInt> for $sty {
            type Output = BigInt;
            fn sub(self, rhs: BigInt) -> Self::Output {
                -(&rhs - self)
            }
        }

        impl std::ops::Sub<&BigInt> for $sty {
            type Output = BigInt;
            fn sub(self, rhs: &BigInt) -> Self::Output {
                -(rhs - self)
            }
        }

        impl std::ops::Sub<$sty> for BigInt {
            type Output = BigInt;
            fn sub(self, rhs: $sty) -> Self::Output {
                (&self).sub(rhs)
            }
        }
    )*)
}

impl_digit_size_subtraction!(u8; u16; u32 | i8; i16; i32);
