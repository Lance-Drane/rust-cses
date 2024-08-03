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

/// Given an array of n positive integers, your task is to count the number of subarrays having sum x.
///
/// <b>Input</b>
///
/// The first input line has two integers n and x: the size of the array and the target sum x.
///
/// The next line has n integers a<sub>1</sub>,a<sub>2</sub>,...,a<sub>n</sub>: the contents of the array.
///
/// <b>Output</b>
///
/// Print one integer: the required number of subarrays.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ x,a<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let target = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let arr: Vec<u32> = (0..n)
        .map(|_| unsafe { u32::to_posint(iter.next().unwrap_unchecked()) })
        .collect();

    let mut counter = 0_u32;
    let mut sum = 0;
    let mut l_pointer = 0;
    let mut r_pointer = 0;

    while r_pointer < arr.len() {
        if sum > target {
            sum -= unsafe { arr.get_unchecked(l_pointer) };
            l_pointer += 1;
        } else {
            if sum == target {
                counter += 1;
            }
            sum += unsafe { arr.get_unchecked(r_pointer) };
            r_pointer += 1;
        }
    }
    while sum > target {
        sum -= unsafe { arr.get_unchecked(l_pointer) };
        l_pointer += 1;
    }
    if sum == target {
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
5 7
2 4 1 2 7
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
5 9
2 4 1 2 7
";
        let target = b"\
2
";

        test(input, target);
    }
}
