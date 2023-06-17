use crate::{impl_from_for_integer, APNum, APNumParseError, BigInt, BigNat, Sign};

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

impl std::ops::Sub for &BigNat {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        use std::cmp::Ordering::*;

        let (bigger, smaller, sign) = match self.cmp(rhs) {
            Less => (rhs, self, Sign::Negative),
            Greater => (self, rhs, Sign::Positive),
            Equal => return BigInt::zero(),
        };

        // Short-circuit
        if bigger.is_zero() {
            return BigInt {
                sign,
                digits: smaller.clone(),
            };
        } else if smaller.is_zero() {
            return BigInt {
                sign,
                digits: bigger.clone(),
            };
        }

        let mut result = BigNat::zero();
        let mut borrowed = false;
        for position in 0..bigger.digits.len() {
            let mut left_digit = *bigger.digits.get(position).unwrap_or(&0);
            let mut right_digit = *smaller.digits.get(position).unwrap_or(&0);

            // [0; 9] => [1; 10]
            // Shift range by 1 to avoid subtraction overflow.
            left_digit += 1;
            right_digit += 1;

            if borrowed {
                // [1; 10] => [0; 9] (borrowed from)
                left_digit -= 1;
            }

            borrowed = left_digit < right_digit;

            if borrowed {
                // [1; 10] => [11; 20]
                // [0; 9]  => [10; 19] (borrowed from)
                left_digit += 10
            }

            // Non-borrowing case (we know that lhs ≥ rhs):
            //   [1; 10] - [1; 10] => [0; 9]
            // Borrowing case:
            //   [11; 20] - [1; 10] = [1; 19]
            //   [10; 19] - [1; 10] = [0; 18] (borrowed from)
            //     Subtraction here cannot exceed 9
            //   For it, we need 1X - Y where X ≥ Y, but X ≥ Y doesn't borrow
            //   effectivly => [1; 9] and [0; 9] respectively
            result.digits.push(left_digit - right_digit);
        }

        // Cannot happen, always smaller one subtracted from bigger one 
        debug_assert!(!borrowed);

        // To unify the representation, trim potential leading zeros. (covers 0 case)
        //   ex. (120 - 112).digits = [8, 0, 0]
        while let Some(0) = result.digits.last() {
            result.digits.pop();
        }

        BigInt {
            sign,
            digits: result,
        }
    }
}

impl std::ops::Sub for BigNat {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

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

impl_from_for_integer!(usize u8 u16 u32 u64 &usize &u8 &u16 &u32 &u64 ; BigNat);

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
