// I/O boilerplate

use std::io::Read;

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

// problem //

/// Given an array of n integers, your task is to find the maximum sum of values in a contiguous, nonempty subarray.
///
/// <b>Input</b>
///
/// The first input line has an integer n: the size of the array.
///
/// The second line has n integers x<sub>1</sub>, x<sub>2</sub>, ..., x<sub>n</sub>: the array values.
///
/// <b>Output</b>
///
/// Print one integer: the maximum subarray sum.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>−10<sup>9</sup> ≤ x<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let size = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let base = unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) };
    writeln!(
        out,
        "{}",
        (1..size)
            .map(|_| unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) })
            .fold((base, base), |(best, current), ele| {
                let next = ele.max(ele + current);
                (best.max(next), next)
            })
            .0
    )
    .unwrap();
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
8
-1 3 -2 5 3 -5 2 2
";
        let target = b"\
9
";

        test(input, target);
    }

    #[test]
    fn test_one_negative() {
        let input = b"\
1
-2
";
        let target = b"\
-2
";

        test(input, target);
    }

    #[test]
    fn test_many_negative() {
        let input = b"\
5
-1 -1 -1 -1 -2
";
        let target = b"\
-1
";

        test(input, target);
    }

    #[test]
    fn test_unusual() {
        let input = b"\
10
24 7 -27 17 -67 65 -23 58 85 -39
";
        let target = b"\
185
";

        test(input, target);
    }
}
