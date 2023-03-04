#![warn(clippy::all, clippy::pedantic, clippy::style, rust_2018_idioms)]
#![allow(
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::module_name_repetitions
)]

use crate::elliptic_curve::signature::Signature;
use elliptic_curve::secp256k1::{Secp256k1Felt, Secp256k1Point};
use num_bigint::BigUint;

pub mod elliptic_curve;
pub mod finite_fields;

fn main() {
    let px = BigUint::parse_bytes(
        b"887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
        16,
    )
    .unwrap();

    let py = BigUint::parse_bytes(
        b"61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
        16,
    )
    .unwrap();

    let point = Secp256k1Point::new(px, py);

    let z = Secp256k1Felt::from_bytes(
        b"ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
    )
    .unwrap();

    let signature = Signature::new(
        Secp256k1Felt::from_bytes(
            b"ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395",
        )
        .unwrap(),
        Secp256k1Felt::from_bytes(
            b"068342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4",
        )
        .unwrap(),
    );

    let before = std::time::Instant::now();

    if point.verify(z, signature) {
        println!("Signature 1 is valid");
    } else {
        println!("Signature 1 is invalid");
    }

    let duration = std::time::Instant::now() - before;
    println!("Duration: {:?}", duration);

    let z = Secp256k1Felt::from_bytes(
        b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
    )
    .unwrap();

    let signature = Signature::new(
        Secp256k1Felt::from_bytes(
            b"eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
        )
        .unwrap(),
        Secp256k1Felt::from_bytes(
            b"c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
        )
        .unwrap(),
    );

    let before = std::time::Instant::now();

    if point.verify(z, signature) {
        println!("Signature 2 is valid");
    } else {
        println!("Signature 2 is invalid");
    }

    let duration = std::time::Instant::now() - before;
    println!("Duration: {:?}", duration);
}
