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

/// You are given an array of n integers. You want to modify the array so that it is increasing, i.e., every element is at least as large as the previous element.
///
/// On each move, you may increase the value of any element by one. What is the minimum number of moves required?
///
/// <b>Input</b>
///
/// The first input line contains an integer n: the size of the array.
///
/// Then, the second line contains n integers x<sub>1</sub>,x<sub>2</sub>,…,x<sub>n</sub>: the contents of the array.
///
/// <b>Output</b>
///
/// Print the minimum number of moves.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ x<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let size = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    // we have to eagerly evaluate the first token of row 2
    let first = unsafe { u64::to_posint(iter.next().unwrap_unchecked()) };

    writeln!(
        out,
        "{}",
        (1..size)
            .map(|_| unsafe { u64::to_posint(iter.next().unwrap_unchecked()) })
            .fold((0, first), |(moves, largest), token| {
                if token < largest {
                    (largest - token + moves, largest)
                } else {
                    (moves, token)
                }
            })
            .0
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
3 2 5 1 7
";
        let target = b"\
5
";

        test(input, target);
    }

    #[test]
    fn test_no_step() {
        let input = b"\
10
1 1 1 1 1 1 1 1 1 1
";
        let target = b"\
0
";

        test(input, target);
    }

    #[test]
    fn test_one_item() {
        let input = b"\
1
329873232
";
        let target = b"\
0
";

        test(input, target);
    }

    #[test]
    fn test_many_steps() {
        let input = b"\
10
1000000000 1 1 1 1 1 1 1 1 1
";
        let target = b"\
8999999991
";

        test(input, target);
    }
}
