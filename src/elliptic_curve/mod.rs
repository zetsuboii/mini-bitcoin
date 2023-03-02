#![allow(unused)]
pub mod curve;
pub mod point;

#[cfg(test)]
mod tests {
    use crate::finite_fields::macros::felt;
    use primitive_types::U256;

    use super::{curve::Curve, *};

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

    #[test]
    fn test_scalar() {
        let scalar_multiples = vec![
            (47, 71),
            (36, 111),
            (15, 137),
            (194, 51),
            (126, 96),
            (139, 137),
            (92, 47),
            (116, 55),
        ];

        let prime = 223u64;
        let curve = Curve::new(felt!(0, prime), felt!(7, prime));
        let generator = curve.point(felt!(47, prime), felt!(71, prime)).unwrap();

        for i in 1..=scalar_multiples.len() as u32 {
            let result = generator.clone() * i;
            let expected = curve
                .point(
                    felt!(scalar_multiples[i as usize - 1].0, prime),
                    felt!(scalar_multiples[i as usize - 1].1, prime),
                )
                .unwrap();

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_order() {
        let prime: u64 = 223;
        let a = felt!(0, prime);
        let b = felt!(7, prime);
        let curve = Curve::new(a.clone(), b.clone());

        let mut i: usize = 0;
        let mut point = curve.identity();
        loop {
            let generator = curve.point(felt!(15, prime), felt!(86, prime)).unwrap();
            point = point + generator;
            i += 1;

            if point == curve.identity() {
                break;
            }
        }

        assert_eq!(i, 7);
    }
}
