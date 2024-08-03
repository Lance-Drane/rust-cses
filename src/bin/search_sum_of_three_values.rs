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

/// You are given an array of n integers, and your task is to find three values (at distinct positions) whose sum is x.
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
/// <li>1 ≤ n ≤ 5000</li>
/// <li>1 ≤ x,a<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { u16::to_posint(iter.next().unwrap_unchecked()) };
    if n < 3 {
        out.write_all(b"IMPOSSIBLE\n").unwrap();
        return;
    }
    let target_sum = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let maximum_smallest_value = target_sum / 3;

    let mut nums: Vec<(u32, u16)> = (1..(n + 1))
        .map(|og_idx| {
            (
                unsafe { u32::to_posint(iter.next().unwrap_unchecked()) },
                og_idx,
            )
        })
        .filter(|num| num.0 < target_sum) // skip excessive values
        .collect();
    nums.sort_unstable_by_key(|k| k.0);

    let mut rskip_amt = 0;
    for (left, left_num) in nums
        .iter()
        .take(nums.len() - 2) // always need at least 3 elements to accomplish anything
        .take_while(|num| num.0 <= maximum_smallest_value)
        .enumerate()
        // skip duplicates
        .filter(|&(idx, num)| idx == 0 || num.0 != nums[idx - 1].0)
    {
        let mid_right_target = target_sum - left_num.0;
        rskip_amt = nums
            .iter()
            .skip(left)
            .rev()
            .skip(rskip_amt)
            .position(|x| x.0 <= mid_right_target)
            .unwrap_or_else(|| nums.len() - left);
        let mut right = nums.len() - 1 - rskip_amt;
        let mut mid = left
            + 1
            + nums[left + 1..right].partition_point(|x| x.0 + nums[right].0 < mid_right_target);
        while mid < right {
            let mid_num = nums[mid];
            let right_num = nums[right];
            match mid_right_target.cmp(&(mid_num.0 + right_num.0)) {
                std::cmp::Ordering::Greater => mid += 1,
                std::cmp::Ordering::Less => right -= 1,
                std::cmp::Ordering::Equal => {
                    writeln!(out, "{} {} {}", left_num.1, mid_num.1, right_num.1).unwrap();
                    return;
                }
            }
        }
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

    // NOTE: While any solution is allowed, we always print a specific solution:
    // 1) first, we print the position of the smallest number possible to construct a solution
    // 2) next, we print the position of the smallest number possible in conjunction with 1)
    // 3) finally, we print the position of the remaining number possible in conjunction with 1) and 2)

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
4 1 3
";

        test(input, target);
    }

    #[test]
    fn test_longer() {
        let input = b"\
13 20
1 12 2 11 1 3 10 4 9 5 8 6 7
";
        let target = b"\
1 13 2
";

        test(input, target);
    }

    #[test]
    fn test_larger() {
        let input = b"\
7 75
1 10 100 20 5 50 100
";
        let target = b"\
5 4 6
";

        test(input, target);
    }

    #[test]
    fn test_impossible() {
        let input = b"\
4 8
1 1 1 1
";
        let target = b"\
IMPOSSIBLE
";

        test(input, target);
    }
}
