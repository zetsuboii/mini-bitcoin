#![allow(unused)]
pub mod element;
pub mod macros;
pub mod modulo;
pub mod pow;

// TODO: Assert all operations are done in the same field

#[cfg(test)]
mod tests {
    use crate::finite_fields::pow::Pow;

    use super::macros::felt;

    #[test]
    fn test_display() {
        let felt_a = felt!(1, 11);
        assert_eq!(format!("{}", felt_a), "Field Element: 1 (11)")
    }

    #[test]
    fn test_add_sub() {
        let felt_a = felt!(11, 19);
        let felt_b = felt!(17, 19);
        assert_eq!(felt_a + felt_b, felt!(9, 19));

        let felt_a = felt!(6, 19);
        let felt_b = felt!(13, 19);
        assert_eq!(felt_a - felt_b, felt!(12, 19));
    }

    #[test]
    fn test_mul_div() {
        let felt_a = felt!(2, 19);
        let felt_b = felt!(17, 19);
        assert_eq!(felt_a * felt_b, felt!(15, 19));

        let felt_a = felt!(2, 19);
        let felt_b = felt!(7, 19);
        assert_eq!(felt_a / felt_b, felt!(3, 19));
    }

    #[test]
    fn test_pow() {
        // 3^-1 == 1/3
        let felt_a = felt!(3, 19).pow(-1i64);
        let felt_b = felt!(1, 19) / felt!(3, 19);
        assert_eq!(felt_a, felt_b);
    }
}
