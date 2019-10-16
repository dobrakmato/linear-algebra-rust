/// Types that have a "zero" value.
///
/// This trait is intended for use in conjunction with `Add`, as an identity:
/// `x + T::zero() == x`.
pub trait Zero: Sized {
    /// The "zero" (usually, additive identity) for this type.
    fn zero() -> Self;
}

/// Types that have a "one" value.
///
/// This trait is intended for use in conjunction with `Mul`, as an identity:
/// `x * T::one() == x`.
pub trait One: Sized {
    /// The "one" (usually, multiplicative identity) for this type.
    fn one() -> Self;
}

macro_rules! zero_one_impl {
    ($($t:ty)*) => ($(
        impl Zero for $t {
            #[inline]
            fn zero() -> Self { 0 }
        }
        impl One for $t {
            #[inline]
            fn one() -> Self { 1 }
        }
    )*)
}
zero_one_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

macro_rules! zero_one_impl_float {
    ($($t:ty)*) => ($(
        impl Zero for $t {
            #[inline]
            fn zero() -> Self { 0.0 }
        }
        impl One for $t {
            #[inline]
            fn one() -> Self { 1.0 }
        }
    )*)
}

zero_one_impl_float! { f32 f64 }