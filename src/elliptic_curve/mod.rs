#![allow(unused)]

use color_eyre::eyre::{eyre, Result};

use crate::finite_fields::{pow::Pow, Felt};

struct Point {
    x: Felt,
    y: Felt,
    a: Felt,
    b: Felt,
}

impl Point {
    fn is_on_curve(&self) -> bool {
        let left = self.y.pow(2u32);
        let right = self.x.pow(3u32) + self.a * self.x + self.b;

        println!("left: {}, right: {}", left, right);

        left == right
    }

    pub fn new(x: Felt, y: Felt, a: Felt, b: Felt) -> Self {
        let point = Self { x, y, a, b };
        assert!(point.is_on_curve());
        point
    }

    pub fn try_new(x: Felt, y: Felt, a: Felt, b: Felt) -> Result<Self> {
        let point = Self { x, y, a, b };
        match point.is_on_curve() {
            true => Ok(point),
            false => Err(eyre!("Point is not on the curve")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::finite_fields::macros::felt;
    use primitive_types::U256;

    use super::*;

    #[test]
    fn test_curve() {
        let prime = 103u64;

        let point = Point::try_new(
            felt!(17, prime),
            felt!(64, prime),
            felt!(0, prime),
            felt!(7, prime),
        );
        assert!(point.is_ok());

        let point = Point::try_new(
            felt!(17, prime),
            felt!(64, prime),
            felt!(0, prime),
            felt!(9, prime),
        );
        assert!(point.is_err());
    }
}
