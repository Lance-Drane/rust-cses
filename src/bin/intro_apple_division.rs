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

/// There are n apples with known weights. Your task is to divide the apples into two groups so that the difference between the weights of the groups is minimal.
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of apples.
///
/// The next line has n integers p<sub>1</sub>,p<sub>2</sub>,...,p<sub>n</sub>: the weight of each apple.
///
/// <b>Output</b>
///
/// Print one integer: the minimum difference between the weights of the groups.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 20</li>
/// <li>1 ≤ p<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
    let mut apples = [0_i64; 20];
    for apple in apples.iter_mut().take(n) {
        *apple = unsafe { i64::to_posint(iter.next().unwrap_unchecked()) };
    }
    apples.sort_unstable_by(|a, b| b.cmp(a));

    let mut weights = [0_i64; 20];
    let mut weight_sum = 0;
    for (weight, apple) in weights
        .iter_mut()
        .take(20 - n)
        .zip(apples.iter().take(n).rev())
    {
        weight_sum += apple;
        *weight = weight_sum;
    }
    weights.reverse();

    writeln!(out, "{}", min_diff(0, &apples, &weights, 0, n - 1)).unwrap();
}

fn min_diff(
    iter: usize,
    apples: &[i64; 20],
    weights: &[i64; 20],
    last_subset_diff: i64,
    len: usize,
) -> i64 {
    unsafe {
        if iter == len {
            // base case, both subsets are filled after the final call
            return (last_subset_diff - apples.get_unchecked(iter)).abs();
        }
        if last_subset_diff + apples.get_unchecked(iter) >= *weights.get_unchecked(iter + 1) {
            // we no longer need to fill subset 1, exclusively fill subset 2
            return std::cmp::min(
                last_subset_diff + apples.get_unchecked(iter) - weights.get_unchecked(iter + 1),
                min_diff(
                    iter + 1,
                    apples,
                    weights,
                    (last_subset_diff - apples.get_unchecked(iter)).abs(),
                    len,
                ),
            );
        }
        // still need to fill both subsets at this point
        std::cmp::min(
            min_diff(
                iter + 1,
                apples,
                weights,
                last_subset_diff + apples.get_unchecked(iter),
                len,
            ),
            min_diff(
                iter + 1,
                apples,
                weights,
                (last_subset_diff - apples.get_unchecked(iter)).abs(),
                len,
            ),
        )
    }
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
3 2 7 4 1
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_one_group() {
        let input = b"\
1
37
";
        let target = b"\
37
";

        test(input, target);
    }

    #[test]
    fn test_large_addition() {
        let input = b"\
20
1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000
";
        let target = b"\
0
";

        test(input, target);
    }
}
