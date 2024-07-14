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

const MODULO: usize = 1_000_000_007;

type Matrix = [[usize; 6]; 6];

/// Your task is to count the number of ways to construct sum n by throwing a dice one or more times. Each throw produces an outcome between 1 and 6.
///
/// For example, if n = 3, there are 4 ways:
/// <ul>
/// <li>1 + 1 + 1</li>
/// <li>1 + 2</li>
/// <li>2 + 1</li>
/// <li>3</li>
/// </ul>
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print the number of ways modulo 10<sup>9</sup> + 7.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut exponent = u32::to_posint(&scan[..scan.len() - 1]);

    let mut base: Matrix = [
        [1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 1, 0],
    ];
    // identity matrix
    let mut goal: Matrix = [
        [1, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 1],
    ];

    while exponent != 0 {
        if exponent & 1 == 1 {
            goal = multiply_matrix(&goal, &base);
        }
        base = multiply_matrix(&base, &base);
        exponent >>= 1;
    }

    writeln!(out, "{}", goal[0][0]).unwrap();
}

fn multiply_matrix(a: &Matrix, b: &Matrix) -> Matrix {
    let mut ret: Matrix = [[0; 6]; 6];

    for i in 0..6 {
        for j in 0..6 {
            ret[i][j] = ((0..6).map(|k| a[i][k] * b[k][j]).sum::<usize>()) % MODULO;
        }
    }

    ret
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
4
";

        test(input, target);
    }

    #[test]
    fn test_dp() {
        let input = b"\
10
";
        let target = b"\
492
";

        test(input, target);
    }

    #[test]
    fn test_max() {
        let input = b"\
1000000
";
        let target = b"\
874273980
";

        test(input, target);
    }
}
