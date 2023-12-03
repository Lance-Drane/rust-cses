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

const MODULO: u64 = 1_000_000_007;

type Matrix = [u64; 3];

/// The Fibonacci numbers can be defined as follows:
///
/// - F<sub>0</sub>=0
/// - F<sub>1</sub>=1
/// - F<sub>n</sub> = F<sub<n-2</sub> + F<sub<n-1</sub>
///
/// Your task is to calculate the value of F<sub>n</sub> for a given n.
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print the value of F<sub>n</sub> modulo 10^9+7.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>0 ≤ n ≤ 10<sup>18</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let mut n = scan.token::<u64>();
    if n >= 2 {
        n -= 1;
        let mut base: Matrix = [1, 1, 0];
        // identity matrix
        let mut goal: Matrix = [1, 0, 1];

        while n != 0 {
            if n & 1 == 1 {
                goal = multiply_matrix(&goal, &base);
            }
            base = multiply_matrix(&base, &base);
            n >>= 1;
        }
        writeln!(out, "{}", goal[0]).ok();
    } else {
        writeln!(out, "{n}").ok();
    }
}

fn multiply_matrix(a: &Matrix, b: &Matrix) -> Matrix {
    [
        (a[0] * b[0] + a[1] * b[1]) % MODULO,
        (a[0] * b[1] + a[1] * b[2]) % MODULO,
        (a[1] * b[1] + a[2] * b[2]) % MODULO,
    ]
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
10
";
        let target = b"\
55
";

        test(input, target);
    }

    #[test]
    fn test_0() {
        let input = b"\
0
";
        let target = b"\
0
";

        test(input, target);
    }

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
    fn test_1000() {
        let input = b"\
1000
";
        let target = b"\
517691607
";

        test(input, target);
    }
}
