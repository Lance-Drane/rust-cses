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

/// You have to process n tasks. Each task has a duration and a deadline, and you will process the tasks in some order one after another. Your reward for a task is d-f where d is its deadline and f is your finishing time. (The starting time is 0, and you have to process all tasks even if a task would yield negative reward.)
///
/// What is your maximum reward if you act optimally?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of tasks.
///
/// After this, there are n lines that describe the tasks. Each line has two integers a and d: the duration and deadline of the task.
///
/// <b>Output</b>
///
/// Print one integer: the maximum reward.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a,d ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let mut tasks: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            (
                unsafe { i64::to_posint(iter.next().unwrap_unchecked()) },
                unsafe { i64::to_posint(iter.next().unwrap_unchecked()) },
            )
        })
        .collect();
    tasks.sort_unstable_by_key(|t| t.0);

    writeln!(
        out,
        "{}",
        tasks
            .into_iter()
            .fold((0, 0), |(score, time_passed), (duration, deadline)| {
                let new_time_passed = time_passed + duration;
                (score + deadline - new_time_passed, new_time_passed)
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
3
6 10
8 15
5 12
";
        let target = b"\
2
";

        test(input, target);
    }
}
