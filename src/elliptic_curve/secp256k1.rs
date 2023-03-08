use super::{
    curve::Curve,
    point::{Point, PointType},
    signature::Signature,
};
use crate::finite_fields::{element::Felt, macros::impl_refs, modulo::Modulo};
use color_eyre::eyre::{eyre, Result};
use num_bigint::BigUint;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Secp256k1Felt(Felt);

impl Secp256k1Felt {
    pub const SECP256K1_PRIME: &[u8; 64] =
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
    pub const SECP256K1_ORDER: &[u8; 64] =
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

    pub fn order() -> BigUint {
        BigUint::parse_bytes(Self::SECP256K1_ORDER, 16).unwrap_or_default()
    }

    pub fn prime() -> BigUint {
        BigUint::parse_bytes(Self::SECP256K1_PRIME, 16).unwrap_or_default()
    }

    pub fn new(inner: BigUint) -> Self {
        Self(Felt::new(inner, Self::prime()))
    }

    pub fn from_bytes(value: &[u8]) -> Self {
        let inner = BigUint::from_bytes_be(value);
        Self::new(inner)
    }

    pub fn inner(&self) -> &BigUint {
        self.0.inner()
    }
}

impl From<Secp256k1Felt> for Felt {
    fn from(felt: Secp256k1Felt) -> Self {
        felt.0
    }
}

impl From<Felt> for Secp256k1Felt {
    fn from(felt: Felt) -> Self {
        Self(felt)
    }
}

impl Display for Secp256k1Felt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{value:0>64}", value = self.inner().to_str_radix(16))
    }
}

impl Add<Secp256k1Felt> for Secp256k1Felt {
    type Output = Secp256k1Felt;

    fn add(self, rhs: Secp256k1Felt) -> Self::Output {
        let one = BigUint::from(1u32);
        let n = Self::order();
        
        let sum = self.inner() + rhs.inner();
        let sum_mod = sum.modpow(&one, &n);

        Self::new(sum_mod)
    }
}

impl_refs!(Add, add, Secp256k1Felt, Secp256k1Felt);

impl Mul<Secp256k1Felt> for Secp256k1Felt {
    type Output = Secp256k1Felt;

    fn mul(self, rhs: Secp256k1Felt) -> Self::Output {
        let order = Self::order();
        let one = BigUint::from(1u64);
        let result = self.0.inner().mul(rhs.0.inner()).modpow(&one, &order);

        Self::new(result)
    }
}

impl_refs!(Mul, mul, Secp256k1Felt, Secp256k1Felt);

impl Div<Secp256k1Felt> for Secp256k1Felt {
    type Output = Secp256k1Felt;

    //
    fn div(self, rhs: Secp256k1Felt) -> Self::Output {
        let one = BigUint::from(1u32);
        let exponent = Self::order() - BigUint::from(2u32);
        let rhs_inner = rhs.0.inner().modpow(&exponent, &Self::order());
        let result = (self.inner() * rhs_inner).modpow(&one, &Self::order());

        Self::new(result)
    }
}

impl_refs!(Div, div, Secp256k1Felt, Secp256k1Felt);

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
        BigUint::parse_bytes(Self::SECP256K1_ORDER, 16).unwrap_or_default()
    }

    pub fn curve() -> Curve {
        Curve::new(
            Secp256k1Felt::new(BigUint::from(0u8)).into(),
            Secp256k1Felt::new(BigUint::from(7u8)).into(),
        )
    }

    pub fn g() -> Self {
        Self::new(
            BigUint::parse_bytes(Self::SECP256K1_X, 16).unwrap_or_default(),
            BigUint::parse_bytes(Self::SECP256K1_Y, 16).unwrap_or_default(),
        )
    }

    pub fn x(&self) -> &PointType {
        &self.0.x
    }

    pub fn y(&self) -> &PointType {
        &self.0.y
    }

    /// Creates a new point on SECP256K1 curve
    ///
    /// # Panics
    ///
    /// Panics if x and y combination is not on the curve
    ///
    pub fn new(x: BigUint, y: BigUint) -> Self {
        let curve = Self::curve();
        let point = curve
            .point(Secp256k1Felt::new(x).into(), Secp256k1Felt::new(y).into())
            .unwrap();

        Self(point)
    }

    pub fn verify(&self, z: &Secp256k1Felt, signature: &Signature) -> bool {
        let u = z / signature.s();
        let v = signature.r() / signature.s();

        let total = Self::g() * u.inner() + self * v.inner();
        total.x().clone().unwrap().inner() == signature.r().inner()
    }
}

impl Display for Secp256k1Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_repr = match self.x() {
            PointType::Normal(x) => x.inner().to_str_radix(16),
            PointType::Infinity => "Infinity".to_string(),
        };

        let y_repr = match self.y() {
            PointType::Normal(y) => y.inner().to_str_radix(16),
            PointType::Infinity => "Infinity".to_string(),
        };

        write!(
            f,
            "Point {{ x: 0x{x:0>64}, y: 0x{y:0>64} }}",
            x = x_repr,
            y = y_repr
        )
    }
}

impl From<Secp256k1Point> for Point {
    fn from(point: Secp256k1Point) -> Self {
        point.0
    }
}

impl Add<Secp256k1Point> for Secp256k1Point {
    type Output = Self;

    fn add(self, rhs: Secp256k1Point) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl_refs!(Add, add, Secp256k1Point, Secp256k1Point);

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

impl_refs!(Mul, mul, Secp256k1Point, BigUint);
impl_refs!(Mul, mul, Secp256k1Point, u32);
