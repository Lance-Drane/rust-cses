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
    let token = scan.token::<u32>();

    match token & 3 {
        3 => {
            write!(out, "YES\n{}\n1 2 ", (token + 1) >> 1).unwrap();
            for i in (4..(token + 1)).step_by(4) {
                write!(out, "{} {} ", i, i + 3).unwrap();
            }
            write!(out, "\n{}\n3 ", token >> 1).unwrap();
            for i in (4..(token + 1)).step_by(4) {
                write!(out, "{} {} ", i + 1, i + 2).unwrap();
            }
            out.write_all(b"\n").unwrap();
        }
        0 => {
            let size = token / 2;
            writeln!(out, "YES\n{size}").unwrap();
            for i in (1..(token + 1)).step_by(4) {
                write!(out, "{} {} ", i, i + 3).unwrap();
            }
            writeln!(out, "\n{size}").unwrap();
            for i in (1..(token + 1)).step_by(4) {
                write!(out, "{} {} ", i + 1, i + 2).unwrap();
            }
            out.write_all(b"\n").unwrap();
        }
        _ => {
            out.write_all(b"NO\n").unwrap();
        }
    };
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

    // NOTE: valid tests can technically print out _any_ solution, but we have a specific implementation.

    #[test]
    fn test_example() {
        let input = b"\
7
";
        let target = b"\
YES
4
1 2 4 7 
3
3 5 6 
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
1 2 4 7 8 11 
5
3 5 6 9 10 
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
1 4 5 8 9 12 
6
2 3 6 7 10 11 
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
