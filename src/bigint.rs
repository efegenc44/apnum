use crate::{APNum, APNumParseError, BigInt, BigNat, Sign, BigDigit};

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

macro_rules! impl_digit_size_arithmetic {
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

impl BigInt {
    cmp_digit_size!(cmp_u8 -> u8; cmp_u16 -> u16; cmp_u32 -> u32 |
                    cmp_i8, cmp_u8 -> i8; cmp_i16, cmp_u16 -> i16; cmp_i32, cmp_u32 -> i32);

    eq_digit_size!(eq_u8 -> u8; eq_u16 -> u16; eq_u32 -> u32 |
                   eq_i8 -> i8; eq_i16 -> i16; eq_i32 -> i32);
                    
    pub fn is_negative(&self) -> bool {
        self.sign == Sign::Negative
    }

    pub fn is_positive(&self) -> bool {
        self.sign == Sign::Positive
    }

    pub fn abs(&self) -> Self {
        let mut result = self.clone();
        if self.is_negative() {
            result.sign = Sign::Positive;
        }
        result
    }
}

impl APNum for BigInt {
    fn zero() -> Self {
        Self {
            sign: Sign::Zero,
            natural: BigNat::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.sign == Sign::Zero
    }

    fn zero_normalized(mut self) -> Self {
        self.natural = self.natural.zero_normalized();
        if self.natural.is_zero() {
            self.sign = Sign::Zero;
        }
        self
    }

    fn digit_count(&self) -> usize {
        self.natural.digits.len()
    }
}

impl_digit_size_arithmetic!(u8; u16; u32 | i8; i16; i32);

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

impl std::ops::Neg for &BigInt {
    type Output = BigInt;

    fn neg(self) -> Self::Output {
        let mut result = self.clone();
        result.sign = match self.sign {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
            Sign::Zero => Sign::Zero,
        };
        result
    }
}

impl std::ops::Neg for BigInt {
    type Output = BigInt;

    fn neg(self) -> Self::Output {
        -(&self)
    }
}

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

macro_rules! impl_from_integer {
    ($($u:ty)* ; $($s:ty)*) => {
        $(impl From<$u> for BigInt {
            fn from(value: $u) -> Self {
                if value == 0 {
                    return BigInt::zero();
                }

                BigInt { sign: Sign::Positive, natural: BigNat::from(value) }
            }
        })*

        $(impl From<$s> for BigInt {
            fn from(value: $s) -> Self {
                if value == 0 {
                    return BigInt::zero();
                }

                let sign = if value < 0 {
                    Sign::Negative
                } else {
                    Sign::Positive
                };

                BigInt { sign, natural: BigNat::from(value.unsigned_abs()) }
            }
        })*
    }
}

impl_from_integer!(usize u8 u16 u32 u64 ;
                   isize i8 i16 i32 i64);

impl From<BigNat> for BigInt {
    fn from(value: BigNat) -> Self {
        BigInt {
            sign: Sign::Positive,
            natural: value,
        }
    }
}

impl std::str::FromStr for BigInt {
    type Err = APNumParseError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(APNumParseError::Empty);
        }

        let sign = if let Some(b'-') = s.as_bytes().first() {
            s = &s[1..];
            Sign::Negative
        } else {
            Sign::Positive
        };

        let natural = s.parse::<BigNat>()?;

        Ok(if natural.is_zero() {
            BigInt::zero()
        } else {
            BigInt { sign, natural }
        })
    }
}

impl TryFrom<&str> for BigInt {
    type Error = APNumParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Sign::Zero = self.sign {
            return write!(f, "0");
        }

        if let Sign::Negative = self.sign {
            write!(f, "-")?;
        }

        std::fmt::Display::fmt(&self.natural, f)
    }
}

impl std::fmt::Debug for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{APNum, APNumParseError, BigInt, Sign};

    #[test]
    fn simple_valid() {
        let result = "-9546970867456973047694867034678".parse::<BigInt>();
        assert!(result.is_ok_and(|bigint| matches!(
            bigint.natural.digits[..],
            [1443885622, 2721072739, 2146252237, 120]
        ) && bigint.sign == Sign::Negative))
    }

    #[test]
    fn zero() {
        let result = "00000".parse::<BigInt>();
        assert!(result.is_ok_and(|bigint| bigint.is_zero()))
    }

    #[test]
    fn empty_with_minus() {
        let result = "-".parse::<BigInt>();
        assert!(result.is_err_and(|err| matches!(err, APNumParseError::Empty)))
    }

    #[test]
    fn multiple_minuses() {
        let result = "--".parse::<BigInt>();
        assert!(result.is_err_and(|err| matches!(err, APNumParseError::Invalid)))
    }
}
