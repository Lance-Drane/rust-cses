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

/// You are given n cubes in a certain order, and your task is to build towers using them. Whenever two cubes are one on top of the other, the upper cube must be smaller than the lower cube.
///
/// You must process the cubes in the given order. You can always either place the cube on top of an existing tower, or begin a new tower. What is the minimum possible number of towers?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of cubes.
///
/// The next line contains n integers k<sub>1</sub>,k<sub>2</sub>,...,k<sub>n</sub>: the sizes of the cubes.
///
/// <b>Output</b>
///
/// Print one integer: the minimum number of towers.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ k<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let mut towers = Vec::with_capacity(n as usize);

    for cube in (0..n).map(|_| unsafe { u32::to_posint(iter.next().unwrap_unchecked()) }) {
        if cube >= *towers.last().unwrap_or(&0) {
            towers.push(cube);
        } else {
            let mut low = 0;
            let mut high = towers.len();
            while low < high {
                let mid = (low + high) >> 1;
                if towers[mid] > cube {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }
            towers[low] = cube;
        }
    }

    writeln!(out, "{}", towers.len()).unwrap();
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
3 8 2 1 5
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_equal_not_on_top_of_equal() {
        let input = b"\
3
8 8 4
";
        let target = b"\
2
";

        test(input, target);
    }
}
