// I/O boilerplate //

use std::io::Read;

#[allow(clippy::pedantic)]
pub mod itoap {
    //! SOURCE CODE AND LICENSE AT https://github.com/Kogia-sima/itoap
    //!
    //! This crate provides even faster functions for printing integers with decimal format
    //! than [itoa](https://crates.io/crates/itoa) crate.
    //!
    //! If you want to write integers in decimal format to `String`, `Vec` or any other
    //! contiguous buffer, then this crate is the best choice.
    //!
    //! If you want to write integers to a `std::io::Write` or `std::fmt::Write`,
    //! [itoa](https://github.com/dtolnay/itoa) crate and `itoap` crate shows almost same
    //! performance.
    //!
    //! The implementation is based on the `sse2` algorithm from
    //! [itoa-benchmark](https://github.com/miloyip/itoa-benchmark) repository.
    //! While `itoa` crate writes integers from **last** digits, this algorithm writes
    //! from **first** digits. It allows integers to be written directly to the buffer.
    //! That's why `itoap` is faster than `itoa`.
    //!
    //! # Feature Flags
    //!
    //! - `alloc`: use [alloc](https://doc.rust-lang.org/alloc/) crate (enabled by default)
    //! - `std`: use [std](https://doc.rust-lang.org/std/) crate (enabled by default)
    //! - `simd`: use SIMD intrinsics if available
    //!
    //! # Examples
    //!
    //! ```
    //! # #[cfg(feature = "std")] {
    //! let value = 17u64;
    //!
    //! let mut buf = String::new();
    //! buf.push_str("value: ");
    //! itoap::write_to_string(&mut buf, value);
    //!
    //! assert_eq!(buf, "value: 17");
    //! # }
    //! ```
    //!
    //! ```
    //! use core::mem::{MaybeUninit, transmute};
    //! use itoap::Integer;
    //!
    //! unsafe {
    //!     let mut buf = [MaybeUninit::<u8>::uninit(); i32::MAX_LEN];
    //!     let len = itoap::write_to_ptr(buf.as_mut_ptr() as *mut u8, -2953);
    //!     let result: &[u8] = transmute(&buf[..len]);
    //!     assert_eq!(result, b"-2953");
    //! }
    //! ```

    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    mod common {
        use core::ops::{Div, Mul, Sub};
        use core::ptr;

        const DEC_DIGITS_LUT: &[u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

        #[inline]
        pub fn divmod<T: Copy + Sub<Output = T> + Mul<Output = T> + Div<Output = T>>(
            x: T,
            y: T,
        ) -> (T, T) {
            // https://bugs.llvm.org/show_bug.cgi?id=38217
            let quot = x / y;
            let rem = x - quot * y;
            (quot, rem)
        }

        #[inline]
        pub unsafe fn lookup<T: Into<u64>>(idx: T) -> *const u8 {
            DEC_DIGITS_LUT.as_ptr().add((idx.into() as usize) << 1)
        }

        /// write integer smaller than 10000
        #[inline]
        pub unsafe fn write4(n: u32, buf: *mut u8) -> usize {
            debug_assert!(n < 10000);

            if n < 100 {
                if n < 10 {
                    *buf = n as u8 + 0x30;
                    1
                } else {
                    ptr::copy_nonoverlapping(lookup(n), buf, 2);
                    2
                }
            } else {
                let (n1, n2) = divmod(n, 100);
                if n < 1000 {
                    *buf = n1 as u8 + 0x30;
                    ptr::copy_nonoverlapping(lookup(n2), buf.add(1), 2);
                    3
                } else {
                    ptr::copy_nonoverlapping(lookup(n1), buf.add(0), 2);
                    ptr::copy_nonoverlapping(lookup(n2), buf.add(2), 2);
                    4
                }
            }
        }

        /// write integer smaller than 10000 with 0 padding
        #[inline]
        pub unsafe fn write4_pad(n: u32, buf: *mut u8) {
            debug_assert!(n < 10000);
            let (n1, n2) = divmod(n, 100);

            ptr::copy_nonoverlapping(lookup(n1), buf, 2);
            ptr::copy_nonoverlapping(lookup(n2), buf.add(2), 2);
        }

        #[inline]
        pub unsafe fn write8(n: u32, buf: *mut u8) -> usize {
            debug_assert!(n < 100_000_000);

            if n < 10000 {
                write4(n, buf)
            } else {
                let (n1, n2) = divmod(n, 10000);

                let l = if n1 < 100 {
                    if n1 < 10 {
                        *buf = n1 as u8 + 0x30;
                        5
                    } else {
                        ptr::copy_nonoverlapping(lookup(n1), buf, 2);
                        6
                    }
                } else {
                    let (n11, n12) = divmod(n1, 100);
                    if n1 < 1000 {
                        *buf = n11 as u8 + 0x30;
                        ptr::copy_nonoverlapping(lookup(n12), buf.add(1), 2);
                        7
                    } else {
                        ptr::copy_nonoverlapping(lookup(n11), buf.add(0), 2);
                        ptr::copy_nonoverlapping(lookup(n12), buf.add(2), 2);
                        8
                    }
                };

                let (n21, n22) = divmod(n2, 100);
                ptr::copy_nonoverlapping(lookup(n21), buf.add(l - 4), 2);
                ptr::copy_nonoverlapping(lookup(n22), buf.add(l - 2), 2);
                l
            }
        }

        #[inline]
        pub unsafe fn write8_pad(n: u32, buf: *mut u8) {
            debug_assert!(n < 100_000_000);

            let (n1, n2) = divmod(n, 10000);
            let (n11, n12) = divmod(n1, 100);
            let (n21, n22) = divmod(n2, 100);

            ptr::copy_nonoverlapping(lookup(n11), buf, 2);
            ptr::copy_nonoverlapping(lookup(n12), buf.add(2), 2);
            ptr::copy_nonoverlapping(lookup(n21), buf.add(4), 2);
            ptr::copy_nonoverlapping(lookup(n22), buf.add(6), 2);
        }

        pub unsafe fn write_u8(n: u8, buf: *mut u8) -> usize {
            if n < 10 {
                *buf = n + 0x30;
                1
            } else if n < 100 {
                ptr::copy_nonoverlapping(lookup(n), buf, 2);
                2
            } else {
                let (n1, n2) = divmod(n, 100);
                *buf = n1 + 0x30;
                ptr::copy_nonoverlapping(lookup(n2), buf.add(1), 2);
                3
            }
        }

        pub unsafe fn write_u16(n: u16, buf: *mut u8) -> usize {
            if n < 100 {
                if n < 10 {
                    *buf = n as u8 + 0x30;
                    1
                } else {
                    ptr::copy_nonoverlapping(lookup(n), buf, 2);
                    2
                }
            } else if n < 10000 {
                let (a1, a2) = divmod(n, 100);

                if n < 1000 {
                    *buf = a1 as u8 + 0x30;
                    ptr::copy_nonoverlapping(lookup(a2), buf.add(1), 2);
                    3
                } else {
                    ptr::copy_nonoverlapping(lookup(a1), buf, 2);
                    ptr::copy_nonoverlapping(lookup(a2), buf.add(2), 2);
                    4
                }
            } else {
                let (a1, a2) = divmod(n, 10000);
                let (b1, b2) = divmod(a2, 100);

                *buf = a1 as u8 + 0x30;
                ptr::copy_nonoverlapping(lookup(b1), buf.add(1), 2);
                ptr::copy_nonoverlapping(lookup(b2), buf.add(3), 2);
                5
            }
        }

        /// Multiply unsigned 128 bit integers, return upper 128 bits of the result
        #[inline]
        fn u128_mulhi(x: u128, y: u128) -> u128 {
            let x_lo = x as u64;
            let x_hi = (x >> 64) as u64;
            let y_lo = y as u64;
            let y_hi = (y >> 64) as u64;

            // handle possibility of overflow
            let carry = (x_lo as u128 * y_lo as u128) >> 64;
            let m = x_lo as u128 * y_hi as u128 + carry;
            let high1 = m >> 64;

            let m_lo = m as u64;
            let high2 = (x_hi as u128 * y_lo as u128 + m_lo as u128) >> 64;

            x_hi as u128 * y_hi as u128 + high1 + high2
        }

        /// Write u128 in decimal format
        ///
        /// Integer division algorithm is based on the following paper:
        ///
        ///   T. Granlund and P. Montgomery, “Division by Invariant IntegersUsing Multiplication,”
        ///   in Proc. of the SIGPLAN94 Conference onProgramming Language Design and
        ///   Implementation, 1994, pp. 61–72
        ///
        unsafe fn write_u128_big(mut n: u128, mut buf: *mut u8) -> usize {
            const DIV_FACTOR: u128 = 76624777043294442917917351357515459181;
            const DIV_SHIFT: u32 = 51;
            const POW_10_8: u64 = 100000000;
            const POW_10_16: u64 = 10000000000000000;

            debug_assert!(n > u64::MAX as u128);

            // hold per-8-digits results
            // i.e. result[0] holds n % 10^8, result[1] holds (n / 10^8) % 10^8, ...
            let mut result = [0u32; 5];

            {
                // performs n /= 10^16
                let quot = u128_mulhi(n, DIV_FACTOR) >> DIV_SHIFT;
                let rem = (n - quot * POW_10_16 as u128) as u64;
                debug_assert_eq!(quot, n / POW_10_16 as u128);
                debug_assert_eq!(rem as u128, n % POW_10_16 as u128);

                n = quot;

                result[1] = (rem / POW_10_8) as u32;
                result[0] = (rem % POW_10_8) as u32;

                debug_assert_ne!(n, 0);
                debug_assert!(n <= u128::MAX / POW_10_16 as u128);
            }

            let result_len = if n >= POW_10_16 as u128 {
                // performs n /= 10^16
                let quot = (n >> 16) as u64 / (POW_10_16 >> 16);
                let rem = (n - POW_10_16 as u128 * quot as u128) as u64;
                debug_assert_eq!(quot as u128, n / POW_10_16 as u128);
                debug_assert_eq!(rem as u128, n % POW_10_16 as u128);
                debug_assert!(quot <= 3402823);

                result[3] = (rem / POW_10_8) as u32;
                result[2] = (rem % POW_10_8) as u32;
                result[4] = quot as u32;
                4
            } else if (n as u64) >= POW_10_8 {
                result[3] = ((n as u64) / POW_10_8) as u32;
                result[2] = ((n as u64) % POW_10_8) as u32;
                3
            } else {
                result[2] = n as u32;
                2
            };

            let l = write8(*result.get_unchecked(result_len), buf);
            buf = buf.add(l);

            for i in (0..result_len).rev() {
                write8_pad(*result.get_unchecked(i), buf);
                buf = buf.add(8);
            }

            l + result_len * 8
        }

        #[inline]
        pub unsafe fn write_u128(n: u128, buf: *mut u8) -> usize {
            if n <= u64::MAX as u128 {
                super::write_u64(n as u64, buf)
            } else {
                write_u128_big(n, buf)
            }
        }
    }
    use common::*;

    #[cfg(not(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2",
        not(miri),
    )))]
    mod fallback {
        use core::ptr;

        use super::common::{divmod, lookup, write4, write4_pad, write8_pad};

        pub unsafe fn write_u32(n: u32, buf: *mut u8) -> usize {
            if n < 10000 {
                write4(n, buf)
            } else if n < 100_000_000 {
                let (n1, n2) = divmod(n, 10000);

                let l = write4(n1, buf);
                write4_pad(n2, buf.add(l));
                l + 4
            } else {
                let (n1, n2) = divmod(n, 100_000_000);

                let l = if n1 >= 10 {
                    ptr::copy_nonoverlapping(lookup(n1), buf, 2);
                    2
                } else {
                    *buf = n1 as u8 + 0x30;
                    1
                };

                write8_pad(n2, buf.add(l));
                l + 8
            }
        }

        pub unsafe fn write_u64(n: u64, buf: *mut u8) -> usize {
            if n < 10000 {
                write4(n as u32, buf)
            } else if n < 100_000_000 {
                let (n1, n2) = divmod(n, 10000);

                let l = write4(n1 as u32, buf);
                write4_pad(n2 as u32, buf.add(l));
                l + 4
            } else if n < 10_000_000_000_000_000 {
                let (n1, n2) = divmod(n, 100_000_000);
                let (n1, n2) = (n1 as u32, n2 as u32);

                let l = if n1 < 10000 {
                    write4(n1, buf)
                } else {
                    let (n11, n12) = divmod(n1, 10000);
                    let l = write4(n11, buf);
                    write4_pad(n12, buf.add(l));
                    l + 4
                };

                write8_pad(n2, buf.add(l));
                l + 8
            } else {
                let (n1, n2) = divmod(n, 10_000_000_000_000_000);
                let (n21, n22) = divmod(n2, 100_000_000);

                let l = write4(n1 as u32, buf);
                write8_pad(n21 as u32, buf.add(l));
                write8_pad(n22 as u32, buf.add(l + 8));
                l + 16
            }
        }
    }

    #[cfg(not(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2",
        not(miri),
    )))]
    use fallback::{write_u32, write_u64};

    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2",
        not(miri),
    ))]
    mod sse2 {
        #![allow(non_upper_case_globals)]

        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        use super::common::{divmod, lookup, write4, write4_pad};
        use core::ptr;

        #[repr(align(16))]
        struct Aligned<T>(T);

        impl<T> std::ops::Deref for Aligned<T> {
            type Target = T;

            #[inline]
            fn deref(&self) -> &T {
                &self.0
            }
        }

        const kDiv10000: u32 = 0xd1b71759;
        const kDivPowersVector: Aligned<[u16; 8]> =
            Aligned([8389, 5243, 13108, 32768, 8389, 5243, 13108, 32768]);
        const kShiftPowersVector: Aligned<[u16; 8]> = Aligned([
            1 << (16 - (23 + 2 - 16)),
            1 << (16 - (19 + 2 - 16)),
            1 << (16 - 1 - 2),
            1 << (15),
            1 << (16 - (23 + 2 - 16)),
            1 << (16 - (19 + 2 - 16)),
            1 << (16 - 1 - 2),
            1 << (15),
        ]);

        #[inline]
        unsafe fn convert_8digits_sse2(value: u32) -> __m128i {
            debug_assert!(value <= 99999999);

            // abcd, efgh = abcdefgh divmod 10000
            let abcdefgh = _mm_cvtsi32_si128(value as i32);
            let abcd = _mm_srli_epi64(
                _mm_mul_epu32(abcdefgh, _mm_set1_epi32(kDiv10000 as i32)),
                45,
            );
            let efgh = _mm_sub_epi32(abcdefgh, _mm_mul_epu32(abcd, _mm_set1_epi32(10000)));

            // v1 = [ abcd, efgh, 0, 0, 0, 0, 0, 0 ]
            let v1 = _mm_unpacklo_epi16(abcd, efgh);

            // v1a = v1 * 4 = [ abcd*4, efgh*4, 0, 0, 0, 0, 0, 0 ]
            let v1a = _mm_slli_epi64(v1, 2);

            // v2 = [abcd*4, abcd*4, abcd*4, abcd*4, efgh*4, efgh*4, efgh*4, efgh*4]
            let v2a = _mm_unpacklo_epi16(v1a, v1a);
            let v2 = _mm_unpacklo_epi32(v2a, v2a);

            // v4 = v2 div 10^3, 10^2, 10^1, 10^0 = [ a, ab, abc, abcd, e, ef, efg, efgh ]
            let v3 = _mm_mulhi_epu16(
                v2,
                _mm_load_si128(kDivPowersVector.as_ptr() as *const __m128i),
            );
            let v4 = _mm_mulhi_epu16(
                v3,
                _mm_load_si128(kShiftPowersVector.as_ptr() as *const __m128i),
            );

            // v5 = v4 * 10 = [ a0, ab0, abc0, abcd0, e0, ef0, efg0, efgh0 ]
            let v5 = _mm_mullo_epi16(v4, _mm_set1_epi16(10));

            // v6 = v5 << 16 = [ 0, a0, ab0, abc0, 0, e0, ef0, efg0 ]
            let v6 = _mm_slli_epi64(v5, 16);

            // v4 - v6 = { a, b, c, d, e, f, g, h }
            _mm_sub_epi16(v4, v6)
        }

        pub unsafe fn write_u32(n: u32, buf: *mut u8) -> usize {
            if n < 10000 {
                write4(n, buf)
            } else if n < 100_000_000 {
                let (n1, n2) = divmod(n, 10000);

                let l = write4(n1, buf);
                write4_pad(n2, buf.add(l));
                l + 4
            } else {
                let (n1, n2) = divmod(n, 100_000_000);

                let l = if n1 >= 10 {
                    ptr::copy_nonoverlapping(lookup(n1), buf, 2);
                    2
                } else {
                    *buf = n1 as u8 + 0x30;
                    1
                };

                let b = convert_8digits_sse2(n2);
                let ba = _mm_add_epi8(
                    _mm_packus_epi16(_mm_setzero_si128(), b),
                    _mm_set1_epi8(b'0' as i8),
                );
                let result = _mm_srli_si128(ba, 8);
                _mm_storel_epi64(buf.add(l) as *mut __m128i, result);

                l + 8
            }
        }

        pub unsafe fn write_u64(n: u64, buf: *mut u8) -> usize {
            if n < 10000 {
                write4(n as u32, buf)
            } else if n < 100_000_000 {
                let (n1, n2) = divmod(n as u32, 10000);

                let l = write4(n1, buf);
                write4_pad(n2, buf.add(l));
                l + 4
            } else if n < 10_000_000_000_000_000 {
                let (n1, n2) = divmod(n, 100_000_000);
                let (n1, n2) = (n1 as u32, n2 as u32);

                let l = if n1 < 10000 {
                    write4(n1, buf)
                } else {
                    let (n11, n12) = divmod(n1, 10000);
                    let l = write4(n11, buf);
                    write4_pad(n12, buf.add(l));
                    l + 4
                };

                let b = convert_8digits_sse2(n2);
                let ba = _mm_add_epi8(
                    _mm_packus_epi16(_mm_setzero_si128(), b),
                    _mm_set1_epi8(b'0' as i8),
                );
                let result = _mm_srli_si128(ba, 8);
                _mm_storel_epi64(buf.add(l) as *mut __m128i, result);

                l + 8
            } else {
                let (n1, n2) = divmod(n, 10_000_000_000_000_000);
                let l = write4(n1 as u32, buf);

                let (n21, n22) = divmod(n2, 100_000_000);

                let a0 = convert_8digits_sse2(n21 as u32);
                let a1 = convert_8digits_sse2(n22 as u32);

                // Convert to bytes, add '0'
                let va = _mm_add_epi8(_mm_packus_epi16(a0, a1), _mm_set1_epi8(b'0' as i8));
                _mm_storeu_si128(buf.add(l) as *mut __m128i, va);

                l + 16
            }
        }
    }

    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2",
        not(miri),
    ))]
    use sse2::{write_u32, write_u64};

    mod private {
        pub trait Sealed {}
    }

    /// An integer that can be written to pointer.
    pub trait Integer: private::Sealed {
        /// Maximum digits of the integer
        const MAX_LEN: usize;

        #[doc(hidden)]
        unsafe fn write_to(self, buf: *mut u8) -> usize;
    }

    macro_rules! impl_integer {
        ($unsigned:ty, $signed:ty, $conv:ty, $func:ident, $max_len:expr) => {
            impl private::Sealed for $unsigned {}
            impl private::Sealed for $signed {}

            impl Integer for $unsigned {
                const MAX_LEN: usize = $max_len;

                #[inline]
                unsafe fn write_to(self, buf: *mut u8) -> usize {
                    $func(self as $conv, buf)
                }
            }

            impl Integer for $signed {
                const MAX_LEN: usize = $max_len + 1;

                #[inline]
                unsafe fn write_to(self, mut buf: *mut u8) -> usize {
                    let mut n = self as $conv;
                    if self < 0 {
                        *buf = b'-';
                        buf = buf.add(1);
                        n = (!n).wrapping_add(1);
                    }

                    $func(n, buf) + (self < 0) as usize
                }
            }
        };
    }

    impl_integer!(u8, i8, u8, write_u8, 3);
    impl_integer!(u16, i16, u16, write_u16, 5);
    impl_integer!(u32, i32, u32, write_u32, 10);
    impl_integer!(u64, i64, u64, write_u64, 20);
    impl_integer!(u128, i128, u128, write_u128, 39);

    #[cfg(target_pointer_width = "16")]
    impl_integer!(usize, isize, u16, write_u16, 5);

    #[cfg(target_pointer_width = "32")]
    impl_integer!(usize, isize, u32, write_u32, 10);

    #[cfg(target_pointer_width = "64")]
    impl_integer!(usize, isize, u64, write_u64, 20);

    /// Write integer to the buffer pointer directly.
    ///
    /// This is fast operation, but does not check any safety.
    ///
    /// # Safety
    ///
    /// Behaviour is undefined if any of the following conditions are violated:
    ///
    /// - `buf` must point to sufficient
    /// [valid](https://doc.rust-lang.org/core/ptr/index.html#safety) bytes of memory to
    /// write `value`
    /// - `buf` must be aligned with `core::mem::align_of::<u8>()` bytes
    #[inline]
    pub unsafe fn write_to_ptr<V: Integer>(buf: *mut u8, value: V) -> usize {
        value.write_to(buf)
    }

    /// Write integer to `Vec<u8>`.
    ///
    /// Note that this function is safe because it checks the capacity of `Vec` and calls
    /// `Vec::reserve()` if the `Vec` doesn't have enough capacity.
    #[inline]
    pub fn write_to_vec<V: Integer>(buf: &mut Vec<u8>, value: V) {
        debug_assert!(buf.len() <= isize::MAX as usize);

        // benchmark result suggests that we gain more speed by manually checking the
        // buffer capacity and limits `reserve()` call
        if buf.len().wrapping_add(V::MAX_LEN) > buf.capacity() {
            buf.reserve(V::MAX_LEN);
        }

        unsafe {
            let l = value.write_to(buf.as_mut_ptr().add(buf.len()));
            buf.set_len(buf.len() + l);
        }
    }

    /// Write integer to `String`.
    ///
    /// Note that this function is safe because it checks the capacity of `String` and calls
    /// `String::reserve()` if the `String` doesn't have enough capacity.
    #[inline]
    pub fn write_to_string<V: Integer>(buf: &mut String, value: V) {
        unsafe { write_to_vec(buf.as_mut_vec(), value) };
    }

    /// Write integer to an `fmt::Write`
    ///
    /// Note that this operation may be slow because it writes the `value` to stack memory,
    /// and then copy the result into `writer`.
    ///
    /// This function is for compatibility with [itoa](https://docs.rs/itoa) crate and you
    /// should use `write_to_vec` or `write_to_string` if possible.
    #[inline]
    pub fn fmt<W: core::fmt::Write, V: Integer>(mut writer: W, value: V) -> core::fmt::Result {
        use core::mem::MaybeUninit;

        unsafe {
            let mut buf = [MaybeUninit::<u8>::uninit(); 40];
            let l = value.write_to(buf.as_mut_ptr() as *mut u8);
            let slc = core::slice::from_raw_parts(buf.as_ptr() as *const u8, l);
            writer.write_str(core::str::from_utf8_unchecked(slc))
        }
    }

    /// Write integer to an `io::Write`
    ///
    /// Note that this operation may be slow because it writes the `value` to stack memory,
    /// and then copy the result into `writer`.
    /// You should use `write_to_vec` or `write_to_string` if possible.
    ///
    /// This function is for compatibility with [itoa](https://docs.rs/itoa) crate and you
    /// should use `write_to_vec` or `write_to_string` if possible.
    #[inline]
    pub fn write<W: std::io::Write, V: Integer>(mut writer: W, value: V) -> std::io::Result<usize> {
        use core::mem::MaybeUninit;

        unsafe {
            let mut buf = [MaybeUninit::<u8>::uninit(); 40];
            let l = value.write_to(buf.as_mut_ptr() as *mut u8);
            let slc = core::slice::from_raw_parts(buf.as_ptr() as *const u8, l);
            writer.write(slc)
        }
    }
}

/// Size of the output buffer. Larger capacities don't seem to help, note that this is OS dependent.
const BUF_SIZE: usize = 32_768;

/// Custom buffer around a writer, more optimistic implementation of `std::io::BufWriter` .
///
/// Rationale:
///   - 1: Skip the Rust formatter easily
///   - 2: Only write to the writer when we explicitly want to (or when we drop the object)
///   - 3: Easy API in front of itoap and other dedicated formatters.
///   - 4: More straightforwards unchecked API.
///
/// If not writing inside of a loop, it may be better to just use the writeln! macro once and skip making this object.
pub struct CustomBufWriter<'a, W: std::io::Write> {
    writer: &'a mut W,
    buffer: [u8; BUF_SIZE],
    buffer_pointer: usize,
}

impl<'a, W: std::io::Write> CustomBufWriter<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self {
            writer,
            buffer: [0; BUF_SIZE],
            buffer_pointer: 0,
        }
    }

    /// transfer data from the buffer to the writer
    pub fn flush(&mut self) {
        unsafe {
            self.writer
                .write_all(self.buffer.get_unchecked(..self.buffer_pointer))
                .unwrap_unchecked();
            self.buffer_pointer = 0;
        }
    }

    /// call this right before writing if you expect that your write may overflow the buffer
    /// `block_size` = expected size you plan to write before checking again
    pub fn maybe_flush(&mut self, block_size: usize) {
        if self.buffer_pointer + block_size > BUF_SIZE {
            self.flush();
        }
    }

    /// unsafely add an integer, call `maybe_flush()` if you think it may overflow
    pub fn add_int(&mut self, integer: impl itoap::Integer) {
        unsafe {
            self.buffer_pointer += itoap::write_to_ptr(
                self.buffer
                    .get_unchecked_mut(self.buffer_pointer..)
                    .as_mut_ptr(),
                integer,
            );
        }
    }

    /// unsafely write one character to buffer
    /// `call maybe_flush()` first if you think you might overflow
    pub fn add_byte(&mut self, byte: u8) {
        unsafe {
            self.buffer
                .as_mut_ptr()
                .add(self.buffer_pointer)
                .write(byte);
            self.buffer_pointer += 1;
        }
    }

    /// unsafely write many characters to buffer
    /// call `maybe_flush()` first if you think you might overflow
    pub fn add_bytes(&mut self, buf: &[u8]) {
        unsafe {
            let len = buf.len();
            let ptr = self
                .buffer
                .get_unchecked_mut(self.buffer_pointer..)
                .as_mut_ptr();
            ptr.copy_from_nonoverlapping(buf.as_ptr(), len);
            self.buffer_pointer += len;
        }
    }
}

impl<'a, W: std::io::Write> Drop for CustomBufWriter<'a, W> {
    fn drop(&mut self) {
        self.flush();
    }
}

// optimistic (immediately works off byte ASCII characters without checking) raw byte parsing to integers/floats
// note that you should never pass an empty buffer slice to these functions

pub trait PosInt {
    /// quickly create an integer from a buffer, without checking any ASCII codes at all
    /// works in cases where you're guaranteed to get a positive integer
    /// (though you can use it with signed integers as well)
    fn to_posint(buf: &[u8]) -> Self;
}

macro_rules! impl_posint {
    (for $($t:ty),+) => {
        $(impl PosInt for $t {
            #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
            fn to_posint(buf: &[u8]) -> Self {
                unsafe {
                    buf.iter()
                        .map(|byte| (byte & 15) as $t)
                        .reduce(|acc, digit| acc * 10 + digit)
                        .unwrap_unchecked()
                }
            }
        })*
    }
}
impl_posint!(for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

pub trait AnyInt {
    /// quickly create an integer from a buffer, only checking the first character's ASCII code (for the minus sign).
    /// Use this if the constraints allow for both positive and negative values
    fn to_anyint(buf: &[u8]) -> Self;
}
macro_rules! impl_anyint {
    (for $($t:ty),+) => {
        $(impl AnyInt for $t {
            #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
            fn to_anyint(buf: &[u8]) -> Self {
                let (neg, digits) = match buf {
                    [b'-', digits @ ..] => (true, digits),
                    digits => (false, digits),
                };

                let result = unsafe {
                    digits.iter()
                        .map(|byte| (byte & 15) as $t)
                        .reduce(|acc, digit| acc * 10 + digit)
                        .unwrap_unchecked()
                };

                if neg {
                    -result
                } else {
                    result
                }
            }
        })*
    }
}
impl_anyint!(for i8, i16, i32, i64, i128, isize);

/// NOTE: This does NOT accept scientific notation or "inf/NaN"
pub trait AnyFloat {
    /// quickly create a floating point value from a buffer.
    /// we explicitly look for a possible negative sign and the floating point value
    /// otherwise we optimistically use the ASCII value as part of the floating point value
    fn to_float(buf: &[u8]) -> Self;
}
macro_rules! impl_float {
    (($t:ty, $ti:ty)) => {
        impl AnyFloat for $t {
            #[allow(
                clippy::cast_lossless,
                clippy::cast_possible_wrap,
                clippy::cast_precision_loss
            )]
            fn to_float(buf: &[u8]) -> Self {
                let (neg, first_digit, digits) = match buf {
                    [b'-', first, digits @ ..] => (true, first, digits),
                    [first, digits @ ..] => (false, first, digits),
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                let mut int_part = (first_digit & 0x0f) as $ti;
                let mut found_dot = false;
                let mut bytes = digits.iter().copied();

                for digit in bytes.by_ref() {
                    if digit == b'.' {
                        found_dot = true;
                        break;
                    }
                    int_part = int_part * 10 + ((digit & 15) as $ti);
                }

                if found_dot {
                    let mut result = int_part as $t;
                    let mut div: $t = 10.0;
                    for digit in bytes {
                        result += (digit & 15) as $t / div;
                        div *= 10.0;
                    }
                    if neg {
                        -result
                    } else {
                        result
                    }
                } else if neg {
                    -int_part as $t
                } else {
                    int_part as $t
                }
            }
        }
    };
}
impl_float!((f64, i64));
impl_float!((f32, i32));

// problem //

/// Given two numbers A and B, calculate their sum A+B.
///
/// <b>Input</b>
///
/// The first line of input consists of two space-separated numbers, A and B. Note that the numbers might be negative.
///
/// <b>Output</b>
///
/// Output a single integer A+B.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>−10<sup>6</sup> ≤ A,B ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    // in certain cases (such as when working with string problems), it may be convenient enough
    // to just make the "scan" input mutable and modify it directly.

    // get all "tokens" through using this spliterator
    // you can pass this around functions if you add it to a Box
    // type is Box<dyn Iterator<Item = &'a [u8]> + 'a>
    //
    // in cases where you only read in one value, you don't need an iterator, just use a slice to extract out the last value.
    let mut iter = scan.split(|n| *n <= b' ');
    let mut writer = CustomBufWriter::new(out);

    // actual domain problem logic begins here

    let a = unsafe { i32::to_anyint(iter.next().unwrap_unchecked()) };
    let b = unsafe { i32::to_anyint(iter.next().unwrap_unchecked()) };

    // NOTE: if you were calling this in a loop, you would probably want to call "writer.maybe_flush(9)" here
    // use 8 bytes for the maximum value (-2000000) and one byte for the newline
    writer.add_int(a + b);
    writer.add_byte(b'\n');
}

// entrypoints //

fn main() {
    // When reading from STDIN, it's fastest to use a vec with no capacity.
    let mut buf_str = vec![];
    std::io::stdin().lock().read_to_end(&mut buf_str).unwrap();
    let mut out = std::io::stdout().lock();
    solve(&buf_str, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let mut out = Vec::with_capacity(target.len());
        solve(input, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
3 5
";
        let target = b"\
8
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
-6 4
";
        let target = b"\
-2
";

        test(input, target);
    }
}
