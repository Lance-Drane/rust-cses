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

/// You are given an integer n. On each step, you may subtract one of the digits from the number.
///
/// How many steps are required to make the number equal to 0?
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print one integer: the minimum number of steps.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
///
/// For "27", one optimal solution is 27→20→18→10→9→0.
///
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let mut target = scan.token::<u32>();
    let mut counter = 0_u32;

    while target != 0 {
        let mut quotient = target;
        let mut remainder = 0;

        while quotient != 0 {
            remainder = remainder.max(quotient % 10);
            quotient /= 10;
        }

        target -= remainder;
        counter += 1;
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
27
";
        let target = b"\
5
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
100
";
        let target = b"\
17
";

        test(input, target);
    }

    #[test]
    fn test_example_3() {
        let input = b"\
99
";
        let target = b"\
16
";

        test(input, target);
    }

    #[test]
    fn test_example_4() {
        let input = b"\
1000000
";
        let target = b"\
128207
";

        test(input, target);
    }
}
