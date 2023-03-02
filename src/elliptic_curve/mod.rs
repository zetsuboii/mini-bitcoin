#![allow(unused)]

use std::ops::{Add, Mul};

use color_eyre::eyre::{eyre, Result};

use crate::finite_fields::{macros::felt, pow::Pow, Felt};

/// Represents an elliptic curve
/// ( y^2 = x^3 + ax + b )
#[derive(Debug, Clone, PartialEq)]
pub struct Curve {
    pub a: Felt,
    pub b: Felt,
}

impl Curve {
    pub fn new(a: Felt, b: Felt) -> Self {
        Self { a, b }
    }

    pub fn point(&self, x: Felt, y: Felt) -> Result<Point> {
        Point::try_from_felts(x, y, self.a.clone(), self.b.clone())
    }

    pub fn identity(&self) -> Point {
        Point::new(
            PointType::Infinity,
            PointType::Infinity,
            PointType::Normal(self.a.clone()),
            PointType::Normal(self.b.clone()),
        )
    }
}

/// Represents type of a point on an elliptic curve
///
/// Can be either a normal point or infinity
#[derive(Debug, Clone, PartialEq)]
pub enum PointType {
    Infinity,
    Normal(Felt),
}

impl PointType {
    pub fn unwrap(self) -> Felt {
        match self {
            Self::Infinity => panic!("Cannot unwrap infinity"),
            Self::Normal(felt) => felt,
        }
    }
}

/// Represents a point on an elliptic curve
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: PointType,
    pub y: PointType,
    pub a: PointType,
    pub b: PointType,
}

impl Point {
    /// Checks if the point is on the curve
    pub fn is_on_curve(&self) -> bool {
        match (&self.x, &self.y) {
            (PointType::Infinity, PointType::Infinity) => true,
            (PointType::Infinity, _) | (_, PointType::Infinity) => false,
            (PointType::Normal(x), PointType::Normal(y)) => {
                let a = self.a.clone().unwrap();
                let b = self.b.clone().unwrap();

                let left = y.pow(2u32);
                let right = x.pow(3u32) + a * x + b;

                left == right
            },
        }
    }



    /// Creates a new point from point types
    pub fn new(x: PointType, y: PointType, a: PointType, b: PointType) -> Self {
        let point = Self { x, y, a, b };
        assert!(point.is_on_curve(), "Point is not on the curve");
        point
    }

    /// Creates a new point from felt values
    /// Asserts that the point is on the curve
    pub fn from_felts(x: Felt, y: Felt, a: Felt, b: Felt) -> Self {
        let point = Self {
            x: PointType::Normal(x),
            y: PointType::Normal(y),
            a: PointType::Normal(a),
            b: PointType::Normal(b),
        };
        assert!(point.is_on_curve(), "Point is not on the curve");
        point
    }

    /// Creates a new point from felt values
    pub fn try_from_felts(x: Felt, y: Felt, a: Felt, b: Felt) -> Result<Self> {
        let point = Self {
            x: PointType::Normal(x),
            y: PointType::Normal(y),
            a: PointType::Normal(a),
            b: PointType::Normal(b),
        };

        if point.is_on_curve() {
            Ok(point)
        } else {
            Err(eyre!("Point is not on the curve"))
        }
    }
}

impl Add for Point {
    type Output = Self;

    /// Performs point addition on two points on an elliptic curve
    /// <https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication#Point_addition>
    ///
    /// In a high level, given points P1(x1,y1) and P2(x2,y2) on an elliptic curve, adding P1 and P2
    /// means drawing a line through P1 and P2 and finding the point where the line intersects the
    /// curve and taking its reflection across the x-axis.
    #[allow(clippy::erasing_op)]
    fn add(self, rhs: Self) -> Self::Output {
        match &self.x {
            PointType::Normal(x1) => match &rhs.x {
                PointType::Normal(x2) => {
                    let y1 = self.y.unwrap();
                    let y2 = rhs.y.unwrap();

                    if x1 == x2 && y1 != y2 {
                        // Case 1: self.x == rhs.x && self.y != rhs.y; return Infinity
                        // If we are on the same x but different y, we are tangent to the vertical line
                        Self::new(PointType::Infinity, PointType::Infinity, self.a, self.b)
                    } else if x1 != x2 {
                        // Case 2: self.x != rhs.x
                        // Formula (x3,y3) == (x1,y1) + (x2,y2)
                        //  s = (y2-y1) / (x2-x1)
                        // x3 = s ** 2 - x1 - x2
                        // y3 = s * (x1-x3) - y1
                        let s = (&y2 - &y1) / (x2 - x1);
                        let x3 = s.pow(2u32) - x1 - x2;
                        let y3 = s * (x1 - &x3) - &y1;

                        Self::new(PointType::Normal(x3), PointType::Normal(y3), self.a, self.b)
                    } else if y1 == x1 * 0u32 {
                        // Case 4: if we are tangent to the vertical line, we return the point at infinity
                        // note instead of figuring out what 0 is for each type we just use 0 * self.x
                        Self::new(PointType::Infinity, PointType::Infinity, self.a, self.b)
                    } else {
                        // Case 3: self == other
                        // Formula (x3,y3)=(x1,y1)+(x1,y1)
                        //  s = (3 * x1**2 + a) / (2 * y1)
                        // x3 = s**2 - 2 * x1
                        // y3 = s * (x1-x3) - y1
                        let a = self.a.unwrap();
                        let b = self.b.unwrap();
                        let s = (&x1.pow(2u32) * 3u32 + &a) / (&y1 * 2u32);
                        let x3 = &s.pow(2u32) - x1 * 2u32;
                        let y3 = &s * (x1 - &x3) - &y1;
                        Self::new(
                            PointType::Normal(x3),
                            PointType::Normal(y3),
                            PointType::Normal(a),
                            PointType::Normal(b),
                        )
                    }
                }
                // Case 0.1: rhs points to Infinity, return self
                // This is the identity element for addition
                PointType::Infinity => self,
            },
            // Case 0.0: self points to Infinity, return rhs
            // This is the identity element for addition
            PointType::Infinity => rhs,
        }
    }
}

impl Mul<u32> for Point {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: u32) -> Self::Output {
        let mut product = Point::new(
            PointType::Infinity,
            PointType::Infinity,
            self.a.clone(),
            self.b.clone(),
        );

        for _ in 0..rhs {
            product = product + self.clone();
        }

        product
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
        assert_eq!(pt1 + pt2, expected_sum);
    }
}
