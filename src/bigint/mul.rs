use crate::{BigInt, Sign};

impl std::ops::Mul for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        use Sign::*;

        let sign = match (&self.sign, &rhs.sign) {
            (Positive, Positive) | (Negative, Negative) => Positive,
            (Positive, Negative) | (Negative, Positive) => Negative,
            (_, Zero) | (Zero, _) => Zero,
        };

        BigInt {
            sign,
            natural: &self.natural * &rhs.natural,
        }
    }
}

impl std::ops::Mul for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}

macro_rules! impl_digit_size_multiplication {
    ( $( $uty:ident );* | $( $sty:ident );*) => ($(
        impl std::ops::Mul<$uty> for &BigInt {
            type Output = BigInt;
        
            fn mul(self, rhs: $uty) -> Self::Output {
                use Sign::*;
        
                let sign = match &self.sign {
                    Positive => Positive,
                    Negative => Negative,
                    Zero => Zero,
                };
        
                BigInt {
                    sign,
                    natural: &self.natural * rhs,
                }
            }
        }

        impl std::ops::Mul<BigInt> for $uty {
            type Output = BigInt;
            fn mul(self, rhs: BigInt) -> Self::Output {
                &rhs * self
            }
        }

        impl std::ops::Mul<&BigInt> for $uty {
            type Output = BigInt;
            fn mul(self, rhs: &BigInt) -> Self::Output {
                rhs * self
            }
        }
        
        impl std::ops::Mul<$uty> for BigInt {
            type Output = BigInt;
            fn mul(self, rhs: $uty) -> Self::Output {
                (&self).mul(rhs)
            }
        }

        impl std::ops::Mul<$sty> for &BigInt {
            type Output = BigInt;
        
            fn mul(self, rhs: $sty) -> Self::Output {
                use Sign::*;
        
                let sign = match (&self.sign, rhs.signum()) {
                    (Positive, 1) | (Negative, -1) => Positive,
                    (Positive, -1) | (Negative, 1) => Negative,
                    (_, 0) | (Zero, _) => Zero,
                    _ => unreachable!()
                };
        
                BigInt {
                    sign,
                    natural: &self.natural * rhs.unsigned_abs(),
                }
            }
        }

        impl std::ops::Mul<BigInt> for $sty {
            type Output = BigInt;
            fn mul(self, rhs: BigInt) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Mul<&BigInt> for $sty {
            type Output = BigInt;
            fn mul(self, rhs: &BigInt) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Mul<$sty> for BigInt {
            type Output = BigInt;
            fn mul(self, rhs: $sty) -> Self::Output {
                (&self).mul(rhs)
            }
        }
    )*)
}

impl_digit_size_multiplication!(u8; u16; u32 | i8; i16; i32);
