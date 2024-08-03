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

// problem //

/// You have n coins with positive integer values. What is the smallest sum you cannot create using a subset of the coins?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of coins.
///
/// The second line has n integers x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>: the value of each coin.
///
/// <b>Output</b>
///
/// Print one integer: the smallest coin sum..
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
    let mut coins: Vec<u64> = (0..n)
        .map(|_| unsafe { u64::to_posint(iter.next().unwrap_unchecked()) })
        .collect();
    coins.sort_unstable();

    let mut sum = 1;

    for coin in coins {
        if coin > sum {
            break;
        }
        sum += coin;
    }
    writeln!(out, "{sum}").unwrap();
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
2 9 1 2 7
";
        let target = b"\
6
";

        test(input, target);
    }
}
