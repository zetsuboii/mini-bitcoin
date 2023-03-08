use crate::helpers::hash::hash256;

use super::{
    secp256k1::{Secp256k1Felt, Secp256k1Point},
    signature::{self, Signature},
};
use hmac::{Hmac, Mac};
use num_bigint::BigUint;
use sha2::Sha256;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct PrivateKey {
    secret: Secp256k1Felt,
    public_key: Secp256k1Point,
}

type Hmac256 = Hmac<Sha256>;

impl PrivateKey {
    /// Creates a new private key from a field element
    pub fn new(secret: Secp256k1Felt) -> Self {
        let public_key = Secp256k1Point::g() * secret.inner();
        Self { secret, public_key }
    }

    /// Signs a field element using the private key
    ///
    /// # Panics
    ///
    /// This method will panic if the field element is not a valid field element
    #[allow(clippy::many_single_char_names)]
    pub fn sign(&self, z: &Secp256k1Felt) -> Signature {
        let k = self.deterministic_k(z.inner().clone());

        // r = (k * G).x
        let g_x = (Secp256k1Point::g() * &k).x().clone().unwrap();
        let r = Secp256k1Felt::from(g_x);

        // s = (z + r * secret) / k
        let k = Secp256k1Felt::new(k);
        let mut s = (z + &r * &self.secret) / &k;

        // if s > n / 2 then s = n - s
        let n = Secp256k1Point::order();
        let n_half = &n / BigUint::from(2u32);

        if s.inner() > &n_half {
            s = Secp256k1Felt::new(n - s.inner());
        }

        Signature::new(r, s)
    }

    /// Signs a byte slice using the private key
    /// This is a convenience method that hashes the message before signing
    ///
    /// # Panics
    ///
    /// This method will panic if the hash of the message is not a valid field element
    pub fn sign_slice(&self, message: &[u8]) -> Signature {
        let hash = hash256(message);
        let z = Secp256k1Felt::from_bytes(hash.as_slice());

        self.sign(&z)
    }

    /// Checks if message is signed by this private key
    pub fn verify(&self, z: &Secp256k1Felt, sig: &Signature) -> bool {
        sig.verify(z, &self.public_key)
    }

    /// Convenience method to verify a slice after hashing it
    pub fn verify_slice(&self, message: &[u8], sig: &Signature) -> bool {
        let message_digest = hash256(message);
        let z = Secp256k1Felt::from_bytes(&message_digest);
        self.verify(&z, sig)
    }

    /// Creates a unique, deterministic k value
    ///
    /// This is important because if the same k value is used twice, the private key can
    /// be recovered using both signatures.
    ///
    /// The specification for determining k is defined in RFC 6779 (<https://tools.ietf.org/html/rfc6979>)
    fn deterministic_k(&self, z: BigUint) -> BigUint {
        let k = [0u8; 32];
        let v = [1u8; 32];

        let mut z = z;
        if z > Secp256k1Point::order() {
            z -= Secp256k1Point::order();
        }

        let z = z.to_bytes_be();
        let mut z_bytes = vec![0; 32 - z.len()];
        z_bytes.extend_from_slice(z.as_slice());

        // Pad secret to 32 bytes, from the left
        let secret = self.secret.inner().to_bytes_be();
        let mut secret_bytes = vec![0; 32 - secret.len()];
        secret_bytes.extend_from_slice(secret.as_slice());

        // k := hmac_k (v || 0x00 || secret_bytes || z_bytes)
        let mut hmac = Hmac256::new_from_slice(&k).unwrap();
        let mut data = Vec::new();
        data.extend_from_slice(&v);
        data.push(0);
        data.extend_from_slice(&secret_bytes);
        data.extend_from_slice(&z_bytes);
        hmac.update(&data);
        let k = hmac.finalize().into_bytes();

        // v := hmac_k (v)
        let mut hmac = Hmac256::new_from_slice(&k).unwrap();
        hmac.update(&v);
        let v = hmac.finalize().into_bytes();

        // k := hmac_k (v || 0x01 || secret_bytes || z_bytes)
        let mut hmac = Hmac256::new_from_slice(&k).unwrap();
        let mut data = Vec::new();
        data.extend_from_slice(&v);
        data.push(1);
        data.extend_from_slice(&secret_bytes);
        data.extend_from_slice(&z_bytes);
        hmac.update(&data);
        let mut k = hmac.finalize().into_bytes();

        // v := hmac_k (v)
        let mut hmac = Hmac256::new_from_slice(&k).unwrap();
        hmac.update(&v);
        let mut v = hmac.finalize().into_bytes();

        loop {
            // v := hmac_k (v)
            let mut hmac = Hmac256::new_from_slice(&k).unwrap();
            hmac.update(&v);
            v = hmac.finalize().into_bytes();

            let candidate = BigUint::from_bytes_be(&v);
            let one = BigUint::from(1u8);

            if candidate > one && candidate < Secp256k1Point::order() {
                return candidate;
            }

            // k := hmac_k (v || 0x00)
            let mut hmac = Hmac256::new_from_slice(&k).unwrap();
            let mut data = Vec::new();
            data.extend_from_slice(&v);
            data.push(0);
            hmac.update(&data);
            k = hmac.finalize().into_bytes();

            // v := hmac_k (v)
            let mut hmac = Hmac256::new_from_slice(&k).unwrap();
            hmac.update(&v);
            v = hmac.finalize().into_bytes();
        }
    }
}
