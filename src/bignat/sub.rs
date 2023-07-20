use crate::{BigInt, BigNat, Sign, APNum, BASE, BiggerDigit, BigDigit};

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

macro_rules! impl_digit_size_subtraction {
    ($( $cmp_func:ident -> $ty:ident );*) => ($(
        impl std::ops::Sub<$ty> for &BigNat {
            type Output = BigInt;
        
            fn sub(self, rhs: $ty) -> Self::Output {
                use std::cmp::Ordering::*;
        
                let (bigger, smaller) = match self.$cmp_func(rhs) {
                    Greater => (self, rhs),
                    Less => return BigInt::from(-((rhs as BigDigit - self.digits[0]) as i64)),
                    Equal => return BigInt::zero(),
                };
        
                // Short-circuit
                if bigger.is_zero() {
                    return BigInt {
                        sign: Sign::Positive,
                        natural: smaller.into(),
                    };
                } else if smaller == 0 {
                    return BigInt {
                        sign: Sign::Positive,
                        natural: bigger.clone(),
                    };
                }
        
                let mut result = BigNat::zero();
        
                let mut left_digit = *bigger.digits.first().unwrap_or(&0) as BiggerDigit;
                let mut right_digit = rhs as BiggerDigit;
        
                left_digit += 1;
                right_digit += 1;
        
                let mut borrowed = left_digit < right_digit;
        
                if borrowed {
                    left_digit += BASE
                }
        
                result.digits.push((left_digit - right_digit) as BigDigit);
        
                for position in 1..bigger.digits.len() {
                    let mut left_digit = *bigger.digits.get(position).unwrap_or(&0) as BiggerDigit;
                    let mut right_digit = 0;
        
                    left_digit += 1;
                    right_digit += 1;
        
                    if borrowed {
                        left_digit -= 1;
                    }
        
                    borrowed = left_digit < right_digit;
        
                    if borrowed {
                        left_digit += BASE
                    }
        
                    result.digits.push((left_digit - right_digit) as BigDigit);
                }
        
                BigInt {
                    sign: Sign::Positive,
                    natural: result.zero_normalized(),
                }
            }
        }

        impl std::ops::Sub<BigNat> for $ty {
            type Output = BigInt;
            fn sub(self, rhs: BigNat) -> Self::Output {
                -(&rhs - self)
            }
        }

        impl std::ops::Sub<&BigNat> for $ty {
            type Output = BigInt;
            fn sub(self, rhs: &BigNat) -> Self::Output {
                -(rhs - self)
            }
        }

        impl std::ops::Sub<$ty> for BigNat {
            type Output = BigInt;
            fn sub(self, rhs: $ty) -> Self::Output {
                (&self).sub(rhs)
            }
        }
    )*)
}

impl_digit_size_subtraction!(cmp_u8 -> u8; cmp_u16 -> u16; cmp_u32 -> u32);
