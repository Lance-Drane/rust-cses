// I/O boilerplate //

use std::fs::File;

/// https://github.com/Kogia-sima/itoap
#[allow(clippy::pedantic)]
pub mod itoap {
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
            let quot = x / y;
            let rem = x - quot * y;
            (quot, rem)
        }

        #[inline]
        pub unsafe fn lookup<T: Into<u64>>(idx: T) -> *const u8 {
            DEC_DIGITS_LUT.as_ptr().add((idx.into() as usize) << 1)
        }

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

        #[inline]
        fn u128_mulhi(x: u128, y: u128) -> u128 {
            let x_lo = x as u64;
            let x_hi = (x >> 64) as u64;
            let y_lo = y as u64;
            let y_hi = (y >> 64) as u64;

            let carry = (x_lo as u128 * y_lo as u128) >> 64;
            let m = x_lo as u128 * y_hi as u128 + carry;
            let high1 = m >> 64;

            let m_lo = m as u64;
            let high2 = (x_hi as u128 * y_lo as u128 + m_lo as u128) >> 64;

            x_hi as u128 * y_hi as u128 + high1 + high2
        }

        unsafe fn write_u128_big(mut n: u128, mut buf: *mut u8) -> usize {
            const DIV_FACTOR: u128 = 76624777043294442917917351357515459181;
            const DIV_SHIFT: u32 = 51;
            const POW_10_8: u64 = 100000000;
            const POW_10_16: u64 = 10000000000000000;

            debug_assert!(n > u64::MAX as u128);

            let mut result = [0u32; 5];

            {
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

            let abcdefgh = _mm_cvtsi32_si128(value as i32);
            let abcd = _mm_srli_epi64(
                _mm_mul_epu32(abcdefgh, _mm_set1_epi32(kDiv10000 as i32)),
                45,
            );
            let efgh = _mm_sub_epi32(abcdefgh, _mm_mul_epu32(abcd, _mm_set1_epi32(10000)));

            let v1 = _mm_unpacklo_epi16(abcd, efgh);

            let v1a = _mm_slli_epi64(v1, 2);

            let v2a = _mm_unpacklo_epi16(v1a, v1a);
            let v2 = _mm_unpacklo_epi32(v2a, v2a);

            let v3 = _mm_mulhi_epu16(
                v2,
                _mm_load_si128(kDivPowersVector.as_ptr() as *const __m128i),
            );
            let v4 = _mm_mulhi_epu16(
                v3,
                _mm_load_si128(kShiftPowersVector.as_ptr() as *const __m128i),
            );

            let v5 = _mm_mullo_epi16(v4, _mm_set1_epi16(10));

            let v6 = _mm_slli_epi64(v5, 16);

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

    pub trait Integer: private::Sealed {
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

    /// # Safety
    ///
    /// "buf" should have sufficient memory to write "value"
    #[inline]
    pub unsafe fn write_to_ptr<V: Integer>(buf: *mut u8, value: V) -> usize {
        value.write_to(buf)
    }
}

const BUF_SIZE: usize = 13;

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

    pub fn flush(&mut self) {
        unsafe {
            self.writer
                .write_all(self.buffer.get_unchecked(..self.buffer_pointer))
                .unwrap_unchecked();
            self.buffer_pointer = 0;
        }
    }

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

    pub fn add_byte(&mut self, byte: u8) {
        unsafe {
            self.buffer
                .as_mut_ptr()
                .add(self.buffer_pointer)
                .write(byte);
            self.buffer_pointer += 1;
        }
    }

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

#[cfg(unix)]
fn stdout_raw() -> File {
    use std::os::fd::FromRawFd;

    unsafe { File::from_raw_fd(1) }
}

#[cfg(windows)]
fn stdout_raw() -> File {
    use std::os::windows::io::{AsRawHandle, FromRawHandle};

    unsafe { File::from_raw_handle(std::io::stdout().as_raw_handle()) }
}

// problem //

/// There is a hidden integer x. Your task is to find the value of x.
///
/// To do this, you can ask questions: you can choose an integer y and you will be told if y < x.
///
/// <b>Interaction</b>
///
/// This is an interactive problem. Your code will interact with the grader using standard input and output. You can start asking questions right away.
///
/// On your turn, you can print one of the following:
///
/// <ul>
/// <li>"? y", where 1 ≤ y ≤ 10^9: ask if y < x. The grader will return YES if y < x and NO otherwise.</li>
/// <li>"! x": report that the hidden integer is x. Your program must terminate after this.</li>
/// </ul>
///
/// Each line should be followed by a line break. You must make sure the output gets flushed after printing each line.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ x ≤ 10<sup>9</sup></li>
/// <li>you can ask at most 30 questions of type "?"</li>
/// </ul>
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut R, out: &mut W) {
    let mut buf_str = Vec::with_capacity(4);
    let mut writer = CustomBufWriter::new(out);

    let mut l = 1;
    let mut r = 1_000_000_000;

    let mut ans = 0;

    while l <= r {
        let mid = (l + r) >> 1;

        writer.add_bytes(b"? ");
        writer.add_int(mid);
        writer.add_byte(b'\n');
        writer.flush();

        read.read_until(b'\n', &mut buf_str).unwrap();

        if buf_str[0] == b'Y' {
            l = mid + 1;
        } else {
            r = mid - 1;
            ans = mid;
        }

        buf_str.clear();
    }

    writer.add_bytes(b"! ");
    writer.add_int(ans);
    writer.add_byte(b'\n');
    writer.flush();
}

// entrypoints //

fn main() {
    let mut input = std::io::stdin().lock();
    let mut out = stdout_raw();
    solve(&mut input, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(mut input: &[u8], target: &[u8]) {
        let mut out = Vec::with_capacity(target.len());
        solve(&mut input, &mut out);

        assert_eq!(out, target);
    }

    // NOTE: while CSES allows for any valid solution, we have a specific implementation.
    // Characters which come first in the alphabet come in at the beginning and end of the string
    // Characters at the end of the alphabet come in at the middle of the string

    #[test]
    fn test_middle() {
        let input = b"\
NO
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
";
        let target = b"\
? 500000000
? 250000000
? 375000000
? 437500000
? 468750000
? 484375000
? 492187500
? 496093750
? 498046875
? 499023437
? 499511718
? 499755859
? 499877929
? 499938964
? 499969482
? 499984741
? 499992370
? 499996185
? 499998092
? 499999046
? 499999523
? 499999761
? 499999880
? 499999940
? 499999970
? 499999985
? 499999992
? 499999996
? 499999998
? 499999999
! 500000000
";

        test(input, target);
    }

    #[test]
    fn test_low() {
        let input = b"\
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
NO
";
        let target = b"\
? 500000000
? 250000000
? 125000000
? 62500000
? 31250000
? 15625000
? 7812500
? 3906250
? 1953125
? 976562
? 488281
? 244140
? 122070
? 61035
? 30517
? 15258
? 7629
? 3814
? 1907
? 953
? 476
? 238
? 119
? 59
? 29
? 14
? 7
? 3
? 1
! 1
";

        test(input, target);
    }

    #[test]
    fn test_high() {
        let input = b"\
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
YES
NO
";
        let target = b"\
? 500000000
? 750000000
? 875000000
? 937500000
? 968750000
? 984375000
? 992187500
? 996093750
? 998046875
? 999023438
? 999511719
? 999755860
? 999877930
? 999938965
? 999969483
? 999984742
? 999992371
? 999996186
? 999998093
? 999999047
? 999999524
? 999999762
? 999999881
? 999999941
? 999999971
? 999999986
? 999999993
? 999999997
? 999999999
? 1000000000
! 1000000000
";

        test(input, target);
    }
}
