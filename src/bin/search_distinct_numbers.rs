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

use std::collections::HashSet;

/// You are given a list of n integers, and your task is to calculate the number of distinct values in the list.
///
/// <b>Input</b>
///
///The first input line has an integer n: the number of values.
///
/// The second line has n integers x1,x2,...,x<sub>n</sub>.
///
/// <b>Output</b>
///
/// Print one integer: the number of distinct values.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ x<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: usize = scan.token();

    writeln!(
        out,
        "{}",
        (0..n)
            .map(|_| scan.token::<u32>())
            .collect::<HashSet<_>>()
            .len()
    )
    .ok();
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
5
2 3 2 2 3
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_all_unique() {
        let input = b"\
4
3 2 1 1000000000
";
        let target = b"\
4
";

        test(input, target);
    }

    #[test]
    fn test_all_same() {
        let input = b"\
6
6 6 6 6 6 6
";
        let target = b"\
1
";

        test(input, target);
    }
}
