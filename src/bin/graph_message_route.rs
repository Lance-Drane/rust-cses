// I/O boilerplate //

use std::io::Read;

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

const BUF_SIZE: usize = 32_768;

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

    pub fn maybe_flush(&mut self, block_size: usize) {
        if self.buffer_pointer + block_size > BUF_SIZE {
            self.flush();
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

impl<'a, W: std::io::Write> Drop for CustomBufWriter<'a, W> {
    fn drop(&mut self) {
        self.flush();
    }
}

pub trait PosInt {
    fn to_posint(buf: &[u8]) -> Self;
}

macro_rules! impl_int {
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
impl_int!(for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

// problem //

use std::collections::VecDeque;

/// credit: `EbTech`
/// A compact graph representation. Edges are numbered in order of insertion.
/// Each adjacency list consists of all edges pointing out from a given vertex.
pub struct Graph {
    /// Maps a vertex id to the first edge in its adjacency list.
    first: Vec<Option<usize>>,
    /// Maps an edge id to the next edge in the same adjacency list.
    next: Vec<Option<usize>>,
    /// Maps an edge id to the vertex that it points to.
    endp: Vec<usize>,
}

impl Graph {
    /// Initializes a graph with vmax vertices and no edges. To reduce
    /// unnecessary allocations, `emax_hint` should be close to the number of
    /// edges that will be inserted.
    #[must_use]
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            first: vec![None; vmax],
            next: Vec::with_capacity(emax_hint),
            endp: Vec::with_capacity(emax_hint),
        }
    }

    /// Returns the number of vertices.
    #[must_use]
    pub fn num_v(&self) -> usize {
        self.first.len()
    }

    /// Returns the number of edges, double-counting undirected edges.
    #[must_use]
    pub fn num_e(&self) -> usize {
        self.endp.len()
    }

    /// Adds a directed edge from u to v.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.next.push(self.first[u]);
        self.first[u] = Some(self.num_e());
        self.endp.push(v);
    }

    /// An undirected edge is two directed edges. If edges are added only via
    /// this funcion, the reverse of any edge e can be found at e^1.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }

    /// Gets vertex u's adjacency list.
    #[must_use]
    pub fn adj_list(&self, u: usize) -> AdjListIterator {
        AdjListIterator {
            graph: self,
            next_e: self.first[u],
        }
    }
}

/// An iterator for convenient adjacency list traversal.
pub struct AdjListIterator<'a> {
    graph: &'a Graph,
    next_e: Option<usize>,
}

impl<'a> Iterator for AdjListIterator<'a> {
    type Item = (usize, usize);

    /// Produces an outgoing edge and vertex.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_e.map(|e| {
            let v = self.graph.endp[e];
            self.next_e = self.graph.next[e];
            (e, v)
        })
    }
}

/// Syrjälä's network has n computers and m connections. Your task is to find out if Uolevi can send a message to Maija, and if it is possible, what is the minimum number of computers on such a route.
///
/// <b>Input</b>
///
/// The first input line has two integers n and m: the number of computers and connections. The computers are numbered 1,2,...,n. Uolevi's computer is 1 and Maija's computer is n.
///
/// Then, there are m lines describing the connections. Each line has two integers a and b: there is a connection between those computers.
///
/// Every connection is between two different computers, and there is at most one connection between any two computers.
///
/// <b>Output</b>
///
/// If it is possible to send a message, first print k: the minimum number of computers on a valid route. After this, print an example of such a route. You can print any valid solution.
///
/// If there are no routes, print "IMPOSSIBLE".
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>2 ≤ n ≤ 10<sup>5</sup></li>
/// <li>1 ≤ m ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a,b ≤ n </li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');
    let mut writer = CustomBufWriter::new(out);

    let n_nodes = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
    let n_connections = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
    let mut graph = Graph::new(n_nodes + 1, n_connections << 1);

    for _ in 0..n_connections {
        let a = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
        let b = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
        graph.add_undirected_edge(a, b);
    }

    // if value in "parents" is not 0, we have already found its parent
    let mut parents = vec![0; n_nodes + 1];
    let mut queue = VecDeque::with_capacity(n_nodes);
    queue.push_back((0_u32, 1));

    let mut additional_comps = usize::MAX;

    'qloop: while let Some((depth, node)) = queue.pop_front() {
        for (_, vertex) in graph.adj_list(node) {
            if parents[vertex] == 0 {
                parents[vertex] = node;
                if vertex == n_nodes {
                    additional_comps = depth as usize;
                    break 'qloop;
                }
                queue.push_back((depth + 1, vertex));
            }
        }
    }

    if additional_comps == usize::MAX {
        writer.add_bytes(b"IMPOSSIBLE\n");
        return;
    }

    let mut answer = vec![0; additional_comps];
    let mut parent = *parents.last().unwrap();
    for ans in answer.iter_mut().rev() {
        *ans = parent;
        parent = parents[parent];
    }

    writer.add_int(additional_comps + 2);
    writer.add_bytes(b"\n1 ");
    for a in answer {
        writer.maybe_flush(7);
        writer.add_int(a);
        writer.add_byte(b' ');
    }
    writer.maybe_flush(7);
    writer.add_int(n_nodes);
    writer.add_byte(b'\n');
}

// entrypoints //

fn main() {
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
5 5
1 2
1 3
1 4
2 3
5 4
";
        let target = b"\
3
1 4 5
";

        test(input, target);
    }

    #[test]
    fn test_empty() {
        let input = b"\
4 2
2 3
3 4
";
        let target = b"\
IMPOSSIBLE
";

        test(input, target);
    }

    #[test]
    fn test_immediate() {
        let input = b"\
2 1
1 2
";
        let target = b"\
2
1 2
";

        test(input, target);
    }
}
