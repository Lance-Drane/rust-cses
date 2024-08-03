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

/// You are given the arrival and leaving times of n customers in a restaurant.
///
/// What was the maximum number of customers in the restaurant at any time?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of customers.
///
/// After this, there are n lines that describe the customers. Each line has two integers a and b: the arrival and leaving times of a customer.
///
/// You may assume that all arrival and leaving times are distinct.
///
/// <b>Output</b>
///
/// Print one integer: the maximum number of customers.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a ≤ b ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');
    let num = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let mut customers: Vec<_> = (0..num)
        .flat_map(|_| {
            [
                (unsafe { u32::to_posint(iter.next().unwrap_unchecked()) }, 1),
                (
                    unsafe { u32::to_posint(iter.next().unwrap_unchecked()) },
                    -1,
                ),
            ]
        })
        .collect();
    customers.sort_unstable_by_key(|m| m.0);

    writeln!(
        out,
        "{}",
        customers
            .into_iter()
            .fold((0, 0), |(best, current), shift| {
                let next = current + shift.1;
                (best.max(next), next)
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

    // NOTE: our implementation prints out an additional space at the end of output

    fn test(input: &[u8], target: &[u8]) {
        let mut out = Vec::with_capacity(target.len());
        solve(input, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
3
5 8
2 4
3 9
";
        let target = b"\
2
";

        test(input, target);
    }
}
