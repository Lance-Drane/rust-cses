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

pub trait AnyInt {
    fn to_anyint(buf: &[u8]) -> Self;
}
macro_rules! impl_anyint {
    (for $($t:ty),+) => {
        $(impl AnyInt for $t {
            #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
            fn to_anyint(buf: &[u8]) -> Self {
                let (neg, digits) = match buf {
                    [b'-', digits @ ..] => (true, digits),
                    digits => (false, digits),
                };

                let result = unsafe {
                    digits.iter()
                        .map(|byte| (byte & 15) as $t)
                        .reduce(|acc, digit| acc * 10 + digit)
                        .unwrap_unchecked()
                };

                if neg {
                    -result
                } else {
                    result
                }
            }
        })*
    }
}
impl_anyint!(for i8, i16, i32, i64, i128, isize);

// problem //

use std::collections::HashMap;

/// Given an array of n integers, your task is to count the number of subarrays having sum x.
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
/// <li>-10<sup>9</sup> ≤ x,a<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
    let target = unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) };
    let mut counter = 0_u64;
    let mut sum = 0_i64;
    let mut map = HashMap::with_capacity(n);
    map.insert(0, 1);

    for num in (0..n).map(|_| unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) }) {
        sum += num;
        if let Some(prev) = map.get(&(sum - target)) {
            counter += prev;
        }
        map.entry(sum).and_modify(|e| *e += 1).or_insert(1);
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
2 -1 3 5 -2
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
5 3
1 1 1 -1 1
";
        let target = b"\
2
";

        test(input, target);
    }
}
