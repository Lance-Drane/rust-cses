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

use std::collections::HashMap;

/// You are given an array of n integers, and your task is to find two values (at distinct positions) whose sum is x.
///
/// <b>Input</b>
///
/// The first input line has two integers n and x: the array size and the target sum.
///
/// The second line has n integers a<sub>1</sub>,a<sub>2</sub>,...,a<sub>n</sub>: the array values.
///
/// <b>Output</b>
///
/// Print two integers: the positions of the values. If there are several solutions, you may print any of them. If there are no solutions, print IMPOSSIBLE.
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
    let target = unsafe { i32::to_posint(iter.next().unwrap_unchecked()) };

    let mut lookups: HashMap<i32, u32> = HashMap::with_capacity(n as usize);

    for (value, idx) in
        (1..(n + 1)).map(|n| (unsafe { i32::to_posint(iter.next().unwrap_unchecked()) }, n))
    {
        if let Some(prev) = lookups.get(&(target - value)) {
            writeln!(out, "{prev} {idx}").unwrap();
            return;
        }
        lookups.insert(value, idx);
    }
    out.write_all(b"IMPOSSIBLE\n").unwrap();
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

    // NOTE: While any solution is allowed, we greedily try to obtain the first solution, and print the indexes in increasing order.

    fn test(input: &[u8], target: &[u8]) {
        let mut out = Vec::with_capacity(target.len());
        solve(input, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
4 8
2 7 5 1
";
        let target = b"\
2 4
";

        test(input, target);
    }

    #[test]
    fn test_impossible() {
        let input = b"\
4 8
1 2 3 4
";
        let target = b"\
IMPOSSIBLE
";

        test(input, target);
    }
}
