#![allow(unused)]

pub mod macros;
pub mod modulo;
pub mod pow;

use primitive_types::U256;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

use self::{modulo::Modulo, pow::Pow};
use macros::felt;

#[derive(Debug, Default, Clone, Copy)]
pub struct Felt {
    inner: U256,
    prime: U256,
}

impl Felt {
    pub fn new(inner: U256, prime: U256) -> Self {
        assert!(inner < prime, "Inner value must be less than prime");
        Self { inner, prime }
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

impl Sub for Felt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = if self.inner > rhs.inner {
            self.inner - rhs.inner
        } else {
            self.prime - (rhs.inner - self.inner)
        };

        Self::new(result, self.prime)
    }
}

impl Mul for Felt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = (self.inner * rhs.inner).modulo(&self.prime);
        Self::new(result, self.prime)
    }
}

impl Div for Felt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let exponent = self.prime - U256::from(2);
        let result = (self.inner * rhs.inner.pow(exponent)).modulo(&self.prime);
        Self::new(result, self.prime)
    }
}

impl Pow<u32> for Felt {
    fn pow(&self, exponent: u32) -> Self {
        let exponent = U256::from(exponent).modulo(&self.prime);
        let result = self.inner.pow(exponent).modulo(&self.prime);
        Self::new(result, self.prime)
    }
}

impl Pow<i64> for Felt {
    fn pow(&self, exponent: i64) -> Self {
        let inner = if exponent > 0 {
            let exponent = U256::from(exponent);
            self.inner.pow(exponent).modulo(&self.prime)
        } else {
            // In finite fields we can use the following property:
            // a^(-1) = a^(p-2) (mod p)
            let prime = self.prime - U256::from(1);
            let exponent = prime - U256::from(exponent.abs());
            self.inner.pow(exponent).modulo(&self.prime)
        };

        Felt::new(inner, self.prime)
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
        assert_eq!(felt_a + felt_b, Felt::new(9.into(), 19.into()));

        let felt_a = felt!(6, 19);
        let felt_b = felt!(13, 19);
        assert_eq!(felt_a - felt_b, Felt::new(12.into(), 19.into()));
    }

    #[test]
    fn test_mul_div() {
        let felt_a = felt!(2, 19);
        let felt_b = felt!(17, 19);
        assert_eq!(felt_a * felt_b, Felt::new(15.into(), 19.into()));

        let felt_a = felt!(2, 19);
        let felt_b = felt!(7, 19);
        assert_eq!(felt_a / felt_b, Felt::new(3.into(), 19.into()));
    }

    #[test]
    fn test_pow() {
        // 3^-1 == 1/3
        let felt_a = felt!(3, 19).pow(-1i64);
        let felt_b = felt!(1, 19) / felt!(3, 19);
        assert_eq!(felt_a, felt_b);
    }
}
