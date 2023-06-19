use crate::{impl_from_for_integer, APNum, APNumParseError, BigInt, BigNat, Sign};

impl BigInt {
    pub fn is_negative(&self) -> bool {
        self.sign == Sign::Negative
    }

    pub fn is_positive(&self) -> bool {
        self.sign == Sign::Positive
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

impl_from_for_integer!(usize u8 u16 u32 u64 &usize &u8 &u16 &u32 &u64
                       isize i8 i16 i32 i64 &isize &i8 &i16 &i32 &i64 ; BigInt);

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

        for digit in self.natural.digits.iter().rev() {
            digit.fmt(f)?;
        }
        Ok(())
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
        let result = "-1234".parse::<BigInt>();
        assert!(
            result.is_ok_and(|bigint| matches!(bigint.natural.digits[..], [4, 3, 2, 1])
                && bigint.sign == Sign::Negative)
        )
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
