use crate::{BigNat, BiggerDigit, BASE, BigDigit, APNum};

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

macro_rules! impl_digit_size_multiplication {
    ($( $ty:ident );*) => ($(
        impl std::ops::Mul<$ty> for &BigNat {
            type Output = BigNat;
        
            fn mul(self, rhs: $ty) -> Self::Output {
                if self.is_zero() || rhs == 0 {
                    return BigNat::zero();
                }
        
                let mut product = BigNat::zero();
                let mut carry = 0;
                for left_digit in &self.digits {
                    let digit_product = *left_digit as BiggerDigit * rhs as BiggerDigit + carry as BiggerDigit;
                    carry = (digit_product / BASE) as BigDigit;
                    product.digits.push((digit_product % BASE) as BigDigit);
                }
        
                if carry > 0 {
                    product.digits.push(carry);
                }
        
                product
            }
        }

        impl std::ops::Mul<BigNat> for $ty {
            type Output = BigNat;
            fn mul(self, rhs: BigNat) -> Self::Output {
                &rhs * self
            }
        }

        impl std::ops::Mul<&BigNat> for $ty {
            type Output = BigNat;
            fn mul(self, rhs: &BigNat) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Mul<$ty> for BigNat {
            type Output = BigNat;
            fn mul(self, rhs: $ty) -> Self::Output {
                (&self).mul(rhs)
            }
        }
    )*)
}

impl_digit_size_multiplication!(u8; u16; u32);
