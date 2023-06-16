use crate::{APNum, APNumParseError, BigNat};

impl APNum for BigNat {
    fn zero() -> Self {
        BigNat { digits: vec![] }
    }

    fn is_zero(&self) -> bool {
        self.digits.is_empty()
    }
}

impl Default for BigNat {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::ops::Add for &BigNat {
    type Output = BigNat;

    fn add(self, rhs: Self) -> Self::Output {
        // Short-circuit
        if self.is_zero() {
            return rhs.clone();
        } else if rhs.is_zero() {
            return self.clone();
        }

        let mut result = BigNat::zero();
        let mut carry = 0;
        for position in 0..self.digits.len().max(rhs.digits.len()) {
            let left_digit = self.digits.get(position).unwrap_or(&0);
            let right_digit = rhs.digits.get(position).unwrap_or(&0);

            // digit_sum ϵ [0; 9 + 9 + 1] ⊂ u8
            let digit_sum = left_digit + right_digit + carry;
            carry = digit_sum / 10;
            result.digits.push(digit_sum % 10);
        }

        if carry > 0 {
            result.digits.push(carry);
        }

        result
    }
}

impl std::ops::Add for BigNat {
    type Output = BigNat;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl std::ops::Mul for &BigNat {
    type Output = BigNat;

    fn mul(self, rhs: Self) -> Self::Output {
        // To avoid 0 results whose .digits is not empty (and to short-circuit).
        //    ex. (123 * 0).digits = [0, 0, 0]
        // This way only one zero value, whose .digits is empty, can be returned by this function.
        if self.is_zero() || rhs.is_zero() {
            return BigNat::zero();
        }

        let mut result = BigNat::zero();
        for (position, right_digit) in rhs.digits.iter().enumerate() {
            let mut product = BigNat {
                digits: vec![0; position],
            };
            let mut carry = 0;
            for left_digit in &self.digits {
                // digit_product ϵ [0; 9*9 + 8] ⊂ u8 (max. carry is 8 by 9*9 = 81)
                let digit_product = left_digit * right_digit + carry;
                carry = digit_product / 10;
                product.digits.push(digit_product % 10);
            }

            if carry > 0 {
                product.digits.push(carry);
            }

            result = result + product;
        }
        result
    }
}

impl std::ops::Mul for BigNat {
    type Output = BigNat;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}

macro_rules! impl_from_for_integer {
    ($($t:ty)*) => ($(
        impl From<$t> for BigNat {
            fn from(value: $t) -> Self {
                BigNat {
                    digits: value.to_string()
                        .as_bytes()
                        .iter()
                        .rev()
                        .map(|ch| ch - b'0' as u8)
                        .collect()
                }
            }
        }
    )*)
}

impl_from_for_integer!(usize u8 u16 u32 u64 &usize &u8 &u16 &u32 &u64);

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

        let mut digits = vec![];
        for ch in s.as_bytes().iter().rev() {
            match ch {
                b'0'..=b'9' => digits.push(*ch - b'0'),
                _ => return Err(APNumParseError::Invalid),
            }
        }

        Ok(BigNat { digits })
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
        for digit in self.digits.iter().rev() {
            digit.fmt(f)?;
        }
        Ok(())
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
        let result = "1234".parse::<BigNat>();
        assert!(result.is_ok_and(|bignat| matches!(bignat.digits[..], [4, 3, 2, 1])))
    }

    #[test]
    fn leading_zeros() {
        let result = "001234".parse::<BigNat>();
        assert!(result.is_ok_and(|bignat| matches!(bignat.digits[..], [4, 3, 2, 1])))
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
