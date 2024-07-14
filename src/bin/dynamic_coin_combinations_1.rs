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

const MODULO: u64 = 1_000_000_007;

/// Consider a money system consisting of n coins. Each coin has a positive integer value. Your task is to calculate the number of distinct ways you can produce a money sum x using the available coins.
///
/// For example, if the coins are {2,3,5} and the desired sum is 9, there are 8 ways:
/// <ul>
/// <li>2+2+5</li>
/// <li>2+5+2</li>
/// <li>5+2+2</li>
/// <li>3+3+3</li>
/// <li>2+2+2+3</li>
/// <li>2+2+3+2</li>
/// <li>2+3+2+2</li>
/// <li>3+2+2+2</li>
/// </ul>
///
/// <b>Input</b>
///
/// The first input line has two integers n and x: the number of coins and the desired sum of money.
///
/// The second line has n distinct integers c1, c2, ..., c<sub>n</sub>: the value of each coin.
///
/// <b>Output</b>
///
/// Print one integer: the number of ways modulo 10<sup>9</sup> + 7.
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
    let mut coins: Vec<usize> = (0..capacity)
        .map(|_| unsafe { usize::to_posint(iter.next().unwrap_unchecked()) })
        .collect();
    coins.sort_unstable_by(|a, b| b.cmp(a));
    let mut cache = vec![0_u64; target + 1];
    cache[0] = 1;

    for idx in *coins.last().unwrap()..=target {
        cache[idx] = coins
            .iter()
            .skip_while(|coin| **coin > idx)
            .map(|coin| cache[idx - coin])
            .sum::<u64>()
            % MODULO;
    }

    writeln!(out, "{}", cache[target]).unwrap();
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
3 9
2 3 5
";
        let target = b"\
8
";

        test(input, target);
    }

    #[test]
    fn test_overlap() {
        let input = b"\
12 74057
1 2 74012 74005 74003 73999 73998 73997 73996 73995 73994 73993
";
        let target = b"\
0
";

        test(input, target);
    }

    #[test]
    fn test_overflow() {
        let input = b"\
100 1000
389 101 552 795 876 269 887 103 154 689 542 920 128 541 44 657 310 531 656 567 386 536 900 374 929 505 255 376 384 709 311 404 699 86 512 518 321 916 408 935 568 662 731 933 238 331 833 235 423 352 205 669 413 152 695 713 878 614 109 164 387 3 287 823 485 716 556 323 924 57 35 705 643 77 200 944 768 490 589 339 701 190 714 349 252 303 74 526 186 644 453 251 429 170 777 216 22 825 514 379
";
        let target = b"\
834994040
";

        test(input, target);
    }
}
