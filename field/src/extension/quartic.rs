use core::fmt::{self, Debug, Display, Formatter};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num::bigint::BigUint;
use num::traits::Pow;
use serde::{Deserialize, Serialize};

use crate::extension::{Extendable, FieldExtension, Frobenius, OEF};
use crate::ops::Square;
use crate::types::{Field, Sample};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct QuarticExtension<F: Extendable<4>>(pub [F; 4]);

impl<F: Extendable<4>> Default for QuarticExtension<F> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<F: Extendable<4>> OEF<4> for QuarticExtension<F> {
    const W: F = F::W;
    const DTH_ROOT: F = F::DTH_ROOT;
}

impl<F: Extendable<4>> Frobenius<4> for QuarticExtension<F> {}

impl<F: Extendable<4>> FieldExtension<4> for QuarticExtension<F> {
    type BaseField = F;

    fn to_basefield_array(&self) -> [F; 4] {
        self.0
    }

    fn from_basefield_array(arr: [F; 4]) -> Self {
        Self(arr)
    }

    fn from_basefield(x: F) -> Self {
        x.into()
    }
}

impl<F: Extendable<4>> From<F> for QuarticExtension<F> {
    fn from(x: F) -> Self {
        Self([x, F::ZERO, F::ZERO, F::ZERO])
    }
}

impl<F: Extendable<4>> Sample for QuarticExtension<F> {
    #[inline]
    fn sample<R>(rng: &mut R) -> Self
    where
        R: rand::RngCore + ?Sized,
    {
        Self::from_basefield_array([
            F::sample(rng),
            F::sample(rng),
            F::sample(rng),
            F::sample(rng),
        ])
    }
}

impl<F: Extendable<4>> Field for QuarticExtension<F> {
    const ZERO: Self = Self([F::ZERO; 4]);
    const ONE: Self = Self([F::ONE, F::ZERO, F::ZERO, F::ZERO]);
    const TWO: Self = Self([F::TWO, F::ZERO, F::ZERO, F::ZERO]);
    const NEG_ONE: Self = Self([F::NEG_ONE, F::ZERO, F::ZERO, F::ZERO]);

    // `p^4 - 1 = (p - 1)(p + 1)(p^2 + 1)`. The `p - 1` term has a two-adicity of `F::TWO_ADICITY`.
    // As long as `F::TWO_ADICITY >= 2`, `p` can be written as `4n + 1`, so `p + 1` can be written as
    // `2(2n + 1)`, which has a 2-adicity of 1. A similar argument can show that `p^2 + 1` also has
    // a 2-adicity of 1.
    const TWO_ADICITY: usize = F::TWO_ADICITY + 2;
    const CHARACTERISTIC_TWO_ADICITY: usize = F::CHARACTERISTIC_TWO_ADICITY;

    const MULTIPLICATIVE_GROUP_GENERATOR: Self = Self(F::EXT_MULTIPLICATIVE_GROUP_GENERATOR);
    const POWER_OF_TWO_GENERATOR: Self = Self(F::EXT_POWER_OF_TWO_GENERATOR);

    const BITS: usize = F::BITS * 4;

    fn order() -> BigUint {
        F::order().pow(4u32)
    }
    fn characteristic() -> BigUint {
        F::characteristic()
    }

    // Algorithm 11.3.4 in Handbook of Elliptic and Hyperelliptic Curve Cryptography.
    fn try_inverse(&self) -> Option<Self> {
        if self.is_zero() {
            return None;
        }

        let a_pow_p = self.frobenius();
        let a_pow_p_plus_1 = a_pow_p * *self;
        let a_pow_p3_plus_p2 = a_pow_p_plus_1.repeated_frobenius(2);
        let a_pow_r_minus_1 = a_pow_p3_plus_p2 * a_pow_p;
        let a_pow_r = a_pow_r_minus_1 * *self;
        debug_assert!(FieldExtension::<4>::is_in_basefield(&a_pow_r));

        Some(FieldExtension::<4>::scalar_mul(
            &a_pow_r_minus_1,
            a_pow_r.0[0].inverse(),
        ))
    }

    fn from_noncanonical_biguint(n: BigUint) -> Self {
        F::from_noncanonical_biguint(n).into()
    }

    fn from_canonical_u64(n: u64) -> Self {
        F::from_canonical_u64(n).into()
    }

    fn from_noncanonical_u128(n: u128) -> Self {
        F::from_noncanonical_u128(n).into()
    }

    fn from_noncanonical_i64(n: i64) -> Self {
        F::from_noncanonical_i64(n).into()
    }

    fn from_noncanonical_u64(n: u64) -> Self {
        F::from_noncanonical_u64(n).into()
    }
}

impl<F: Extendable<4>> Display for QuarticExtension<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} + {}*a + {}*a^2 + {}*a^3",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

impl<F: Extendable<4>> Debug for QuarticExtension<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<F: Extendable<4>> Neg for QuarticExtension<F> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self([-self.0[0], -self.0[1], -self.0[2], -self.0[3]])
    }
}

impl<F: Extendable<4>> Add for QuarticExtension<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl<F: Extendable<4>> AddAssign for QuarticExtension<F> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<F: Extendable<4>> Sum for QuarticExtension<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

impl<F: Extendable<4>> Sub for QuarticExtension<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl<F: Extendable<4>> SubAssign for QuarticExtension<F> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<F: Extendable<4>> Mul for QuarticExtension<F> {
    type Output = Self;

    // (a0+a1*x+a2*x^2+a3*x^3)*(b0+b1*x+b2*x^2+b3*x^3)
    // = a0*b0 + (a1*b0+a0*b1)*x + (a0*b2+a1*b1+a2*b0)*x^2 + (a0*b3+a1*b2+a2*b1+a3*b0)*x^3
    //   + (a1*b3+a2*b2+a3*b1)*x^4 = (a1*b3+a2*b2+a3*b1)*w
    //   + (a2*b3+a3*b2) * x^5 = (a2*b3+a3*b2)*w*x
    //   + (a3*b3)*x^6 = (a3*b3)*w*x^2
    // = (a0*b0 + (a1*b3+a2*b2+a3*b1)*w) + (a1*b0+a0*b1 + (a2*b3+a3*b2)*w)*x +
    //    (a0*b2+a1*b1+a2*b0 + a3*b3*w) * x^2 +
    //    (a0*b3+a1*b2+a2*b1+a3*b0) * x^3

    // i0 = a0*b0
    // i1 = a1*b1
    // i2 = a2*b2
    // i3 = a3*b3
    //
    // A = (a0 + a1)*(b0 + b1)
    // a0*b1 + a1*b0 = A - a0*b0 - a1*b1 = A - i0 - i1
    // B = (a2 + a3) *(b2 + b3)
    // a2*b3 + a3*b2 = B - a2*b2 - a3*b3 = B - i2 - i3
    // C = (a0 + a2)*(b0 + b2)
    // a0*b2 + a2*b0 = C - a0*b0 - a2*b2 = C - i0 - i2
    // D = (a1 + a3)*(b1 + b3)
    // a1*b3 + a3*b1 = D - a1*b1 - a3*b3 = D - i1 - i3
    // E = (a0 + a3)*(b0 + b3)
    // (a0*b3 + a3*b0) = E - a0*b0 - a3*b3 = E - i0 - i3
    // F = (a1 + a2)*(b1 + b2)
    // a1*b2 + a2*b1 = F - a1*b1 - a2*b2 = F - i1 -i2
    //
    // c0 = a0*b0 + (D - a1*b1 - a3*b3 + a2*b2)*w
    //    = i0 + (D - i1-i3+i2)*w
    // c1 = A - a0*b0 - a1*b1 + (B - a2*b2 - a3*b3)*w
    //    = A - i0-i1 + (B - i2-i3)*w

    // c2 = C - a0*b0 - a2*b2 + a1*b1 + a3*b3*w
    //    = C - i0 - i2 + i1 + i3*w
    // c3 = E - i0-i3 + F - i1 -i2
    #[inline]
    default fn mul(self, rhs: Self) -> Self {
        let Self([a0, a1, a2, a3]) = self;
        let Self([b0, b1, b2, b3]) = rhs;

        // 19 mul, 12 add
        let c0 = a0 * b0 + <Self as OEF<4>>::W * (a1 * b3 + a2 * b2 + a3 * b1);
        let c1 = a0 * b1 + a1 * b0 + <Self as OEF<4>>::W * (a2 * b3 + a3 * b2);
        let c2 = a0 * b2 + a1 * b1 + a2 * b0 + <Self as OEF<4>>::W * a3 * b3;
        let c3 = a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0;

        Self([c0, c1, c2, c3])
    }
}

impl<F: Extendable<4>> MulAssign for QuarticExtension<F> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<F: Extendable<4>> Square for QuarticExtension<F> {
    #[inline(always)]
    fn square(&self) -> Self {
        let Self([a0, a1, a2, a3]) = *self;
        let w = <Self as OEF<4>>::W;

        let c0 = a0.square() + w * (a1 * a3.double() + a2.square());
        let c1 = (a0 * a1 + w * a2 * a3).double();
        let c2 = a0 * a2.double() + a1.square() + w * a3.square();
        let c3 = (a0 * a3 + a1 * a2).double();

        Self([c0, c1, c2, c3])
    }
}

impl<F: Extendable<4>> Product for QuarticExtension<F> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, x| acc * x)
    }
}

impl<F: Extendable<4>> Div for QuarticExtension<F> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl<F: Extendable<4>> DivAssign for QuarticExtension<F> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    mod goldilocks {
        use crate::{test_field_arithmetic, test_field_extension};

        test_field_extension!(crate::goldilocks_field::GoldilocksField, 4);
        test_field_arithmetic!(
            crate::extension::quartic::QuarticExtension<
                crate::goldilocks_field::GoldilocksField,
            >
        );
    }
}
