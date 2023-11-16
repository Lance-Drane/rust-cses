// I/O boilerplate //

pub struct UnsafeScanner {
    // not actually dead code, needed for buf_iter to work
    #[allow(dead_code)]
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl UnsafeScanner {
    pub fn new<R: std::io::BufRead>(mut reader: R) -> Self {
        let mut buf_str = vec![];
        unsafe {
            reader.read_to_end(&mut buf_str).unwrap_unchecked();
        }
        let buf_iter = unsafe {
            let slice = std::str::from_utf8_unchecked(&buf_str);
            std::mem::transmute(slice.split_ascii_whitespace())
        };
        // optional memory clear
        buf_str.clear();

        Self { buf_str, buf_iter }
    }

    /// Use "turbofish" syntax `token::<T>()` to select data type of next token.
    ///
    /// # Panics
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
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

/// Your task is to divide the numbers 1,2,…,n into two sets of equal sum.
///
/// <b>Input</b>
///
/// The only input line contains an integer n.
///
/// <b>Output</b>
///
/// Print "YES", if the division is possible, and "NO" otherwise.
///
/// After this, if the division is possible, print an example of how to create the sets. First, print the number of elements in the first set followed by the elements themselves in a separate line, and then, print the second set in a similar way.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let token = scan.token::<i32>();

    match token & 3 {
        3 => {
            out.write_all(b"YES\n").ok();
            let size = token / 2;
            let step = token / 4;
            let quarter = 4 + step;
            let three_quarter = token - step + 1;

            writeln!(out, "{}", size + 1).ok();

            (4..quarter).chain(three_quarter..=token).for_each(|i| {
                write!(out, "{i} ").ok();
            });
            out.write_all(b"1 2\n").ok();

            writeln!(out, "{size}").ok();

            for i in quarter..three_quarter {
                write!(out, "{i} ").ok();
            }
            out.write_all(b"3\n").ok();
        }
        0 => {
            out.write_all(b"YES\n").ok();
            let size = token / 2;
            let quarter = size / 2;
            let three_quarter = token - quarter;

            writeln!(out, "{size}").ok();

            (1..=quarter)
                .chain((three_quarter + 1)..token)
                .for_each(|i| {
                    write!(out, "{i} ").ok();
                });
            writeln!(out, "{token}").ok();

            writeln!(out, "{size}").ok();

            for i in (quarter + 1)..three_quarter {
                write!(out, "{i} ").ok();
            }
            writeln!(out, "{three_quarter}").ok();
        }
        _ => {
            out.write_all(b"NO\n").ok();
        }
    };
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin().lock());
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

    // NOTE: valid tests can technically print out _any_ solution, but we have a specific implementation.

    #[test]
    fn test_example() {
        let input = b"\
7
";
        let target = b"\
YES
4
4 7 1 2
3
5 6 3
";

        test(input, target);
    }

    #[test]
    fn test_longer() {
        let input = b"\
11
";
        let target = b"\
YES
6
4 5 10 11 1 2
5
6 7 8 9 3
";

        test(input, target);
    }

    #[test]
    fn test_mod_3() {
        let input = b"\
3
";
        let target = b"\
YES
2
1 2
1
3
";

        test(input, target);
    }

    #[test]
    fn test_mod_0() {
        let input = b"\
4
";
        let target = b"\
YES
2
1 4
2
2 3
";

        test(input, target);
    }

    #[test]
    fn test_mod_0_longer() {
        let input = b"\
12
";
        let target = b"\
YES
6
1 2 3 10 11 12
6
4 5 6 7 8 9
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
6
";
        let target = b"\
NO
";

        test(input, target);
    }
}
