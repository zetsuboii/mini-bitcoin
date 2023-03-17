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
mod helpers;

fn main() {
    let secret = Secp256k1Felt::new(0xdeadbeef12345_u64.into());
    let wallet = PrivateKey::new(secret);
    let public_key = wallet.public_key();

    println!(
        "Public key: {:?}",
        public_key
            .sec_uncompressed()
            .map(|v| v.iter().fold(String::new(), |mut acc, v| {
                acc.push_str(&format!("{:02x}", v));
                acc
            }))
    );

    // let signature = wallet.sign_slice(b"Programming Bitcoin!");

    // // let is_legit = wallet.verify_slice(b"Programming Bitcoin", &signature);
    // // println!("Shouldn't be legit: {}", is_legit);

    // let now = std::time::Instant::now();
    // let is_legit = wallet.verify_slice(b"Programming Bitcoin!", &signature);
    // println!("Should be legit: {}", is_legit);

    // println!("Time: {:?}", now.elapsed());
}
