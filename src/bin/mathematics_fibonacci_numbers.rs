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

type Matrix = [u64; 3];

/// The Fibonacci numbers can be defined as follows:
///
/// - F<sub>0</sub>=0
/// - F<sub>1</sub>=1
/// - F<sub>n</sub> = F<sub<n-2</sub> + F<sub<n-1</sub>
///
/// Your task is to calculate the value of F<sub>n</sub> for a given n.
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print the value of F<sub>n</sub> modulo 10^9+7.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>0 ≤ n ≤ 10<sup>18</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut n = u64::to_posint(&scan[..scan.len() - 1]);
    if n >= 2 {
        n -= 1;
        let mut base: Matrix = [1, 1, 0];
        // identity matrix
        let mut goal: Matrix = [1, 0, 1];

        while n != 0 {
            if n & 1 == 1 {
                goal = multiply_matrix(&goal, &base);
            }
            base = multiply_matrix(&base, &base);
            n >>= 1;
        }
        writeln!(out, "{}", goal[0]).unwrap();
    } else {
        writeln!(out, "{n}").unwrap();
    }
}

fn multiply_matrix(a: &Matrix, b: &Matrix) -> Matrix {
    [
        (a[0] * b[0] + a[1] * b[1]) % MODULO,
        (a[0] * b[1] + a[1] * b[2]) % MODULO,
        (a[1] * b[1] + a[2] * b[2]) % MODULO,
    ]
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
10
";
        let target = b"\
55
";

        test(input, target);
    }

    #[test]
    fn test_0() {
        let input = b"\
0
";
        let target = b"\
0
";

        test(input, target);
    }

    #[test]
    fn test_1() {
        let input = b"\
1
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_1000() {
        let input = b"\
1000
";
        let target = b"\
517691607
";

        test(input, target);
    }

    #[test]
    fn test_max() {
        let input = b"\
1000000000000000000
";
        let target = b"\
209783453
";

        test(input, target);
    }
}
