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
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
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

    writeln!(out, "{counter}").ok();
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
