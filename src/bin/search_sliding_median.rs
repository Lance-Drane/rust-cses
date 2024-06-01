// I/O boilerplate //

pub struct UnsafeScanner<'a> {
    // not actually dead code, needed for buf_iter to work
    #[allow(dead_code)]
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'a>,
}

impl UnsafeScanner<'_> {
    pub fn new<R: std::io::Read>(mut reader: R) -> Self {
        let mut buf_str = vec![];
        unsafe {
            reader.read_to_end(&mut buf_str).unwrap_unchecked();
        }
        let buf_iter = unsafe {
            let slice = std::str::from_utf8_unchecked(&buf_str);
            std::mem::transmute(slice.split_ascii_whitespace())
        };

        Self { buf_str, buf_iter }
    }

    /// Use "turbofish" syntax `token::<T>()` to select data type of next token.
    ///
    /// # Panics
    /// Panics if there's no more tokens or if the token cannot be parsed as T.
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        unsafe {
            self.buf_iter
                .next()
                .unwrap_unchecked()
                .parse()
                .unwrap_unchecked()
        }
    }
}

// problem //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// You are given an array of n integers. Your task is to calculate the median of each window of k elements, from left to right.
///
/// The median is the middle element when the elements are sorted. If the number of elements is even, there are two possible medians and we assume that the median is the smaller of them.
///
/// <b>Input</b>
///
/// The first input line contains two integers n and k: the number of elements and the size of the window.
///
/// Then there are n integers x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>: the contents of the array.
///
/// <b>Output</b>
///
/// Print n-k+1 values: the medians.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ k ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ x<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u32 = scan.token();
    let k: u32 = scan.token();
    let mid_idx = (k >> 1) - ((k + 1) & 1);
    let numbers: Vec<u32> = (0..n).map(|_| scan.token()).collect();

    // collect initial numbers before median (exclusive)
    let mut before_median: BinaryHeap<_> = numbers
        .iter()
        .take(mid_idx as usize)
        .zip(0..)
        .map(|(num, i)| (*num, i))
        .collect();
    before_median.reserve_exact((n as usize) - before_median.len());
    let mut after_median = BinaryHeap::with_capacity(n as usize);

    // collect numbers after median (exclusive) but still inside sliding window, leave off last value in sliding window
    for (num, idx) in numbers
        .iter()
        .take(k as usize - 1)
        .skip(mid_idx as usize)
        .zip(mid_idx..)
    {
        before_median.push((*num, idx));
        after_median.push(Reverse(before_median.pop().unwrap()));
    }
    // start shifting sliding window
    for (((r_num, r_idx), l_num), l_idx) in numbers
        .iter()
        .skip(k as usize - 1)
        .zip((k - 1)..)
        .zip(numbers.iter())
        .zip(0..)
    {
        before_median.push((*r_num, r_idx));
        after_median.push(Reverse(before_median.pop().unwrap()));
        let median = after_median.peek().unwrap().0 .0;
        write!(out, "{median} ").unwrap();
        // keep balance between priority queues
        if *l_num <= median {
            before_median.push(after_median.pop().unwrap().0);
        }
        // lazily remove all values now outside the sliding window
        while let Some(max_val) = before_median.peek() {
            if max_val.1 <= l_idx {
                before_median.pop();
            } else {
                break;
            }
        }
        while let Some(min_val) = after_median.peek() {
            if min_val.0 .1 <= l_idx {
                after_median.pop();
            } else {
                break;
            }
        }
    }
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    solve(scan, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
8 3
2 4 3 5 8 1 2 1
";
        let target = b"\
3 4 5 5 2 1 ";
        test(input, target);
    }

    #[test]
    fn test_non_sliding_window() {
        let input = b"\
10 10
1 2 3 4 5 6 7 8 9 10
";
        let target = b"\
5 ";
        test(input, target);
    }

    #[test]
    fn test_minimal_window() {
        let input = b"\
10 1
1 2 3 4 5 6 7 8 9 10
";
        let target = b"\
1 2 3 4 5 6 7 8 9 10 ";
        test(input, target);
    }
}
