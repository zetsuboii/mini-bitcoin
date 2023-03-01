#![allow(unused)]

#[derive(Debug, Clone, Default)]
pub struct U256([u64; 4]);

impl U256 {
    fn new(value: [u64; 4]) -> Self {
        U256(value)
    }
}

impl From<u64> for U256 {
    fn from(v: u64) -> Self {
        Self::new([0, 0, 0, v])
    }
}

impl From<i32> for U256 {
    fn from(v: i32) -> Self {
        (v as u64).into()
    }
}

impl PartialEq for U256 {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(self_part, other_part)| self_part == other_part)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for U256 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for (self_part, other_part) in self.0.iter().zip(other.0.iter()) {
            if self_part > other_part {
                return Some(std::cmp::Ordering::Greater);
            } else if self_part < other_part {
                return Some(std::cmp::Ordering::Less);
            }
        }
        return Some(std::cmp::Ordering::Equal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let a = U256::from(1);
        let b = U256::from(1);

        assert_eq!(a, b);

        let a = U256::new([0, 10, 0, 10]);
        let b = U256::new([0, 10, 0, 10]);

        assert_eq!(a, b);
    }

    #[test]
    fn test_cmp() {
        let a = U256::from(10);
        let b = U256::from(20);

        assert!(a < b);
        assert!(b > a);

        let a = U256::new([0, 10, 0, 0]);
        let b = U256::new([0, 0, 10, 0]);

        assert!(a > b);
    }
}
