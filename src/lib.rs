//! This crate provides functions for aligning addresses.
//!
//! [`Align`] is implemented for all unsigned integers and provides methods for:
//! * [`align_down`]
//! * [`align_up`]
//! * [`is_aligned_to`]
//!
//! [`align_down`]: Align::align_down
//! [`align_up`]: Align::align_up
//! [`is_aligned_to`]: Align::is_aligned_to
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
//! assert_eq!(123_u64.align_up(2), 124);
//! ```

#![no_std]
#![forbid(unsafe_code)]

/// An adress that can be aligned.
pub trait Align<A = Self>: Copy + PartialEq {
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
    #[inline]
    fn is_aligned_to(self, align: A) -> bool {
        self.align_down(align) == self
    }
}

macro_rules! align_impl {
    ($u:ty, $align_down:ident, $align_up:ident, $is_aligned_to:ident) => {
        /// Align address downwards.
        ///
        /// Returns the greatest `x` with alignment `align` so that `x <= addr`.
        ///
        /// Panics if the alignment is not a power of two.
        ///
        /// This is a `const` version of [`Align::align_down`].
        // Adapted from `x86_64`
        #[inline]
        pub const fn $align_down(addr: $u, align: $u) -> $u {
            assert!(align.is_power_of_two(), "`align` must be a power of two");
            addr & !(align - 1)
        }

        /// Align address upwards.
        ///
        /// Returns the smallest `x` with alignment `align` so that `x >= addr`.
        ///
        /// Panics if the alignment is not a power of two or if an overflow occurs.
        ///
        /// This is a `const` version of [`Align::align_up`].
        // Adapted from `x86_64`
        #[inline]
        pub const fn $align_up(addr: $u, align: $u) -> $u {
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

        /// Checks whether the address has the demanded alignment.
        ///
        /// This is a `const` version of [`Align::is_aligned_to`].
        #[inline]
        pub const fn $is_aligned_to(addr: $u, align: $u) -> bool {
            $align_down(addr, align) == addr
        }

        impl Align for $u {
            #[inline]
            fn align_down(self, align: Self) -> Self {
                $align_down(self, align)
            }

            #[inline]
            fn align_up(self, align: Self) -> Self {
                $align_up(self, align)
            }
        }
    };
}

align_impl!(u8, u8_align_down, u8_align_up, u8_is_aligned_to);
align_impl!(u16, u16_align_down, u16_align_up, u16_is_aligned_to);
align_impl!(u32, u32_align_down, u32_align_up, u32_is_aligned_to);
align_impl!(u64, u64_align_down, u64_align_up, u64_is_aligned_to);
align_impl!(u128, u128_align_down, u128_align_up, u128_is_aligned_to);
align_impl!(usize, usize_align_down, usize_align_up, usize_is_aligned_to);

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

    test_align_up_impl!(u8, u8_align_up, test_u8_align_up);
    test_align_up_impl!(u16, u16_align_up, test_u16_align_up);
    test_align_up_impl!(u32, u32_align_up, test_u32_align_up);
    test_align_up_impl!(u64, u64_align_up, test_u64_align_up);
    test_align_up_impl!(u128, u128_align_up, test_u128_align_up);
    test_align_up_impl!(usize, usize_align_up, test_usize_align_up);

    macro_rules! test_align_up_overflow_impl {
        ($u:ty, $test_align_up_overflow:ident, $two:expr) => {
            #[test]
            #[should_panic]
            fn $test_align_up_overflow() {
                <$u>::MAX.align_up($two);
            }
        };
    }

    test_align_up_overflow_impl!(u8, test_u8_align_up_overflow, 2);
    test_align_up_overflow_impl!(u16, test_u16_align_up_overflow, 2);
    test_align_up_overflow_impl!(u32, test_u32_align_up_overflow, 2);
    test_align_up_overflow_impl!(u64, test_u64_align_up_overflow, 2);
    test_align_up_overflow_impl!(u128, test_u128_align_up_overflow, 2);
    test_align_up_overflow_impl!(usize, test_usize_align_up_overflow, 2);
}
