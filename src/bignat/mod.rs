pub mod add;
pub mod mul;
pub mod sub;
pub mod div;
pub mod cmp;
pub mod eq;

use crate::{APNum, APNumParseError, BigNat, BASE, BigDigit};

impl BigNat {    
    fn pow_uint(&self, power: usize) -> BigNat {
        if power == 0 {
            return BigNat::from(1usize);
        }

        let mut acc = BigNat::from(1usize);
        for _ in 0..power {
            acc = &acc * self;
        }
        acc
    }
}

impl APNum for BigNat {
    fn zero() -> Self {
        BigNat { digits: vec![] }
    }

    fn is_zero(&self) -> bool {
        self.digits.is_empty()
    }

    fn zero_normalized(mut self) -> Self {
        while let Some(0) = self.digits.last() {
            self.digits.pop();
        }
        self
    }

    fn digit_count(&self) -> usize {
        self.digits.len()
    }
}

impl TryInto<BigDigit> for &BigNat {
    type Error = ();

    fn try_into(self) -> Result<BigDigit, Self::Error> {
        if self.is_zero() {
            Ok(0)
        } else if self.digit_count() == 1 {
            Ok(self.digits[0])
        } else {
            Err(())
        }
    }
}

impl TryInto<BigDigit> for BigNat {
    type Error = ();

    fn try_into(self) -> Result<BigDigit, Self::Error> {
        (&self).try_into()
    }
}

impl Default for BigNat {
    fn default() -> Self {
        Self::zero()
    }
}

macro_rules! impl_from_integer {
    ($($t:ty)*) => ($(
        impl From<$t> for BigNat {
            fn from(value: $t) -> Self {
                if value == 0 {
                    return BigNat::zero();
                }

                let mut value = value as u64;
                let mut result = BigNat::zero();
                while value != 0 {
                    result.digits.push((value % BASE) as u32);
                    value /= BASE;
                }
                result
            }
        }
    )*)
}

impl_from_integer!(usize u8 u16 u32 u64);

impl From<&[u32]> for BigNat {
    fn from(value: &[u32]) -> Self {
        BigNat {
            digits: value.into(),
        }
    }
}

impl std::str::FromStr for BigNat {
    type Err = APNumParseError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(APNumParseError::Empty);
        }

        // Remove leading zeros
        while let Some(b'0') = s.as_bytes().first() {
            s = &s[1..];
        }

        let mut result = BigNat::zero();
        for (position, ch) in s.as_bytes().iter().rev().enumerate() {
            match ch {
                b'0'..=b'9' => {
                    result =
                        result + BigNat::from(ch - b'0') * BigNat::from(10usize).pow_uint(position)
                }
                _ => return Err(APNumParseError::Invalid),
            }
        }
        Ok(result)
    }
}

impl TryFrom<&str> for BigNat {
    type Error = APNumParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::fmt::Display for BigNat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        // see. Knuth, The Art Of Computer Programming Vol. 2 Section 4.4, Method 1a
        let mut number = self.clone();
        let mut number_base10 = String::new();
        while !number.is_zero() {
            let (nn, digit) = number / BigNat::from(10usize);

            if digit.is_zero() {
                number_base10 += "0";
            } else {
                debug_assert!(digit.digit_count() == 1);
                number_base10 += &digit.digits[0].to_string();
            };

            number = nn;
        }

        unsafe {
            number_base10.as_bytes_mut().reverse();
        }

        write!(f, "{number_base10}")
    }
}

impl std::fmt::Debug for BigNat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{APNumParseError, BigNat};

    #[test]
    fn simple_valid() {
        let result = "9546970867456973047694867034678".parse::<BigNat>();
        assert!(result.is_ok_and(|bignat| matches!(
            bignat.digits[..],
            [1443885622, 2721072739, 2146252237, 120]
        )))
    }

    #[test]
    fn leading_zeros() {
        let result = "0009546970867456973047694867034678".parse::<BigNat>();
        assert!(result.is_ok_and(|bignat| matches!(
            bignat.digits[..],
            [1443885622, 2721072739, 2146252237, 120]
        )))
    }

    #[test]
    fn empty_string() {
        let result = "".parse::<BigNat>();
        assert!(result.is_err_and(|err| matches!(err, APNumParseError::Empty)))
    }

    #[test]
    fn invalid_string() {
        let result = "1234f".parse::<BigNat>();
        assert!(result.is_err_and(|err| matches!(err, APNumParseError::Invalid)));
        let result = "-1234".parse::<BigNat>();
        assert!(result.is_err_and(|err| matches!(err, APNumParseError::Invalid)))
    }
}
