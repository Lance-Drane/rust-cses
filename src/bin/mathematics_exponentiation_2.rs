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

const MODULO: u64 = 1_000_000_007;

/// Your task is to efficiently calculate values a<sup>b<sup>c</sup></sup> modulo 10<sup>9</sup>+7.
///
/// Note that in this task we assume that 0<sup>0</sup>=1.
///
/// <b>Input</b>
///
/// The first input line contains an integer n: the number of calculations.
///
/// After this, there are n lines, each containing three integers a, b, and c.
///
/// <b>Output</b>
///
/// Print each value a<sup>b<sup>c</sup></sup> modulo 10<sup>9</sup>+7.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>5</sup></li>
/// <li>0 ≤ a,b,x ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u32 = scan.token();
    for _ in 0..n {
        let a = scan.token();
        let b = scan.token();
        let c = scan.token();
        // Fermat's Little Theorem
        writeln!(out, "{}", fastpow(a, fastpow(b, c, MODULO - 1), MODULO)).unwrap();
    }
}

fn fastpow(mut base: u64, mut exponent: u64, modulo: u64) -> u64 {
    let mut goal = 1;

    while exponent != 0 {
        if exponent & 1 == 1 {
            goal = goal * base % modulo;
        }
        base = base * base % modulo;
        exponent >>= 1;
    }

    goal
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

    #[test]
    fn test_example() {
        let input = b"\
3
3 7 1
15 2 2
3 4 5
";
        let target = b"\
2187
50625
763327764
";

        test(input, target);
    }

    #[test]
    fn test_zeroes() {
        let input = b"\
4
0 0 0
123456 0 0
0 123456 0
0 0 123456
";
        let target = b"\
0
123456
0
1
";

        test(input, target);
    }

    #[test]
    fn test_big() {
        let input = b"\
1
7 8 10
";
        let target = b"\
928742408
";

        test(input, target);
    }
}
