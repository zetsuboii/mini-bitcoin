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
    /// Creates a new curve (y^2 = x^3 + ax + b) from a and b
    pub fn new(a: Felt, b: Felt) -> Self {
        Self { a, b }
    }

    /// Creates a point on the curve 
    /// 
    /// # Errors
    /// 
    /// Propagates if `Point::try_from_felts` returns an error
    pub fn point(&self, x: Felt, y: Felt) -> Result<Point> {
        Point::try_from_felts(x, y, self.a.clone(), self.b.clone())
    }

    /// Returns the identity point of the curve which is (Inf, Inf)
    pub fn identity(&self) -> Point {
        Point::new(PointType::Infinity, PointType::Infinity, self.clone())
    }
}
