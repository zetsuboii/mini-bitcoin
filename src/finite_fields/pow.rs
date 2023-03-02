pub trait Pow<T> {
    type Output;
    fn pow(&self, exponent: T) -> Self::Output;
}