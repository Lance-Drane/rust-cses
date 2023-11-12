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

const MODULO: u64 = 1_000_000_007;

/// Your task is to calculate the number of bit strings of length n.
///
/// For example, if n=3, the correct answer is 8, because the possible bit strings are 000, 001, 010, 011, 100, 101, 110, and 111.
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print the result modulo 10<sup>9</sup>+7.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut exponent = scan.token::<u32>();

    let mut goal: u64 = 1;
    let mut base: u64 = 2;

    while exponent != 0 {
        if exponent & 1 == 1 {
            goal = goal * base % MODULO;
        }
        base = base * base % MODULO;
        exponent >>= 1;
    }

    writeln!(out, "{goal}").ok();
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
3
";
        let target = b"\
8
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
255
";
        let target = b"\
396422633
";

        test(input, target);
    }

    #[test]
    fn test_smallest() {
        let input = b"\
1
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_largest() {
        let input = b"\
1000000
";
        let target = b"\
235042059
";

        test(input, target);
    }

    #[test]
    fn test_example_3() {
        let input = b"\
665215
";
        let target = b"\
976383320
";

        test(input, target);
    }
}
