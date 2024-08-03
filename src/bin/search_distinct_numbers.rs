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

use std::collections::HashSet;

/// You are given a list of n integers, and your task is to calculate the number of distinct values in the list.
///
/// <b>Input</b>
///
///The first input line has an integer n: the number of values.
///
/// The second line has n integers x1,x2,...,x<sub>n</sub>.
///
/// <b>Output</b>
///
/// Print one integer: the number of distinct values.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ x<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');
    let n = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };

    writeln!(
        out,
        "{}",
        (0..n)
            .map(|_| unsafe { u32::to_posint(iter.next().unwrap_unchecked()) })
            .collect::<HashSet<_>>()
            .len()
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
2 3 2 2 3
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_all_unique() {
        let input = b"\
4
3 2 1 1000000000
";
        let target = b"\
4
";

        test(input, target);
    }

    #[test]
    fn test_all_same() {
        let input = b"\
6
6 6 6 6 6 6
";
        let target = b"\
1
";

        test(input, target);
    }
}
