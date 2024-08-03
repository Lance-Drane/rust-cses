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

/// Find the middle element when the numbers in an n * n multiplication table are sorted in increasing order. It is assumed that n is odd.
///
/// For example, the 3 * 3 multiplication table is as follows:
///
/// <code>
/// 1 2 3
/// 2 4 6
/// 3 6 9
/// </code>
///
/// The numbers in increasing order are [1,2,2,3,3,4,6,6,9], so the answer is 3.
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print one integer: the answer to the task.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n < 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let n = u64::to_posint(&scan[..scan.len() - 1]);

    let mut low = 1;
    let median = (n * n + 1) >> 1;
    let mut high = median; // skip an iteration, as over half the numbers are always smaller than n * n / 2

    while low < high {
        let mid = (low + high + 1) >> 1;

        // get number of values in each row that are less than the midpoint
        let mut number_smaller_in_row = n;
        let mut subsum = n;
        let mut number_smaller = 0;

        for row in 1..(n + 1) {
            while subsum >= mid {
                number_smaller_in_row -= 1;
                subsum -= row;
            }
            number_smaller += number_smaller_in_row;
            subsum += number_smaller_in_row;
        }

        if number_smaller >= median {
            high = mid - 1;
        } else {
            low = mid;
        }
    }
    writeln!(out, "{high}").unwrap();
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
3
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
5
";
        let target = b"\
8
";

        test(input, target);
    }

    #[test]
    fn test_low() {
        let input = b"\
1
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_high() {
        let input = b"\
999999
";
        let target = b"\
186682420008
";

        test(input, target);
    }
}
