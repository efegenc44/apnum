use crate::{APNum, APNumParseError, BigDigit, BigInt, BigNat, BiggerDigit, Sign, BASE};

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

            // digit_sum ϵ [0; (2^32 - 1) + (2^32 - 1) + 1] ⊂ u64
            let digit_sum =
                *left_digit as BiggerDigit + *right_digit as BiggerDigit + carry as BiggerDigit;
            // carry ϵ { 0, 1 }
            carry = (digit_sum / BASE) as BigDigit;
            // digit_sum % BASE ϵ [0; (2^32 - 1)] ⊂ u32
            result.digits.push((digit_sum % BASE) as BigDigit);
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
                // digit_product ϵ [0; (2^32 - 1)*(2^32 - 1) + 4294967294] ⊂ u64
                // (max. carry is 4294967294 by ((2^32 - 1) * (2^32 - 1)) / 2^32)
                let digit_product =
                    *left_digit as BiggerDigit * *right_digit as BiggerDigit + carry as BiggerDigit;
                // carry ϵ [0; 4294967294] ⊂ u32
                carry = (digit_product / BASE) as BigDigit;
                // digit_product % BASE ϵ [0; (2^32 - 1)] ⊂ u32
                product.digits.push((digit_product % BASE) as BigDigit);
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
                natural: smaller.clone(),
            };
        } else if smaller.is_zero() {
            return BigInt {
                sign,
                natural: bigger.clone(),
            };
        }

        let mut result = BigNat::zero();
        let mut borrowed = false;
        for position in 0..bigger.digits.len() {
            let mut left_digit = *bigger.digits.get(position).unwrap_or(&0) as BiggerDigit;
            let mut right_digit = *smaller.digits.get(position).unwrap_or(&0) as BiggerDigit;

            // [0; 2^32 - 1] => [1; 2^32] ⊂ u64
            // Shift range by 1 to avoid subtraction overflow.
            left_digit += 1;
            right_digit += 1;

            if borrowed {
                // [1; 2^32] => [0; 2^32 - 1] (borrowed from)
                left_digit -= 1;
            }

            borrowed = left_digit < right_digit;

            if borrowed {
                // [1; 2^32]      => [1 + 2^32; 2^33] ⊂ u64
                // [0; 2^32 - 1]  => [2^32; 2^33 - 1] ⊂ u64 (borrowed from)
                left_digit += BASE
            }

            // Non-borrowing case (we know that lhs ≥ rhs):
            //   [1; 2^32] - [1; 2^32] => [0; 2^32 - 1] ⊂ u32
            // Borrowing case:
            //   [1 + 2^32; 2^33] - [1; 2^32] = [1; 2^33 - 1]
            //   [2^32; 2^33 - 1] - [1; 2^32] = [0; 2^33 - 2] (borrowed from)
            //     Subtraction here cannot exceed 2^32 - 1
            //   For it, we need 2^32 + X - Y where X ≥ Y, but X ≥ Y doesn't borrow
            //      borrow value -^     ^---^-- digit ϵ [0; 2^32 - 1]
            //   effectively => [1; 2^32 - 1] and [0; 2^32 - 1] respectively
            result.digits.push((left_digit - right_digit) as BigDigit);
        }

        // Cannot happen, always smaller one subtracted from bigger one
        debug_assert!(!borrowed);

        BigInt {
            sign,
            // To unify the representation, trim potential leading zeros. (covers 0 case)
            //   ex. (120 - 112).digits = [8, 0, 0]
            natural: result.zero_normalized(),
        }
    }
}

impl std::ops::Sub for BigNat {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

// see. Knuth, The Art Of Computer Programming Vol. 2 Section 4.3.1, Algorithm D
impl std::ops::Div for &BigNat {
    type Output = (BigNat, BigNat);

    fn div(self, rhs: Self) -> Self::Output {
        use std::cmp::Ordering::*;

        // Short-circuit
        if rhs == &BigNat::from(1usize) {
            return (self.clone(), BigNat::zero());
        }

        let (mut u, mut v) = match self.cmp(rhs) {
            Less => return (BigNat::zero(), self.clone()),
            Equal => return (BigNat::from(1usize), BigNat::zero()),
            Greater => (self.clone(), rhs.clone()),
        };

        if v.is_zero() {
            panic!("Division by Zero");
        }

        let n = v.digit_count();
        let m = u.digit_count() - n;

        // D1 [Normalize.]
        // 2^32 - 1 / [1; (2^32 - 1)] = [1; 2^32 - 1] ⊂ u32
        let d = BigNat::from(((BASE - 1) / v.digits[n - 1] as BiggerDigit) as BigDigit);
        u.digits.push(0);
        u = &u * &d;
        v = &v * &d;

        // D2 [Initialize j.]
        let mut q = BigNat::zero();

        let mut j = m as isize;
        while j >= 0 {
            let ju = j as usize;

            // D3 [Calculate qh.]
            let (f, s) = (
                u.digits[ju + n] as BiggerDigit,
                u.digits[ju + n - 1] as BiggerDigit,
            );
            // qh ϵ [0; (2^32 - 1)*(2^32) + (2^32 - 1) / 1] = u64
            let mut qh = (f * BASE + s) / v.digits[n - 1] as BiggerDigit;
            // rh ϵ [0; 2^32 - 2] ⊂ u64
            let mut rh = (f * BASE + s) % v.digits[n - 1] as BiggerDigit;

            loop {
                if qh == BASE
                    || (n > 1
                        && BigNat::from(qh) * BigNat::from(v.digits[n - 2])
                            > BigNat::from(BASE) * BigNat::from(rh)
                                + BigNat::from(u.digits[ju + n - 2]))
                {
                    qh -= 1;
                    rh += v.digits[n - 1] as BiggerDigit;

                    if rh < BASE {
                        continue;
                    }
                }
                break;
            }

            // D4 [Multiply and subtract.]
            let mut mul_and_sub = BigNat::from(&u.digits[ju..=ju + n]) - &BigNat::from(qh) * &v;

            // D5 [Test remainder.]
            if mul_and_sub.is_negative() {
                qh -= 1;

                // D6 [Add back.]
                mul_and_sub = &v - &mul_and_sub.natural;
            }

            // Set the lenght of representation to n+1 (len(ju..=ju + n)) for .splice below
            for _ in 0..(n + 1 - mul_and_sub.digit_count()) {
                mul_and_sub.natural.digits.push(0);
            }
            u.digits.splice(ju..=ju + n, mul_and_sub.natural.digits);

            q.digits.push(qh as u32);

            // D7 [Loop on j.]
            j -= 1;
        }

        // D8 [Unnormalize]
        let (r, rr) = BigNat::from(&u.digits[0..=n - 1]).zero_normalized() / d;

        // rr has to be zero, because it's multliplied by d at D1 [Normalize.]
        debug_assert!(rr.is_zero());

        q.digits.reverse();
        (q.zero_normalized(), r)
    }
}

impl std::ops::Div for BigNat {
    type Output = (BigNat, BigNat);

    fn div(self, rhs: Self) -> Self::Output {
        (&self).div(&rhs)
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

macro_rules! impl_from_integer {
    ($($t:ty)*) => ($(
        impl From<$t> for BigNat {
            fn from(value: $t) -> Self {
                let mut value = value as u64;

                if value == 0 {
                    return BigNat::zero();
                }

                if value <= u32::MAX as u64 {
                    return BigNat { digits: vec![value as u32] };
                }

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
