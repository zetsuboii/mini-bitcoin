use super::secp256k1::Secp256k1Felt;
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
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signature({}, {})", self.r(), self.s())
    }
}
