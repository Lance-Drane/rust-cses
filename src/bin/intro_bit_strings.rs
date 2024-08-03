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

/// Your task is to calculate the number of bit strings of length n.
///
/// For example, if n=3, the correct answer is 8, because the possible bit strings are 000, 001, 010, 011, 100, 101, 110, and 111.
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print the result modulo 10<sup>9</sup>+7.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut exponent = u32::to_posint(&scan[..scan.len() - 1]);

    let mut goal: u64 = 1;
    let mut base: u64 = 2;

    while exponent != 0 {
        if exponent & 1 == 1 {
            goal = goal * base % MODULO;
        }
        base = base * base % MODULO;
        exponent >>= 1;
    }

    writeln!(out, "{goal}").unwrap();
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
8
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
255
";
        let target = b"\
396422633
";

        test(input, target);
    }

    #[test]
    fn test_smallest() {
        let input = b"\
1
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_largest() {
        let input = b"\
1000000
";
        let target = b"\
235042059
";

        test(input, target);
    }

    #[test]
    fn test_example_3() {
        let input = b"\
665215
";
        let target = b"\
976383320
";

        test(input, target);
    }
}
