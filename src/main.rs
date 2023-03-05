#![warn(clippy::all, clippy::pedantic, clippy::style, rust_2018_idioms)]
#![allow(
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::module_name_repetitions
)]

use crate::elliptic_curve::private_key::PrivateKey;
use elliptic_curve::secp256k1::Secp256k1Felt;

pub mod elliptic_curve;
pub mod finite_fields;

fn main() {
    let secret = Secp256k1Felt::from_bytes(
        b"ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
    )
    .unwrap();

    let wallet = PrivateKey::new(secret);
    let signature = wallet.sign_slice(b"Programming Bitcoin!");

    println!("Signature: {}", signature);

    let is_legit = wallet.verify_slice(b"Programming Bitcoin", &signature);
    println!("Shouldn't be legit: {}", is_legit);

    let is_legit = wallet.verify_slice(b"Programming Bitcoin!", &signature);
    println!("Should be legit: {}", is_legit);
}
