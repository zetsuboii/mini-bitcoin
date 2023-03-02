#![warn(clippy::all, clippy::pedantic, clippy::style, rust_2018_idioms)]

pub mod elliptic_curve;
pub mod finite_fields;

use elliptic_curve::curve::Curve;
use finite_fields::macros::felt;

fn main() {
    let prime: u64 = 223;
    let a = felt!(0, prime);
    let b = felt!(7, prime);
    let curve = Curve::new(a.clone(), b.clone());

    let mut i: usize = 0;
    let mut point = curve.identity();
    loop {
        let generator = curve.point(felt!(15, prime), felt!(86, prime)).unwrap();
        point = point + generator;
        i += 1;

        if point == curve.identity() {
            break;
        }
    }

    println!("Order of the curve: {}", i);
}
