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
            std::mem::transmute::<
                std::str::SplitAsciiWhitespace<'_>,
                std::str::SplitAsciiWhitespace<'_>,
            >(slice.split_ascii_whitespace())
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
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u32 = scan.token();
    let target: i32 = scan.token();

    let mut lookups: HashMap<i32, u32> = HashMap::with_capacity(n as usize);

    for (value, idx) in (1..=n).map(|n| (scan.token(), n)) {
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
    let scan = UnsafeScanner::new(std::io::stdin());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    solve(scan, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    // NOTE: While any solution is allowed, we greedily try to obtain the first solution, and print the indexes in increasing order.

    fn test(input: &[u8], target: &[u8]) {
        let scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(scan, &mut out);

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
