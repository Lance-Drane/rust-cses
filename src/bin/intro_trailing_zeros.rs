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

/// Your task is to calculate the number of trailing zeros in the factorial n!.
///
/// For example, 20!=2432902008176640000 and it has 4 trailing zeros.
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print the number of trailing zeros in n!.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut token = u32::to_posint(&scan[..scan.len() - 1]);
    let mut answer = 0;

    while token > 0 {
        token /= 5;
        answer += token;
    }

    writeln!(out, "{answer}").unwrap();
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
20
";
        let target = b"\
4
";

        test(input, target);
    }

    #[test]
    fn test_large() {
        let input = b"\
100
";
        let target = b"\
24
";

        test(input, target);
    }

    #[test]
    fn test_small() {
        let input = b"\
4
";
        let target = b"\
0
";

        test(input, target);
    }

    #[test]
    fn test_largest() {
        let input = b"\
1000000000
";
        let target = b"\
249999998
";

        test(input, target);
    }

    #[test]
    fn test_power_of_5() {
        let input = b"\
625
";
        let target = b"\
156
";

        test(input, target);
    }
}
