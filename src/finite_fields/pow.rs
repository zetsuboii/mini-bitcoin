use primitive_types::U256;

pub trait Pow<T> {
    fn pow(&self, exponent: T) -> Self;
}

impl Pow<u32> for U256 {
    fn pow(&self, exponent: u32) -> U256 {
        let mut result = U256::one();
        let mut base = *self;

        for _ in 0..exponent {
            result = result * base;
        }

        result
    }
}

impl Pow<u32> for u64 {
    fn pow(&self, exponent: u32) -> u64 {
        let mut result = 1;
        let mut base = *self;

        for _ in 0..exponent {
            result = result * base;
        }

        result
    }
}