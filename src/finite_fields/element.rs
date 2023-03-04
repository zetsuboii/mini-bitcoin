use super::{macros::impl_refs, modulo::Modulo, pow::Pow};
use num_bigint::BigUint;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

/// Represents a field element
///
/// Prime used in secp256k1 is pretty large and values used in the field can be
/// between 0 and prime ** prime so I used `BigUint` to represent the values
#[derive(Debug, Default, Clone)]
pub struct Felt {
    inner: BigUint,
    prime: BigUint,
}

impl Felt {
    /// Creates a new field element
    ///
    /// # Panics
    ///
    /// Panics if the inner value is greater than or equal to the prime
    ///
    /// # Examples
    ///
    /// ```
    /// use elliptic_curve::finite_fields::Felt;
    /// use num_bigint::BigUint;
    ///
    /// let prime = BigUint::from(2u32).pow(256u32) - BigUint::from(2u32).pow(32u32) - BigUint::from(977u32);
    /// let inner = BigUint::from(5u32);
    /// let felt = Felt::new(inner, prime);
    ///
    /// assert_eq!(felt.inner(), &BigUint::from(5u32));
    /// assert_eq!(felt.prime(), &prime);
    /// ```
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

        Self::new(result.modulo(&self.prime), self.prime.clone())
    }
}

impl_refs!(Sub, sub, Felt, Felt);

impl Mul for Felt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result =  (self.inner * rhs.inner).modulo(&self.prime);
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
        let result = (self.inner * rhs.inner.modpow(&exponent, &self.prime)).modulo(&self.prime);
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
        let cloned = (*self).clone();
        cloned.pow(exponent)
    }
}

impl Pow<i64> for Felt {
    type Output = Felt;

    fn pow(&self, exponent: i64) -> Self::Output {
        let inner = if exponent > 0 {
            let exponent = BigUint::from(u32::try_from(exponent).unwrap());
            self.inner.modpow(&exponent, &self.prime)
        } else {
            // In finite fields we can use the following property:
            // a^(-1) = a^(p-2) (mod p)
            let prime = &self.prime - BigUint::from(1u32);
            let exponent = prime - BigUint::from(u32::try_from(exponent.abs()).unwrap());
            self.inner.modpow(&exponent, &self.prime)
        };

        Felt::new(inner, self.prime.clone())
    }
}

impl<'b> Pow<i64> for &'b Felt {
    type Output = Felt;

    fn pow(&self, exponent: i64) -> Self::Output {
        let cloned = (*self).clone();
        cloned.pow(exponent)
    }
}
