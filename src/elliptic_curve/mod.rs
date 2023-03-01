#![allow(unused)]

use color_eyre::eyre::{ Result, eyre };

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
    use primitive_types::U256;
    use super::*;

    #[test]
    fn test_curve() {
        let prime = U256::from(103);
        let point = Point::try_new(
            Felt::new(U256::from(17), prime), 
            Felt::new(U256::from(64), prime), 
            Felt::new(U256::from(0), prime), 
            Felt::new(U256::from(7), prime), 
        );
        assert!(point.is_ok());

        let point = Point::try_new(
            Felt::new(U256::from(17), prime), 
            Felt::new(U256::from(64), prime), 
            Felt::new(U256::from(0), prime), 
            Felt::new(U256::from(9), prime), 
        );
        assert!(point.is_err());
    }
}
