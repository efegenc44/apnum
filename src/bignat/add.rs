use crate::{BigNat, APNum, BiggerDigit, BASE, BigDigit};

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

macro_rules! impl_digit_size_addition {
    ($( $ty:ident );*) => ($(
        impl std::ops::Add<$ty> for &BigNat {
            type Output = BigNat;

            fn add(self, rhs: $ty) -> Self::Output {
                // Short-circuit
                if self.is_zero() {
                    return BigNat::from(rhs);
                } else if rhs == 0 {
                    return self.clone();
                }

                let mut result = BigNat::zero();
                let mut carry = 0;

                let digit_sum = *self.digits.first().unwrap_or(&0) as BiggerDigit
                    + rhs as BiggerDigit
                    + carry as BiggerDigit;
                carry = (digit_sum / BASE) as BigDigit;
                result.digits.push((digit_sum % BASE) as BigDigit);

                for position in 1..self.digits.len() {
                    let left_digit = self.digits[position];

                    let digit_sum = left_digit as BiggerDigit + carry as BiggerDigit;
                    carry = (digit_sum / BASE) as BigDigit;
                    result.digits.push((digit_sum % BASE) as BigDigit);
                }

                if carry > 0 {
                    result.digits.push(carry);
                }

                result
            }
        }

        impl std::ops::Add<BigNat> for $ty {
            type Output = BigNat;
            fn add(self, rhs: BigNat) -> Self::Output {
                &rhs + self
            }
        }

        impl std::ops::Add<&BigNat> for $ty {
            type Output = BigNat;
            fn add(self, rhs: &BigNat) -> Self::Output {
                rhs + self
            }
        }

        impl std::ops::Add<$ty> for BigNat {
            type Output = BigNat;
            fn add(self, rhs: $ty) -> Self::Output {
                (&self).add(rhs)
            }
        }
    )*)
}

impl_digit_size_addition!(u8; u16; u32);
