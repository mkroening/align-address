//! This crate provides functions for aligning addresses.
//!
//! [`Align`] is implemented for all unsigned integers and provides methods for:
//! * [`align_down`]
//! * [`align_up`]
//! * [`is_aligned`]
//!
//! [`align_down`]: Align::align_down
//! [`align_up`]: Align::align_up
//! [`is_aligned`]: Align::is_aligned
//!
//! This crate is based on work from the [`x86_64`] crate, but is available for all architectures and all unsigned integer types.
//!
//! [`x86_64`]: https://docs.rs/x86_64
//!
//! # Examples
//!
//! ```
//! use align_address::Align;
//!
//! assert_eq!(123_u64.align_up(2_u64), 124);
//! ```

#![no_std]
#![forbid(unsafe_code)]

/// An adress that can be aligned.
pub trait Align<A = Self>: Copy {
    /// Align address downwards.
    ///
    /// Returns the greatest `x` with alignment `align` so that `x <= addr`.
    ///
    /// Panics if the alignment is not a power of two.
    fn align_down(self, align: A) -> Self;

    /// Align address upwards.
    ///
    /// Returns the smallest `x` with alignment `align` so that `x >= addr`.
    ///
    /// Panics if the alignment is not a power of two or if an overflow occurs.
    fn align_up(self, align: A) -> Self;

    /// Checks whether the address has the demanded alignment.
    #[allow(clippy::wrong_self_convention)]
    fn is_aligned(self, align: A) -> bool;
}

macro_rules! align_impl {
    ($u:ty, $align_down:ident, $align_up:ident) => {
        /// Align address downwards.
        ///
        /// Returns the greatest `x` with alignment `align` so that `x <= addr`.
        ///
        /// Panics if the alignment is not a power of two.
        // Adapted from `x86_64`
        #[inline]
        const fn $align_down(addr: $u, align: $u) -> $u {
            assert!(align.is_power_of_two(), "`align` must be a power of two");
            addr & !(align - 1)
        }

        /// Align address upwards.
        ///
        /// Returns the smallest `x` with alignment `align` so that `x >= addr`.
        ///
        /// Panics if the alignment is not a power of two or if an overflow occurs.
        // Adapted from `x86_64`
        #[inline]
        const fn $align_up(addr: $u, align: $u) -> $u {
            assert!(align.is_power_of_two(), "`align` must be a power of two");
            let align_mask = align - 1;
            if addr & align_mask == 0 {
                addr // already aligned
            } else {
                // FIXME: Replace with .expect, once `Option::expect` is const.
                if let Some(aligned) = (addr | align_mask).checked_add(1) {
                    aligned
                } else {
                    panic!("attempt to add with overflow")
                }
            }
        }

        impl<A: Into<Self>> Align<A> for $u {
            #[inline]
            fn align_down(self, align: A) -> Self {
                $align_down(self, align.into())
            }

            #[inline]
            fn align_up(self, align: A) -> Self {
                $align_up(self, align.into())
            }

            #[inline]
            fn is_aligned(self, align: A) -> bool {
                self.align_down(align) == self
            }
        }
    };
}

align_impl!(u8, align_down_u8, align_up_u8);
align_impl!(u16, align_down_u16, align_up_u16);
align_impl!(u32, align_down_u32, align_up_u32);
align_impl!(u64, align_down_u64, align_up_u64);
align_impl!(u128, align_down_u128, align_up_u128);
align_impl!(usize, align_down_usize, align_up_usize);

// Adapted from `x86_64`
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_align_up_impl {
        ($u:ty, $align_up:ident, $test_align_up:ident) => {
            #[test]
            fn $test_align_up() {
                // align 1
                assert_eq!($align_up(0, 1), 0);
                assert_eq!($align_up(123, 1), 123);
                assert_eq!($align_up(<$u>::MAX, 1), <$u>::MAX);
                // align 2
                assert_eq!($align_up(0, 2), 0);
                assert_eq!($align_up(123, 2), 124);
                assert_eq!($align_up(<$u>::MAX - 1, 2), <$u>::MAX - 1);
                // address 0
                assert_eq!($align_up(0, 128), 0);
                assert_eq!($align_up(0, 1), 0);
                assert_eq!($align_up(0, 2), 0);
                assert_eq!($align_up(0, <$u>::MAX & 1 << (<$u>::BITS - 1)), 0);
            }
        };
    }

    test_align_up_impl!(u8, align_up_u8, test_align_up_u8);
    test_align_up_impl!(u16, align_up_u16, test_align_up_u16);
    test_align_up_impl!(u32, align_up_u32, test_align_up_u32);
    test_align_up_impl!(u64, align_up_u64, test_align_up_u64);
    test_align_up_impl!(u128, align_up_u128, test_align_up_u128);
    test_align_up_impl!(usize, align_up_usize, test_align_up_usize);

    macro_rules! test_align_up_overflow_impl {
        ($u:ty, $test_align_up_overflow:ident, $two:expr) => {
            #[test]
            #[should_panic]
            fn $test_align_up_overflow() {
                <$u>::MAX.align_up($two);
            }
        };
    }

    test_align_up_overflow_impl!(u8, test_align_up_overflow_u8, 2_u8);
    test_align_up_overflow_impl!(u16, test_align_up_overflow_u16, 2_u16);
    test_align_up_overflow_impl!(u32, test_align_up_overflow_u32, 2_u32);
    test_align_up_overflow_impl!(u64, test_align_up_overflow_u64, 2_u64);
    test_align_up_overflow_impl!(u128, test_align_up_overflow_u128, 2_u128);
    test_align_up_overflow_impl!(usize, test_align_up_overflow_usize, 2_usize);
}
