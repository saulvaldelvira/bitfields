//! # Example
//! ```
//! use bitfi::bitfield;
//!
//! bitfield! {
//!     Flags = u16 {
//!         on: 0;
//!         field1: 1 ..= 3;
//!     }
//! }
//!
//! let mut flags = Flags::default();
//!
//! assert!(!flags.get_on());
//! flags.set_on();
//! assert!(flags.get_on());
//!
//! assert_eq!(flags.get_field1(), 0);
//! flags.set_field1(0b101);
//! assert_eq!(flags.get_field1(), 0b101);
//!
//! assert_eq!(flags.get_inner(), 0b1011, "{:b} {:b}", flags.get_inner(), 0b1011);
//! ```

#![no_std]

use core::ops::{Add, BitAnd, BitAndAssign, BitOrAssign, BitXorAssign, Not, RangeBounds, Shl, Shr, Sub};

pub mod helper_traits;
use helper_traits::{One, Zero, BitSize};

/// Declares a [bitfield](BitField)
#[doc(inline)]
pub use macros::bitfield;

trait BitFieldRequisites:
       BitOrAssign<Self> + BitXorAssign<Self> +
       Shl<Self, Output = Self> + Shr<Self, Output = Self> +
       BitAnd<Self, Output = Self> + BitAndAssign<Self> +
       Sub<Self, Output = Self> + Add<Self, Output = Self> +
       Not<Output = Self> +
       PartialEq<Self> + Default + Copy +
       One + Zero + BitSize
{}

impl<T> BitFieldRequisites for T
where
    T:
       BitOrAssign<T> + BitXorAssign<T> +
       Shl<T, Output = T> + Shr<T, Output = T> +
       BitAnd<T, Output = Self> + BitAndAssign<T> +
       Sub<T, Output = Self> + Add<T, Output = Self> +
       Not<Output = T> +
       PartialEq<T> + Default + Copy +
       One + Zero + BitSize
{}

/// Defines methods for bit-field manipulation
#[allow(private_bounds)]
pub trait BitField<T: BitFieldRequisites> {
    /// Sets the i'th bit to 1
    fn set_bit(&mut self, i: T);

    /// Clears the i'th bit to 0
    fn clear_bit(&mut self, i: T);

    /// Toggles the i'th bit
    fn toggle_bit(&mut self, i: T);

    /// Gets the i'th bit.
    ///
    /// true for 1
    /// false for 0
    fn get_bit(&self, i: T) -> bool;

    /// Sets bits in the given [range](RangeBounds) to b
    fn set_bit_range(&mut self, range: impl RangeBounds<T>, b: T);

    /// Gets the bits in the given [range](RangeBounds)
    fn get_bit_range(&self, range: impl RangeBounds<T>) -> T;
}

fn range_convrt<T>(range: impl RangeBounds<T>) -> (T, T)
where
    T: Sub<Output = T> + One + Zero + BitSize + Copy,
{
    use core::ops::Bound;

    let start = match range.start_bound() {
        Bound::Included(n) => *n,
        Bound::Excluded(n) => *n - T::one(),
        Bound::Unbounded => T::zero(),
    };

    let end = match range.end_bound() {
        Bound::Included(n) => *n,
        Bound::Excluded(n) => *n - T::one(),
        Bound::Unbounded => T::bit_size(),
    };

    (start, end)
}

impl<T: BitFieldRequisites> BitField<T> for T {
    fn set_bit(&mut self, i: T) {
        (*self) |= T::one() << i;
    }

    fn clear_bit(&mut self, i: T) {
        (*self) &= !(T::one() << i);
    }

    fn get_bit(&self, i: T) -> bool {
        ( ( *self & (T::one() << i) ) >> i ) != T::zero()
    }

    fn toggle_bit(&mut self, i: T) {
        (*self) ^= T::one() << i;
    }

    fn set_bit_range(&mut self, range: impl RangeBounds<T>, b: T) {
        let (start, end) = range_convrt(range);
        let mut mask = (!T::zero()) << (end - start + T::one());
        mask = !mask;

        (*self) &= !(mask << start);
        (*self) |= (b & mask) << start;
    }

    fn get_bit_range(&self, range: impl RangeBounds<T>) -> T {
        let (start, end) = range_convrt(range);
        let mut mask = (!T::zero()) << (end - start + T::one());
        mask = !mask << start;

        let elem = *self & mask;
        elem >> start
    }
}

#[doc(hidden)]
#[expect(unused)]
fn check_numeric_types() {
    macro_rules! check {
        ($($t:ty),*) => {
            $( assert!(!<$t as BitField<$t>>::get_bit(&0, 0)); )*
        };
    }

    /* Make sure that BitField is implemented for the following numeric types */
    check!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
}
