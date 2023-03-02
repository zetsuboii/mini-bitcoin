use std::ops::{Add, Rem};

use num_bigint::BigUint;

/// Rust rem operator is not the same as the mathematical modulo operator
/// This trait implements the mathematical modulo operator
/// It's helpful when we are taking modulo of negative numbers such that
/// -1.rem(5) = -1
/// -1.modulo(5) = 4
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