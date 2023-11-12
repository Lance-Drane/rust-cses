// I/O boilerplate //

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: std::io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    /// Use "turbofish" syntax `token::<T>()` to select data type of next token.
    ///
    /// # Panics
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
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
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let num = scan.token::<u32>();

    match num {
        2 | 3 => {
            out.write(b"NO SOLUTION\n").ok();
        }
        n => {
            (2..=n).step_by(2).chain((1..=n).step_by(2)).for_each(|i| {
                write!(out, "{i} ").ok();
            });
            out.write(b"\n").ok();
        }
    };
}

// entrypoints //

fn main() {
    let mut scan = UnsafeScanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    solve(&mut scan, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

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
1 
";

        test(input, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
5
";
        let target = b"\
2 4 1 3 5 
";

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
