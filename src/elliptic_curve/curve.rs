use super::point::{Point, PointType};
use crate::finite_fields::element::Felt;
use color_eyre::eyre::Result;

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
        Point::new(PointType::Infinity, PointType::Infinity, self.clone())
    }
}
