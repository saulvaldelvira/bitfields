pub trait One {
    fn one() -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait BitSize {
    fn bit_size() -> Self;
}

#[macro_export]
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


#[macro_export]
macro_rules! impl_bitsize {
    ($($ty:ty: $bs:literal),*) => {
        $(
            impl BitSize for $ty {
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
