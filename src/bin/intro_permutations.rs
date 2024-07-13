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

/// A permutation of integers 1,2,…,n is called <i>beautiful</i> if there are no adjacent elements whose difference is 1.
///
/// Given n, construct a beautiful permutation if such a permutation exists.
///
/// <b>Input</b>
///
/// The only input line contains an integer n.
///
/// <b>Output</b>
///
/// Print a beautiful permutation of integers 1,2,…,n. If there are several solutions, you may print any of them. If there are no solutions, print "NO SOLUTION".
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let num = scan.token::<u32>();

    if num == 2 || num == 3 {
        out.write_all(b"NO SOLUTION\n").unwrap();
    } else {
        for i in (2..(num + 1)).step_by(2) {
            write!(out, "{i} ").unwrap();
        }
        for i in (1..(num + 1)).step_by(2) {
            write!(out, "{i} ").unwrap();
        }
    }
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin());
    let mut out = std::io::BufWriter::with_capacity(32_768, std::io::stdout().lock());
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

    // NOTE: valid tests can technically print out _any_ permutation, but we have a specific implementation.
    // NOTE: All "targets" have a space at the end, as this is fine for CSES and allows for more concise logic.

    #[test]
    fn test_1() {
        let input = b"\
1
";
        let target = b"\
1 ";

        test(input, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
5
";
        let target = b"\
2 4 1 3 5 ";

        test(input, target);
    }

    #[test]
    fn test_no_solution() {
        let input = b"\
3
";
        let target = b"\
NO SOLUTION
";

        test(input, target);
    }
}
