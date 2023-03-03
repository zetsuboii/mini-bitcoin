/// Convenience macro for creating a new Felt
macro_rules! felt {
    ($num:expr, $prime: expr) => {
        crate::finite_fields::element::Felt::new(
            ::num_bigint::BigUint::from($num as u32), 
            ::num_bigint::BigUint::from($prime as u32)
        )
    };
}

/// Implements a trait for all reference combinations (&T-U, T-&U, &T-&U) 
/// of a type given
/// - The type implements the trait for T-U 
/// - T and U are Clone
/// ```
/// use std::ops::Add;
///
/// #[derive(Debug, Clone, PartialEq)]
/// struct Foo {
///     bar: u32,
/// }
///
/// impl Add for Foo {
///    type Output = Self;
///
///   fn add(self, rhs: Self) -> Self::Output {
///      Self {
///         bar: self.bar + rhs.bar,
///      }
///   }
/// }
/// 
/// impl_refs!(Add, add, Foo, Foo);
///
/// let baz = Foo { bar: 1 };
/// let qux = Foo { bar: 2 };
/// 
/// assert_eq!((&baz).add(&qux), Foo { bar: 3 });
/// ```
macro_rules! impl_refs {
    ($trait:ident, $fn:ident, $type_lhs:ty, $type_rhs:ty) => {
        impl<'a, 'b> $trait<&'b $type_rhs> for &'a $type_lhs {
            type Output = $type_lhs;

            fn $fn(self, rhs: &'b $type_rhs) -> Self::Output {
                let clone_self = self.clone();
                let clone_rhs = rhs.clone();
                clone_self.$fn(clone_rhs)
            }
        }

        impl<'a> $trait<&'a $type_rhs> for $type_lhs {
            type Output = $type_lhs;

            fn $fn(self, rhs: &'a $type_rhs) -> Self::Output {
                (&self).$fn(rhs)
            }
        }

        impl<'b> $trait<$type_rhs> for &'b $type_lhs {
            type Output = $type_lhs;

            fn $fn(self, rhs: $type_rhs) -> Self::Output {
                self.$fn(&rhs)
            }
        }
    };
}

pub(crate) use felt;
pub(crate) use impl_refs;