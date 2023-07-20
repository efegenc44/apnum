use crate::{BigNat, APNum, BASE, BigDigit, BiggerDigit};

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
        let d = BigNat::from((BASE - 1) as BigDigit / v.digits[n - 1]);
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
            // Originally in the algorithm, it's just a check rather than a loop,
            // but there are some cases where qh is off by more than one (see tests).
            // Beacuse that I don't totally understand what's going on the alogrithm,
            // so I can't tell what's wrong.
            while mul_and_sub.is_negative() {
                qh -= 1;

                // D6 [Add back.]
                mul_and_sub = &v - &mul_and_sub.natural;
            }

            // Set the length of the representation to n+1 (len(ju..=ju + n)) for .splice below
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

macro_rules! impl_digit_size_division {
    ($( $cmp_func:ident -> $ty:ident );*) => ($(
        // see. Knuth, The Art Of Computer Programming Vol. 2 Section 4.3.1, Solution of Exercise 16
        // Short Division
        impl std::ops::Div<$ty> for &BigNat {
            type Output = (BigNat, $ty);

            fn div(self, rhs: $ty) -> Self::Output {
                use std::cmp::Ordering::*;

                // Short-circuit
                if rhs == 1 {
                    return (self.clone(), 0);
                }

                let (u, v) = match self.$cmp_func(rhs) {
                    Less => return (BigNat::zero(), self.digits[0] as $ty),
                    Equal => return (BigNat::from(1usize), 0),
                    Greater => (self.clone(), rhs.clone()),
                };

                if v == 0 {
                    panic!("Division by Zero");
                }

                // S1
                let mut w = BigNat::zero();
                let n = u.digit_count();
                let mut j = n - 1;
                let mut r = 0;

                // S2
                loop {
                    w.digits.push(((r * BASE + u.digits[j] as BiggerDigit) / v as BiggerDigit) as BigDigit);
                    r = (r * BASE + u.digits[j] as BiggerDigit) % v as BiggerDigit;
                    
                    // S3
                    j = match j.checked_sub(1) {
                        Some(j) => j,
                        None => break,
                    };
                }

                w.digits.reverse();
                (w.zero_normalized(), r as $ty)
            }
        }

        impl std::ops::Div<$ty> for BigNat {
            type Output = (BigNat, $ty);
            fn div(self, rhs: $ty) -> Self::Output {
                (&self).div(rhs)
            }
        }
    )*)
}

impl_digit_size_division!(cmp_u8 -> u8; cmp_u16 -> u16; cmp_u32 -> u32);
