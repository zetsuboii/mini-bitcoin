#![allow(unused)]

use color_eyre::eyre::{eyre, Result};

use crate::finite_fields::{pow::Pow, Felt};

struct Curve {
    a: Felt,
    b: Felt,
}

impl Curve {
    pub fn new(a: Felt, b: Felt) -> Self {
        Self { a, b }
    }

    pub fn point(&self, x: Felt, y: Felt) -> Result<Point> {
        Point::try_new(x, y, self.a, self.b)
    }
}

struct Point {
    x: Felt,
    y: Felt,
    a: Felt,
    b: Felt,
}

impl Point {
    /// Checks if the point is on the curve
    pub fn is_on_curve(&self) -> bool {
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
        let prime = 223u64;
        let curve = Curve::new(felt!(0, prime), felt!(7, prime));

        let valid_points = vec![(192, 105), (17, 56), (1, 193)];
        let invalid_points = vec![(200, 119), (42, 99)];

        for (x, y) in valid_points {
            let point = curve.point(felt!(x, prime), felt!(y, prime));
            assert!(point.is_ok());
        }

        for (x, y) in invalid_points {
            let point = curve.point(felt!(x, prime), felt!(y, prime));
            assert!(point.is_err());
        }
    }
}
