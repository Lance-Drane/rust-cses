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

type Matrix = [[u64; 6]; 6];

/// Your task is to count the number of ways to construct sum n by throwing a dice one or more times. Each throw yields an integer between 1 \ldots 6.
///
/// For example, if n=10, some possible ways are 3+3+4, 1+4+1+4 and 1+1+6+1+1.
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print the number of ways modulo 10<sup>9</sup> + 7.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>18</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let mut exponent = scan.token::<u64>();

    let mut base: Matrix = [
        [1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 1, 0],
    ];
    // identity matrix
    let mut goal: Matrix = [
        [1, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 1],
    ];

    while exponent != 0 {
        if exponent & 1 == 1 {
            goal = multiply_matrix(&goal, &base);
        }
        base = multiply_matrix(&base, &base);
        exponent >>= 1;
    }

    writeln!(out, "{}", goal[0][0]).ok();
}

fn multiply_matrix(a: &Matrix, b: &Matrix) -> Matrix {
    let mut ret: Matrix = [[0; 6]; 6];

    for i in 0..6 {
        for j in 0..6 {
            ret[i][j] = ((0..6).map(|k| a[i][k] * b[k][j]).sum::<u64>()) % MODULO;
        }
    }

    ret
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
8
";
        let target = b"\
125
";

        test(input, target);
    }

    #[test]
    fn test_dp() {
        let input = b"\
10
";
        let target = b"\
492
";

        test(input, target);
    }

    #[test]
    fn test_one_million() {
        let input = b"\
1000000
";
        let target = b"\
874273980
";

        test(input, target);
    }

    #[test]
    fn test_max() {
        let input = b"\
1000000000000000000
";
        let target = b"\
106021903
";

        test(input, target);
    }
}
