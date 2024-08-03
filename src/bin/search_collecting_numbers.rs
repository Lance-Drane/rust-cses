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

/// You are given an array that contains each number between 1 ... n exactly once. Your task is to collect the numbers from 1 to n in increasing order.
///
/// On each round, you go through the array from left to right and collect as many numbers as possible. What will be the total number of rounds?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the array size.
///
/// The next line contains n integers x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>: the numbers in the array.
///
/// <b>Output</b>
///
/// Print one integer: the number of rounds.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let mut arr = vec![0; (n + 1) as usize];

    for (index, number) in (0..n).map(|idx| {
        (idx, unsafe {
            usize::to_posint(iter.next().unwrap_unchecked())
        })
    }) {
        unsafe {
            *arr.get_unchecked_mut(number) = index;
        }
    }

    writeln!(
        out,
        "{}",
        arr.windows(2).filter(|w| w[1] < w[0]).count() + 1
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
4 2 1 5 3
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_asc() {
        let input = b"\
10
1 2 3 4 5 6 7 8 9 10
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_desc() {
        let input = b"\
10
10 9 8 7 6 5 4 3 2 1
";
        let target = b"\
10
";

        test(input, target);
    }

    #[test]
    fn test_one() {
        let input = b"\
1
1
";
        let target = b"\
1
";

        test(input, target);
    }
}
