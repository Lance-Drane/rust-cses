// I/O boilerplate //

use std::fs::File;
use std::io::Read;

/// https://github.com/Kogia-sima/itoap
#[allow(clippy::pedantic)]
pub mod itoap {
    mod common {
        use core::ptr;

        const DEC_DIGITS_LUT: &[u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

        #[inline]
        pub unsafe fn lookup<T: Into<u64>>(idx: T) -> *const u8 {
            DEC_DIGITS_LUT.as_ptr().add((idx.into() as usize) << 1)
        }

        pub unsafe fn write_u8(n: u8, buf: *mut u8) -> usize {
            if n < 10 {
                *buf = n + 0x30;
                1
            } else {
                ptr::copy_nonoverlapping(lookup(n), buf, 2);
                2
            }
        }
    }
    use common::*;
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

    /// # Safety
    ///
    /// "buf" should have sufficient memory to write "value"
    #[inline]
    pub unsafe fn write_to_ptr<V: Integer>(buf: *mut u8, value: V) -> usize {
        value.write_to(buf)
    }
}

#[cfg(unix)]
fn stdin_raw() -> File {
    use std::os::fd::FromRawFd;

    unsafe { File::from_raw_fd(0) }
}

#[cfg(unix)]
fn stdout_raw() -> File {
    use std::os::fd::FromRawFd;

    unsafe { File::from_raw_fd(1) }
}

#[cfg(windows)]
fn stdin_raw() -> File {
    use std::os::windows::io::{AsRawHandle, FromRawHandle};

    unsafe { File::from_raw_handle(std::io::stdin().as_raw_handle()) }
}

#[cfg(windows)]
fn stdout_raw() -> File {
    use std::os::windows::io::{AsRawHandle, FromRawHandle};

    unsafe { File::from_raw_handle(std::io::stdout().as_raw_handle()) }
}

// problem //

const COLUMNS: u8 = 8;

/// Your task is to place eight queens on a chessboard so that no two queens are attacking each other. As an additional challenge, each square is either free or reserved, and you can only place queens on the free squares. However, the reserved squares do not prevent queens from attacking each other.
///
/// How many possible ways are there to place the queens?
///
/// <b>Input</b>
///
/// The input has eight lines, and each of them has eight characters. Each square is either free (.) or reserved (*).
///
/// <b>Output</b>
///
/// Print one integer: the number of ways you can place the queens.
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let mut chessboard = 0;
    for (row_idx, row) in (0..COLUMNS).map(|idx| (idx, unsafe { iter.next().unwrap_unchecked() })) {
        for (_, col_idx) in row.iter().zip(0..).filter(|(chr, _)| **chr == b'*') {
            chessboard |= 1 << (row_idx * COLUMNS + col_idx);
        }
    }

    let mut counter = 0;

    recurse(0, &mut counter, chessboard, 0, 0, 0);

    let mut out_buf = [0_u8; 2];
    unsafe {
        let ptr = itoap::write_to_ptr(out_buf.as_mut_ptr(), counter);
        out.write_all(out_buf.get_unchecked(..ptr)).unwrap();
    }
}

fn recurse(
    row: u8,
    counter: &mut u8,
    chessboard: u64,
    visited_cols: u8,
    visited_diag_tl_br: u16,
    visited_diag_tr_bl: u16,
) {
    if row == COLUMNS {
        *counter += 1;
        return;
    }
    for column in 0..COLUMNS {
        if (visited_cols >> column) & 1 == 1
            || (visited_diag_tl_br >> (row + column)) & 1 == 1
            || (visited_diag_tr_bl >> (COLUMNS + column - row - 1)) & 1 == 1
            || (chessboard >> (COLUMNS * row + column)) & 1 == 1
        {
            continue;
        }

        recurse(
            row + 1,
            counter,
            chessboard,
            visited_cols | (1 << column),
            visited_diag_tl_br | (1 << (row + column)),
            visited_diag_tr_bl | (1 << (COLUMNS + column - row - 1)),
        );
    }
}

// entrypoints //

fn main() {
    let mut buf_str = vec![];
    let mut reader = stdin_raw();
    reader.read_to_end(&mut buf_str).unwrap();
    let mut out = stdout_raw();
    solve(&buf_str, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &str) {
        let mut out = Vec::with_capacity(target.len());
        solve(input, &mut out);

        assert_eq!(String::from_utf8(out).unwrap(), target);
    }

    #[test]
    fn test_example() {
        let input = b"\
........
........
..*.....
........
........
.....**.
...*....
........
";
        let target = "\
65";

        test(input, target);
    }

    #[test]
    fn test_no_free_spaces() {
        let input = b"\
********
********
********
********
********
********
********
********
";
        let target = "\
0";

        test(input, target);
    }

    #[test]
    fn test_all_free_spaces() {
        let input = b"\
........
........
........
........
........
........
........
........
";
        let target = "\
92";

        test(input, target);
    }
}
