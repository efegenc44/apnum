use crate::{BigInt, Sign};

impl std::ops::Add for &BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        use Sign::*;

        match (&self.sign, &rhs.sign) {
            (Positive, Positive) => BigInt {
                sign: Sign::Positive,
                natural: &self.natural + &rhs.natural,
            },
            (Negative, Negative) => BigInt {
                sign: Sign::Negative,
                natural: &self.natural + &rhs.natural,
            },
            (Positive, Negative) => &self.natural - &rhs.natural,
            (Negative, Positive) => &rhs.natural - &self.natural,
            (_, Zero) => self.clone(),
            (Zero, _) => rhs.clone(),
        }
    }
}

impl std::ops::Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

macro_rules! impl_digit_size_addition {
    ( $( $uty:ident );* | $( $sty:ident );*) => ($(
        impl std::ops::Add<$uty> for &BigInt {
            type Output = BigInt;
        
            fn add(self, rhs: $uty) -> Self::Output {
                use Sign::*;
        
                match &self.sign {
                    Positive => BigInt {
                        sign: Sign::Positive,
                        natural: &self.natural + rhs,
                    },
                    Negative => rhs - &self.natural,
                    Zero => rhs.into(),
                }
            }
        }

        impl std::ops::Add<BigInt> for $uty {
            type Output = BigInt;
            fn add(self, rhs: BigInt) -> Self::Output {
                &rhs + self
            }
        }

        impl std::ops::Add<&BigInt> for $uty {
            type Output = BigInt;
            fn add(self, rhs: &BigInt) -> Self::Output {
                rhs + self
            }
        }

        impl std::ops::Add<$uty> for BigInt {
            type Output = BigInt;
            fn add(self, rhs: $uty) -> Self::Output {
                (&self).add(rhs)
            }
        }

        impl std::ops::Add<$sty> for &BigInt {
            type Output = BigInt;
        
            fn add(self, rhs: $sty) -> Self::Output {
                match rhs.signum() {
                    1  => self + rhs.unsigned_abs(),
                    0  => self.clone(),
                    -1 => self - rhs.unsigned_abs(),
                    _ => unreachable!()
                }
            }
        }

        impl std::ops::Add<BigInt> for $sty {
            type Output = BigInt;
            fn add(self, rhs: BigInt) -> Self::Output {
                &rhs + self
            }
        }

        impl std::ops::Add<&BigInt> for $sty {
            type Output = BigInt;
            fn add(self, rhs: &BigInt) -> Self::Output {
                rhs + self
            }
        }

        impl std::ops::Add<$sty> for BigInt {
            type Output = BigInt;
            fn add(self, rhs: $sty) -> Self::Output {
                (&self).add(rhs)
            }
        }
    )*)
}

impl_digit_size_addition!(u8; u16; u32 | i8; i16; i32);
