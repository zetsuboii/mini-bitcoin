use num_bigint::BigUint;

use crate::finite_fields::{element::Felt, pow::Pow};

use super::{
    point::PointType,
    secp256k1::{Secp256k1Felt, Secp256k1Point},
};
use std::fmt::Display;

/// Represents a Signature on SECP256K1 curve
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    r: Secp256k1Felt,
    s: Secp256k1Felt,
}

impl Signature {
    /// Creates a new Signature
    pub fn new(r: Secp256k1Felt, s: Secp256k1Felt) -> Self {
        Self { r, s }
    }

    /// Returns r of the signature
    pub fn r(&self) -> &Secp256k1Felt {
        &self.r
    }

    /// Returns s of the signature
    pub fn s(&self) -> &Secp256k1Felt {
        &self.s
    }

    /// Verifies the signature, given the message, signature and the public key
    #[allow(clippy::many_single_char_names)]
    pub fn verify(
        &self,
        z: &Secp256k1Felt,
        public_key: &Secp256k1Point,
    ) -> bool {
        let g = Secp256k1Point::g();

        let u = z / self.s();
        let u = g * u.inner();

        let v = self.r() / self.s();
        let v = public_key * v.inner();

        let p = u + v;
        let r = self.r().inner();

        match p.x() {
            PointType::Infinity => false,
            PointType::Normal(x) => x.inner() == r,
        }
    }

    /// Convenience method to verify a signature given a message as a slice
    pub fn verify_slice(
        &self,
        z: &[u8],
        public_key: &Secp256k1Point,
    ) -> bool {
        let z = Secp256k1Felt::from_bytes(z);
        self.verify(&z, public_key)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signature({}, {})", self.r(), self.s())
    }
}
