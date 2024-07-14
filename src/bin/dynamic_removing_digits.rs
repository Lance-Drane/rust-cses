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

/// You are given an integer n. On each step, you may subtract one of the digits from the number.
///
/// How many steps are required to make the number equal to 0?
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print one integer: the minimum number of steps.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
///
/// For "27", one optimal solution is 27→20→18→10→9→0.
///
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut target = u32::to_posint(&scan[..scan.len() - 1]);
    let mut counter = 0_u32;

    while target != 0 {
        let mut quotient = target;
        let mut remainder = 0;

        while quotient != 0 {
            remainder = remainder.max(quotient % 10);
            quotient /= 10;
        }

        target -= remainder;
        counter += 1;
    }

    writeln!(out, "{counter}").unwrap();
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
27
";
        let target = b"\
5
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
100
";
        let target = b"\
17
";

        test(input, target);
    }

    #[test]
    fn test_example_3() {
        let input = b"\
99
";
        let target = b"\
16
";

        test(input, target);
    }

    #[test]
    fn test_example_4() {
        let input = b"\
1000000
";
        let target = b"\
128207
";

        test(input, target);
    }
}
