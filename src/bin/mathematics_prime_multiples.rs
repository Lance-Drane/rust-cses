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

/// You are given k distinct prime numbers a<sub>1</sub>,a<sub>2</sub>,...,a<sub>k</sub> and an integer n.
///
/// Your task is to calculate how many of the first n positive integers are divisible by at least one of the given prime numbers.
///
/// <b>Input</b>
///
/// The first input line has two integers n and k.
///
/// The second line has k prime numbers a<sub>1</sub>,a<sub>2</sub>,...,a<sub>k</sub>.
///
/// <b>Output</b>
///
/// Print one integer: the number integers within the interval 1,2,...,n that are divisible by at least one of the prime numbers.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>18</sup></li>
/// <li>1 ≤ k ≤ 20</li>
/// <li>2 ≤ a<sub>i</sub> ≤ n</li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let end = unsafe { i64::to_posint(iter.next().unwrap_unchecked()) };
    let k = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
    let primes: Vec<i64> = (0..k)
        .map(|_| unsafe { i64::to_posint(iter.next().unwrap_unchecked()) })
        .collect();

    writeln!(out, "{}", recurse(&primes, k, 0, 0, end)).unwrap();
}

// recursion is actually faster than the bit manipulation approach, despite using more memory but having the same time complexity
fn recurse(primes: &[i64], k: usize, i: usize, count: u8, end: i64) -> i64 {
    if end == 0 {
        0
    } else if i == k {
        match count {
            0 => 0,
            _ if count & 1 == 1 => end,
            _ => -end,
        }
    } else {
        recurse(
            primes,
            k,
            i + 1,
            count + 1,
            end / unsafe { primes.get_unchecked(i) },
        ) + recurse(primes, k, i + 1, count, end)
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
20 2
2 5
";
        let target = b"\
12
";

        test(input, target);
    }

    #[test]
    fn test_three_primes() {
        let input = b"\
60 3
2 5 3
";
        let target = b"\
44
";

        test(input, target);
    }

    #[test]
    fn test_three_primes_two() {
        let input = b"\
59 3
2 5 3
";
        let target = b"\
43
";

        test(input, target);
    }

    #[test]
    fn test_bigger_factors() {
        let input = b"\
3000 4
17 71 37 11
";
        let target = b"\
538
";

        test(input, target);
    }

    #[test]
    fn test_large_primes() {
        let input = b"\
999999999999999999 20
24929660627620033 16706748220911473 2021305013539879 4901318384837333 12211 127819 1514541599759 9590976029 27061247885314589 17451648198763151 6763 2579 11 101 7 3 522661842626879699 459279887912130907 15396727 61953589
";
        let target = b"\
485984468367181881
";

        test(input, target);
    }
}
