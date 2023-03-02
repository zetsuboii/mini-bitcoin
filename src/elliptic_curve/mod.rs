#![allow(unused)]
pub mod curve;
pub mod point;

#[cfg(test)]
mod tests {
    use crate::finite_fields::macros::felt;
    use primitive_types::U256;

    use super::{*, curve::Curve};

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
