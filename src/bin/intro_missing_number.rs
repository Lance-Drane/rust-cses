// I/O boilerplate //

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

// problem //

/// You are given all numbers between 1,2,…,n except one. Your task is to find the missing number.
///
/// <b>Input</b>
///
/// The first input line contains an integer n.
///
/// The second line contains n−1 numbers. Each number is distinct and between 1 and n (inclusive).
///
/// <b>Output</b>
///
/// Print the missing number.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>2 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let upper_bound = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    writeln!(
        out,
        "{}",
        (1..upper_bound).fold(upper_bound, |acc, num| acc
            ^ num
            ^ unsafe { u32::to_posint(iter.next().unwrap_unchecked()) })
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
5
2 3 1 5
";
        let target = b"\
4
";

        test(input, target);
    }
}
