/// Convenience macro for creating a new Felt
macro_rules! felt {
    ($num:expr, $prime: expr) => {
        crate::finite_fields::Felt::new(
            ::primitive_types::U256::from($num), 
            ::primitive_types::U256::from($prime)
        )
    };
}

pub(crate) use felt;