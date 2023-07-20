use crate::{BigInt, Sign, APNum};

impl std::ops::Div for &BigInt {
    type Output = (BigInt, BigInt);

    fn div(self, rhs: Self) -> Self::Output {
        use Sign::*;

        let (q, r) = &self.natural / &rhs.natural;
        let (q, r): (BigInt, BigInt) = (q.into(), r.into());

        match (&self.sign, &rhs.sign) {
            (Positive, Positive) => (q, r),
            (Negative, Negative) => (q, -r),
            (Positive, Negative) => (-q - BigInt::from(1), rhs + &r),
            (Negative, Positive) => (-q - BigInt::from(1), rhs - &r),
            (Zero, _) => (BigInt::zero(), BigInt::zero()),
            // Handled at BigNat division
            (_, Zero) => unreachable!(),
        }
    }
}

impl std::ops::Div for BigInt {
    type Output = (BigInt, BigInt);

    fn div(self, rhs: Self) -> Self::Output {
        (&self).div(&rhs)
    }
}

macro_rules! impl_digit_size_division {
    ( $( $uty:ident );* | $( $sty:ident );*) => ($(
        impl std::ops::Div<$uty> for &BigInt {
            type Output = (BigInt, $uty);
        
            fn div(self, rhs: $uty) -> Self::Output {
                use Sign::*;
        
                let (q, r) = &self.natural / rhs;
                let (q, r): (BigInt, _) = (q.into(), r);
        
                match &self.sign {
                    Positive => (q, r),
                    Negative => (-q - BigInt::from(1), rhs - r),
                    Zero => (BigInt::zero(), 0),
                }
            }
        }

        impl std::ops::Div<$uty> for BigInt {
            type Output = (BigInt, $uty);
            fn div(self, rhs: $uty) -> Self::Output {
                (&self).div(rhs)
            }
        }

        impl std::ops::Div<$sty> for &BigInt {
            type Output = (BigInt, $sty);
        
            fn div(self, rhs: $sty) -> Self::Output {
                use Sign::*;
        
                let (q, r) = &self.natural / rhs.unsigned_abs();
                let (q, r): (BigInt, _) = (q.into(), r);
        
                match (&self.sign, &rhs.signum()) {
                    (Positive, 1) => (q, r as $sty),
                    (Negative, -1) => (q, -(r as $sty)),
                    (Positive, -1) => (-q - BigInt::from(1), rhs + (r as $sty)),
                    (Negative, 1) => (-q - BigInt::from(1), rhs - (r as $sty)),
                    (Zero, _) => (BigInt::zero(), 0),
                    // Handled at BigNat division
                    (_, 0) => unreachable!(),
                    _ => unreachable!()
                }
            }
        }

        impl std::ops::Div<$sty> for BigInt {
            type Output = (BigInt, $sty);
            fn div(self, rhs: $sty) -> Self::Output {
                (&self).div(rhs)
            }
        }
    )*)
}

impl_digit_size_division!(u8; u16; u32 | i8; i16; i32);
