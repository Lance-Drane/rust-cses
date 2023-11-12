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

/// A Gray code is a list of all 2<sup>n</sup bit strings of length n, where any two successive strings differ in exactly one bit (i.e., their Hamming distance is one).
///
/// Your task is to create a Gray code for a given length n.
///
/// <b>Input</b>
///
/// The only input line has an integer n.
///
/// <b>Output</b>
///
/// Print 2<sup>n</sup> lines that describe the Gray code. You can print any valid solution.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ A,B ≤ 16</li>
/// </ul>
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let exponent = scan.token::<usize>();

    for num in 0..(1 << exponent) {
        writeln!(out, "{:0exponent$b}", num ^ (num >> 1)).ok();
    }
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
2
";
        let target = b"\
00
01
11
10
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
5
";
        let target = b"\
00000
00001
00011
00010
00110
00111
00101
00100
01100
01101
01111
01110
01010
01011
01001
01000
11000
11001
11011
11010
11110
11111
11101
11100
10100
10101
10111
10110
10010
10011
10001
10000
";

        test(input, target);
    }
}
