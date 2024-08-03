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

/// There are n sticks with some lengths. Your task is to modify the sticks so that each stick has the same length.
///
/// You can either lengthen and shorten each stick. Both operations cost x where x is the difference between the new and original length.
///
/// What is the minimum total cost?
///
/// <b>Input</b>
///
/// The first input line contains an integer n: the number of sticks.
///
/// Then there are n integers: p<sub>1</sub>,p<sub>2</sub>,...,p<sub>n</sub>: the lengths of the sticks.
///
/// <b>Output</b>
///
/// Print one integer: the minimum total cost.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n,m ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ p<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
    let mut sticks: Vec<i64> = (0..n)
        .map(|_| unsafe { i64::to_posint(iter.next().unwrap_unchecked()) })
        .collect();
    sticks.select_nth_unstable(n >> 1);
    let target = unsafe { *sticks.get_unchecked(n >> 1) };

    writeln!(
        out,
        "{}",
        sticks
            .into_iter()
            .fold(0, |acc, curr| acc + (curr - target).abs())
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
2 3 1 5 2
";
        let target = b"\
5
";

        test(input, target);
    }

    #[test]
    fn test_first_powers_of_10() {
        let input = b"\
4
1 10 100 1000
";
        let target = b"\
1089
";

        test(input, target);
    }
}
