#![allow(unused)]

use std::ops::Add;

use color_eyre::eyre::{eyre, Result};

use crate::finite_fields::{pow::Pow, Felt};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Curve {
    pub a: Felt,
    pub b: Felt,
}

impl Curve {
    pub fn new(a: Felt, b: Felt) -> Self {
        Self { a, b }
    }

    pub fn point(&self, x: Felt, y: Felt) -> Result<Point> {
        Point::try_new(x, y, self.a, self.b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
        left == right
    }

    pub fn new(x: Felt, y: Felt, a: Felt, b: Felt) -> Self {
        Self { x, y, a, b }
    }

    pub fn try_new(x: Felt, y: Felt, a: Felt, b: Felt) -> Result<Self> {
        let point = Self { x, y, a, b };
        if point.is_on_curve() {
            Ok(point)
        } else {
            Err(eyre!("Point is not on the curve"))
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        println!("x: {} + {} = {}", self.x, rhs.x, self.x + rhs.x);
        println!("y: {} + {} = {}", self.y, rhs.y, self.y + rhs.y);
        Self::new(self.x + rhs.x, self.y + rhs.y, self.a, self.b)
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

    #[test]
    fn test_point_add() {
        let prime = 223u64;

        let curve = Curve::new(felt!(0, prime), felt!(7, prime));

        let pt1 = curve.point(felt!(170, prime), felt!(142, prime)).unwrap();
        let pt2 = curve.point(felt!(60, prime), felt!(139, prime)).unwrap();
        let expected_sum = curve.point(felt!(220, prime), felt!(181, prime)).unwrap();

        dbg!(pt1);
        dbg!(pt2);
        dbg!(pt1 + pt2);

        assert_eq!(pt1 + pt2, expected_sum);
    }
}

fn xa0s_s1() {}
