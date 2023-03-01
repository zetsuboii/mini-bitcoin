pub struct U256 {
    inner: [u64; 4],
}

impl PartialEq for U256 {
    fn eq(&self, other: &Self) -> bool {
        self.inner
            .iter()
            .zip(other.inner.iter())
            .all(|(self_part, other_part)| self_part == other_part)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for U256 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for (self_part, other_part) in self.inner.iter().zip(other.inner.iter()) {
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
    #[test]
    fn test_eq() {
        todo!()
    }

    #[test]
    fn test_cmp() {
        todo!()
    }
}
