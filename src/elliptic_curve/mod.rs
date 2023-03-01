#![allow(unused)]

use color_eyre::eyre::Result;

use crate::finite_fields::Felt;

struct Point {
    x: Felt,
    y: Felt,
    a: Felt,
    b: Felt,
}

impl Point {
    pub fn new() -> Self {
        todo!();
    }

    pub fn try_new() -> Result<Self> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_curve() {
        let point = Point::try_new();
        assert!(point.is_ok());

        let point = Point::try_new();
        assert!(point.is_err());
    }
}