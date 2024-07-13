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
            std::mem::transmute::<std::str::SplitAsciiWhitespace<'_>, std::str::SplitAsciiWhitespace<'_>>(slice.split_ascii_whitespace())
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
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: usize = scan.token();
    let target: i64 = scan.token();
    let mut counter = 0_u64;
    let mut sum = 0_i64;
    let mut map = HashMap::with_capacity(n);
    map.insert(0, 1);

    for num in (0..n).map(|_| scan.token::<i64>()) {
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
