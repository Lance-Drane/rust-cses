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

/// There are n children who want to go to a Ferris wheel, and your task is to find a gondola for each child.
///
/// Each gondola may have one or two children in it, and in addition, the total weight in a gondola may not exceed x. You know the weight of every child.
///
/// What is the minimum number of gondolas needed for the children?
///
/// <b>Input</b>
///
/// The first input line contains two integers n and x: the number of children and the maximum allowed weight.
///
/// The next line contains n integers p<sub>1</sub>, p<sub>2</sub>, ..., p<sub>n</sub>: the weight of each child.
///
/// <b>Output</b>
///
/// Print one integer: the minimum number of gondolas.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ x ≤ 10<sup>9</sup></li>
/// <li>1 ≤ p<sub>i</sub> ≤ x</li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let num_children = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let max_weight = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };

    let mut children: Vec<u32> = (0..num_children)
        .map(|_| unsafe { u32::to_posint(iter.next().unwrap_unchecked()) })
        .collect();
    children.sort_unstable();

    let mut count = 0_u32;

    let mut iter = children.into_iter();
    while let Some(thin) = iter.next() {
        loop {
            count += 1;
            if let Some(fat) = iter.next_back() {
                if thin + fat <= max_weight {
                    break;
                }
            } else {
                break;
            }
        }
    }

    writeln!(out, "{count}").unwrap();
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
4 10
7 2 3 9
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
1 1
1
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_example_3() {
        let input = b"\
5 4
2 2 2 2 2
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_example_4() {
        let input = b"\
5 3
2 2 2 2 2
";
        let target = b"\
5
";

        test(input, target);
    }
}
