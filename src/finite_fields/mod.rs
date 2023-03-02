#![allow(unused)]

pub mod macros;
pub mod modulo;
pub mod pow;

use num_bigint::BigUint;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

use self::{modulo::Modulo, pow::Pow};
use macros::{felt, impl_refs};

#[derive(Debug, Default, Clone)]
pub struct Felt {
    inner: BigUint,
    prime: BigUint,
}

impl Felt {
    pub fn new(inner: BigUint, prime: BigUint) -> Self {
        assert!(inner < prime, "Inner value must be less than prime");
        Self { inner, prime }
    }

    pub fn inner(&self) -> &BigUint {
        &self.inner
    }

    pub fn prime(&self) -> &BigUint {
        &self.prime
    }
}

impl PartialEq for Felt {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl PartialOrd for Felt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.inner.cmp(&other.inner))
    }
}

impl Display for Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field Element: {} ({})", self.inner, self.prime)
    }
}

impl Add for Felt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result = (self.inner + rhs.inner).modulo(&self.prime);
        Self::new(result, self.prime)
    }
}

impl_refs!(Add, add, Felt, Felt);

impl Sub for Felt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = if self.inner > rhs.inner {
            &self.inner - rhs.inner
        } else {
            &self.prime - (rhs.inner - self.inner)
        };

        Self::new(result, self.prime.clone())
    }
}

impl_refs!(Sub, sub, Felt, Felt);

impl Mul for Felt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = (self.inner * rhs.inner).modulo(&self.prime);
        Self::new(result, self.prime)
    }
}

impl Mul<u32> for Felt {
    type Output = Felt;

    /// Scalar multiplication for Felt
    fn mul(self, rhs: u32) -> Self::Output {
        let result: BigUint = self.inner.mul(BigUint::from(rhs)).modulo(&self.prime);
        Self::new(result, self.prime)
    }
}

impl_refs!(Mul, mul, Felt, Felt);
impl_refs!(Mul, mul, Felt, u32);

impl Div for Felt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let exponent = &self.prime - BigUint::from(2u32);
        let result = (self.inner * rhs.inner.pow(exponent.try_into().unwrap())).modulo(&self.prime);
        Self::new(result, self.prime)
    }
}

impl_refs!(Div, div, Felt, Felt);

impl Pow<u32> for Felt {
    type Output = Felt;

    fn pow(&self, exponent: u32) -> Self::Output {
        let exponent = BigUint::from(exponent).modulo(&self.prime);
        let result = self
            .inner
            .pow(exponent.try_into().unwrap())
            .modulo(&self.prime);

        Self::new(result, self.prime.clone())
    }
}

impl<'b> Pow<u32> for &'b Felt {
    type Output = Felt;

    fn pow(&self, exponent: u32) -> Self::Output {
        let cloned = self.clone();
        cloned.pow(exponent)
    }
}

impl Pow<i64> for Felt {
    type Output = Felt;

    fn pow(&self, exponent: i64) -> Self::Output {
        let inner = if exponent > 0 {
            let exponent = BigUint::from(exponent as u32);
            self.inner
                .pow(exponent.try_into().unwrap())
                .modulo(&self.prime)
        } else {
            // In finite fields we can use the following property:
            // a^(-1) = a^(p-2) (mod p)
            let prime = &self.prime - BigUint::from(1u32);
            let exponent = prime - BigUint::from(exponent.abs() as u32);
            self.inner
                .pow(exponent.try_into().unwrap())
                .modulo(&self.prime)
        };

        Felt::new(inner, self.prime.clone())
    }
}

impl<'b> Pow<i64> for &'b Felt {
    type Output = Felt;

    fn pow(&self, exponent: i64) -> Self::Output {
        let cloned = self.clone();
        cloned.pow(exponent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let felt_a = felt!(1, 11);
        assert_eq!(format!("{}", felt_a), "Field Element: 1 (11)")
    }

    #[test]
    fn test_add_sub() {
        let felt_a = felt!(11, 19);
        let felt_b = felt!(17, 19);
        assert_eq!(felt_a + felt_b, felt!(9, 19));

        let felt_a = felt!(6, 19);
        let felt_b = felt!(13, 19);
        assert_eq!(felt_a - felt_b, felt!(12, 19));
    }

    #[test]
    fn test_mul_div() {
        let felt_a = felt!(2, 19);
        let felt_b = felt!(17, 19);
        assert_eq!(felt_a * felt_b, felt!(15, 19));

        let felt_a = felt!(2, 19);
        let felt_b = felt!(7, 19);
        assert_eq!(felt_a / felt_b, felt!(3, 19));
    }

    #[test]
    fn test_pow() {
        // 3^-1 == 1/3
        let felt_a = felt!(3, 19).pow(-1i64);
        let felt_b = felt!(1, 19) / felt!(3, 19);
        assert_eq!(felt_a, felt_b);
    }
}
