use std::ops::{Add, Rem};

use primitive_types::U256;

pub trait Modulo {
    fn modulo(&self, other: &Self) -> Self;
}

impl Modulo for U256 {
    fn modulo(&self, other: &Self) -> U256 {
        // a mod b = ((a rem b) + b) rem b
        self.rem(other).add(other).rem(other)
    }
}

impl Modulo for i64 {
    fn modulo(&self, other: &Self) -> Self {
        ((self % other) + other) % other
    }
}
