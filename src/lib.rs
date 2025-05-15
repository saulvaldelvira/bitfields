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

/// Declares a [bitfield](BitField)
#[doc(inline)]
pub use macros::bitfield;

/// Defines methods for bit-field manipulation
pub trait BitField<T> {
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
    T: Sub<Output = T> + From<u8> + Copy,
{
    use core::ops::Bound;

    let start = match range.start_bound() {
        Bound::Included(n) => *n,
        Bound::Excluded(n) => *n - T::from(1),
        Bound::Unbounded => T::from(0),
    };

    let end = match range.end_bound() {
        Bound::Included(n) => *n,
        Bound::Excluded(n) => *n - T::from(1),
        Bound::Unbounded => {
            T::from ( (core::mem::size_of::<T>() * 8) as u8)
        },
    };

    (start, end)
}

impl<T> BitField<T> for T
where
    T: BitOrAssign<T> + BitXorAssign<T> +
       Shl<T, Output = T> + Shr<T, Output = T> +
       BitAnd<T, Output = Self> + BitAndAssign<T> +
       Sub<T, Output = Self> + Add<T, Output = Self> +
       Not<Output = Self> +
       From<u8> + PartialEq<T> + Default + Copy,
{
    fn set_bit(&mut self, i: T) {
        (*self) |= T::from(1) << i;
    }

    fn clear_bit(&mut self, i: T) {
        (*self) &= !(T::from(1) << i);
    }

    fn get_bit(&self, i: T) -> bool {
        ( ( *self & (T::from(1) << i) ) >> i ) != T::from(0)
    }

    fn toggle_bit(&mut self, i: T) {
        (*self) ^= T::from(1) << i;
    }

    fn set_bit_range(&mut self, range: impl RangeBounds<T>, b: T) {
        let (start, end) = range_convrt(range);
        let mut mask = (!T::from(0)) << (end - start + T::from(1));
        mask = !mask;

        (*self) &= !(mask << start);
        (*self) |= (b & mask) << start;
    }

    fn get_bit_range(&self, range: impl RangeBounds<T>) -> T {
        let (start, end) = range_convrt(range);
        let mut mask = (!T::from(0)) << (end - start + T::from(1));
        mask = !mask << start;

        let elem = *self & mask;
        elem >> start
    }
}
