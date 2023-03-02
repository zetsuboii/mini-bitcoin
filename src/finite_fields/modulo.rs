use std::ops::{Add, Rem};

use num_bigint::BigUint;

pub trait Modulo {
    fn modulo(&self, other: &Self) -> Self;
}

impl Modulo for BigUint {
    fn modulo(&self, other: &Self) -> BigUint {
        // a mod b = ((a rem b) + b) rem b
        self.rem(other).add(other).rem(other)
    }
}

impl Modulo for i64 {
    fn modulo(&self, other: &Self) -> Self {
        ((self % other) + other) % other
    }
}