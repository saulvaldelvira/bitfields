//! Helper traits: [One] [Zero] and [BitSize]

/// Helper identity trait
pub trait One {
    /// Gets one for Self
    ///
    /// # Example
    /// ```
    /// use bitfi::helper_traits::One;
    /// assert_eq!(<u16 as One>::one(), 1_u16);
    /// ```
    fn one() -> Self;
}

/// Helper trait for types that can be "zero"
pub trait Zero {
    /// Gets zero for Self
    ///
    /// # Example
    /// ```
    /// use bitfi::helper_traits::Zero;
    /// assert_eq!(<u16 as Zero>::zero(), 0_u16);
    /// ```
    fn zero() -> Self;
}

/// Helper trait to get the number of bits in a type
pub trait BitSize {
    /// Gets the number of bytes in this type.
    ///
    /// This means `core::mem::size_of::<Self>() * 8`  as Self
    ///
    /// # Example
    /// ```
    /// use bitfi::helper_traits::BitSize;
    /// assert_eq!(<u16 as BitSize>::bit_size(), 16);
    /// ```
    fn bit_size() -> Self;
}

macro_rules! impl_ident {
    ($($ty:ty),*) => {
        $(
            impl One for $ty {
                #[inline(always)]
                fn one() -> Self { 1 }
            }

            impl Zero for $ty {
                #[inline(always)]
                fn zero() -> Self { 0 }
            }
        )*
    };
}

/// Implements [BitSize] for a type, given it's number of bytes
///
/// # Example
/// ```
/// use bitfi::impl_bitsize;
///
/// struct S(u8);
///
/// impl_bitsize! {
///     S: S(8)
/// }
/// ```
#[macro_export]
macro_rules! impl_bitsize {
    ($($ty:ty: $bs:expr),*) => {
        $(
            impl $crate::helper_traits::BitSize for $ty {
                #[inline(always)]
                fn bit_size() -> Self { $bs }
            }
        )*
    };
}

impl_ident!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

impl_bitsize! {
    u8: 8,
    u16: 16,
    u32: 32,
    u64: 64,
    u128: 128,
    i8: 8,
    i16: 16,
    i32: 32,
    i64: 64,
    i128: 128
}

impl BitSize for usize {
    fn bit_size() -> Self {
        core::mem::size_of::<usize>() * 8
    }
}

impl BitSize for isize {
    fn bit_size() -> Self {
        (core::mem::size_of::<isize>() as u32 * 8) as isize
    }
}
