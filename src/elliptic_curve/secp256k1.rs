use std::{fmt::Display, ops::Mul};

use num_bigint::BigUint;

use super::{curve::Curve, point::Point};
use crate::finite_fields::{element::Felt, modulo::Modulo};

#[derive(Debug, Clone, PartialEq)]
pub struct Secp256k1Felt(Felt);

impl Secp256k1Felt {
    pub const SECP256K1_PRIME: &[u8; 64] =
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";

    pub fn prime() -> BigUint {
        BigUint::parse_bytes(Self::SECP256K1_PRIME, 16).unwrap()
    }

    pub fn new(inner: BigUint) -> Self {
        Self(Felt::new(inner, Self::prime()))
    }

    pub fn inner(&self) -> &Felt {
        &self.0
    }
}

impl From<Secp256k1Felt> for Felt {
    fn from(felt: Secp256k1Felt) -> Self {
        felt.0
    }
}

impl Display for Secp256k1Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>64}", self.0.inner())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Secp256k1Point(Point);

impl Secp256k1Point {
    pub const SECP256K1_ORDER: &[u8; 64] =
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";
    pub const SECP256K1_X: &[u8; 64] =
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    pub const SECP256K1_Y: &[u8; 64] =
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

    pub fn order() -> BigUint {
        BigUint::parse_bytes(Self::SECP256K1_ORDER, 16).unwrap()
    }

    pub fn curve() -> Curve {
        Curve::new(
            Secp256k1Felt::new(BigUint::from(0u8)).into(),
            Secp256k1Felt::new(BigUint::from(7u8)).into(),
        )
    }

    pub fn g() -> Self {
        Self::new(
            BigUint::parse_bytes(Self::SECP256K1_X, 16).unwrap(),
            BigUint::parse_bytes(Self::SECP256K1_Y, 16).unwrap(),
        )
    }

    pub fn new(x: BigUint, y: BigUint) -> Self {
        let curve = Self::curve();
        let point = curve
            .point(Secp256k1Felt::new(x).into(), Secp256k1Felt::new(y).into())
            .unwrap();

        Self(point)
    }
}

impl From<Secp256k1Point> for Point {
    fn from(point: Secp256k1Point) -> Self {
        point.0
    }
}

impl Mul<u32> for Secp256k1Point {
    type Output = Self;

    /// Multiplies a secp256k1 point by a scalar
    /// 
    /// Since we know the order of the curve generated by the point, we can use take the 
    /// modulo of the scalar as `n * G` is identity
    fn mul(self, coefficient: u32) -> Self::Output {
        let coefficient = BigUint::from(coefficient).modulo(&Self::order());
        Self(self.0 * coefficient)
    }
}

impl Mul<BigUint> for Secp256k1Point {
    type Output = Self;

    /// Multiplies a secp256k1 point by a scalar
    /// 
    /// Since we know the order of the curve generated by the point, we can use take the 
    /// modulo of the scalar as `n * G` is identity
    fn mul(self, coefficient: BigUint) -> Self::Output {
        let coefficient = coefficient.modulo(&Self::order());
        Self(self.0 * coefficient)
    }
}