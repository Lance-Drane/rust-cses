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

const DEFAULT: u32 = 1_000_000_007;

/// Consider a money system consisting of n coins. Each coin has a positive integer value. Your task is to produce a sum of money x using the available coins in such a way that the number of coins is minimal.
///
/// For example, if the coins are {1,5,7} and the desired sum is 11, an optimal solution is 5+5+1 which requires 3 coins.
///
/// <b>Input</b>
///
/// The first input line has two integers n and x: the number of coins and the desired sum of money.
///
/// The second line has n distinct integers c1, c2, ..., c<sub>n</sub>: the value of each coin.
///
/// <b>Output</b>
///
/// Print one integer: the minimum number of coins. If it is not possible to produce the desired sum, print −1.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 100</li>
/// <li>1 ≤ x ≤ 10<sup>6</sup></li>
/// <li>1 ≤ c<sub>i</sub> ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let capacity = unsafe { u8::to_posint(iter.next().unwrap_unchecked()) };
    let target = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };

    let mut cache = vec![DEFAULT; target + 1];
    cache[0] = 0;

    for coin in (0..capacity).map(|_| unsafe { usize::to_posint(iter.next().unwrap_unchecked()) }) {
        let mut cache_cp = cache.as_mut_slice();
        while cache_cp.len() > coin {
            let (left, right) = cache_cp.split_at_mut(coin);
            for (a, b) in left.iter().zip(right.iter_mut()) {
                *b = (*a + 1).min(*b);
            }
            cache_cp = right;
        }
    }

    match unsafe { *cache.get_unchecked(target) } {
        DEFAULT => {
            out.write_all(b"-1\n").unwrap();
        }
        n => {
            writeln!(out, "{n}").unwrap();
        }
    }
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
3 11
1 5 7
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_unreachable() {
        let input = b"\
3 11
2 4 6
";
        let target = b"\
-1
";

        test(input, target);
    }
}
